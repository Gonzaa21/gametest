use bevy::prelude::*;
use crate::game::card::component::Card;
use crate::ui::card_animation::component::{CardAnimation, AnimationType, AnimationState};

pub fn animate_deal(
    mut commands: Commands,
    mut card_query: Query<(Entity, &mut Transform, &mut CardAnimation, &mut Card)>,
    time: Res<Time>,
) {
    for (entity, mut transform, mut animation, mut card) in card_query.iter_mut() {
        
        if animation.animation_type == AnimationType::Deal {
            
            // wait delay
            if animation.delay_elapsed < animation.delay {
                animation.delay_elapsed += time.delta_secs();
                continue;
            }
            
            // animate
            if let Some(target) = animation.target_position {
                // Interpolate con easing
                let t = animation.progress * animation.progress * (3.0 - 2.0 * animation.progress);
                transform.translation = animation.original_position.lerp(target, t);
                
                // Update progress
                animation.progress += time.delta_secs() / animation.duration;
                
                // when animation finish
                if animation.progress >= 1.0 {
                    card.is_being_dealt = false;
                    transform.translation = target;  // final position
                    transform.translation.z = 10.0;
                    
                    // to do flip
                    if card.face_up {
                        commands.entity(entity).insert(CardAnimation {
                            animation_type: AnimationType::Flip,
                            progress: 0.0,
                            duration: 0.4,
                            state: AnimationState::Animating,
                            original_scale: transform.scale,
                            original_position: transform.translation,
                            original_rotation: transform.rotation,
                            target_position: None,
                            delay: 0.0,
                            delay_elapsed: 0.0,
                        });
                    } else {
                        // if not flip, remove animation
                        commands.entity(entity).remove::<CardAnimation>();
                    }
                }
            }
        }
    }
}