//! The game on screen: Bevy draws the core's state and turns taps into steps.

mod art;
mod board;
mod choose;
mod over;
mod pictures;
mod play;
mod storage;
mod tap;
mod text;

use bevy::{camera::ScalingMode, prelude::*};
use diplo_core::{COLUMNS, Cell, Game, Leaderboard};

/// World units a cell spans: one pixel of the pictures is one unit.
pub const CELL: f32 = 16.0;
pub const FIELD_WIDTH: f32 = COLUMNS as f32 * CELL;
const NIGHT: Color = Color::srgb(
    0x1d as f32 / 255.0,
    0x2b as f32 / 255.0,
    0x53 as f32 / 255.0,
);

#[derive(States, Default, Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Screen {
    #[default]
    Choose,
    Play,
    Over,
    Board,
}

/// The game being played, or just lost.
#[derive(Resource)]
pub struct Run(pub Game);

/// The ten best games, and the place of the one just entered.
#[derive(Resource)]
pub struct Board {
    pub leaderboard: Leaderboard,
    pub highlight: Option<usize>,
}

#[derive(Component)]
pub struct MainCamera;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Diplo".into(),
                        fit_canvas_to_parent: true,
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .insert_resource(ClearColor(NIGHT))
        .insert_resource(Board {
            leaderboard: storage::load(),
            highlight: None,
        })
        .add_plugins((pictures::plugin, camera_plugin, text::plugin, tap::plugin))
        .init_state::<Screen>()
        .add_plugins((choose::plugin, play::plugin, over::plugin, board::plugin))
        .run();
}

/// The centre of a cell, in world units, the level's bottom-left corner at the origin.
pub fn centre_of(cell: Cell) -> Vec2 {
    Vec2::new(
        (cell.column as f32 + 0.5) * CELL,
        (cell.row as f32 + 0.5) * CELL,
    )
}

/// What the camera shows, around its own position.
pub fn view_of(projection: &Projection) -> Rect {
    match projection {
        Projection::Orthographic(orthographic) => orthographic.area,
        _ => Rect::default(),
    }
}

/// Spawns the camera while the app is assembled, for the first screen to find it.
fn camera_plugin(app: &mut App) {
    app.world_mut()
        .spawn((
            MainCamera,
            Camera2d,
            Projection::Orthographic(OrthographicProjection {
                scaling_mode: ScalingMode::AutoMin {
                    min_width: FIELD_WIDTH,
                    min_height: 15.0 * CELL,
                },
                ..OrthographicProjection::default_2d()
            }),
            Transform::from_xyz(FIELD_WIDTH / 2.0, 0.0, 0.0),
        ))
        .with_children(|camera| {
            // Curtains either side of the field, hiding carnivores while they run round the loop off screen.
            for side in [-1.0, 1.0] {
                camera.spawn((
                    Sprite::from_color(NIGHT, Vec2::new(1000.0, 8000.0)),
                    Transform::from_xyz(side * (FIELD_WIDTH / 2.0 + 500.0), 0.0, 50.0),
                ));
            }
        });
}

/// Puts the camera back in the middle, for the screens around the game.
pub fn centre_camera(mut camera: Single<&mut Transform, With<MainCamera>>) {
    camera.translation = Vec3::new(FIELD_WIDTH / 2.0, 0.0, 0.0);
}

/// A meadow behind the screens around the game.
pub fn spawn_meadow<S: States>(commands: &mut Commands, pictures: &pictures::Pictures, screen: S) {
    for row in -20..20 {
        for column in 0..COLUMNS {
            commands.spawn((
                Sprite {
                    image: pictures.grass.clone(),
                    color: Color::srgb(0.6, 0.6, 0.6),
                    ..default()
                },
                Transform::from_xyz(
                    (column as f32 + 0.5) * CELL,
                    (row as f32 + 0.5) * CELL,
                    -1.0,
                ),
                DespawnOnExit(screen.clone()),
            ));
        }
    }
}
