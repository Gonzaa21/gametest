use bevy::prelude::*;
use crate::game::card::component::{Card, CardHandles, CardBack, Suit};
use rand::seq::SliceRandom;
use rand::rng;

// spawn card values after setup and allocate suit/value and textures
// iterate and mix all cards front/back
pub fn setup_cards(mut commands: Commands, asset_server: Res<AssetServer>) {
    let suits = ["coarse", "cup", "gold", "sword"];
    let mut card_routes = Vec::new();

    for suit_name in suits.iter() {
        for value in 1..=12 {
            let route = format!("textures/deck/{}/{}.PNG", suit_name, value);
            card_routes.push(route);
        }
    }

    // load textures
    let mut card_handles: Vec<Handle<Image>> = card_routes
        .into_iter()
        .map(|route| asset_server.load(route))
        .collect();

    // card back texture
    let card_back_handle = asset_server.load("textures/deck/back/back.PNG");
    card_handles.shuffle(&mut rng());

    // insert handles resources
    commands.insert_resource(CardHandles(card_handles.clone()));
    commands.insert_resource(CardBack(card_back_handle));

    let suits_logic = [Suit::Coarse, Suit::Cup, Suit::Gold, Suit::Sword];
    let mut cards_data: Vec<(Suit, u8)> = suits_logic
        .iter()
        .flat_map(|suit| (1..=12).map(move |value| (suit.clone(), value)))
        .collect();

    cards_data.shuffle(&mut rng());
}


pub fn card_face(
    card_back: Res<CardBack>,
    mut query: Query<(&Card, &mut Sprite)>
) {
    for (card, mut sprite) in query.iter_mut() {
        if card.face_up {
            // asign front
            sprite.image = card.front_face.clone();
        } else {
            // Asign back
            sprite.image = card_back.0.clone();
        }
    }
}