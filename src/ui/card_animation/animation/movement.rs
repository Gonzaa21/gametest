use bevy::prelude::*;
use crate::game::card::component::{Card, PreviousCardPosition, CardPosition, PreviousTranslation};
use crate::ui::card_animation::component::{CardAnimation, AnimationState, AnimationType};

// detect card movement
pub fn detect_movement(
    mut commands: Commands,
    mut card_query: Query<(Entity, &Card, &mut Transform, Option<&PreviousCardPosition>, Option<&PreviousTranslation>)>,
    animation_query: Query<&CardAnimation>,
) {
    for (entity, card, mut transform, previous_pos, previous_translation) in card_query.iter_mut() {
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
                commands.entity(entity).insert(PreviousTranslation(transform.translation));
                continue;
            }
        };

        // obtain previous card translation
        let prev_trans = match previous_translation {
            Some(p) => p.0,
            None => {
                commands.entity(entity).insert(PreviousTranslation(transform.translation));
                continue;
            }
        };
        
        // verify if have changes
        let position_changed = prev_pos != &card.position;
        let translation_changed = prev_trans != transform.translation;

        if !position_changed && !translation_changed {
            continue; // do nothing if not changed
        }

        // detect when the card must animate
        let should_animate_movement = match (prev_pos, &card.position) {
            (CardPosition::Deck, CardPosition::DrawnCard(_)) => true,
            (CardPosition::DrawnCard(_), CardPosition::Graveyard) => true,
            (CardPosition::DrawnCard(_), CardPosition::Hand(_)) => true,
            (CardPosition::Hand(_), CardPosition::Graveyard) => true,
            (CardPosition::Graveyard, CardPosition::DrawnCard(_)) => true,
            _ => false,
        };
    
        if should_animate_movement {
            let target_pos = transform.translation;  // actual position = destine
            transform.translation = prev_trans; // move the card to original position before animating

            commands.entity(entity).insert(CardAnimation {
                animation_type: AnimationType::Movement,
                progress: 0.0,
                duration: 0.25,
                state: AnimationState::Animating,
                original_position: prev_trans,
                original_scale: transform.scale,
                original_rotation: transform.rotation,
                target_position: Some(target_pos),
                delay: 0.0,
                delay_elapsed: 0.0,
            });
        }

        // update previous position/translation
        if position_changed {
            commands.entity(entity).insert(PreviousCardPosition(card.position.clone()));
        }
        if translation_changed {
            commands.entity(entity).insert(PreviousTranslation(transform.translation));
        }
    }
}

pub fn animate_movement(
    mut commands: Commands,
    mut card_query: Query<(Entity, &mut Transform, &mut CardAnimation), With<CardAnimation>>,
    time: Res<Time>,
) {
    for (entity, mut transform, mut animation) in card_query.iter_mut() {
        if animation.animation_type == AnimationType::Movement {
            if let Some(target) = animation.target_position {
                
                // Interpolate from original_position to new position
                let t = animation.progress;
                transform.translation = animation.original_position.lerp(target, t);

                // update progress
                animation.progress += time.delta_secs() / animation.duration;
                
                // when the progress has finished, remove card animation
                if animation.progress >= 1.0 {
                    transform.translation = target;
                    commands.entity(entity).remove::<CardAnimation>();
                }
            }
        }
    }
}