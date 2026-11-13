//! Game over: the score, and three letters to enter when it makes the top ten.

use bevy::prelude::*;
use diplo_core::Initials;

use crate::{
    Board, FIELD_WIDTH, Run, Screen, pictures::Pictures, spawn_meadow, storage, tap::Tap,
    text::PixelText,
};

/// Taps made in a rush while the last catch happened must not skip this screen.
const SETTLE_SECONDS: f32 = 2.5;
const SLOT_GAP: f32 = 34.0;
const LETTERS_Y: f32 = -12.0;
const ARROW_GAP: f32 = 30.0;
const CHECK_Y: f32 = -82.0;

#[derive(Resource)]
struct Draft {
    initials: Initials,
    admitted: bool,
    since: f32,
}

#[derive(Component)]
struct Slot(usize);

#[derive(Component, Clone, Copy)]
enum Press {
    Up(usize),
    Down(usize),
    Done,
}

pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Over), (crate::centre_camera, spawn))
        .add_systems(
            Update,
            (press, show_letters).chain().run_if(in_state(Screen::Over)),
        );
}

fn slot_x(slot: usize) -> f32 {
    FIELD_WIDTH / 2.0 + (slot as f32 - 1.0) * SLOT_GAP
}

fn spawn(
    mut commands: Commands,
    pictures: Res<Pictures>,
    run: Res<Run>,
    board: Res<Board>,
    time: Res<Time>,
) {
    let score = run.0.score();
    let admitted = board.leaderboard.admits(score);
    commands.insert_resource(Draft {
        initials: Initials::default(),
        admitted,
        since: time.elapsed_secs(),
    });
    spawn_meadow(&mut commands, &pictures, Screen::Over);
    let middle = FIELD_WIDTH / 2.0;
    let mut text = |text: String, y: f32, scale: f32, color: Color| {
        commands.spawn((
            PixelText::new(text, color),
            Transform::from_xyz(middle, y, 5.0).with_scale(Vec3::splat(scale)),
            DespawnOnExit(Screen::Over),
        ));
    };
    text("GAME OVER".into(), 100.0, 2.0, Color::srgb(1.0, 0.0, 0.3));
    text(score.to_string(), 62.0, 4.0, Color::srgb(1.0, 0.93, 0.15));
    if !admitted {
        commands.spawn((
            Sprite::from_image(pictures.herbivore(run.0.dino())),
            Transform::from_xyz(middle, -20.0, 5.0).with_scale(Vec3::splat(3.0)),
            DespawnOnExit(Screen::Over),
        ));
        return;
    }
    for slot in 0..3 {
        commands.spawn((
            Slot(slot),
            PixelText::new("A", Color::WHITE),
            Transform::from_xyz(slot_x(slot), LETTERS_Y, 5.0).with_scale(Vec3::splat(4.0)),
            DespawnOnExit(Screen::Over),
        ));
        for (press, dy, flip_y) in [
            (Press::Up(slot), ARROW_GAP, false),
            (Press::Down(slot), -ARROW_GAP, true),
        ] {
            commands.spawn((
                press,
                Sprite {
                    image: pictures.arrow.clone(),
                    flip_y,
                    ..default()
                },
                Transform::from_xyz(slot_x(slot), LETTERS_Y + dy, 5.0).with_scale(Vec3::splat(1.5)),
                DespawnOnExit(Screen::Over),
            ));
        }
    }
    commands.spawn((
        Press::Done,
        Sprite::from_image(pictures.check.clone()),
        Transform::from_xyz(middle, CHECK_Y, 5.0).with_scale(Vec3::splat(2.5)),
        DespawnOnExit(Screen::Over),
    ));
}

fn press(
    tap: Res<Tap>,
    time: Res<Time>,
    run: Res<Run>,
    mut draft: ResMut<Draft>,
    mut board: ResMut<Board>,
    buttons: Query<(&Press, &Transform)>,
    mut next: ResMut<NextState<Screen>>,
) {
    let Some(tap) = tap.0 else { return };
    if time.elapsed_secs() - draft.since < SETTLE_SECONDS {
        return;
    }
    if !draft.admitted {
        board.highlight = None;
        next.set(Screen::Board);
        return;
    }
    let reach = |transform: &Transform| 9.0 * transform.scale.x;
    let pressed = buttons
        .iter()
        .find(|(_, transform)| transform.translation.truncate().distance(tap) < reach(transform))
        .map(|(press, _)| *press);
    match pressed {
        Some(Press::Up(slot)) => draft.initials = draft.initials.cycled(slot, 1),
        Some(Press::Down(slot)) => draft.initials = draft.initials.cycled(slot, -1),
        Some(Press::Done) => {
            let (leaderboard, place) = board.leaderboard.record(draft.initials, run.0.score());
            storage::save(&leaderboard);
            *board = Board {
                leaderboard,
                highlight: place,
            };
            next.set(Screen::Board);
        }
        None => {}
    }
}

fn show_letters(draft: Res<Draft>, mut slots: Query<(&Slot, &mut PixelText)>) {
    if !draft.is_changed() {
        return;
    }
    let letters = draft.initials.letters();
    for (Slot(slot), mut text) in &mut slots {
        text.text = letters[*slot].to_string();
    }
}
