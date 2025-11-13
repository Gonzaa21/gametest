use bevy::prelude::*;
use crate::game::card::component::{Card, CardPosition, PreviousCardPosition, PreviousFaceUp};
use crate::ui::card_animation::component::{CardAnimation, AnimationState, AnimationType};

// detect when face_up change
pub fn detect_flip(
    mut commands: Commands,
    card_query: Query<(Entity, &Card, &Transform, Option<&PreviousCardPosition>, Option<&PreviousFaceUp>)>,
    animation_query: Query<&CardAnimation>,
) {
    for (entity, card, transform, previous_pos, previous_face) in card_query.iter() {
        // verify if is animating already
        if animation_query.get(entity).is_ok() {
            continue;
        }
        
        // obtain previous card position
        let prev_pos = match previous_pos {
            Some(p) => &p.0,
            None => {
                // save position
                commands.entity(entity).insert(PreviousCardPosition(card.position.clone()));
                commands.entity(entity).insert(PreviousFaceUp(card.face_up));
                continue;
            }
        };
        
        // obtain previous face
        let prev_face = match previous_face {
            Some(p) => p.0,
            None => {
                commands.entity(entity).insert(PreviousFaceUp(card.face_up));
                continue;
            }
        };

        // verify if have changes
        let position_changed = prev_pos != &card.position;
        let face_changed = prev_face != card.face_up;

        if !position_changed && !face_changed {
            continue; // do nothing if not changed
        }

        // detect when the card must animate
        let should_animate = match (prev_pos, &card.position) {
            _ if !prev_face && card.face_up && matches!(card.position, CardPosition::Hand(_)) => true,
            _ => false,
        };
        
        if should_animate {
            // insert animation
            commands.entity(entity).insert(CardAnimation {
                animation_type: AnimationType::Flip,
                progress: 0.0,
                duration: 0.4,
                state: AnimationState::Animating,
                original_scale: transform.scale,
                original_position: transform.translation,
                original_rotation: transform.rotation,
                target_position: Some(transform.translation),
            });
        }
        
        // update previous position if it changes
        if position_changed {
            commands.entity(entity).insert(PreviousCardPosition(card.position.clone()));
        }
        if face_changed {
            commands.entity(entity).insert(PreviousFaceUp(card.face_up));
        }
    }
}

pub fn animate_flip(
    mut commands: Commands,
    mut card_query: Query<(Entity, &mut Transform, &mut CardAnimation), With<CardAnimation>>,
    time: Res<Time>,   
) {
    for (entity, mut transform, mut animation) in card_query.iter_mut() {
        if animation.animation_type == AnimationType::Flip {
            let progress = animation.progress;

            if progress <= 0.5 {
                let t = progress * 2.0;  // Normalizar a 0.0-1.0
    
                // X size: compress horizontally (simulates rotation)
                transform.scale.x = animation.original_scale.x * (1.0 - t).max(0.05);

                // Y size: smaller (perspective)
                transform.scale.y = animation.original_scale.y * (1.0 - t * 0.15);

                // Z rotation: tilt from corner ("page" effect)
                let rotation_angle = -t * 0.5;  // Rotar hasta ~17 grados
                transform.rotation = Quat::from_rotation_z(rotation_angle);

                // move to bottom right corner
                transform.translation.x = animation.original_position.x + (t * 25.0);
                transform.translation.y = animation.original_position.y - (t * 25.0);
                transform.translation.z = animation.original_position.z + (t * 5.0);
            } else {
                // get back to normal
                let t = (progress - 0.5) * 2.0;

                // X size: expand to normal
                transform.scale.x = animation.original_scale.x * (t).max(0.05);

                // Y size: expand to normal
                transform.scale.y = animation.original_scale.y * (0.85 + t * 0.15);

                // Z rotation: return to 0
                let rotation_angle = -0.5 * (1.0 - t);
                transform.rotation = Quat::from_rotation_z(rotation_angle);

                // return to normal position
                transform.translation.x = animation.original_position.x + (25.0 * (1.0 - t));
                transform.translation.y = animation.original_position.y - (25.0 * (1.0 - t));
                transform.translation.z = animation.original_position.z + ((1.0 - t) * 5.0);
            }

            animation.progress += time.delta_secs() / animation.duration;
            
            if animation.progress >= 1.0 {
                transform.scale = animation.original_scale;
                transform.translation = animation.original_position;
                transform.rotation = animation.original_rotation;
                commands.entity(entity).remove::<CardAnimation>();
            }
        }
    }
}