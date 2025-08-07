use bevy::prelude::{Component, Entity};

#[derive(Component)]
pub struct Deck {
    pub cards_values: Vec<Entity>
}