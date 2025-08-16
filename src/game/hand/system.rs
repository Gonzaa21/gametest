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
    for (player_entity, player_component) in player_query.iter() {
        if deck.cards_values.len() < 4 {
            warn!(target: "mygame","⚠️ There are not enough cards for this player");
            continue;
        }

        // take 4 deck cards
        let hand_cards: Vec<Entity> = deck.cards_values.drain(0..4).collect();

        let base_y = -320.0;
        let base_x = -180.0; // main point
        let gap    = 120.0;  // separation between cards

        // iterate deck cards and distribute it to players
        for (i, &card_e) in hand_cards.iter().enumerate() {
            if let Ok((mut card, mut tf)) = card_query.get_mut(card_e) {
                card.owner_id = Some(player_entity);
                card.position = CardPosition::Hand(player_entity);
                card.face_up  = i < 2;
            
                tf.translation = Vec3::new(base_x + i as f32 * gap, base_y, 10.0 + i as f32);
            }
        }

        if let Ok(mut hand) = hand_query.get_mut(player_component.hand) {
            hand.cards = hand_cards;
        }
    }
}