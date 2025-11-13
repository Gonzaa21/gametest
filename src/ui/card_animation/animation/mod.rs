use bevy::prelude::*;
use crate::ui::card_animation::animation::flip::{detect_flip, animate_flip};
use crate::ui::card_animation::animation::movement::{detect_movement, animate_movement};

mod flip;
mod deal;
mod movement;

pub struct AnimatePlugin;

impl Plugin for AnimatePlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Update, (detect_flip, detect_movement))
        .add_systems(Update, (animate_flip, animate_movement).after(detect_flip).after(detect_movement));
    }
}