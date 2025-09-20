use bevy::prelude::*;
use bevy::window::{PrimaryWindow, WindowResized};
use crate::ui::background_game::component::BackgroundImage;

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