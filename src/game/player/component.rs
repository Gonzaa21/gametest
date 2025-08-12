use bevy::prelude::{Component, Entity};

#[derive(Component)]
pub struct Player {
    pub name: String,
    pub hand: Entity,
    pub is_local_player: bool
}