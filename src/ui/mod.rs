use bevy::prelude::*;

pub mod background_game;

use background_game::BackgroundPlugin;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_plugins(BackgroundPlugin);
    }
}