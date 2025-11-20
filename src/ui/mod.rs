use bevy::prelude::*;

pub mod background_game;
pub mod menu;
pub mod back_button;
pub mod turn_indicator;
pub mod card_animation;
pub mod soundtrack;

use background_game::BackgroundPlugin;
use menu::MenuPlugin;
use back_button::BackButtonPlugin;
use turn_indicator::TurnIndicatorPlugin;
use card_animation::CardAnimationPlugin;
use soundtrack::AudioPlugin;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_plugins(BackgroundPlugin)
        .add_plugins(MenuPlugin)
        .add_plugins(BackButtonPlugin)
        .add_plugins(TurnIndicatorPlugin)
        .add_plugins(CardAnimationPlugin)
        .add_plugins(AudioPlugin);
    }
}