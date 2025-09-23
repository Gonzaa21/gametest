use bevy::prelude::*;
pub mod component;
mod system;
mod handles;
pub mod utils;

use crate::game::{card::{component::DoubleClick, system::{card_face, card_selection, card_visual, setup_cards, configure_texture}}, gamestate::AppState};
pub struct CardPlugin;

impl Plugin for CardPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_cards)
        .add_systems(Update, (card_face, configure_texture))
        .add_systems(Update, card_selection.run_if(in_state(AppState::PlayerTurn)))
        .add_systems(Update, card_visual.run_if(in_state(AppState::PlayerTurn)).after(card_selection))
        .insert_resource(DoubleClick { last_card: None, last_click_time: 0.0, time_limit: 0.4 });
    }
}