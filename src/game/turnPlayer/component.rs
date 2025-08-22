use bevy::prelude::*;

#[derive(Resource)]
pub struct Turn {
    pub current_player: Entity,
    pub has_drawn_card: bool,
}