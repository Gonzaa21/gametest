use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use super::component::Hand;

use crate::game::deck::component::Deck;
use crate::game::player::component::Player;
use crate::game::card::component::{Card, CardPosition};

pub fn deal_initial_hands (
    mut deck_query: Query<&mut Deck, With<Deck>>,
    player_query: Query<(Entity, &Player), With<Player>>,
    mut card_query: Query<(&mut Card, &mut Transform)>,
    mut hand_query: Query<&mut Hand>,
    windows: Query<&Window, With<PrimaryWindow>>,
) {
    // search deck
    let mut deck = match deck_query.single_mut() {
        Ok(d) => d,
        Err(_) => {
            error!(target: "mygame","❌ deck not founded.");
            return;
        }
    };

    // verify if player have 4 cards
    for (i, (player_entity, player_component)) in player_query.iter().enumerate() {
        if deck.cards_values.len() < 4 {
            warn!(target: "mygame","⚠️ There are not enough cards for this player");
            continue;
        }
        // take 4 deck cards
        let hand_cards: Vec<Entity> = deck.cards_values.drain(0..4).collect();

        // obtain window dimensions
        let Ok(window) = windows.single() else { return; };

        // player positions
        let positions = match i {
            0 => get_player_positions(0, window.width(), window.height()),
            1 => get_player_positions(1, window.width(), window.height()),
            _ => get_player_positions(0, window.width(), window.height()),// default
        };

        // iterate deck cards and distribute it to players
        for (j, &card_e) in hand_cards.iter().enumerate() {
            if let Ok((mut card, mut tf)) = card_query.get_mut(card_e) {
                card.owner_id = Some(player_entity);
                card.position = CardPosition::Hand(player_entity);
                card.face_up  = i == 0 && j < 2;
            
                tf.translation = positions[j];
            }
        }

        if let Ok(mut hand) = hand_query.get_mut(player_component.hand) {
            hand.cards = hand_cards;
        }
    }
}

// player positions auxiliar system
fn get_player_positions(player_i: usize, window_width: f32, window_height: f32) -> [Vec3; 4] {
    match player_i {
        0 => {
            let base_y = window_height * -0.35;  // 40% down
            let base_x = window_width * -0.13;  // 13% left
            let gap = window_width * 0.1;       // 10% win width
            [
                Vec3::new(base_x, base_y, 10.0),
                Vec3::new(base_x + gap, base_y, 11.0),
                Vec3::new(base_x + gap * 2.0, base_y, 12.0),
                Vec3::new(base_x + gap * 3.0, base_y, 13.0),
            ]
        },
        1 => {
            let base_y = window_height * 0.35;  // 25% win height
            let base_x = window_width * -0.13;
            let gap = window_width * 0.1;
            [
                Vec3::new(base_x, base_y, 10.0),
                Vec3::new(base_x + gap, base_y, 11.0),
                Vec3::new(base_x + gap * 2.0, base_y, 12.0),
                Vec3::new(base_x + gap * 3.0, base_y, 13.0),
            ]
        },
        _ => get_player_positions(0, window_width, window_height)
    }
}