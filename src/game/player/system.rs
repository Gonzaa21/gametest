use bevy::prelude::*;
use crate::game::player::component::Player;
use crate::game::hand::component::Hand;

pub fn spawn_player(mut commands: Commands) {
    let player_names = ["Player 1", "Player 2"];

    for (i, name) in player_names.iter().enumerate() {
        // create hand
        let hand = commands.spawn(Hand { cards: Vec::new() }).id();

        commands.spawn(Player{
            name: name.to_string(),
            hand: hand,
            is_local_player: i == 0 // first player = local 
        });
    }
}