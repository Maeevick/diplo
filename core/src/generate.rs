//! Levels drawn from a seed, harder with each number but never past what a young player can cross.

use crate::random::Random;
use crate::{COLUMNS, Carnivore, Cell, Heading, LANE_LENGTH, Lane, Level, Obstacle};

const MOST_LANES: usize = 12;
const MOST_ROWS: i32 = 36;

impl Level {
    /// Level `number`, from 1, always the same for the same seed, and always crossable from its start.
    pub fn generate(number: u32, seed: u64) -> Level {
        let number = number.max(1);
        let mut random = Random::new(seed ^ u64::from(number).wrapping_mul(0xD1B5_4A32_D192_ED03));
        loop {
            let level = attempt(number, &mut random);
            if level.path_to_goal().is_some() {
                return level;
            }
        }
    }
}

fn attempt(number: u32, random: &mut Random) -> Level {
    let lane_count = (2 + number as usize).min(MOST_LANES);
    let carnivores = carnivores(number);
    let mut lanes = vec![];
    let mut grass = vec![0];
    let mut row = 1;
    let mut full = false;
    while lanes.len() < lane_count && !full {
        let group = 1 + random.below(widest_group(number));
        let before = lanes.len();
        for _ in 0..group {
            let carnivore = carnivores[random.below(carnivores.len() as u32) as usize];
            if lanes.len() == lane_count {
                break;
            }
            if row + carnivore.height() + 1 > MOST_ROWS {
                full = true;
                break;
            }
            lanes.push(lane(number, row, carnivore, random));
            row += carnivore.height();
        }
        if lanes.len() > before {
            grass.push(row);
            row += 1;
        }
    }
    let mut obstacles = vec![];
    for lane in &lanes {
        if random.chance(rock_chance(number)) {
            let column = random.below(COLUMNS as u32) as i32;
            let width = lane.carnivore().width() as f32;
            let free = lane
                .hunters()
                .iter()
                .all(|hunter| !crate::grid::overlap(hunter.x(), width, column as f32, 1.0));
            if free {
                let rock_row = lane.row() + random.below(lane.carnivore().height() as u32) as i32;
                obstacles.push((Cell::new(column, rock_row), Obstacle::Rock));
            }
        }
    }
    let start = Cell::new(COLUMNS / 2, 0);
    for &grass_row in &grass {
        for _ in 0..random.below(most_trees(number) + 1) {
            let cell = Cell::new(random.below(COLUMNS as u32) as i32, grass_row);
            if cell != start {
                obstacles.push((cell, Obstacle::Tree));
            }
        }
    }
    Level::new(row, lanes, obstacles)
}

fn lane(number: u32, row: i32, carnivore: Carnivore, random: &mut Random) -> Lane {
    let heading = if random.chance(0.5) {
        Heading::Right
    } else {
        Heading::Left
    };
    let width = carnivore.width() as f32;
    let count = ((crowding(number) * LANE_LENGTH / width).floor() as usize).max(1);
    let spacing = LANE_LENGTH / count as f32;
    let offset = random.unit() * LANE_LENGTH;
    (0..count).fold(
        Lane::new(row, carnivore, carnivore.pace() * speed_factor(number)),
        |lane, i| lane.with_hunter(offset + i as f32 * spacing, heading),
    )
}

/// New carnivores come one level at a time, the spinosaurus from the start since it is the favourite.
fn carnivores(number: u32) -> &'static [Carnivore] {
    use Carnivore::*;
    match number {
        1 => &[Velociraptor, Spinosaurus],
        2 => &[Velociraptor, Atrociraptor, Spinosaurus],
        3 => &[Velociraptor, Atrociraptor, Spinosaurus, TRex],
        _ => &Carnivore::ALL,
    }
}

fn widest_group(number: u32) -> u32 {
    match number {
        1..=2 => 1,
        3..=5 => 2,
        _ => 3,
    }
}

fn steps_in(number: u32) -> f32 {
    (number - 1) as f32
}

fn speed_factor(number: u32) -> f32 {
    (0.7 + 0.08 * steps_in(number)).min(1.6)
}

/// The share of a lane its carnivores fill.
fn crowding(number: u32) -> f32 {
    (0.2 + 0.03 * steps_in(number)).min(0.45)
}

fn rock_chance(number: u32) -> f32 {
    if number < 2 {
        0.0
    } else {
        (0.1 + 0.04 * steps_in(number)).min(0.4)
    }
}

fn most_trees(number: u32) -> u32 {
    (1 + number / 3).min(4)
}
