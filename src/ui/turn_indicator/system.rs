use bevy::prelude::*;
use crate::ui::turn_indicator::component::CardOpacity;
use crate::game::card::component::{Card, CardPosition};
use crate::game::turn_player::component::Turn;

// add CardOpacity component to cards
pub fn add_card_opacity(
    mut commands: Commands,
    card_query: Query<Entity, (With<Card> ,Without<CardOpacity>)>,
) {
    for card_entity in card_query.iter() {
            commands.entity(card_entity).insert(CardOpacity::default());
    }
}

// read actual turn and update opacity target
pub fn update_turn_indicator(
    mut card_query: Query<(&Card, &mut CardOpacity)>,
    turn_query: Res<Turn>,
) {
    for (card, mut opacity) in card_query.iter_mut() {
        if let CardPosition::Hand(hand_owner) = card.position {
            if hand_owner == turn_query.current_player {
                // if is turn of player
                opacity.target = 1.0
            } else {
                // if is not turn of player
                opacity.target = 0.2
            }
        } else {
            // card is not in hand
            opacity.target = 1.0
        }
    }
}

// opacity animation
pub fn animation_opacity(
    mut card_query: Query<(&mut CardOpacity, &mut Sprite)>,
    time: Res<Time>
) {
    for (mut opacity, mut sprite) in card_query.iter_mut() {
        let result = opacity.target - opacity.current;

        if result.abs() > 0.01 {
            // update current and sprite
            opacity.current += result * opacity.transition_speed * time.delta_secs();
            sprite.color.set_alpha(opacity.current);
        } else {
            opacity.current = opacity.target;
        }
    }
}