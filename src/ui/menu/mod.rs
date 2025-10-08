use bevy::prelude::*;
use crate::game::gamestate::AppState;

pub mod component;
mod system;
mod handles;

use crate::ui::menu::system::{spawn_background, spawn_buttons, adjust_background, clean_menu};
use crate::ui::menu::handles::{handle_button_clicks, update_button_visuals, update_button_hover};
pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(OnEnter(AppState::MainMenu), (spawn_background, spawn_buttons))
        .add_systems(Update, (adjust_background, update_button_hover, handle_button_clicks, update_button_visuals).run_if(in_state(AppState::MainMenu)))
        .add_systems(OnExit(AppState::MainMenu), clean_menu);
    }
}