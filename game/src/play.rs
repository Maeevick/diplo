//! The game itself: the level drawn from the core's state, taps turned into steps, and a little drama around catches
//! and finished levels.

use bevy::prelude::*;
use diplo_core::{COLUMNS, Event, Game, Ground, Heading, LANE_LENGTH, LIVES, Step};

use crate::{
    CELL, FIELD_WIDTH, MainCamera, Run, Screen, centre_of, pictures::Pictures, tap::Tap,
    text::PixelText, view_of,
};

/// World units a second the dino hops at, so a step reads as a hop rather than a teleport.
const HOP_SPEED: f32 = CELL * 14.0;
const CAUGHT_PAUSE: f32 = 0.8;
const BLINK_SECONDS: f32 = 1.2;
const LEVEL_PAUSE: f32 = 1.4;
const GAME_OVER_PAUSE: f32 = 1.6;
const HUD_HEIGHT: f32 = 16.0;
/// Grass drawn beyond the level, so the screen never shows the void.
const MARGIN_ROWS: i32 = 14;

#[derive(Component)]
struct InLevel;

#[derive(Component)]
struct HunterView {
    lane: usize,
    index: usize,
    /// The copy drawn one loop behind, showing the part of a carnivore that has already come back on the left.
    ghost: bool,
}

#[derive(Component)]
struct DinoView;

/// The strip along the top: a dark band, a drumstick a life, and the score.
#[derive(Component, Clone, Copy, PartialEq)]
enum Hud {
    Band,
    Life(u8),
    Score,
}

#[derive(Component)]
struct Banner(Timer);

/// Everything the screen keeps between frames while a game runs.
#[derive(Resource)]
struct Show {
    pause: Timer,
    blink: Timer,
    facing: Heading,
    target: Vec2,
    /// A finished level stays on screen until the pause ends, then the next one replaces it.
    swap_pending: bool,
    ending: Option<Timer>,
    happened: Vec<Event>,
}

pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Play), enter).add_systems(
        Update,
        (
            (run_hunters, walk, react).chain(),
            (place_hunters, place_dino, follow, place_hud, fade_banners),
        )
            .chain()
            .run_if(in_state(Screen::Play)),
    );
}

fn finished(seconds: f32) -> Timer {
    let mut timer = Timer::from_seconds(seconds, TimerMode::Once);
    timer.finish();
    timer
}

fn enter(
    mut commands: Commands,
    pictures: Res<Pictures>,
    run: Res<Run>,
    camera: Single<Entity, With<MainCamera>>,
) {
    let game = &run.0;
    commands.insert_resource(Show {
        pause: finished(0.0),
        blink: finished(0.0),
        facing: Heading::Right,
        target: centre_of(game.level().start()),
        swap_pending: false,
        ending: None,
        happened: vec![],
    });
    spawn_level(&mut commands, &pictures, game);
    commands.spawn((
        DinoView,
        Sprite::from_image(pictures.herbivore(game.dino())),
        Transform::from_translation(centre_of(game.level().start()).extend(3.0)),
        DespawnOnExit(Screen::Play),
    ));
    let camera = *camera;
    commands.spawn((
        Hud::Band,
        Sprite::from_color(
            Color::srgba(0.0, 0.0, 0.0, 0.55),
            Vec2::new(FIELD_WIDTH, HUD_HEIGHT),
        ),
        Transform::from_xyz(0.0, 0.0, 40.0),
        ChildOf(camera),
        DespawnOnExit(Screen::Play),
    ));
    for life in 0..LIVES {
        commands.spawn((
            Hud::Life(life),
            Sprite::from_image(pictures.drumstick.clone()),
            Transform::from_xyz(0.0, 0.0, 41.0).with_scale(Vec3::splat(0.8)),
            ChildOf(camera),
            DespawnOnExit(Screen::Play),
        ));
    }
    commands.spawn((
        Hud::Score,
        PixelText::new("0", Color::WHITE),
        Transform::from_xyz(0.0, 0.0, 41.0).with_scale(Vec3::splat(1.5)),
        ChildOf(camera),
        DespawnOnExit(Screen::Play),
    ));
}

