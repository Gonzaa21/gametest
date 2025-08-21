use bevy::prelude::*;
use crate::game::{player::component::Player, gamestate::AppState};
use super::component::Turn;

// start first player turn
pub fn start_turn_system(
    mut commands: Commands,
    players: Query<Entity, With<Player>>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    if let Some(first_player) = players.iter().next() {
        commands.insert_resource(Turn {
            current_player: first_player,
        });
        next_state.set(AppState::PlayerTurn);
        info!(target: "mygame","Shift started for player: {:?}", first_player);
    } else {
        warn!(target: "mygame","There are no players");
    }
}

// next turn player
pub fn next_turn_system(
    mut turn: ResMut<Turn>,
    players: Query<Entity, With<Player>>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if keyboard.just_pressed(KeyCode::Space) {
        let players_vec: Vec<Entity> = players.iter().collect();

        if let Some(pos) = players_vec.iter().position(|&p| p == turn.current_player) {
            let next_index = (pos + 1) % players_vec.len();
            turn.current_player = players_vec[next_index];
            info!(target: "mygame","Shift changed to player: {:?}", turn.current_player);
        }

        // si presionan "ronda terminada":
        // next_state.set(AppState::RoundEnd);
    }
}