use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::game::graveyard::component::Graveyard;

pub fn spawn_graveyard(mut commands: Commands, windows: Query<&Window, With<PrimaryWindow>>,) {
    let Ok(window) = windows.single() else { 
        warn!("No primary window found");
        return; 
    };

    commands.spawn((
        Graveyard { cards: Vec::new() },
        Transform::from_xyz(window.width() * -0.06, window.height() * 0.0, 5.0),
    ));
    info!(target: "mygame", "Graveyard spawned at center-left");
}