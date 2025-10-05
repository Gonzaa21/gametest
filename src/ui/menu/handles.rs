use bevy::prelude::*;

use bevy::window::PrimaryWindow;
use crate::ui::menu::component::{PlayButton, ExitButton};
use crate::game::gamestate::AppState;

// handle_play - detect click and change to Setup
// handle_exit - detect click and close game

pub fn handle_button_clicks(
    mouse_input: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    play_button_query: Query<(&Transform, &Sprite), With<PlayButton>>,
    exit_button_query: Query<(&Transform, &Sprite), With<ExitButton>>,
    images: Res<Assets<Image>>,
    mut next_state: ResMut<NextState<AppState>>,
    mut exit: EventWriter<AppExit>,
) {
    // only if click left mouse button
    if !mouse_input.just_pressed(MouseButton::Left) {
        return;
    }
    
    // obtain window, camera, cursor/world position
    let Ok(window) = windows.single() else { return; };
    let Ok((camera, camera_transform)) = camera_query.single() else { return; };
    let Some(cursor_pos) = window.cursor_position() else { return; };
    let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_pos) else { return; };

    // verify colision with play button
    if let Ok((play_transform, sprite)) = play_button_query.single() {
        let handle: &Handle<Image> = &sprite.image; // obtain img sprite
        if let Some(image) = images.get(handle) {
            if detect_button(world_pos, play_transform, image) {
                next_state.set(AppState::Setup);
                info!("Starting game...");
                return;
            }
        }
    }

    // verify colision with exit button
    if let Ok((exit_transform, sprite)) = exit_button_query.single() {
        let handle: &Handle<Image> = &sprite.image;
        if let Some(image) = images.get(handle) {
            if detect_button(world_pos, exit_transform, image) {
                exit.write(AppExit::Success);
                return;
            }
        }
    }
}

// auxiliar function - verify if click is inside the button
fn detect_button(
    cursor_pos: Vec2,
    button_transform: &Transform,
    image: &Image,
) -> bool {
    let size = image.size_f32() * button_transform.scale.truncate(); // obtain img size discarding z
    let half_size = size / 2.0; // obtain half size

    let button_pos = button_transform.translation.truncate(); // obtain button position

    cursor_pos.x >= button_pos.x - half_size.x &&
    cursor_pos.x <= button_pos.x + half_size.x &&
    cursor_pos.y >= button_pos.y - half_size.y &&
    cursor_pos.y <= button_pos.y + half_size.y
}