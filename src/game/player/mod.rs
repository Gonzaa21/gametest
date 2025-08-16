use bevy::prelude::*;
pub mod component;
mod system;
use crate::game::player::system::spawn_player;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player);
    }
}