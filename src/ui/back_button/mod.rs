use bevy::prelude::*;
use crate::game::gamestate::AppState;
use crate::ui::back_button::system::{spawn_button, clean_button, button_hover, button_visuals, handle_button};

pub mod component;
mod system;

pub struct BackButtonPlugin;

impl Plugin for BackButtonPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::Setup), spawn_button)
            .add_systems(OnEnter(AppState::PlayerTurn), spawn_button)
            .add_systems(OnEnter(AppState::RoundEnd), spawn_button)

            .add_systems(OnExit(AppState::Setup), clean_button)
            .add_systems(OnExit(AppState::PlayerTurn), clean_button)
            .add_systems(OnExit(AppState::RoundEnd), clean_button)
            
            .add_systems(Update, (button_hover, button_visuals, handle_button).run_if(is_in_game));
    }
}

fn is_in_game(state: Res<State<AppState>>) -> bool {
    matches!(state.get(), AppState::Setup | AppState::PlayerTurn | AppState::RoundEnd)
}