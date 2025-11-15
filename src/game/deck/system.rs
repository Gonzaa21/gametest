use bevy::prelude::*;
use rand::seq::SliceRandom;
use bevy::window::PrimaryWindow;

use super::component::Deck;
use crate::game::{card::component::{Card, CardBack, CardHandles, CardPosition, Suit}, gamestate::GameEntity};

pub fn spawn_cards(mut commands: Commands, card_handles: Option<Res<CardHandles>>, card_back: Option<Res<CardBack>>, windows: Query<&Window, With<PrimaryWindow>>,) {
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

    // access CardHandles and CardBack
    let Some(card_back) = card_back else { return; };
    let Some(card_handles) = card_handles else { return; };

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

        // offsets
        let stack_index = card_entities.len() as f32;  // use card spawned index
        let max_stack_effect = 15.0;  // first 15 cards add offset

        // Apply offset only if it is one of the first cards
        let (stack_offset_x, stack_offset_y) = if stack_index < max_stack_effect {
            (stack_index * 0.4, stack_index * 0.3)
        } else {
            // others cards without offset
            (max_stack_effect * 0.4, max_stack_effect * 0.3)
        };

        let card_entity = commands.spawn((
            Sprite::from_image(card_back.0.clone()),
            Transform::from_xyz(
                window.width() * 0.15 + stack_offset_x, 
                window.height() * 0.0 + stack_offset_y, 
                idx as f32
            ).with_scale(Vec3::splat(0.7)),
            Card {
                suit,
                value,
                face_up: false,
                owner_id: None,
                position: CardPosition::Deck,
                front_face: handle.clone(),
                from_deck: false,
                is_being_dealt: false,
            },
            GameEntity,
        )).id();

        card_entities.push(card_entity);
    }

    // spawn deck entity
    commands.spawn((
        Deck {
            cards_values: card_entities
        },
        GameEntity,
    ));
}