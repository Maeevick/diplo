//! The rules of the game, with no engine: a dino crosses lanes of carnivores one cell at a time, level after level,
//! until its lives run out.

mod dinos;
mod game;
mod generate;
mod grid;
mod leaderboard;
mod level;
mod random;

pub use dinos::{Carnivore, Herbivore};
pub use game::{Event, Game, LIVES};
pub use grid::{COLUMNS, Cell, Heading, LANE_LENGTH, Step};
pub use leaderboard::{Entry, Initials, Leaderboard, PLACES};
pub use level::{Ground, Hunter, Lane, Level, Obstacle};
