use bevy::prelude::*;
use crate::game::gamestate::AppState;

pub mod component;
mod system;
use system::{spawn_background, adjust_background};

pub struct BackgroundPlugin;

impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(OnEnter(AppState::Setup), spawn_background)
        .add_systems(Update, adjust_background.run_if(in_state(AppState::Setup)));
    }
}