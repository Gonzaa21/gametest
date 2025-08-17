use bevy::prelude::{Component, Entity};

#[derive(Component)]
pub struct Graveyard {
    pub cards: Vec<Entity>
}