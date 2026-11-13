use std::collections::{BTreeMap, HashMap, HashSet, VecDeque};
use std::ops::Range;

use crate::grid::overlap;
use crate::{COLUMNS, Carnivore, Cell, Heading, LANE_LENGTH, Step};

/// How much of its cell the dino's body fills on each side: a tail or a neck brushing a carnivore does not count.
const DINO_MARGIN: f32 = 0.2;
/// How much of a carnivore's length is drawn but does not bite, at each end.
const HUNTER_MARGIN: f32 = 0.15;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Obstacle {
    Tree,
    Rock,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ground {
    /// Where the dino is safe.
    Grass,
    /// Where carnivores run.
    Track,
}

/// One carnivore running along its lane, `x` being its tail end or its snout, whichever is on the left.
#[derive(Clone, Debug, PartialEq)]
pub struct Hunter {
    x: f32,
    heading: Heading,
}

impl Hunter {
    pub fn x(&self) -> f32 {
        self.x
    }

    pub fn heading(&self) -> Heading {
        self.heading
    }
}

/// Rows where one kind of carnivore runs, as tall as the carnivore.
#[derive(Clone, Debug, PartialEq)]
pub struct Lane {
    row: i32,
    carnivore: Carnivore,
    speed: f32,
    hunters: Vec<Hunter>,
}

impl Lane {
    /// A lane from `row` upwards, its carnivores running at `speed` cells a second.
    pub fn new(row: i32, carnivore: Carnivore, speed: f32) -> Self {
        Self {
            row,
            carnivore,
            speed,
            hunters: vec![],
        }
    }

    pub fn with_hunter(mut self, x: f32, heading: Heading) -> Self {
        self.hunters.push(Hunter {
            x: x.rem_euclid(LANE_LENGTH),
            heading,
        });
        self
    }

    pub fn row(&self) -> i32 {
        self.row
    }

    pub fn rows(&self) -> Range<i32> {
        self.row..self.row + self.carnivore.height()
    }

    pub fn carnivore(&self) -> Carnivore {
        self.carnivore
    }

    pub fn speed(&self) -> f32 {
        self.speed
    }

    pub fn hunters(&self) -> &[Hunter] {
        &self.hunters
    }

    fn width(&self) -> f32 {
        self.carnivore.width() as f32
    }

    /// Each carnivore runs on, or turns round when a rock or another carnivore is in its way.
    fn advance(&mut self, seconds: f32, rocks: &[f32]) {
        let width = self.width();
        for i in 0..self.hunters.len() {
            let hunter = &self.hunters[i];
            let next =
                (hunter.x + hunter.heading.sign() * self.speed * seconds).rem_euclid(LANE_LENGTH);
            let blocked = rocks
                .iter()
                .any(|&column| overlap(next, width, column, 1.0))
                || self
                    .hunters
                    .iter()
                    .enumerate()
                    .any(|(j, other)| j != i && overlap(next, width, other.x, width));
            let hunter = &mut self.hunters[i];
            if blocked {
                hunter.heading = hunter.heading.reversed();
            } else {
                hunter.x = next;
            }
        }
    }

    fn catches(&self, cell: Cell) -> bool {
        let bite = self.width() - 2.0 * HUNTER_MARGIN;
        self.rows().contains(&cell.row)
            && self.hunters.iter().any(|hunter| {
                overlap(
                    hunter.x + HUNTER_MARGIN,
                    bite,
                    cell.column as f32 + DINO_MARGIN,
                    1.0 - 2.0 * DINO_MARGIN,
                )
            })
    }
}

/// The ground to cross, from the start row at the bottom to the goal row at the top.
#[derive(Clone, Debug, PartialEq)]
pub struct Level {
    rows: i32,
    lanes: Vec<Lane>,
    obstacles: BTreeMap<Cell, Obstacle>,
}

impl Level {
    pub fn new(
        rows: i32,
        lanes: Vec<Lane>,
        obstacles: impl IntoIterator<Item = (Cell, Obstacle)>,
    ) -> Self {
        Self {
            rows,
            lanes,
            obstacles: obstacles.into_iter().collect(),
        }
    }

    pub fn rows(&self) -> i32 {
        self.rows
    }

    pub fn goal_row(&self) -> i32 {
        self.rows - 1
    }

    pub fn start(&self) -> Cell {
        Cell::new(COLUMNS / 2, 0)
    }

    pub fn lanes(&self) -> &[Lane] {
        &self.lanes
    }

    pub fn obstacles(&self) -> impl Iterator<Item = (Cell, Obstacle)> + '_ {
        self.obstacles
            .iter()
            .map(|(&cell, &obstacle)| (cell, obstacle))
    }

    pub fn ground(&self, row: i32) -> Ground {
        if self.lanes.iter().any(|lane| lane.rows().contains(&row)) {
            Ground::Track
        } else {
            Ground::Grass
        }
    }

    /// Inside the level and free of trees and rocks.
    pub fn is_open(&self, cell: Cell) -> bool {
        (0..COLUMNS).contains(&cell.column)
            && (0..self.rows).contains(&cell.row)
            && !self.obstacles.contains_key(&cell)
    }

    /// The shortest walk from the start to the goal row, around trees, rocks and carnivores as they stand now.
    pub fn path_to_goal(&self) -> Option<Vec<Step>> {
        let start = self.start();
        if !self.is_walkable(start) {
            return None;
        }
        let mut came_from: HashMap<Cell, (Cell, Step)> = HashMap::new();
        let mut seen = HashSet::from([start]);
        let mut queue = VecDeque::from([start]);
        while let Some(cell) = queue.pop_front() {
            if cell.row == self.goal_row() {
                let mut steps = vec![];
                let mut at = cell;
                while let Some(&(from, step)) = came_from.get(&at) {
                    steps.push(step);
                    at = from;
                }
                steps.reverse();
                return Some(steps);
            }
            for step in Step::ALL {
                let next = cell.after(step);
                if self.is_walkable(next) && seen.insert(next) {
                    came_from.insert(next, (cell, step));
                    queue.push_back(next);
                }
            }
        }
        None
    }

    pub(crate) fn catches(&self, cell: Cell) -> bool {
        self.lanes.iter().any(|lane| lane.catches(cell))
    }

    pub(crate) fn advance(&mut self, seconds: f32) {
        for lane in &mut self.lanes {
            let rocks: Vec<f32> = self
                .obstacles
                .keys()
                .filter(|cell| lane.rows().contains(&cell.row))
                .map(|cell| cell.column as f32)
                .collect();
            lane.advance(seconds, &rocks);
        }
    }

    fn is_walkable(&self, cell: Cell) -> bool {
        self.is_open(cell) && !self.catches(cell)
    }
}
