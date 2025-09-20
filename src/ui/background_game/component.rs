use bevy::prelude::*;

// Component to identify the background image
#[derive(Component)]
pub struct BackgroundImage(pub Handle<Image>);