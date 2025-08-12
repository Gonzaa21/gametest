use bevy::prelude::*;

#[derive(Resource)]
pub struct Turn {
    pub current_player: Entity,
}