fn spawn_level(commands: &mut Commands, pictures: &Pictures, game: &Game) {
    let level = game.level();
    for row in -MARGIN_ROWS..level.rows() + MARGIN_ROWS {
        let inside = (0..level.rows()).contains(&row);
        let (image, color) = match (inside, level.ground(row)) {
            (false, _) => (pictures.grass.clone(), Color::srgb(0.45, 0.45, 0.45)),
            (true, Ground::Track) => (pictures.track.clone(), Color::WHITE),
            (true, Ground::Grass) if row == level.goal_row() => {
                (pictures.flowers.clone(), Color::WHITE)
            }
            (true, Ground::Grass) => (pictures.grass.clone(), Color::WHITE),
        };
        for column in 0..COLUMNS {
            commands.spawn((
                InLevel,
                Sprite {
                    image: image.clone(),
                    color,
                    ..default()
                },
                Transform::from_xyz((column as f32 + 0.5) * CELL, (row as f32 + 0.5) * CELL, 0.0),
                DespawnOnExit(Screen::Play),
            ));
        }
    }
    for (cell, obstacle) in level.obstacles() {
        commands.spawn((
            InLevel,
            Sprite::from_image(pictures.obstacle(obstacle)),
            Transform::from_translation(centre_of(cell).extend(1.0)),
            DespawnOnExit(Screen::Play),
        ));
    }
    for (lane_index, lane) in level.lanes().iter().enumerate() {
        for index in 0..lane.hunters().len() {
            for ghost in [false, true] {
                commands.spawn((
                    InLevel,
                    HunterView {
                        lane: lane_index,
                        index,
                        ghost,
                    },
                    Sprite::from_image(pictures.carnivore(lane.carnivore())),
                    Transform::from_xyz(-100.0, 0.0, 2.0),
                    DespawnOnExit(Screen::Play),
                ));
            }
        }
    }
}

fn run_hunters(
    time: Res<Time>,
    mut show: ResMut<Show>,
    mut run: ResMut<Run>,
    mut next: ResMut<NextState<Screen>>,
) {
    show.pause.tick(time.delta());
    show.blink.tick(time.delta());
    if let Some(ending) = show.ending.as_mut() {
        if ending.tick(time.delta()).is_finished() {
            next.set(Screen::Over);
        }
        return;
    }
    if show.pause.is_finished() && !show.swap_pending {
        let events = run.0.tick(time.delta_secs());
        show.happened.extend(events);
    }
}

fn walk(
    tap: Res<Tap>,
    keys: Res<ButtonInput<KeyCode>>,
    mut show: ResMut<Show>,
    mut run: ResMut<Run>,
) {
    if !show.pause.is_finished() || show.swap_pending || show.ending.is_some() {
        return;
    }
    let game = &mut run.0;
    let tapped = tap
        .0
        .and_then(|tap| Step::towards(game.player(), (tap.x / CELL, tap.y / CELL)));
    let pressed = [
        (KeyCode::ArrowUp, Step::Up),
        (KeyCode::ArrowDown, Step::Down),
        (KeyCode::ArrowLeft, Step::Left),
        (KeyCode::ArrowRight, Step::Right),
    ]
    .into_iter()
    .find(|(key, _)| keys.just_pressed(*key))
    .map(|(_, step)| step);
    let Some(step) = tapped.or(pressed) else {
        return;
    };
    match step {
        Step::Left => show.facing = Heading::Left,
        Step::Right => show.facing = Heading::Right,
        Step::Up | Step::Down => {}
    }
    let events = game.step(step);
    show.happened.extend(events);
}

fn react(
    mut commands: Commands,
    pictures: Res<Pictures>,
    run: Res<Run>,
    mut show: ResMut<Show>,
    level_view: Query<Entity, With<InLevel>>,
    camera: Single<Entity, With<MainCamera>>,
    mut dino: Single<&mut Transform, With<DinoView>>,
) {
    let game = &run.0;
    let start = centre_of(game.level().start());
    for event in std::mem::take(&mut show.happened) {
        match event {
            Event::Moved(cell) => show.target = centre_of(cell),
            Event::Caught { .. } => {
                show.pause = Timer::from_seconds(CAUGHT_PAUSE, TimerMode::Once);
                show.blink = Timer::from_seconds(BLINK_SECONDS, TimerMode::Once);
                show.target = start;
                dino.translation = start.extend(3.0);
            }
            Event::LevelCompleted { level, .. } => {
                show.pause = Timer::from_seconds(LEVEL_PAUSE, TimerMode::Once);
                show.swap_pending = true;
                commands.spawn((
                    Banner(Timer::from_seconds(LEVEL_PAUSE, TimerMode::Once)),
                    PixelText::new(format!("+{level}"), Color::srgb(1.0, 0.93, 0.15)),
                    Transform::from_xyz(0.0, 20.0, 45.0).with_scale(Vec3::splat(5.0)),
                    ChildOf(*camera),
                    DespawnOnExit(Screen::Play),
                ));
            }
            Event::GameOver { .. } => {
                show.blink = Timer::from_seconds(GAME_OVER_PAUSE, TimerMode::Once);
                show.ending = Some(Timer::from_seconds(GAME_OVER_PAUSE, TimerMode::Once));
            }
        }
    }
    if show.swap_pending && show.pause.is_finished() {
        show.swap_pending = false;
        for entity in &level_view {
            commands.entity(entity).despawn();
        }
        spawn_level(&mut commands, &pictures, game);
        show.target = start;
        dino.translation = start.extend(3.0);
    }
}

