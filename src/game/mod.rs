use bevy::prelude::*;

pub mod gamestate;
pub mod player;
pub mod hand;
pub mod graveyard;
pub mod deck;
pub mod card;
pub mod boardslot;

use gamestate::GameStatePlugin;
use player::PlayerPlugin;
use hand::HandPlugin;
use graveyard::GraveyardPlugin;
use deck::DeckPlugin;
use card::CardPlugin;
use boardslot::BoardslotPlugin;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((GameStatePlugin, PlayerPlugin, HandPlugin, GraveyardPlugin, DeckPlugin, CardPlugin, BoardslotPlugin));
    }
}