use bevy::prelude::*;
use crate::game::{deck::system::spawn_cards, gamestate::AppState};
use crate::game::card::system::setup_cards;
pub mod component;
mod system;

// deckset
#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct DeckSet;

pub struct DeckPlugin;

impl Plugin for DeckPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Setup), spawn_cards.in_set(DeckSet).after(setup_cards));
    }
}