fn place_hunters(
    run: Res<Run>,
    show: Res<Show>,
    mut views: Query<(&HunterView, &mut Transform, &mut Sprite)>,
) {
    if show.swap_pending {
        return;
    }
    let level = run.0.level();
    for (view, mut transform, mut sprite) in &mut views {
        let Some(lane) = level.lanes().get(view.lane) else {
            continue;
        };
        let Some(hunter) = lane.hunters().get(view.index) else {
            continue;
        };
        let width = lane.carnivore().width() as f32;
        let height = lane.carnivore().height() as f32;
        let x = if view.ghost {
            hunter.x() - LANE_LENGTH
        } else {
            hunter.x()
        };
        transform.translation.x = (x + width / 2.0) * CELL;
        transform.translation.y = (lane.row() as f32 + height / 2.0) * CELL;
        sprite.flip_x = hunter.heading() == Heading::Left;
    }
}

fn place_dino(
    time: Res<Time>,
    show: Res<Show>,
    dino: Single<(&mut Transform, &mut Sprite, &mut Visibility), With<DinoView>>,
) {
    let (mut transform, mut sprite, mut visibility) = dino.into_inner();
    let here = transform.translation.truncate();
    let hop = HOP_SPEED * time.delta_secs();
    let there = if here.distance(show.target) <= hop {
        show.target
    } else {
        here + (show.target - here).normalize() * hop
    };
    transform.translation = there.extend(3.0);
    sprite.flip_x = show.facing == Heading::Left;
    let blinking =
        !show.blink.is_finished() && ((show.blink.elapsed_secs() * 10.0) as u32).is_multiple_of(2);
    *visibility = if blinking {
        Visibility::Hidden
    } else {
        Visibility::Inherited
    };
}

fn follow(
    time: Res<Time>,
    run: Res<Run>,
    dino: Single<&Transform, (With<DinoView>, Without<MainCamera>)>,
    camera: Single<(&mut Transform, &Projection), With<MainCamera>>,
) {
    let (mut camera, projection) = camera.into_inner();
    let half = view_of(projection).height() / 2.0;
    let lowest = half;
    let highest = run.0.level().rows() as f32 * CELL + HUD_HEIGHT - half;
    let wanted = if highest < lowest {
        (lowest + highest) / 2.0
    } else {
        dino.translation.y.clamp(lowest, highest)
    };
    let ease = (8.0 * time.delta_secs()).min(1.0);
    camera.translation.x = FIELD_WIDTH / 2.0;
    camera.translation.y += (wanted - camera.translation.y) * ease;
}

fn place_hud(
    run: Res<Run>,
    projection: Single<&Projection, With<MainCamera>>,
    mut items: Query<(
        &Hud,
        &mut Transform,
        &mut Visibility,
        Option<&mut PixelText>,
    )>,
) {
    let game = &run.0;
    let middle = view_of(&projection).max.y - HUD_HEIGHT / 2.0;
    for (hud, mut transform, mut visibility, text) in &mut items {
        transform.translation.y = middle;
        match (*hud, text) {
            (Hud::Life(life), _) => {
                transform.translation.x = -FIELD_WIDTH / 2.0 + 9.0 + f32::from(life) * 14.0;
                *visibility = if life < game.lives() {
                    Visibility::Inherited
                } else {
                    Visibility::Hidden
                };
            }
            (Hud::Score, Some(mut text)) => {
                let shown = game.score().to_string();
                if text.text != shown {
                    text.text = shown;
                }
                transform.translation.x =
                    FIELD_WIDTH / 2.0 - 4.0 - text.width() * transform.scale.x / 2.0;
            }
            _ => {}
        }
    }
}

fn fade_banners(
    mut commands: Commands,
    time: Res<Time>,
    mut banners: Query<(Entity, &mut Banner)>,
) {
    for (entity, mut banner) in &mut banners {
        if banner.0.tick(time.delta()).is_finished() {
            commands.entity(entity).despawn();
        }
    }
}
