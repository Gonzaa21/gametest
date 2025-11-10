use bevy::prelude::*;

pub mod component;
mod animation;

use crate::ui::card_animation::animation::AnimatePlugin;

pub struct CardAnimationPlugin;

impl Plugin for CardAnimationPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_plugins(AnimatePlugin);
    }
}