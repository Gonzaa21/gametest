use bevy::prelude::*;

pub mod background_game;
pub mod menu;

use background_game::BackgroundPlugin;
use menu::MenuPlugin;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_plugins(BackgroundPlugin)
        .add_plugins(MenuPlugin);
    }
}