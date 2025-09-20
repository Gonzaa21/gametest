use bevy::prelude::*;

mod global;
mod game;
mod ui;

use game::GamePlugin;
use ui::UiPlugin;
use global::setup_camera;

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_systems(Startup,setup_camera)
    .add_plugins(GamePlugin)
    .add_plugins(UiPlugin)
    .run();
}
