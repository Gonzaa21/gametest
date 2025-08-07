use bevy::prelude::{Component, Entity};

#[derive(Component)]
pub struct Player {
    pub id: Entity,
    pub name: String,
    pub hand_id: Entity,
    pub is_local_player: bool
}