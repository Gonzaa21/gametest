use bevy::prelude::*;
use crate::game::gamestate::AppState;

pub mod component;
mod system;

use system::{start_turn_system, end_round_system};

pub struct TurnPlugin;

impl Plugin for TurnPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::Setup), start_turn_system)
            .add_systems(Update, (end_round_system).run_if(in_state(AppState::PlayerTurn)));
    }
}