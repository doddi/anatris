use rand::distributions::{Distribution, Standard};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum TetronimoShape {
    IShape,
    JShape,
    LShape,
    OShape,
    SShape,
    TShape,
    ZShape,
}

impl Distribution<TetronimoShape> for Standard {
    fn sample<R: rand::prelude::Rng + ?Sized>(&self, rng: &mut R) -> TetronimoShape {
        match rng.gen_range(0..=5) {
            0 => TetronimoShape::IShape,
            1 => TetronimoShape::JShape,
            2 => TetronimoShape::LShape,
            3 => TetronimoShape::OShape,
            4 => TetronimoShape::SShape,
            5 => TetronimoShape::TShape,
            _ => TetronimoShape::ZShape,
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum TetronimoRotation {
    North,
    East,
    South,
    West,
}

impl TetronimoRotation {
    fn clockwise(&self) -> TetronimoRotation {
        match self {
            TetronimoRotation::North => TetronimoRotation::East,
            TetronimoRotation::East => TetronimoRotation::South,
            TetronimoRotation::South => TetronimoRotation::West,
            TetronimoRotation::West => TetronimoRotation::North,
        }
    }
}

#[derive(Clone)]
pub(crate) struct Tetronimo {
    pub(crate) shape: TetronimoShape,
    rotation: TetronimoRotation,
}

// const I_COLOR: char = '🟦';
// const I_COLOR: char = 'I';
const I_UP: [bool; 4] = [true, true, true, true];
const I_RIGHT: [bool; 4] = [true, true, true, true];

// const J_COLOR: char = '🟪';
// const J_COLOR: char = 'O';
const J_UP: [bool; 6] = [true, true, true, false, false, true];
const J_RIGHT: [bool; 6] = [false, true, false, true, true, true];
const J_DOWN: [bool; 6] = [true, false, false, true, true, true];
const J_LEFT: [bool; 6] = [true, true, true, false, true, false];

// const L_COLOR: char = '🟥';
// const L_COLOR: char = 'L';
const L_UP: [bool; 6] = [false, false, true, true, true, true];
const L_RIGHT: [bool; 6] = [true, false, true, false, true, true];
const L_DOWN: [bool; 6] = [true, true, true, true, false, false];
const L_LEFT: [bool; 6] = [true, true, false, true, false, true];

// const O_COLOR: char = '🟨';
// const O_COLOR: char = 'O';
const O_UP: [bool; 4] = [true, true, true, true];

// const S_COLOR: char = '🟩';
// const S_COLOR: char = 'S';
const S_UP: [bool; 6] = [false, true, true, true, true, false];
const S_RIGHT: [bool; 6] = [true, false, true, true, false, true];

// const T_COLOR: char = '🟫';
// const T_COLOR: char = 'T';
const T_UP: [bool; 6] = [false, true, false, true, true, true];
const T_RIGHT: [bool; 6] = [true, false, true, true, true, false];
const T_DOWN: [bool; 6] = [true, true, true, false, true, false];
const T_LEFT: [bool; 6] = [false, true, true, true, false, true];

// const Z_COLOR: char = '🟧';
// const Z_COLOR: char = 'Z';
const Z_UP: [bool; 6] = [true, true, false, false, true, true];
const Z_RIGHT: [bool; 6] = [false, true, true, true, true, false];

impl Tetronimo {
    pub(crate) fn new(shape: TetronimoShape) -> Self {
        Self {
            shape,
            rotation: TetronimoRotation::North,
        }
    }

    pub(crate) fn random() -> Self {
        Self {
            // shape: TetronimoShape::TShape,
            shape: rand::random(),
            rotation: TetronimoRotation::North,
        }
    }

    pub(crate) fn get_chars(&self) -> (&[bool], usize) {
        match self.shape {
            TetronimoShape::IShape => match self.rotation {
                TetronimoRotation::North => (&I_UP, 1),
                TetronimoRotation::East => (&I_RIGHT, 4),
                TetronimoRotation::South => (&I_UP, 1),
                TetronimoRotation::West => (&I_RIGHT, 4),
            },
            TetronimoShape::JShape => match self.rotation {
                TetronimoRotation::North => (&J_UP, 3),
                TetronimoRotation::East => (&J_RIGHT, 2),
                TetronimoRotation::South => (&J_DOWN, 3),
                TetronimoRotation::West => (&J_LEFT, 2),
            },
            TetronimoShape::LShape => match self.rotation {
                TetronimoRotation::North => (&L_UP, 3),
                TetronimoRotation::East => (&L_RIGHT, 2),
                TetronimoRotation::South => (&L_DOWN, 3),
                TetronimoRotation::West => (&L_LEFT, 2),
            },
            TetronimoShape::OShape => match self.rotation {
                TetronimoRotation::North => (&O_UP, 2),
                TetronimoRotation::East => (&O_UP, 2),
                TetronimoRotation::South => (&O_UP, 2),
                TetronimoRotation::West => (&O_UP, 2),
            },
            TetronimoShape::SShape => match self.rotation {
                TetronimoRotation::North => (&S_UP, 3),
                TetronimoRotation::East => (&S_RIGHT, 2),
                TetronimoRotation::South => (&S_UP, 3),
                TetronimoRotation::West => (&S_RIGHT, 2),
            },
            TetronimoShape::TShape => match self.rotation {
                TetronimoRotation::North => (&T_UP, 3),
                TetronimoRotation::East => (&T_RIGHT, 2),
                TetronimoRotation::South => (&T_DOWN, 3),
                TetronimoRotation::West => (&T_LEFT, 2),
            },
            TetronimoShape::ZShape => match self.rotation {
                TetronimoRotation::North => (&Z_UP, 3),
                TetronimoRotation::East => (&Z_RIGHT, 2),
                TetronimoRotation::South => (&Z_UP, 3),
                TetronimoRotation::West => (&Z_RIGHT, 2),
            },
        }
    }

    pub(crate) fn rotate(&mut self) {
        self.rotation = self.rotation.clockwise();
    }
}
