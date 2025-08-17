use bevy::prelude::*;
pub mod component;
mod system;

use crate::game::card::system::{setup_cards, card_face, card_selection, card_visual};
pub struct CardPlugin;

impl Plugin for CardPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_cards)
        .add_systems(Update, (card_face, card_selection, card_visual));
    }
}