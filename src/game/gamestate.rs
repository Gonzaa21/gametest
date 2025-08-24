use bevy::prelude::*;

#[derive(Clone, Eq, PartialEq, Hash, Debug, Default, States)]
pub enum AppState {
    #[default]
    MainMenu, // game menu
    Setup, // in game
    PlayerTurn, // local player turn
    RoundEnd // end button
}

pub fn transition_to_setup(
    keyboard: Res<ButtonInput<KeyCode>>, 
    mut next_state: ResMut<NextState<AppState>>
) {
    if keyboard.just_pressed(KeyCode::Enter) {
        next_state.set(AppState::Setup);
    }
}

pub struct GameStatePlugin;

impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<AppState>()
        .add_systems(Update, transition_to_setup.run_if(in_state(AppState::MainMenu)));
    }
}

