use bevy::{ecs::entity::Entity, prelude::{Component, Resource, Handle, Image}};

#[derive(Component)]
pub struct Card {
    pub value: u8,
    pub face_up: bool,
    pub owner_id: Option<Entity>,
    pub position: CardPosition,
    pub suit: Suit,
    pub front_face: Handle<Image>
}

pub enum CardPosition {
    Deck,
    Hand(Entity),
    Graveyard,
    BoardSlot(Entity),
}

#[derive(Clone, Debug)]
pub enum Suit {
    Coarse,
    Cup,
    Gold,
    Sword,
}

#[derive(Resource)]
pub struct CardHandles(pub Vec<Handle<Image>>);

#[derive(Resource)]
pub struct CardBack(pub Handle<Image>);