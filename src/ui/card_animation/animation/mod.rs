use bevy::prelude::*;
use crate::ui::card_animation::animation::flip::{detect_flip, animate_flip};

mod flip;
mod deal;
mod movement;

pub struct AnimatePlugin;

impl Plugin for AnimatePlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Update, (detect_flip, animate_flip).chain());
    }
}