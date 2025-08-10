use bevy::prelude::*;

// create camera
pub fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d::default());
}