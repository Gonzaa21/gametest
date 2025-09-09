use bevy::prelude::*;
use rand::{Rng, seq::SliceRandom};
use crate::game::{special_cards::resource::{SpecialCardEffect, SpecialEffect}, card::component::Card, hand::component::Hand, player::component::Player, turn_player::component::Turn};

pub fn reveal_effect(
    special_effect: Option<ResMut<SpecialCardEffect>>,
    mut card_query: Query<&mut Card>,
    hand_query: Query<&Hand>,
    player_query: Query<(Entity, &Player)>,
    turn_query: Res<Turn>,
) {
    // run if effect type is reveal
    let Some(mut special_effect) = special_effect else { return; };
    if !matches!(special_effect.effect_type, Some(SpecialEffect::Reveal)) {
        return;
    }
    
    if let Some((_, player)) = player_query.iter().find(|(entity, _)| *entity == turn_query.current_player) {
        if let Ok(hand) = hand_query.get(player.hand) {
            
            // find one random card with face_up = false to return true
            let face_down_cards: Vec<Entity> = hand.cards.iter()
                .filter_map(|&card_entity| {
                    card_query.get(card_entity).ok()
                        .filter(|card| !card.face_up)
                        .map(|_| card_entity)
                })
                .collect();

            if !face_down_cards.is_empty() {
                let mut rng = rand::rng();
                let random_index = rng.random_range(0..face_down_cards.len());
                let selected_card = face_down_cards[random_index];
                
                if let Ok(mut card) = card_query.get_mut(selected_card) {
                    card.face_up = true;
                    info!(target: "mygame", "Revealed card: {} of {:?}", card.value, card.suit);
                }
            }

            // reset from_deck so that the effect is used only once
            if let Some(special_card_entity) = special_effect.card_entity {
                if let Ok(mut special_card) = card_query.get_mut(special_card_entity) {
                    special_card.from_deck = false;
                }
            }
        }
    }
    // clean effect
    *special_effect = SpecialCardEffect::default();
    info!(target: "mygame", "Effect completed");
}

pub fn shuffle_effect(
    special_effect: Option<ResMut<SpecialCardEffect>>,
    mut card_query: Query<(Entity, &mut Transform, &mut Card), With<Card>>,
    hand_query: Query<&Hand>,
    player_query: Query<(Entity, &Player)>,
) {
    // run if effect type is shuffle
    if let Some(mut effect) = special_effect {
        if matches!(effect.effect_type, Some(SpecialEffect::Shuffle)) {
            
            // verify do not expecting target
            if effect.awaiting_target { return; }

            // verify have target_player
            let Some(target_player_id) = effect.target_player else {
                *effect = SpecialCardEffect::default();
                return;
            };

            // find target player
            let target_player = player_query.iter()
                .find(|(entity, _)| *entity == target_player_id);
    
            let Some((_, player)) = target_player else {
                info!(target: "mygame", "Target player not found");
                *effect = SpecialCardEffect::default();
                return;
            };

            // find target player hand
            let Ok(hand) = hand_query.get(player.hand) else {
                info!(target: "mygame", "Target player hand not found");
                *effect = SpecialCardEffect::default();
                return;
            };

            // verify if have 4 cards in hand
            if hand.cards.len() != 4 {
                info!(target: "mygame", "Target player doesn't have 4 cards");
                *effect = SpecialCardEffect::default();
                return;
            };

            // verify card positions
            let mut positions: Vec<Vec3> = Vec::new();
            for &card_entity in &hand.cards {
                if let Ok((_, transform, mut card)) = card_query.get_mut(card_entity) {
                    positions.push(transform.translation); // push to Vec (positions)
                    card.face_up = false;
                }
            }
            
            // randomize positions
            let mut rng = rand::rng();
            positions.shuffle(&mut rng);

            // update card positions
            for (i, &card_entity) in hand.cards.iter().enumerate() {
               if let Ok((_, mut transform, _)) = card_query.get_mut(card_entity) {
                   transform.translation = positions[i];
               }
            }

            // reset from_deck so that the effect is used only once
            if let Some(special_card_entity) = effect.card_entity {
                if let Ok(mut special_card) = card_query.get_mut(special_card_entity) {
                    special_card.2.from_deck = false;
                }
            }

            info!(target: "mygame", "Cards shuffled for target player!");

            *effect = SpecialCardEffect::default();
            info!(target: "mygame", "Effect completed");
        }
    }
}

pub fn swap_effect(
    special_effect: Option<ResMut<SpecialCardEffect>>,
    mut card_query: Query<(Entity, &mut Transform, &mut Card), With<Card>>,
    mut hand_query: Query<&mut Hand>,
    player_query: Query<(Entity, &Player)>,
) {
    // run if effect type is swap
    if let Some(mut effect) = special_effect {
        if matches!(effect.effect_type, Some(SpecialEffect::Swap)) {
            
            // verify if have target_card (rival card) and own_card (own card)
            if let (Some(target_card_entity), Some(own_card_entity)) = (effect.target_card, effect.own_card) {
                if let Ok([(_, mut target_transform, mut target_card), (_, mut own_transform, mut own_card)]) =
                    card_query.get_many_mut([target_card_entity, own_card_entity]) {
                    
                    // obtain positions and owner id
                    let target_pos = target_transform.translation;
                    let own_pos = own_transform.translation;
                    let target_id = target_card.owner_id;
                    let own_id = own_card.owner_id;

                    // exchange positions, owner ids and no face_up
                    target_transform.translation = own_pos;
                    target_card.owner_id = own_id;
                    target_card.face_up = false;
                    own_transform.translation = target_pos;
                    own_card.owner_id = target_id;
                    own_card.face_up = false;

                    // update and save changes of player hand with target_card
                    if let Some((_, target_player)) = player_query.iter().find(|(entity, _)| Some(*entity) == target_id) {
                        if let Ok(mut target_hand) = hand_query.get_mut(target_player.hand) {
                            // remove target_card => add own_card
                            target_hand.cards.retain(|&card_entity| card_entity != target_card_entity);
                            target_hand.cards.push(own_card_entity);
                        }
                    }

                    // update and save changes of player hand with own_card
                    if let Some((_, own_player)) = player_query.iter().find(|(entity, _)| Some(*entity) == own_id) {
                        if let Ok(mut own_hand) = hand_query.get_mut(own_player.hand) {
                            // remove own_card => add target_card
                            own_hand.cards.retain(|&card_entity| card_entity != own_card_entity);
                            own_hand.cards.push(target_card_entity);
                        }
                    }
                    
                    info!(target: "mygame", "Card swap completed: {:?} -> {:?}", target_card_entity, own_card_entity);
                }
            } else {
                info!(target: "mygame", "One more card needs to be selected");
                return;
            }

            // reset from_deck so that the effect is used only once
            if let Some(special_card_entity) = effect.card_entity {
                if let Ok((_, _, mut special_card)) = card_query.get_mut(special_card_entity) {
                    special_card.from_deck = false;
                }
            }

            *effect = SpecialCardEffect::default();
            info!(target: "mygame", "Effect completed");
        }
    }
}