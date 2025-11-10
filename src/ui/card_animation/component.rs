use bevy::prelude::*;

#[derive(Component, PartialEq)]
pub struct CardAnimation {
    pub animation_type: AnimationType,
    pub progress: f32,
    pub duration: f32,
    pub state: AnimationState,
    pub original_scale: Vec3,
    pub original_position: Vec3,
    pub original_rotation: Quat,
}

#[derive(PartialEq)]
pub enum AnimationType {
    Flip,
    // Move, Deal para después
}

#[derive(PartialEq)]
pub enum AnimationState {
    // Idle,
    Animating,
}