use bevy::prelude::*;
use super::component::Hand;

use crate::game::deck::component::Deck;
use crate::game::player::component::Player;
use crate::game::card::component::{Card, CardPosition};

pub fn deal_initial_hands (
    mut commands: Commands,
    mut deck_query: Query<&mut Deck, With<Deck>>,
    player_query: Query<Entity, With<Player>>,
    mut card_query: Query<&mut Card>,
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
    for player_entity in player_query.iter() {
        if deck.cards_values.len() < 4 {
            warn!(target: "mygame","⚠️ There are not enough cards for this player");
            continue;
        }

        // take 4 deck cards
        let hand_cards: Vec<Entity> = deck.cards_values.drain(0..4).collect();

        // iterate deck cards and distribute it to players
        for (i, &card_entity) in hand_cards.iter().enumerate() {
            if let Ok(mut card) = card_query.get_mut(card_entity) {
                card.owner_id = Some(player_entity);
                card.position = CardPosition::Hand(player_entity);
                card.face_up = i < 2; // only two can show
            }
        }

        commands.spawn(Hand{
            cards: hand_cards
        });
    }
}