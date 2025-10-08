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

// button images
#[derive(Component)]
pub struct ButtonImages {
    pub normal: Handle<Image>,
    pub hovered: Handle<Image>,
    pub pressed: Handle<Image>,
}

// button state
#[derive(Component, PartialEq)]
pub enum ButtonState {
    Normal,
    Hovered,
    Pressed,
}