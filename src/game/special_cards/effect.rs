use bevy::prelude::*;
use rand::Rng;
use crate::game::{special_cards::resource::{SpecialCardEffect, SpecialEffect}, card::component::Card, hand::component::Hand, player::component::Player, turn_player::component::Turn};

pub fn reveal_effect(
    mut special_effect: ResMut<SpecialCardEffect>,
    mut card_query: Query<&mut Card>,
    hand_query: Query<&Hand>,
    player_query: Query<(Entity, &Player)>,
    turn_query: Res<Turn>,
) {
    // run if effect type is reveal    
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
        }
    }
    
    // clean effect
    *special_effect = SpecialCardEffect::default();
    info!(target: "mygame", "Effect completed");
}

pub fn shuffle_effect(
    special_effect: Option<ResMut<SpecialCardEffect>>,
) {
    // run if effect type is shuffle
    if let Some(mut effect) = special_effect {
        if matches!(effect.effect_type, Some(SpecialEffect::Shuffle)) {
            
            info!(target: "mygame", "Shuffle effect would execute here");
            *effect = SpecialCardEffect::default();
        }
    }
}

pub fn swap_effect(
    special_effect: Option<ResMut<SpecialCardEffect>>,
) {
    // run if effect type is swap
    if let Some(mut effect) = special_effect {
        if matches!(effect.effect_type, Some(SpecialEffect::Swap)) {
            
            info!(target: "mygame", "Swap effect would execute here");
            *effect = SpecialCardEffect::default();
        }
    }
}