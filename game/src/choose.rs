//! The first screen: tap the dino to play with, or the cup to see the top ten.

use bevy::prelude::*;
use diplo_core::{Game, Herbivore};

use crate::{
    Board, FIELD_WIDTH, MainCamera, Run, Screen, pictures::Pictures, spawn_meadow, tap::Tap,
    text::PixelText, view_of,
};

#[derive(Component)]
struct Choice(Herbivore);

/// The version in the bottom-right corner, to know which build a phone, a tablet or the Mac is running.
#[derive(Component)]
struct Version;

const ROWS_Y: [f32; 3] = [55.0, 0.0, -55.0];
/// How far above or below a dino a tap still picks it: the whole band, for small fingers.
const REACH: f32 = 26.0;
const CUP_Y: f32 = -100.0;
/// A tap below this line opens the top ten: the whole width under the last dino.
const CUP_LINE: f32 = -84.0;

pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Choose), (crate::centre_camera, spawn))
        .add_systems(
            Update,
            (pick, place_version).run_if(in_state(Screen::Choose)),
        );
}

fn spawn(mut commands: Commands, pictures: Res<Pictures>) {
    spawn_meadow(&mut commands, &pictures, Screen::Choose);
    let middle = FIELD_WIDTH / 2.0;
    commands.spawn((
        PixelText::new("DIPLO", Color::srgb(1.0, 0.93, 0.15)),
        Transform::from_xyz(middle, 100.0, 5.0).with_scale(Vec3::splat(3.0)),
        DespawnOnExit(Screen::Choose),
    ));
    for (herbivore, y) in Herbivore::ALL.into_iter().zip(ROWS_Y) {
        commands.spawn((
            Choice(herbivore),
            Sprite::from_image(pictures.herbivore(herbivore)),
            Transform::from_xyz(middle, y + 4.0, 5.0).with_scale(Vec3::splat(2.5)),
            DespawnOnExit(Screen::Choose),
        ));
        commands.spawn((
            PixelText::new(format!("{herbivore:?}").to_uppercase(), Color::WHITE),
            Transform::from_xyz(middle, y - 21.0, 5.0).with_scale(Vec3::splat(0.75)),
            DespawnOnExit(Screen::Choose),
        ));
    }
    commands.spawn((
        Sprite::from_image(pictures.cup.clone()),
        Transform::from_xyz(middle, CUP_Y, 5.0).with_scale(Vec3::splat(1.75)),
        DespawnOnExit(Screen::Choose),
    ));
    commands.spawn((
        Version,
        PixelText::new(
            format!("V{}", env!("CARGO_PKG_VERSION")),
            Color::srgb(0.76, 0.76, 0.78),
        ),
        Transform::from_xyz(0.0, 0.0, 5.0),
        DespawnOnExit(Screen::Choose),
    ));
}

/// Placed every frame: the camera only knows the screen's size once the first frame is drawn.
fn place_version(
    projection: Single<&Projection, With<MainCamera>>,
    version: Single<(&PixelText, &mut Transform), With<Version>>,
) {
    let (text, mut transform) = version.into_inner();
    transform.translation.x = FIELD_WIDTH - text.width() / 2.0 - 3.0;
    transform.translation.y = view_of(&projection).min.y + 6.0;
}

fn pick(
    mut commands: Commands,
    tap: Res<Tap>,
    time: Res<Time>,
    choices: Query<(&Choice, &Transform)>,
    mut board: ResMut<Board>,
    mut next: ResMut<NextState<Screen>>,
) {
    let Some(tap) = tap.0 else { return };
    if tap.y < CUP_LINE {
        board.highlight = None;
        board.after_game = false;
        next.set(Screen::Board);
        return;
    }
    let picked = choices
        .iter()
        .find(|(_, transform)| (tap.y - transform.translation.y).abs() < REACH);
    if let Some((Choice(herbivore), _)) = picked {
        // The moment of the tap is random enough to make every game different.
        let seed = time.elapsed().as_nanos() as u64;
        commands.insert_resource(Run(Game::new(*herbivore, seed)));
        next.set(Screen::Play);
    }
}
