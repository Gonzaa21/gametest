use bevy::prelude::*;
use crate::game::{player::component::Player, gamestate::AppState, card::component::{Card, CardPosition}, deck::component::Deck};
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

// take card of the deck 
pub fn draw_card(
    mut deck_query: Query<&mut Deck>,
    mut card_query: Query<(&mut Card, &mut Transform)>,
    turn: Res<Turn>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if keyboard.just_pressed(KeyCode::KeyD) {
        // obtain deck
        let mut deck = match deck_query.single_mut() {
            Ok(d) => d,
            Err(_) => {
                warn!(target: "mygame", "No deck found");
                return;
            }
        };

        // verify if have cards in deck
        if deck.cards_values.is_empty() {
            warn!(target: "mygame", "Deck is empty");
            return;
        }

        let drawn_card_entity = deck.cards_values.remove(0); // first card of the deck

        if let Ok((mut card, mut transform)) = card_query.get_mut(drawn_card_entity) {
            card.position = CardPosition::DrawnCard(turn.current_player);
            card.owner_id = Some(turn.current_player);
            card.face_up = true; // show card taken
            
            // card taken position
            transform.translation = Vec3::new(0.0, -100.0, 30.0);
            
            info!(target: "mygame", "Player {:?} drew card: {:?}", turn.current_player, drawn_card_entity);
        }
    }
}