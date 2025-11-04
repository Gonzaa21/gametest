use bevy::prelude::*;

pub mod background_game;
pub mod menu;
pub mod back_button;
pub mod turn_indicator;

use background_game::BackgroundPlugin;
use menu::MenuPlugin;
use back_button::BackButtonPlugin;
use turn_indicator::TurnIndicatorPlugin;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_plugins(BackgroundPlugin)
        .add_plugins(MenuPlugin)
        .add_plugins(BackButtonPlugin)
        .add_plugins(TurnIndicatorPlugin);
    }
}