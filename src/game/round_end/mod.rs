use bevy::prelude::*;
use crate::game::gamestate::AppState;

mod system;
use system::{reveal_all_cards, calculate_scores, prepare_new_round};

pub struct RoundEndPlugin;

impl Plugin for RoundEndPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::RoundEnd), (reveal_all_cards, calculate_scores).chain())
        .add_systems(Update, prepare_new_round.run_if(in_state(AppState::RoundEnd)));
    }
}