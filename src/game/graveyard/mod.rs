use bevy::prelude::*;
pub mod component;
mod system;
use crate::game::{graveyard::system::spawn_graveyard, gamestate::AppState};
pub struct GraveyardPlugin;

impl Plugin for GraveyardPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Setup), spawn_graveyard);
    }
}