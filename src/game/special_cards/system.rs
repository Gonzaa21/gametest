use bevy::prelude::*;
use crate::game::{card::component::{Card, CardPosition}, turn_player::component::Turn};
use crate::game::special_cards::resource::{SpecialCardEffect, SpecialEffect};

pub fn detect_special_card(
    mut commands: Commands,
    card_query: Query<(Entity, &Card)>,
    keyboard: Res<ButtonInput<KeyCode>>,
    turn_query: Res<Turn>,
    special_effect: Option<Res<SpecialCardEffect>>,
) {
    // verify if have effect
    if special_effect.as_ref().map_or(false, |s| s.awaiting_target) {
        return;
    }

    // verify if the key was pressed to activate special card
    if !keyboard.just_pressed(KeyCode::KeyE) { return; }

    // obtain current drawn card
    let drawn_card = card_query.iter()
        .find(|(_, card)| {
            matches!(card.position, CardPosition::DrawnCard(player_id) 
                 if player_id == turn_query.current_player)
                 && card.from_deck
        });

    if let Some((card_entity, card)) = drawn_card {
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
            let new_effect = SpecialCardEffect {
                card_entity: Some(card_entity),
                effect_type: Some(effect.clone()),
                awaiting_target: matches!(effect, SpecialEffect::Shuffle | SpecialEffect::Swap),
                target_player: None,
                target_card: None,
                awaiting_own_card: matches!(effect, SpecialEffect::Swap),
                own_card: None
            };
            commands.insert_resource(new_effect);
            return;
        }
    } else {
        info!(target: "mygame", "No special cards from deck available (cards from graveyard lose their effect)");
    }
    info!(target: "mygame", "No face-up special cards available");
}

pub fn handle_special_effects(
    special_effect: Option<ResMut<SpecialCardEffect>>,
) {
    // run if resource exist
    if let Some(effect) = special_effect {
        if let Some(effect_type) = &effect.effect_type.clone() {
            match effect_type {
                SpecialEffect::Reveal => {
                    info!(target: "mygame", "Reveal effect active");
                },
                SpecialEffect::Shuffle => {
                    if effect.awaiting_target {
                        info!(target: "mygame", "Waiting for target selection for shuffle effect...");
                    }
                },
                SpecialEffect::Swap => {
                    if effect.awaiting_target {
                        info!(target: "mygame", "Waiting for target selection for swap effect...");
                    }
                },
            }
        }
    }
}