use bevy::prelude::Component;

#[derive(Component)]
pub struct Deck {
    pub cards_values: Vec<u8>
}