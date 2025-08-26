use bevy::prelude::*;
use crate::game::card::component::{Card, CardPosition, };
use crate::game::{graveyard::component::Graveyard, turn_player::component::Turn, hand::component::Hand, player::component::Player};

// AUXILIAR SYSTEMS
pub fn discard_card(
    clicked_entity: Entity,
    card_query: &mut Query<(Entity, &mut Transform, &mut Card), With<Card>>,
    graveyard_query: &mut Query<&mut Graveyard>,
    turn_query: &ResMut<Turn>,
) {
    if let Ok((_, _transform, card)) = card_query.get_mut(clicked_entity) {
        if matches!(card.position, CardPosition::DrawnCard(player_id) if player_id == turn_query.current_player) {
            if let Ok((_, mut card_transform, mut card)) = card_query.get_mut(clicked_entity) {
                card.position = CardPosition::Graveyard;
                card.face_up = true;

                // update graveyard
                if let Ok(mut graveyard) = graveyard_query.single_mut() {
                    graveyard.cards.push(clicked_entity);

                    card_transform.translation = Vec3::new(-150.0, 50.0, graveyard.cards.len() as f32);

                    info!(target: "mygame", "Card discarded directly to graveyard: {:?}", clicked_entity);
                }
            }
            return;
        }
    }
}

pub fn card_swap(
    clicked_entity: Entity,
    card_query: &mut Query<(Entity, &mut Transform, &mut Card), With<Card>>,
    graveyard_query: &mut Query<&mut Graveyard>,
    turn_query: &ResMut<Turn>,
    hand_query: &mut Query<&mut Hand>,
    player_query: &Query<(Entity, &Player)>
) {
    let clicked_card = card_query.iter()
        .find(|(entity, _, _)| *entity == clicked_entity)
        .map(|(_, _, card)| card);

    if let Some(clicked_card) = clicked_card {
        // verify: card is inside the hand
        if !matches!(clicked_card.position, CardPosition::Hand(player_id) if player_id == turn_query.current_player) {
            info!(target: "mygame", "Card is not in current player's hand");
            return;
        }
    
        // search the drawn card
        let drawn_card = card_query.iter()
            .find(|(_, _, card)| matches!(card.position, CardPosition::DrawnCard(player_id) if player_id == turn_query.current_player));
        
        if drawn_card.is_none() { // if it was not found
            info!(target: "mygame", "No drawn card found for current player");
            return;
        }
    
        // obtain drawn_card entity and original position card
        let drawn_card_entity = drawn_card.unwrap().0; // .0 indicate the first parameter (Entity)
        
        // exchange positions
        if let Ok([(_, mut drawn_transform, mut drawn_card), (_, mut clicked_transform, mut clicked_card)]) = 
            card_query.get_many_mut([drawn_card_entity, clicked_entity]) {
            
            let clicked_pos = clicked_transform.translation; // obtain position
            
            drawn_card.position = CardPosition::Hand(turn_query.current_player);
            drawn_card.face_up = false;
            drawn_transform.translation = clicked_pos;
            clicked_card.position = CardPosition::Graveyard; // card of hand selected to graveyard
            clicked_card.face_up = true; // card front
            
            if let Ok(mut graveyard) = graveyard_query.single_mut() {
                graveyard.cards.push(clicked_entity); // update changes
                clicked_transform.translation = Vec3::new(-150.0, 50.0, graveyard.cards.len() as f32); // position in graveyard
                
                if let Some((_, player)) = player_query.iter().find(|(entity, _)| *entity == turn_query.current_player) {
                    if let Ok(mut hand) = hand_query.get_mut(player.hand) {
                        // remove hand
                        hand.cards.retain(|&card_entity| card_entity != clicked_entity);
                        // add new card in hand
                        hand.cards.push(drawn_card_entity);
                    }
                }

                info!(target: "mygame", "Card swap completed: {:?} -> Hand, {:?} -> Graveyard", drawn_card_entity, clicked_entity);
            }
        }
    }
}