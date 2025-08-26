use bevy::prelude::*;
use crate::game::card::component::{Card, CardPosition, Selected, DoubleClick};
use crate::game::{graveyard::component::Graveyard, turn_player::component::Turn, deck::component::Deck, hand::component::Hand, player::component::Player};
use crate::game::card::utils::{card_swap, discard_card};

// HANDLE CLICK SYSTEMS
pub fn handle_card_click(
    clicked_entity: Entity,
    commands: &mut Commands,
    selected_query: &Query<Entity, With<Selected>>,
    double_click: &mut ResMut<DoubleClick>,
    time: &Res<Time>,
    turn_query: &ResMut<Turn>,
    card_query: &mut Query<(Entity, &mut Transform, &mut Card), With<Card>>,
    graveyard_query: &mut Query<&mut Graveyard>,
    player_query: &Query<(Entity, &Player)>,
    hand_query: &mut Query<&mut Hand>,
) {
    // verify: if it is direct discard
    let card_comp = card_query.iter()
        .find(|(entity, _, _)| *entity == clicked_entity)
        .map(|(_, _, card)| card);

    if let Some(card_comp) = card_comp {
        if matches!(card_comp.position, CardPosition::DrawnCard(player_id) if player_id == turn_query.current_player) {
            discard_card(clicked_entity, card_query, graveyard_query, turn_query);
            return;
        }
    }

    // double click
    let current_time = time.elapsed_secs();
    let mut is_double_click = false;
    
    if let Some(last_card) = double_click.last_card {
        if last_card == clicked_entity {
            let time_diff = current_time - double_click.last_click_time;
            if time_diff <= double_click.time_limit {
                is_double_click = true;
            }
        }
    }

    if is_double_click {
        card_swap(clicked_entity, card_query, graveyard_query, turn_query, hand_query, player_query);
    } else {
        // selection component
        for selected_entity in selected_query.iter() {
            commands.entity(selected_entity).remove::<Selected>();
        }
        commands.entity(clicked_entity).insert(Selected);
        
        double_click.last_card = Some(clicked_entity);
        double_click.last_click_time = current_time;
        info!(target: "mygame", "Card selected: {:?}", clicked_entity);
    }
}

pub fn handle_deck_click(
    mut deck_query: Query<&mut Deck>,
    mut turn_query: ResMut<Turn>,
    card_query: &mut Query<(Entity, &mut Transform, &mut Card), With<Card>>,
) {
    // verify if player already drew a card
    if turn_query.has_drawn_card {
        info!(target: "mygame", "Player already drew a card this turn");
        return;
    }

    // obtain deck
    let mut deck = match deck_query.single_mut() {
        Ok(d) => d,
        Err(_) => {
            warn!(target: "mygame", "No deck found");
            return;
        }
    };

    // verify if it have cards
    if deck.cards_values.is_empty() {
        warn!(target: "mygame", "Deck is empty");
        return;
    }

    let drawn_card_entity = deck.cards_values.remove(0); // take first card of the deck
    
    if let Ok((_, mut transform, mut card)) = card_query.get_mut(drawn_card_entity) {
        card.position = CardPosition::DrawnCard(turn_query.current_player);
        card.owner_id = Some(turn_query.current_player);
        card.face_up = true; // show card taken
        
        transform.translation = Vec3::new(0.0, -100.0, 30.0); // card taken position
        turn_query.has_drawn_card = true; // player already drew a card
        info!(target: "mygame", "Player {:?} drew card: {:?}", turn_query.current_player, drawn_card_entity);
    }
}

pub fn handle_graveyard_click(
    mut graveyard_query: Query<&mut Graveyard>,
    mut turn_query: ResMut<Turn>,
    card_query: &mut Query<(Entity, &mut Transform, &mut Card), With<Card>>,
) {
    // verify if player already drew a card
    if turn_query.has_drawn_card {
        info!(target: "mygame", "Player already drew a card this turn");
        return;
    }

    // obtain graveyard
    let mut graveyard = match graveyard_query.single_mut() {
        Ok(d) => d,
        Err(_) => {
            warn!(target: "mygame", "No graveyard found");
            return;
        }
    };

    // verify if it have cards
    if graveyard.cards.is_empty() {
        warn!(target: "mygame", "Graveyard is empty");
        return;
    }
    
    // take last card of the graveyard
    let drawn_card_entity = match graveyard.cards.pop() {
        Some(d) => d,
        None => {
            warn!(target: "mygame", "No graveyard found");
            return;
        }
    };

    if let Ok((_, mut transform, mut card)) = card_query.get_mut(drawn_card_entity) {
        card.position = CardPosition::DrawnCard(turn_query.current_player);
        card.owner_id = Some(turn_query.current_player);
        card.face_up = true; // show card taken
        
        transform.translation = Vec3::new(0.0, -100.0, 30.0); // card taken position
        turn_query.has_drawn_card = true; // player already drew a card
        info!(target: "mygame", "Player {:?} drew card: {:?}", turn_query.current_player, drawn_card_entity);
    }
}