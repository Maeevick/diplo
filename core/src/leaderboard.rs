use std::fmt;

pub const PLACES: usize = 10;

/// The names a fresh board starts with, all at the lowest score so the first good game gets on it.
const FRESH: [&str; PLACES] = [
    "SPI", "REX", "GIG", "VEL", "ATR", "DIP", "TRI", "ANK", "STE", "PTE",
];

/// Three capital letters, as on an arcade cabinet.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Initials([u8; 3]);

impl Initials {
    pub fn parse(text: &str) -> Option<Self> {
        match text.as_bytes() {
            &[a, b, c] if [a, b, c].iter().all(u8::is_ascii_uppercase) => Some(Self([a, b, c])),
            _ => None,
        }
    }

    pub fn letters(&self) -> [char; 3] {
        self.0.map(char::from)
    }

    /// The letter at `slot` moved `by` places round the alphabet, Z coming back to A.
    pub fn cycled(self, slot: usize, by: i32) -> Self {
        let mut letters = self.0;
        letters[slot] = b'A' + (i32::from(letters[slot] - b'A') + by).rem_euclid(26) as u8;
        Self(letters)
    }
}

impl Default for Initials {
    fn default() -> Self {
        Self(*b"AAA")
    }
}

impl fmt::Display for Initials {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.letters()
            .iter()
            .try_for_each(|letter| write!(f, "{letter}"))
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Entry {
    pub initials: Initials,
    pub score: u32,
}

/// The ten best games, best first, and older first among equal scores.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Leaderboard {
    entries: Vec<Entry>,
}

impl Leaderboard {
    pub fn fresh() -> Self {
        let entries = FRESH
            .iter()
            .filter_map(|name| Initials::parse(name))
            .map(|initials| Entry { initials, score: 1 })
            .collect();
        Self { entries }
    }

    pub fn entries(&self) -> &[Entry] {
        &self.entries
    }

    pub fn admits(&self, score: u32) -> bool {
        score > 0 && self.entries.last().is_none_or(|last| score >= last.score)
    }

    /// The board with the game on it, and its place, when the score is good enough.
    pub fn record(&self, initials: Initials, score: u32) -> (Self, Option<usize>) {
        if !self.admits(score) {
            return (self.clone(), None);
        }
        let place = self
            .entries
            .iter()
            .position(|entry| entry.score < score)
            .unwrap_or(self.entries.len())
            .min(PLACES - 1);
        let mut entries = self.entries.clone();
        entries.insert(place, Entry { initials, score });
        entries.truncate(PLACES);
        (Self { entries }, Some(place))
    }

    pub fn to_text(&self) -> String {
        self.entries
            .iter()
            .map(|entry| format!("{} {}\n", entry.initials, entry.score))
            .collect()
    }

    /// The board written by `to_text`, or a fresh one when the text is anything else.
    pub fn from_text(text: &str) -> Self {
        let entries: Option<Vec<Entry>> = text
            .lines()
            .map(|line| {
                let (initials, score) = line.split_once(' ')?;
                Some(Entry {
                    initials: Initials::parse(initials)?,
                    score: score.parse().ok()?,
                })
            })
            .collect();
        match entries {
            Some(entries)
                if entries.len() == PLACES
                    && entries
                        .windows(2)
                        .all(|pair| pair[0].score >= pair[1].score) =>
            {
                Self { entries }
            }
            _ => Self::fresh(),
        }
    }
}
