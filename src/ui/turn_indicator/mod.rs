use bevy::prelude::*;

use crate::game::gamestate::AppState;
use crate::ui::turn_indicator::system::{add_card_opacity, update_turn_indicator, animation_opacity};

pub mod component;
mod system;

pub struct TurnIndicatorPlugin;

impl Plugin for TurnIndicatorPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Update, add_card_opacity.run_if(in_state(AppState::Setup)))
        .add_systems(Update, add_card_opacity.run_if(in_state(AppState::PlayerTurn)))
        .add_systems(Update, (update_turn_indicator, animation_opacity).chain().run_if(in_state(AppState::PlayerTurn)));
    }
}