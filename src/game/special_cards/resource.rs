use bevy::prelude::{Resource, Entity};

#[derive(Resource, Default)]
pub struct SpecialCardEffect {
    pub card_entity: Option<Entity>,
    pub effect_type: Option<SpecialEffect>,
    pub awaiting_target: bool,
    pub target_player: Option<Entity>,
    pub target_card: Option<Entity>,
    pub awaiting_own_card: bool,
    pub own_card: Option<Entity>
}

#[derive(Debug, Clone)]
pub enum SpecialEffect {
    Shuffle,    // card 11
    Reveal,     // card 9
    Swap,       // card 7
}
