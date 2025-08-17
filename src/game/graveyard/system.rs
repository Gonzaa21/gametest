use bevy::prelude::*;

use crate::game::graveyard::component::Graveyard;

pub fn spawn_graveyard(mut commands: Commands) {
    commands.spawn((
        Graveyard { cards: Vec::new() },
        Transform::from_xyz(-150.0, 50.0, 5.0)
    ));
    info!(target: "mygame", "Graveyard spawned at center-left");
}