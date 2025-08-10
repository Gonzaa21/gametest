use bevy::prelude::*;
use crate::game::{hand::system::deal_initial_hands, gamestate::AppState};
use crate::game::deck::DeckSet;
pub mod component;
mod system;

pub struct HandPlugin;

impl Plugin for HandPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Setup), deal_initial_hands.after(DeckSet));
    }
}