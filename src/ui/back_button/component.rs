use bevy::prelude::*;

// setup
#[derive(Component)]
pub struct SetupUI;

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

// back button
#[derive(Component)]
pub struct BackButton;