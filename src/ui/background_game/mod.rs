use bevy::prelude::*;
use crate::game::gamestate::AppState;

pub mod component;
mod system;
use system::{spawn_background, adjust_background, update_all_positions};
use crate::game::hand::system::deal_initial_hands;

pub struct BackgroundPlugin;

impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(OnEnter(AppState::Setup), spawn_background)
        .add_systems(Update, adjust_background.run_if(in_state(AppState::PlayerTurn)))
        .add_systems(Update, update_all_positions.run_if(in_state(AppState::PlayerTurn)).after(adjust_background))
        .add_systems(Update, update_all_positions.run_if(in_state(AppState::Setup)).after(deal_initial_hands));
    }
}