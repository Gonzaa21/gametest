use bevy::prelude::*;

// menu
#[derive(Component)]
pub struct MainMenuUI;

// background
#[derive(Component)]
pub struct MenuBackground(pub Handle<Image>);

// play button
#[derive(Component)]
pub struct PlayButton;

// exit button
#[derive(Component)]
pub struct ExitButton;