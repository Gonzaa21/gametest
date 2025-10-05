use bevy::prelude::*;

use crate::ui::menu::component::{MainMenuUI, MenuBackground, PlayButton, ExitButton};
use bevy::window::{PrimaryWindow, WindowResized};

// spawn_background
pub fn spawn_background(
    mut commands: Commands, 
    asset_server: Res<AssetServer>
) {
    let image_handle: Handle<Image> = asset_server.load("textures/ui/background/background_menu.png");
    
    // Spawn with default scale
    commands.spawn((
        Sprite::from_image(image_handle.clone()),
        Transform::from_xyz(0.0, 0.0, -10.0),
        MenuBackground(image_handle),
        MainMenuUI,
    ));
}

pub fn adjust_background(
    mut bg_query: Query<(&mut Transform, &MenuBackground)>,
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
                let final_scale = scale_x.max(scale_y);

                transform.scale = Vec3::splat(final_scale);
            }
        }
    }
}

// spawn_buttons
pub fn spawn_buttons(
    mut commands: Commands, 
    asset_server: Res<AssetServer>
) {
    let image_play: Handle<Image> = asset_server.load("textures/ui/buttons/PLAY_normal.png");
    let image_exit: Handle<Image> = asset_server.load("textures/ui/buttons/EXIT_normal.png");
    
    // Spawn with default scale
    commands.spawn((
        Sprite::from_image(image_play.clone()),
        Transform::from_xyz(100.0, 0.0, 0.0),
        PlayButton,
        MainMenuUI,
    ));

    commands.spawn((
        Sprite::from_image(image_exit.clone()),
        Transform::from_xyz(-100.0, 0.0, 0.0),
        ExitButton,
        MainMenuUI,
    ));
}

// clean_menu
pub fn clean_menu(
    mut commands: Commands,
    menu_query: Query<Entity, With<MainMenuUI>>,
) {
    for entity in menu_query.iter() {
        commands.entity(entity).despawn();
    }
}