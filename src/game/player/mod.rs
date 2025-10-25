use bevy::prelude::*;
pub mod component;
pub mod system;
// use crate::game::player::system::spawn_player;
// use crate::game::AppState;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, _app: &mut App) {
        // app.add_systems(OnEnter(AppState::Setup), spawn_player);
    }
}