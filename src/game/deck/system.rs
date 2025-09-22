use bevy::prelude::*;
use rand::seq::SliceRandom;
use bevy::window::PrimaryWindow;

use super::component::Deck;
use crate::game::card::{component::{Card, CardPosition, Suit, CardHandles, CardBack}};

pub fn spawn_cards(mut commands: Commands, card_handles: Res<CardHandles>, card_back: Res<CardBack>, windows: Query<&Window, With<PrimaryWindow>>,) {
    let suits = [Suit::Coarse, Suit::Cup, Suit::Gold, Suit::Sword];

    // generate all combinations (suit-value)
    let mut cards: Vec<(Suit, u8)> = suits
        .iter()
        .flat_map(|suit| (1..=12).map(move |value| (suit.clone(), value)))
        .collect();

    // randomize cards
    let mut rng = rand::rng();
    cards.shuffle(&mut rng);

    // spawn card entities and save in Vec
    let mut card_entities = Vec::new();
    for (suit, value) in cards {

        let suit_idx = match suit {
            Suit::Coarse => 0,
            Suit::Cup    => 1,
            Suit::Gold   => 2,
            Suit::Sword  => 3,
        };
        let idx = suit_idx * 12 + (value as usize - 1);
        let front = card_handles.0[idx].clone();
        let handle = front;

        // obtain window
        let Ok(window) = windows.single() else { 
            warn!("No primary window found");
            return; 
        };

        let card_entity = commands.spawn((
            Sprite::from_image(card_back.0.clone()),
            Transform::from_xyz(window.width() * 0.25, window.height() * 0.08, idx as f32).with_scale(Vec3::splat(0.8)),
            Card {
                suit,
                value,
                face_up: false,
                owner_id: None,
                position: CardPosition::Deck,
                front_face: handle.clone(),
                from_deck: false
        })).id();

        card_entities.push(card_entity);
    }

    // spawn deck entity
    commands.spawn(Deck {
        cards_values: card_entities,
    });
}