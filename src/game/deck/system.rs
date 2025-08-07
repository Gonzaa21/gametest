use bevy::prelude::*;
use rand::seq::SliceRandom;

use super::component::Deck;
use crate::game::card::{component::{Card, CardPosition}};

pub fn spawn_cards(mut commands: Commands) {
    let mut rng = rand::rng();
    let mut card_values: Vec<u8> = (1..=12).flat_map(|n| std::iter::repeat(n).take(4)).collect(); // 12 values x4
    card_values.shuffle(&mut rng); // randomize cards

    // spawn card entities and save in Vec
    let mut card_entities = Vec::new();
    for value in card_values {
        let card_entity = commands.spawn(Card {
            value,
            face_up: false,
            owner_id: None,
            position: CardPosition::Deck
        }).id();
        card_entities.push(card_entity);
    }

    // spawn deck entity
    commands.spawn(Deck {
        cards_values: card_entities,
    });
}