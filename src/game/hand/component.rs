use bevy::prelude::{Component, Entity};

#[derive(Component)]
pub struct Hand {
    pub cards_id: Vec<Entity>,
}