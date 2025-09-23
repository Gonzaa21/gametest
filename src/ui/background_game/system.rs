use bevy::prelude::*;
use bevy::window::{PrimaryWindow, WindowResized};
use crate::ui::background_game::component::BackgroundImage;
use crate::game::{card::component::{Card, CardPosition}, player::component::Player, hand::component::Hand, graveyard::component::Graveyard};
use crate::game::hand::system::get_player_positions;

pub fn spawn_background(
    mut commands: Commands, 
    asset_server: Res<AssetServer>
) {
    let image_handle: Handle<Image> = asset_server.load("textures/ui/background/background_game.png");
    
    // Spawn with default scale
    commands.spawn((
        Sprite::from_image(image_handle.clone()),
        Transform::from_xyz(0.0, 0.0, -10.0),
        BackgroundImage(image_handle),
    ));
}

pub fn adjust_background(
    mut bg_query: Query<(&mut Transform, &BackgroundImage)>,
    window: Query<&Window, With<PrimaryWindow>>,
    images: Res<Assets<Image>>,
    mut resize_events: EventReader<WindowResized>,
) {
    // adjust background resize if have event
    for _resize_event in resize_events.read() {
        // adjust background scale
        for (mut transform, bg_image) in bg_query.iter_mut() {
            if let Some(img) = images.get(&bg_image.0) {
                let Ok(window) = window.single() else { continue; };

                let image_width = img.size().x as f32;
                let image_height = img.size().y as f32;
                let scale_x = window.width() / image_width;
                let scale_y = window.height() / image_height;
                let final_scale = scale_x.min(scale_y);

                transform.scale = Vec3::splat(final_scale);
            }
        }
    }
}

pub fn update_all_positions(
    mut resize_events: EventReader<WindowResized>,
    window: Query<&Window, With<PrimaryWindow>>,
    mut card_query: Query<(&mut Transform, &Card), (With<Card>, Without<BackgroundImage>)>,
    player_query: Query<(Entity, &Player)>,
    hand_query: Query<&Hand>,
    mut graveyard_query: Query<&mut Transform, (With<Graveyard>, Without<Card>, Without<BackgroundImage>)>,
) {
    for _resize_event in resize_events.read() {
        // Update cards positions in hand
        for (_player_entity, player) in player_query.iter() {
            if let Ok(hand) = hand_query.get(player.hand) {
                // determine player
                let player_index = if player.is_local_player { 0 } else { 1 };

                // obtain window and new positions
                let Ok(window) = window.single() else { return; };
                let positions = get_player_positions(player_index, window.width(), window.height());

                // update each card
                for (card_index, &card_entity) in hand.cards.iter().enumerate() {
                    if let Ok((mut transform, card)) = card_query.get_mut(card_entity) {
                        if matches!(card.position, CardPosition::Hand(_)) && card_index < 4 {
                            transform.translation = positions[card_index];
                        }
                    }
                }

                // update graveyard pos
                if let Ok(mut graveyard_transform) = graveyard_query.single_mut() {
                    graveyard_transform.translation.x = window.width() * 0.0;
                    graveyard_transform.translation.y = window.height() * -0.05;
                }

                // update drawn cards pos
                for (mut transform, card) in card_query.iter_mut() {
                    if matches!(card.position, CardPosition::DrawnCard(_)) {
                        transform.translation.x = window.width() * 0.1;
                        transform.translation.y = window.height() * 0.0;
                    }
                }
            }
        }
    }
}