use bevy::prelude::*;

#[derive(Clone, Eq, PartialEq, Hash, Debug, Default, States)]
pub enum AppState {
    #[default]
    MainMenu, // game menu
    Setup, // in game
    PlayerTurn, // local player turn
    RoundEnd // end button
}


// component for despawn all entities to close setup state
#[derive(Component)]
pub struct GameEntity;

pub struct GameStatePlugin;

impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<AppState>();
    }
}

