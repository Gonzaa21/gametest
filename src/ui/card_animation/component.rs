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
    pub target_position: Option<Vec3>, 
}

#[derive(PartialEq)]
pub enum AnimationType {
    Flip,
    Movement,
}

#[derive(PartialEq)]
pub enum AnimationState {
    // Idle,
    Animating,
}