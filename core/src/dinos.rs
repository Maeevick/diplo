/// The dinos the player can choose.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Herbivore {
    Diplodocus,
    Triceratops,
    Ankylosaurus,
}

impl Herbivore {
    pub const ALL: [Herbivore; 3] = [
        Herbivore::Diplodocus,
        Herbivore::Triceratops,
        Herbivore::Ankylosaurus,
    ];
}

/// The hunters, each filling a rectangle of cells: the bigger, the slower.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Carnivore {
    Velociraptor,
    Atrociraptor,
    Spinosaurus,
    TRex,
    Giganotosaurus,
}

impl Carnivore {
    pub const ALL: [Carnivore; 5] = [
        Carnivore::Velociraptor,
        Carnivore::Atrociraptor,
        Carnivore::Spinosaurus,
        Carnivore::TRex,
        Carnivore::Giganotosaurus,
    ];

    pub fn width(self) -> i32 {
        match self {
            Carnivore::Velociraptor => 1,
            Carnivore::Atrociraptor | Carnivore::Spinosaurus => 2,
            Carnivore::TRex | Carnivore::Giganotosaurus => 3,
        }
    }

    pub fn height(self) -> i32 {
        match self {
            Carnivore::Velociraptor | Carnivore::Atrociraptor => 1,
            Carnivore::Spinosaurus | Carnivore::TRex => 2,
            Carnivore::Giganotosaurus => 3,
        }
    }

    /// Cells a second at the gentlest difficulty.
    pub(crate) fn pace(self) -> f32 {
        match self {
            Carnivore::Velociraptor => 2.0,
            Carnivore::Atrociraptor => 1.7,
            Carnivore::Spinosaurus => 1.2,
            Carnivore::TRex => 1.0,
            Carnivore::Giganotosaurus => 0.7,
        }
    }
}
