use bevy::prelude::*;
use crate::game::card::component::{Card, CardPosition};
use crate::ui::card_animation::component::{CardAnimation, AnimationState, AnimationType};

// detect when face_up change
pub fn detect_flip(
    mut commands: Commands,
    card_query: Query<(Entity, &Card, &Transform), Changed<Card>>,  // Changed<> detect changes in Card
    animation_query: Query<&CardAnimation>,
) {
    for (entity, card, transform) in card_query.iter() {
        // verify if already have an animation in progress
        if animation_query.get(entity).is_ok() {
            continue;
        }

        // only animate when face_up == true
        if card.face_up {
            match card.position {
                CardPosition::Hand(_) | CardPosition::DrawnCard(_) => {
                    commands.entity(entity).insert(CardAnimation {
                        animation_type: AnimationType::Flip,
                        progress: 0.0,
                        duration: 0.4,
                        state: AnimationState::Animating,
                        original_scale: transform.scale,
                        original_position: transform.translation,
                        original_rotation: transform.rotation,
                    });
                }
                _ => {} // Skip deck/graveyard
            }
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
    
                // 1. Escala X: comprimir horizontalmente (simula rotación)
                transform.scale.x = animation.original_scale.x * (1.0 - t).max(0.05);

                // 2. Escala Y: ligeramente más pequeño (perspectiva)
                transform.scale.y = animation.original_scale.y * (1.0 - t * 0.15);

                // 3. Rotación Z: inclinar desde esquina (efecto "página")
                let rotation_angle = -t * 0.5;  // Rotar hasta ~17 grados
                transform.rotation = Quat::from_rotation_z(rotation_angle);

                // 4. Posición: mover hacia esquina inferior derecha
                transform.translation.x = animation.original_position.x + (t * 25.0);
                transform.translation.y = animation.original_position.y - (t * 25.0);
                transform.translation.z = animation.original_position.z + (t * 5.0);
            } else {
                // Segunda mitad: hacer lo inverso
                let t = (progress - 0.5) * 2.0;  // Normalizar a 0.0-1.0

                // 1. Escala X: expandir de vuelta
                transform.scale.x = animation.original_scale.x * (t).max(0.05);

                // 2. Escala Y: volver a tamaño normal
                transform.scale.y = animation.original_scale.y * (0.85 + t * 0.15);

                // 3. Rotación Z: volver a 0
                let rotation_angle = -0.5 * (1.0 - t);
                transform.rotation = Quat::from_rotation_z(rotation_angle);

                // 4. Posición: volver a la posición original
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