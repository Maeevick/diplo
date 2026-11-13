//! A finger on the tablet, or a click on the Mac, as one point in the world.

use bevy::{input::InputSystems, prelude::*, window::PrimaryWindow};

use crate::MainCamera;

/// Where the screen was tapped this frame, in world units.
#[derive(Resource, Default)]
pub struct Tap(pub Option<Vec2>);

/// A browser may follow a touch with a mouse click of its own: clicks this soon after a touch are that echo.
const ECHO_SECONDS: f32 = 0.6;

#[derive(Default)]
struct LastTouch(Option<f32>);

pub fn plugin(app: &mut App) {
    app.init_resource::<Tap>()
        .add_systems(PreUpdate, read.after(InputSystems));
}

fn read(
    mut tap: ResMut<Tap>,
    mut last_touch: Local<LastTouch>,
    time: Res<Time>,
    touches: Res<Touches>,
    mouse: Res<ButtonInput<MouseButton>>,
    window: Single<&Window, With<PrimaryWindow>>,
    camera: Single<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    let now = time.elapsed_secs();
    let touched = touches
        .iter_just_pressed()
        .map(|touch| touch.position())
        .next();
    if touched.is_some() {
        last_touch.0 = Some(now);
    }
    let echo = last_touch.0.is_some_and(|at| now - at < ECHO_SECONDS);
    let clicked = (mouse.just_pressed(MouseButton::Left) && !echo)
        .then(|| window.cursor_position())
        .flatten();
    let (camera, transform) = *camera;
    tap.0 = touched
        .or(clicked)
        .and_then(|position| camera.viewport_to_world_2d(transform, position).ok());
}
