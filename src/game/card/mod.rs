use bevy::prelude::*;
pub mod component;
mod system;

use crate::game::{card::{component::DoubleClick, system::{card_face, card_selection, card_visual, setup_cards}}, gamestate::AppState};
pub struct CardPlugin;

impl Plugin for CardPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_cards)
        .add_systems(Update, card_face)
        .add_systems(Update, (card_selection, card_visual).run_if(in_state(AppState::PlayerTurn)))
        .insert_resource(DoubleClick { last_card: None, last_click_time: 0.0, time_limit: 0.4 });
    }
}