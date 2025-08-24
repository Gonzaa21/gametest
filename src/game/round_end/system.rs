use bevy::prelude::*;
use crate::game::{card::component::{Card, CardPosition}, player::component::Player, hand::component::Hand, gamestate::AppState, turnPlayer::component::Turn, graveyard::component::Graveyard, deck::component::Deck};

// Reveal all cards
pub fn reveal_all_cards(
    mut card_query: Query<&mut Card>,
    player_query: Query<(Entity, &Player)>,
) {
    // iterate player cards and if cards are in his hand, face_up = true
    for (player_entity, _player) in player_query.iter() {
        for mut card in card_query.iter_mut() {
            if let CardPosition::Hand(hand_owner) = card.position {
                if hand_owner == player_entity {
                    card.face_up = true;
                    info!(target: "mygame", "Card revealed: {} of {:?}", card.value, card.suit);
                }
            }
        }
    }
}

// Calculate scores
pub fn calculate_scores(
    card_query: Query<&Card>,
    player_query: Query<(Entity, &Player)>,
    hand_query: Query<&Hand>,
) {
    info!(target: "mygame", "=== ROUND END SCORES ===");
    
    let mut scores = Vec::new();
    
    for (player_entity, player) in player_query.iter() {
        if let Ok(hand) = hand_query.get(player.hand) {
            let mut total_score = 0; // score default
            
            for &card_entity in &hand.cards {
                if let Ok(card) = card_query.get(card_entity) {
                    total_score += card.value as u32; // sum card value
                }
            }
            
            scores.push((player_entity, player.name.clone(), total_score)); // push final score
            info!(target: "mygame", "Player {}: {} points", player.name, total_score);
        }
    }
    
    // search winner
    if let Some((_winner_entity, winner_name, winner_score)) = scores.iter().min_by_key(|(_, _, score)| *score) {
        info!(target: "mygame", "WINNER: {} with {} points!", winner_name, winner_score);
    }
    
    info!(target: "mygame", "Press N for new round");
}

// start new round
pub fn prepare_new_round(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<AppState>>,
    mut turn: ResMut<Turn>,
    mut commands: Commands,
    card_query: Query<Entity, With<Card>>,
    deck_query: Query<Entity, With<Deck>>,
    graveyard_query: Query<Entity, With<Graveyard>>,
    mut hand_query: Query<&mut Hand>,
) {
    if keyboard.just_pressed(KeyCode::KeyN) {

        // reset player hands
        for mut hand in hand_query.iter_mut() {
            hand.cards.clear(); // clean hand
            info!(target: "mygame", "Hand cleared");
        }

        // despawn cards, deck, graveyard
        for entity in card_query.iter() {
            commands.entity(entity).despawn();
        }
        for entity in deck_query.iter() {
            commands.entity(entity).despawn();
        }
        for entity in graveyard_query.iter() {
            commands.entity(entity).despawn();
        }

        turn.has_drawn_card = false;
        next_state.set(AppState::Setup);
        info!(target: "mygame", "Starting new round...");
    }
}