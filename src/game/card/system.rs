use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use crate::game::card::component::{Card, CardPosition, CardHandles, CardBack, Suit, Selected, DoubleClick};
use crate::game::{graveyard::component::Graveyard, turn_player::component::Turn, deck::component::Deck, hand::component::Hand, player::component::Player};
use crate::game::card::handles::{handle_deck_click, handle_card_click, handle_graveyard_click};
use bevy::asset::Assets;
use bevy::image::{Image, ImageSampler};
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
    let card_handles: Vec<Handle<Image>> = card_routes
        .into_iter()
        .map(|route| asset_server.load(route))
        .collect();

    // card back texture
    let card_back_handle = asset_server.load("textures/deck/back/back.PNG");

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

// card_selection - detect clicks
//      detect_card_click
//          handle_card_click - manage cards
//              discard_card()
//              card_swap()
//      detect_deck_click
//          handle_deck_click - draw card of the deck
//      detect_graveyard_click
//          handle_graveyard_click - draw of the graveyard

// card selection system
pub fn card_selection(
    mut commands: Commands,
    mouse_input: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    mut card_query: Query<(Entity, &mut Transform, &mut Card), With<Card>>,
    selected_query: Query<Entity, With<Selected>>,
    mut double_click: ResMut<DoubleClick>,
    time: Res<Time>,
    turn_query: ResMut<Turn>,
    mut hand_query: Query<&mut Hand>,
    mut graveyard_query: Query<&mut Graveyard>,
    deck_query: Query<&mut Deck>,
    player_query: Query<(Entity, &Player)>,
) {
    if !mouse_input.just_pressed(MouseButton::Left) {
        return;
    }

    // obtain window, camera, cursor/world position
    let Ok(window) = windows.single() else { return; };
    let Ok((camera, camera_transform)) = camera_query.single() else { return; };
    let Some(cursor_pos) = window.cursor_position() else { return; };
    let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_pos) else { return; };

    // detect click in deck
    if detect_deck_click(world_pos) {
        handle_deck_click(deck_query, turn_query, &mut card_query);
        return;
    }

    // detect click in graveyard 
    if detect_graveyard_click(world_pos) {
        handle_graveyard_click(graveyard_query, turn_query, &mut card_query);
        return;
    }

    // detect click in hand
    if let Some(clicked_entity) = detect_card_click(&card_query, world_pos) {
        handle_card_click(
            clicked_entity, &mut commands, &selected_query, &mut double_click,
            &time, &turn_query, &mut card_query, &mut graveyard_query, &player_query, &mut hand_query
        );
        return;
    }
}

// feedback visual when selecting
pub fn card_visual(
    mut card_query: Query<(&mut Transform, Option<&Selected>, &Card), With<Card>>

) {
    for (mut transform, selected, card) in card_query.iter_mut() {
        if let CardPosition::Hand(_) = card.position {
            if selected.is_some() {
                transform.translation.y = -300.0; // raise position
                transform.translation.z = 50.0; 
            } else {
                if transform.translation.y > -310.0 {
                    transform.translation.y = -320.0; // normal position
                    transform.translation.z = 10.0;
                }
            }
        }
    }
}

pub fn configure_texture(
    mut images: ResMut<Assets<Image>>,
    card_handles: Option<Res<CardHandles>>,
    card_back: Option<Res<CardBack>>,
) {
    let Some(card_handles) = card_handles else { return; };
    let Some(card_back) = card_back else { return; };
    
    // front cards
    for handle in &card_handles.0 {
        if let Some(image) = images.get_mut(handle) {
            image.sampler = ImageSampler::nearest();
        }
    }
    
    // back card
    if let Some(image) = images.get_mut(&card_back.0) {
        image.sampler = ImageSampler::nearest();
    }
}

// DETECTION CLICK SYSTEMS
fn detect_card_click(
    card_query: &Query<(Entity, &mut Transform, &mut Card), With<Card>>,
    world_pos: Vec2
) -> Option<Entity> {
    for (card_entity, card_transform, _card_comp) in card_query.iter() {
        let card_pos = card_transform.translation;
        let card_size = Vec2::new(80.0, 120.0);
        
        // Check if the click is inside the card
        if world_pos.x >= card_pos.x - card_size.x / 2.0 
        && world_pos.x <= card_pos.x + card_size.x / 2.0
        && world_pos.y >= card_pos.y - card_size.y / 2.0
        && world_pos.y <= card_pos.y + card_size.y / 2.0 {
            return Some(card_entity);
        }
    }
    None
}

fn detect_deck_click(world_pos: Vec2) -> bool {
    let deck_pos = Vec3::new(150.0, 50.0, 0.0);
    let deck_size = Vec2::new(80.0, 120.0);
    
    world_pos.x >= deck_pos.x - deck_size.x / 2.0 
    && world_pos.x <= deck_pos.x + deck_size.x / 2.0
    && world_pos.y >= deck_pos.y - deck_size.y / 2.0
    && world_pos.y <= deck_pos.y + deck_size.y / 2.0
}

fn detect_graveyard_click(world_pos: Vec2) -> bool {
    let graveyard_pos = Vec3::new(-150.0, 50.0, 0.0);
    let graveyard_size = Vec2::new(80.0, 120.0);
    
    world_pos.x >= graveyard_pos.x - graveyard_size.x / 2.0 
    && world_pos.x <= graveyard_pos.x + graveyard_size.x / 2.0
    && world_pos.y >= graveyard_pos.y - graveyard_size.y / 2.0
    && world_pos.y <= graveyard_pos.y + graveyard_size.y / 2.0
}