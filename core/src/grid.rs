/// The columns the dino can walk on, from 0 on the left.
pub const COLUMNS: i32 = 11;

/// A lane runs round a loop longer than the screen, so that a carnivore leaving one side is fully out of sight
/// before it comes back from the other.
pub const LANE_LENGTH: f32 = 15.0;

/// A square of the level, rows counted upwards from the start.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Cell {
    pub column: i32,
    pub row: i32,
}

impl Cell {
    pub const fn new(column: i32, row: i32) -> Self {
        Self { column, row }
    }

    pub fn after(self, step: Step) -> Cell {
        match step {
            Step::Up => Cell::new(self.column, self.row + 1),
            Step::Down => Cell::new(self.column, self.row - 1),
            Step::Left => Cell::new(self.column - 1, self.row),
            Step::Right => Cell::new(self.column + 1, self.row),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Step {
    Up,
    Down,
    Left,
    Right,
}

impl Step {
    pub const ALL: [Step; 4] = [Step::Up, Step::Down, Step::Left, Step::Right];

    /// The step a press on the pad asks for: along the axis the press is furthest from the pad's centre, and none
    /// on the centre itself. The press is in buttons from the centre, a button being as wide as the centre.
    pub fn pointed(press: (f32, f32)) -> Option<Step> {
        let (across, along) = press;
        if across.abs() <= 0.5 && along.abs() <= 0.5 {
            None
        } else if across.abs() > along.abs() {
            Some(if across > 0.0 {
                Step::Right
            } else {
                Step::Left
            })
        } else {
            Some(if along > 0.0 { Step::Up } else { Step::Down })
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Heading {
    Left,
    Right,
}

impl Heading {
    pub fn sign(self) -> f32 {
        match self {
            Heading::Left => -1.0,
            Heading::Right => 1.0,
        }
    }

    pub fn reversed(self) -> Heading {
        match self {
            Heading::Left => Heading::Right,
            Heading::Right => Heading::Left,
        }
    }
}

/// Whether two stretches of a lane overlap, the lane being a loop.
pub(crate) fn overlap(a: f32, a_length: f32, b: f32, b_length: f32) -> bool {
    let gap = (b - a).rem_euclid(LANE_LENGTH);
    gap < a_length || gap > LANE_LENGTH - b_length
}
