use bevy::{ecs::entity::Entity, prelude::{Component, Resource, Handle, Image}};

// COMPONENTS
#[derive(Component)]
pub struct Card {
    pub value: u8,
    pub face_up: bool,
    pub owner_id: Option<Entity>,
    pub position: CardPosition,
    pub suit: Suit,
    pub front_face: Handle<Image>,
    pub from_deck: bool
}

#[derive(Component)]
pub struct Selected;

#[derive(Component)]
pub struct PreviousCardPosition(pub CardPosition);

#[derive(Component)]
pub struct PreviousFaceUp(pub bool);

// ENUMS
#[derive(Clone, PartialEq)]
pub enum CardPosition {
    Deck,
    Hand(Entity),
    Graveyard,
    DrawnCard(Entity)
}

#[derive(Clone, Debug)]
pub enum Suit {
    Coarse,
    Cup,
    Gold,
    Sword,
}

// RESOURCES
#[derive(Resource)]
pub struct CardHandles(pub Vec<Handle<Image>>);

#[derive(Resource)]
pub struct CardBack(pub Handle<Image>);

#[derive(Resource)]
pub struct DoubleClick {
    pub last_card: Option<Entity>,
    pub last_click_time: f32,
    pub time_limit: f32
}