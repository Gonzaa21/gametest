use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use crate::game::card::component::{Card, CardPosition, CardHandles, CardBack, Suit, Selected};
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


// card selection system
pub fn card_selection(
    mut commands: Commands,
    mouse_input: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    card_query: Query<(Entity, &Transform), With<Card>>,
    selected_query: Query<Entity, With<Selected>>,
) {
    // if mouse input is not pressed
    if !mouse_input.just_pressed(MouseButton::Left) {
        return;
    }

    // obtain window and camera
    let Ok(window) = windows.single() else { return; };
    let Ok((camera, camera_transform)) = camera_query.single() else { return; };

    // obtain cursor position
    if let Some(cursor_pos) = window.cursor_position() {
        // world coords
        if let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_pos) {
            
            // iterating for find clicked card
            for (card_entity, card_transform) in card_query.iter() {
                let card_pos = card_transform.translation;
                let card_size = Vec2::new(80.0, 120.0);
                
                // Check if the click is inside the card
                if world_pos.x >= card_pos.x - card_size.x / 2.0 
                && world_pos.x <= card_pos.x + card_size.x / 2.0
                && world_pos.y >= card_pos.y - card_size.y / 2.0
                && world_pos.y <= card_pos.y + card_size.y / 2.0 {
                    
                    // deselect all cards
                    for selected_entity in selected_query.iter() {
                        commands.entity(selected_entity).remove::<Selected>();
                    }
                    
                    // select only this card
                    commands.entity(card_entity).insert(Selected);
                    info!(target: "mygame", "Card clicked: {:?} at {:?}", card_entity, card_pos);
                    break;
                }
            }
        }
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