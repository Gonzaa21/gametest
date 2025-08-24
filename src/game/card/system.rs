use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use crate::game::card::component::{Card, CardPosition, CardHandles, CardBack, Suit, Selected, DoubleClick};
use crate::game::{graveyard::component::Graveyard, turnPlayer::component::Turn, deck::component::Deck};
use bevy::asset::Assets;
use bevy::image::{Image, ImageSampler};
use rand::seq::SliceRandom;
use rand::rng;

// spawn card values after setup and allocate suit/value and textures
// iterate and mix all cards front/back
pub fn setup_cards(mut commands: Commands, asset_server: Res<AssetServer>) {
    let suits = ["coarse", "cup", "gold", "sword"];
    let mut card_routes = Vec::new();

    for suit_name in suits.iter() {
        for value in 1..=12 {
            let route = format!("textures/deck/{}/{}.PNG", suit_name, value);
            card_routes.push(route);
        }
    }

    // load textures
    let card_handles: Vec<Handle<Image>> = card_routes
        .into_iter()
        .map(|route| asset_server.load(route))
        .collect();

    // card back texture
    let card_back_handle = asset_server.load("textures/deck/back/back.PNG");

    // insert handles resources
    commands.insert_resource(CardHandles(card_handles.clone()));
    commands.insert_resource(CardBack(card_back_handle));

    let suits_logic = [Suit::Coarse, Suit::Cup, Suit::Gold, Suit::Sword];
    let mut cards_data: Vec<(Suit, u8)> = suits_logic
        .iter()
        .flat_map(|suit| (1..=12).map(move |value| (suit.clone(), value)))
        .collect();

    cards_data.shuffle(&mut rng());
}


pub fn card_face(
    card_back: Res<CardBack>,
    mut query: Query<(&Card, &mut Sprite)>
) {
    for (card, mut sprite) in query.iter_mut() {
        if card.face_up {
            // asign front
            sprite.image = card.front_face.clone();
        } else {
            // Asign back
            sprite.image = card_back.0.clone();
        }
    }
}

// card_selection - detect clicks
//      detect_card_click
//          handle_card_click - manage cards
//              discard_card()
//              card_swap()
//      detect_deck_click
//          handle_deck_click - draw card of the deck
//      detect_graveyard_click
//          handle_graveyard_click - draw of the graveyard

// card selection system
pub fn card_selection(
    mut commands: Commands,
    mouse_input: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    mut card_query: Query<(Entity, &mut Transform, &mut Card), With<Card>>,
    selected_query: Query<Entity, With<Selected>>,
    mut double_click: ResMut<DoubleClick>,
    time: Res<Time>,
    turn_query: ResMut<Turn>,
    mut graveyard_query: Query<&mut Graveyard>,
    deck_query: Query<&mut Deck>,
) {
    if !mouse_input.just_pressed(MouseButton::Left) {
        return;
    }

    // obtain window, camera, cursor/world position
    let Ok(window) = windows.single() else { return; };
    let Ok((camera, camera_transform)) = camera_query.single() else { return; };
    let Some(cursor_pos) = window.cursor_position() else { return; };
    let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_pos) else { return; };

    // detect click in deck
    if detect_deck_click(world_pos) {
        handle_deck_click(deck_query, turn_query, &mut card_query);
        return;
    }

    // detect click in graveyard 
    if detect_graveyard_click(world_pos) {
        handle_graveyard_click(graveyard_query, turn_query, &mut card_query);
        return;
    }

    // detect click in hand
    if let Some(clicked_entity) = detect_card_click(&card_query, world_pos) {
        handle_card_click(
            clicked_entity, &mut commands, &selected_query, &mut double_click,
            &time, &turn_query, &mut card_query, &mut graveyard_query
        );
        return;
    }
}

