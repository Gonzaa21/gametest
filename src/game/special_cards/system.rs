use bevy::prelude::*;
use crate::game::{card::component::{Card, CardPosition}, deck::component::Deck, player::component::Player, hand::component::Hand, turn_player::component::Turn};
use crate::game::special_cards::resource::{SpecialCardEffect, SpecialEffect};

pub fn detect_special_card(
    mut commands: Commands,
    card_query: Query<&Card>,
    deck_query: Query<&Deck>,
    keyboard: Res<ButtonInput<KeyCode>>,
    turn_query: Res<Turn>,
    hand_query: Query<&Hand>,
    player_query: Query<(Entity, &Player)>,
    special_effect: Option<ResMut<SpecialCardEffect>>,
) {
    // verify if have effect
    if special_effect.as_ref().map_or(false, |s| s.awaiting_target) {
        return;
    }

    // verify if the key was pressed to activate special card
    let key_pressed = keyboard.just_pressed(KeyCode::KeyE);
    if !key_pressed {return;}

    // obtain current drawn card
    let drawn_card = card_query.iter()
        .find(|card| {
            matches!(card.position, CardPosition::DrawnCard(player_id) 
                     if player_id == turn_query.current_player)
        });

    if let Some(card) = drawn_card {
        // match if drawn card is special card
        let special_effect_type = match card.value {
            11 => Some(SpecialEffect::Shuffle),
            9 => Some(SpecialEffect::Reveal), 
            7 => Some(SpecialEffect::Swap),
            _ => None,
        };
        
        if let Some(effect) = special_effect_type {
            info!(target: "mygame", "Special card {} activated! Effect: {:?}", card.value, effect);
            
            // create/update special effect
            match special_effect {
                Some(mut se) => {
                    se.card_entity = Some(card_entity);
                    se.effect_type = Some(effect.clone());
                    se.awaiting_target = matches!(effect, SpecialEffect::Shuffle | SpecialEffect::Swap);
                },
                None => {
                    commands.insert_resource(SpecialCardEffect {
                        card_entity: Some(card_entity),
                        effect_type: Some(effect.clone()),
                        awaiting_target: matches!(effect, SpecialEffect::Shuffle | SpecialEffect::Swap),
                    });
                }
            }
            
            if matches!(effect, SpecialEffect::Reveal) {
                info!(target: "mygame", "Reveal effect will be applied immediately");
            } else {
                info!(target: "mygame", "Select target for special effect (click on target)");
            }

            return;
        }
    }
    info!(target: "mygame", "No face-up special cards available");
}