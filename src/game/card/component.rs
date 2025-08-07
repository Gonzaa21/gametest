use bevy::{ecs::entity::Entity, prelude::Component};

#[derive(Component)]
pub struct Card {
    pub value: u8,
    pub face_up: bool,
    pub owner_id: Option<Entity>,
    pub position: CardPosition
}

pub enum CardPosition {
    Deck,
    Hand(Entity),
    Graveyard,
    BoardSlot(Entity),
}