// feedback visual when selecting
pub fn card_visual(
    mut card_query: Query<(&mut Transform, Option<&Selected>, &Card), With<Card>>

) {
    for (mut transform, selected, card) in card_query.iter_mut() {
        if let CardPosition::Hand(_) = card.position {
            if selected.is_some() {
                transform.translation.y = -300.0; // raise position
                transform.translation.z = 50.0; 
            } else {
                if transform.translation.y > -310.0 {
                    transform.translation.y = -320.0; // normal position
                    transform.translation.z = 10.0;
                }
            }
        }
    }
}

pub fn configure_texture(
    mut images: ResMut<Assets<Image>>,
    card_handles: Option<Res<CardHandles>>,
    card_back: Option<Res<CardBack>>,
) {
    let Some(card_handles) = card_handles else { return; };
    let Some(card_back) = card_back else { return; };
    
    // front cards
    for handle in &card_handles.0 {
        if let Some(image) = images.get_mut(handle) {
            image.sampler = ImageSampler::nearest();
        }
    }
    
    // back card
    if let Some(image) = images.get_mut(&card_back.0) {
        image.sampler = ImageSampler::nearest();
    }
}

// DETECTION CLICK SYSTEMS
fn detect_card_click(
    card_query: &Query<(Entity, &mut Transform, &mut Card), With<Card>>,
    world_pos: Vec2
) -> Option<Entity> {
    for (card_entity, card_transform, _card_comp) in card_query.iter() {
        let card_pos = card_transform.translation;
        let card_size = Vec2::new(80.0, 120.0);
        
        // Check if the click is inside the card
        if world_pos.x >= card_pos.x - card_size.x / 2.0 
        && world_pos.x <= card_pos.x + card_size.x / 2.0
        && world_pos.y >= card_pos.y - card_size.y / 2.0
        && world_pos.y <= card_pos.y + card_size.y / 2.0 {
            return Some(card_entity);
        }
    }
    None
}

fn detect_deck_click(world_pos: Vec2) -> bool {
    let deck_pos = Vec3::new(150.0, 50.0, 0.0);
    let deck_size = Vec2::new(80.0, 120.0);
    
    world_pos.x >= deck_pos.x - deck_size.x / 2.0 
    && world_pos.x <= deck_pos.x + deck_size.x / 2.0
    && world_pos.y >= deck_pos.y - deck_size.y / 2.0
    && world_pos.y <= deck_pos.y + deck_size.y / 2.0
}

fn detect_graveyard_click(world_pos: Vec2) -> bool {
    let graveyard_pos = Vec3::new(-150.0, 50.0, 0.0);
    let graveyard_size = Vec2::new(80.0, 120.0);
    
    world_pos.x >= graveyard_pos.x - graveyard_size.x / 2.0 
    && world_pos.x <= graveyard_pos.x + graveyard_size.x / 2.0
    && world_pos.y >= graveyard_pos.y - graveyard_size.y / 2.0
    && world_pos.y <= graveyard_pos.y + graveyard_size.y / 2.0
}

// HANDLE CLICK SYSTEMS
fn handle_card_click(
    clicked_entity: Entity,
    commands: &mut Commands,
    selected_query: &Query<Entity, With<Selected>>,
    double_click: &mut ResMut<DoubleClick>,
    time: &Res<Time>,
    turn_query: &ResMut<Turn>,
    card_query: &mut Query<(Entity, &mut Transform, &mut Card), With<Card>>,
    graveyard_query: &mut Query<&mut Graveyard>,
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
        card_swap(clicked_entity, card_query, graveyard_query, turn_query);
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

fn handle_deck_click(
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

fn handle_graveyard_click(
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

// AUXILIAR SYSTEMS
fn discard_card(
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

fn card_swap(
    clicked_entity: Entity,
    card_query: &mut Query<(Entity, &mut Transform, &mut Card), With<Card>>,
    graveyard_query: &mut Query<&mut Graveyard>,
    turn_query: &ResMut<Turn>,
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
                
                info!(target: "mygame", "Card swap completed: {:?} -> Hand, {:?} -> Graveyard", drawn_card_entity, clicked_entity);
            }
        }
    }
}