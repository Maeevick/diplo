//! The ten best games, the one just entered shining, and a tap to play again.

use bevy::prelude::*;

use crate::{
    Board, FIELD_WIDTH, Screen, pictures::Pictures, spawn_meadow, tap::Tap, text::PixelText,
};

/// Taps made in a rush at the end of a game must not skip the board; a visit from the first screen needs no such wait.
const SETTLE_SECONDS: f32 = 2.5;
const VISIT_SETTLE_SECONDS: f32 = 0.3;

#[derive(Resource)]
struct Since(f32);

#[derive(Component)]
struct Highlighted;

pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Board), (crate::centre_camera, spawn))
        .add_systems(Update, (again, shine).run_if(in_state(Screen::Board)));
}

fn spawn(mut commands: Commands, pictures: Res<Pictures>, board: Res<Board>, time: Res<Time>) {
    commands.insert_resource(Since(time.elapsed_secs()));
    spawn_meadow(&mut commands, &pictures, Screen::Board);
    let middle = FIELD_WIDTH / 2.0;
    commands.spawn((
        PixelText::new("TOP 10", Color::srgb(1.0, 0.93, 0.15)),
        Transform::from_xyz(middle, 100.0, 5.0).with_scale(Vec3::splat(2.0)),
        DespawnOnExit(Screen::Board),
    ));
    for (place, entry) in board.leaderboard.entries().iter().enumerate() {
        let line = format!("{:>2} {} {:>4}", place + 1, entry.initials, entry.score);
        let mut row = commands.spawn((
            PixelText::new(line, Color::WHITE),
            Transform::from_xyz(middle, 72.0 - place as f32 * 16.0, 5.0)
                .with_scale(Vec3::splat(1.5)),
            DespawnOnExit(Screen::Board),
        ));
        if board.highlight == Some(place) {
            row.insert(Highlighted);
        }
    }
}

fn again(
    tap: Res<Tap>,
    time: Res<Time>,
    since: Res<Since>,
    board: Res<Board>,
    mut next: ResMut<NextState<Screen>>,
) {
    let settle = if board.after_game {
        SETTLE_SECONDS
    } else {
        VISIT_SETTLE_SECONDS
    };
    if tap.0.is_some() && time.elapsed_secs() - since.0 >= settle {
        next.set(Screen::Choose);
    }
}

fn shine(time: Res<Time>, mut rows: Query<&mut PixelText, With<Highlighted>>) {
    let lit = ((time.elapsed_secs() * 3.0) as u32).is_multiple_of(2);
    let color = if lit {
        Color::srgb(1.0, 0.93, 0.15)
    } else {
        Color::srgb(1.0, 0.64, 0.0)
    };
    for mut text in &mut rows {
        if text.color != color {
            text.color = color;
        }
    }
}
