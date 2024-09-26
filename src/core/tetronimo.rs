use rand::distributions::{Distribution, Standard};

#[derive(Clone)]
pub(super) enum TetronimoShape {
    IShape,
    JShape,
    LShape,
    OShape,
    SShape,
    TShape,
    ZShape,
}

impl From<&TetronimoShape> for char {
    fn from(value: &TetronimoShape) -> Self {
        match value {
            TetronimoShape::IShape => I_COLOR,
            TetronimoShape::JShape => J_COLOR,
            TetronimoShape::LShape => L_COLOR,
            TetronimoShape::OShape => O_COLOR,
            TetronimoShape::SShape => S_COLOR,
            TetronimoShape::TShape => T_COLOR,
            TetronimoShape::ZShape => Z_COLOR,
        }
    }
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
    shape: TetronimoShape,
    rotation: TetronimoRotation,
}

const I_COLOR: char = '🟦';
const I_UP: [bool; 4] = [true, true, true, true];
const I_RIGHT: [bool; 4] = [true, true, true, true];

const J_COLOR: char = '🟪';
const J_UP: [bool; 6] = [true, true, true, false, false, true];
const J_RIGHT: [bool; 6] = [false, true, false, true, true, true];
const J_DOWN: [bool; 6] = [true, false, false, true, true, true];
const J_LEFT: [bool; 6] = [true, true, true, false, true, false];

const L_COLOR: char = '🟥';
const L_UP: [bool; 6] = [false, false, true, true, true, true];
const L_RIGHT: [bool; 6] = [true, false, true, false, true, true];
const L_DOWN: [bool; 6] = [true, true, true, true, false, false];
const L_LEFT: [bool; 6] = [true, true, false, true, false, true];

const O_COLOR: char = '🟨';
const O_UP: [bool; 4] = [true, true, true, true];

const S_COLOR: char = '🟩';
const S_UP: [bool; 6] = [false, true, true, true, true, false];
const S_RIGHT: [bool; 6] = [true, false, true, true, false, true];

const T_COLOR: char = '🟫';
const T_UP: [bool; 6] = [false, true, false, true, true, true];
const T_RIGHT: [bool; 6] = [true, false, true, true, true, false];
const T_DOWN: [bool; 6] = [true, true, true, false, true, false];
const T_LEFT: [bool; 6] = [false, true, true, true, false, true];

const Z_COLOR: char = '🟧';
const Z_UP: [bool; 6] = [true, true, false, false, true, true];
const Z_RIGHT: [bool; 6] = [false, true, true, true, true, false];

impl Tetronimo {
    pub(crate) fn new() -> Self {
        Self {
            // shape: TetronimoShape::TShape,
            shape: rand::random(),
            rotation: TetronimoRotation::North,
        }
    }

    pub(crate) fn get_chars(&self) -> (&char, &[bool], usize) {
        match self.shape {
            TetronimoShape::IShape => match self.rotation {
                TetronimoRotation::North => (&I_COLOR, &I_UP, 1),
                TetronimoRotation::East => (&I_COLOR, &I_RIGHT, 4),
                TetronimoRotation::South => (&I_COLOR, &I_UP, 1),
                TetronimoRotation::West => (&I_COLOR, &I_RIGHT, 4),
            },
            TetronimoShape::JShape => match self.rotation {
                TetronimoRotation::North => (&J_COLOR, &J_UP, 3),
                TetronimoRotation::East => (&J_COLOR, &J_RIGHT, 2),
                TetronimoRotation::South => (&J_COLOR, &J_DOWN, 3),
                TetronimoRotation::West => (&J_COLOR, &J_LEFT, 2),
            },
            TetronimoShape::LShape => match self.rotation {
                TetronimoRotation::North => (&L_COLOR, &L_UP, 3),
                TetronimoRotation::East => (&L_COLOR, &L_RIGHT, 2),
                TetronimoRotation::South => (&L_COLOR, &L_DOWN, 3),
                TetronimoRotation::West => (&L_COLOR, &L_LEFT, 2),
            },
            TetronimoShape::OShape => match self.rotation {
                TetronimoRotation::North => (&O_COLOR, &O_UP, 2),
                TetronimoRotation::East => (&O_COLOR, &O_UP, 2),
                TetronimoRotation::South => (&O_COLOR, &O_UP, 2),
                TetronimoRotation::West => (&O_COLOR, &O_UP, 2),
            },
            TetronimoShape::SShape => match self.rotation {
                TetronimoRotation::North => (&S_COLOR, &S_UP, 3),
                TetronimoRotation::East => (&S_COLOR, &S_RIGHT, 2),
                TetronimoRotation::South => (&S_COLOR, &S_UP, 3),
                TetronimoRotation::West => (&S_COLOR, &S_RIGHT, 2),
            },
            TetronimoShape::TShape => match self.rotation {
                TetronimoRotation::North => (&T_COLOR, &T_UP, 3),
                TetronimoRotation::East => (&T_COLOR, &T_RIGHT, 2),
                TetronimoRotation::South => (&T_COLOR, &T_DOWN, 3),
                TetronimoRotation::West => (&T_COLOR, &T_LEFT, 2),
            },
            TetronimoShape::ZShape => match self.rotation {
                TetronimoRotation::North => (&Z_COLOR, &Z_UP, 3),
                TetronimoRotation::East => (&Z_COLOR, &Z_RIGHT, 2),
                TetronimoRotation::South => (&Z_COLOR, &Z_UP, 3),
                TetronimoRotation::West => (&Z_COLOR, &Z_RIGHT, 2),
            },
        }
    }

    pub(crate) fn rotate(&mut self) {
        self.rotation = self.rotation.clockwise();
    }
}
