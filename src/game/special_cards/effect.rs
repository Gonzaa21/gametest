use bevy::prelude::*;
use crate::game::{special_cards::resource::SpecialCardEffect, card::component::Card, hand::component::Hand, player::component::Player, turn_player::component::Turn};

fn execute_reveal_effect(
    commands: &mut Commands,
    special_effect: &mut ResMut<SpecialCardEffect>,
    card_query: &mut Query<&mut Card>,
    hand_query: &Query<&Hand>,
    player_query: &Query<(Entity, &Player)>,
    turn_query: &Res<Turn>,
) {
    // find card with face_up = false to return true
    if let Some((_, player)) = player_query.iter().find(|(entity, _)| *entity == turn_query.current_player) {
        if let Ok(hand) = hand_query.get(player.hand) {
            for &card_entity in &hand.cards {
                if let Ok(mut card) = card_query.get_mut(card_entity) {
                    if !card.face_up {
                        card.face_up = true;
                        info!(target: "mygame", "Revealed card: {} of {:?}", card.value, card.suit);
                        
                        // clean effect
                        special_effect.card_entity = None;
                        special_effect.effect_type = None;
                        special_effect.awaiting_target = false;
                        return;
                    }
                }
            }
            info!(target: "mygame", "No face-down cards to reveal");
        }
    }
    
    // clean effect if it couldn't be executed
    special_effect.card_entity = None;
    special_effect.effect_type = None;
    special_effect.awaiting_target = false;
}