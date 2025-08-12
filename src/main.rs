use bevy::prelude::*;

mod global;
mod game;

use game::GamePlugin;
use global::setup_camera;

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_systems(Startup,setup_camera)
    .add_plugins(GamePlugin)
    .run();
}
