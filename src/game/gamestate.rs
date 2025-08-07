use bevy::prelude::*;

#[derive(Clone, Eq, PartialEq, Hash, Debug, Default, States)]
pub enum AppState {
    #[default]
    MainMenu,
    Setup,
    PlayerTurn,
    RoundEnd
}

pub struct GameStatePlugin;

impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(State::new(AppState::MainMenu));
    }
}