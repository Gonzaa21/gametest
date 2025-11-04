use bevy::prelude::*;

// opacity animation
#[derive(Component)]
pub struct CardOpacity {
    pub current: f32,
    pub target: f32,
    pub transition_speed: f32, // How fast the opacity changes
}

// default values
impl Default for CardOpacity {
    fn default() -> Self {
        Self {
            current: 1.0,
            target: 1.0,
            transition_speed: 3.0,
        }
    }
}