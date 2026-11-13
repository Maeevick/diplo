use crate::{Cell, Herbivore, Level, Step};

pub const LIVES: u8 = 3;

/// A long pause, such as the tablet waking up, must not let carnivores jump through rocks.
const LONGEST_TICK: f32 = 0.1;

#[derive(Clone, Debug, PartialEq)]
pub enum Event {
    Moved(Cell),
    Caught { lives: u8 },
    LevelCompleted { level: u32, score: u32 },
    GameOver { score: u32 },
}

/// One run, from the first level until the last life is lost.
#[derive(Clone, Debug)]
pub struct Game {
    dino: Herbivore,
    seed: u64,
    level_number: u32,
    level: Level,
    player: Cell,
    lives: u8,
    score: u32,
}

impl Game {
    pub fn new(dino: Herbivore, seed: u64) -> Self {
        Self::on(dino, Level::generate(1, seed), seed)
    }

    /// A game whose first level is given rather than generated; the next ones come from the seed.
    pub fn on(dino: Herbivore, level: Level, seed: u64) -> Self {
        Self {
            dino,
            seed,
            level_number: 1,
            player: level.start(),
            level,
            lives: LIVES,
            score: 0,
        }
    }

    pub fn dino(&self) -> Herbivore {
        self.dino
    }

    pub fn level(&self) -> &Level {
        &self.level
    }

    pub fn level_number(&self) -> u32 {
        self.level_number
    }

    pub fn player(&self) -> Cell {
        self.player
    }

    pub fn lives(&self) -> u8 {
        self.lives
    }

    pub fn score(&self) -> u32 {
        self.score
    }

    pub fn is_over(&self) -> bool {
        self.lives == 0
    }

    pub fn step(&mut self, step: Step) -> Vec<Event> {
        let next = self.player.after(step);
        if self.is_over() || !self.level.is_open(next) {
            return vec![];
        }
        self.player = next;
        let mut events = vec![Event::Moved(next)];
        if self.level.catches(next) {
            events.push(self.caught());
        } else if next.row == self.level.goal_row() {
            events.push(self.complete());
        }
        events
    }

    pub fn tick(&mut self, seconds: f32) -> Vec<Event> {
        if self.is_over() {
            return vec![];
        }
        self.level.advance(seconds.min(LONGEST_TICK));
        if self.level.catches(self.player) {
            vec![self.caught()]
        } else {
            vec![]
        }
    }

    fn caught(&mut self) -> Event {
        self.lives -= 1;
        self.player = self.level.start();
        if self.is_over() {
            Event::GameOver { score: self.score }
        } else {
            Event::Caught { lives: self.lives }
        }
    }

    fn complete(&mut self) -> Event {
        let completed = self.level_number;
        self.score += completed;
        self.level_number += 1;
        self.level = Level::generate(self.level_number, self.seed);
        self.player = self.level.start();
        Event::LevelCompleted {
            level: completed,
            score: self.score,
        }
    }
}
