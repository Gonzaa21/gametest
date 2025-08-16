use bevy::prelude::*;
use crate::game::player::component::Player;
use crate::game::hand::component::Hand;

pub fn spawn_player(mut commands: Commands) {
    // create hand
    let hand = commands.spawn(Hand { cards: Vec::new() }).id();

    commands.spawn(Player{
        name: "Local Player".to_string(),
        hand: hand,
        is_local_player: true
    });
}