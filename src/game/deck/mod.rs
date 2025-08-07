use bevy::prelude::*;
use crate::game::{deck::system::spawn_cards, gamestate::AppState};
pub mod component;
mod system;
pub struct DeckPlugin;

impl Plugin for DeckPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Setup), spawn_cards);
    }
}