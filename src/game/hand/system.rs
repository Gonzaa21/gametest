use bevy::prelude::*;
use super::component::Hand;

use crate::game::deck::component::Deck;
use crate::game::player::component::Player;
use crate::game::card::component::{Card, CardPosition};

pub fn deal_initial_hands (
    mut deck_query: Query<&mut Deck, With<Deck>>,
    player_query: Query<(Entity, &Player), With<Player>>,
    mut card_query: Query<(&mut Card, &mut Transform)>,
    mut hand_query: Query<&mut Hand>,
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

        // player positions
        let positions = match i {
            0 => get_player_positions(0), // local player
            1 => get_player_positions(1), // secondary player
            _ => get_player_positions(0), // default
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
fn get_player_positions(player_i: usize) -> [Vec3; 4] {
    match player_i {
        0 => {
            let base_y = -320.0;
            let base_x = -180.0;
            let gap = 120.0;
            [
                Vec3::new(base_x, base_y, 10.0),
                Vec3::new(base_x + gap, base_y, 11.0),
                Vec3::new(base_x + gap * 2.0, base_y, 12.0),
                Vec3::new(base_x + gap * 3.0, base_y, 13.0),
            ]
        },
        1 => {
            let base_y = 200.0;
            let base_x = -180.0;
            let gap = 120.0;
            [
                Vec3::new(base_x, base_y, 10.0),
                Vec3::new(base_x + gap, base_y, 11.0),
                Vec3::new(base_x + gap * 2.0, base_y, 12.0),
                Vec3::new(base_x + gap * 3.0, base_y, 13.0),
            ]
        },
        _ => get_player_positions(0)
    }
}