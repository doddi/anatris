use rand::distributions::{Distribution, Standard};

enum TetronimoShape {
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
        match rng.gen_range(0..=1) {
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

#[derive(Debug)]
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

pub(crate) struct Tetronimo {
    shape: TetronimoShape,
    rotation: TetronimoRotation,
}

const I_COLOR: char = '🟦';
const I_UP: [bool; 4] = [true, true, true, true];
const I_RIGHT: [bool; 4] = [true, true, true, true];
const I_DOWN: [bool; 4] = [true, true, true, true];
const I_LEFT: [bool; 4] = [true, true, true, true];

const J_COLOR: char = '🟪';
const J_UP: [bool; 9] = [false, false, false, true, false, false, true, true, true];
const J_RIGHT: [bool; 9] = [true, true, false, true, false, false, true, false, false];
const J_DOWN: [bool; 9] = [true, true, true, false, false, true, false, false, false];
const J_LEFT: [bool; 9] = [false, false, true, false, false, true, false, true, true];

const L_COLOR: char = '🟥';
const L_UP: [bool; 9] = [false, false, false, false, false, true, true, true, true];
const L_RIGHT: [bool; 9] = [true, false, false, true, false, false, true, true, false];
const L_DOWN: [bool; 9] = [true, true, true, true, false, false, false, false, false];
const L_LEFT: [bool; 9] = [false, true, true, false, false, true, false, false, true];

const O_COLOR: char = '🟨';
const O_UP: [bool; 4] = [true, true, true, true];

const S_COLOR: char = '🟩';
const S_UP: [bool; 9] = [false, false, false, false, true, true, true, true, false];
const S_RIGHT: [bool; 9] = [true, false, false, true, true, false, false, true, false];
const S_DOWN: [bool; 9] = [false, false, false, false, true, true, true, true, false];
const S_LEFT: [bool; 9] = [true, false, false, true, true, false, false, true, false];

const T_COLOR: char = '🟫';
const T_UP: [bool; 9] = [false, false, false, false, true, false, true, true, true];
const T_RIGHT: [bool; 9] = [true, false, false, true, true, false, true, false, false];
const T_DOWN: [bool; 9] = [true, true, true, false, true, false, false, false, false];
const T_LEFT: [bool; 9] = [false, false, true, false, true, true, false, false, true];

const Z_COLOR: char = '🟧';
const Z_UP: [bool; 9] = [false, false, false, true, true, false, false, true, true];
const Z_RIGHT: [bool; 9] = [false, false, true, false, true, true, false, true, false];
const Z_DOWN: [bool; 9] = [false, false, false, true, true, false, false, true, true];
const Z_LEFT: [bool; 9] = [false, false, true, false, true, true, false, true, false];

impl Tetronimo {
    pub(crate) fn new() -> Self {
        Self {
            // shape: TetronimoShape::IShape,
            shape: rand::random(),
            rotation: TetronimoRotation::East,
        }
    }

    pub(crate) fn get_chars(&self) -> (&char, &[bool], usize) {
        match self.shape {
            TetronimoShape::IShape => match self.rotation {
                TetronimoRotation::North => (&I_COLOR, &I_UP, 1),
                TetronimoRotation::East => (&I_COLOR, &I_RIGHT, 4),
                TetronimoRotation::South => (&I_COLOR, &I_DOWN, 1),
                TetronimoRotation::West => (&I_COLOR, &I_LEFT, 4),
            },
            TetronimoShape::JShape => match self.rotation {
                TetronimoRotation::North => (&J_COLOR, &J_UP, 3),
                TetronimoRotation::East => (&J_COLOR, &J_RIGHT, 3),
                TetronimoRotation::South => (&J_COLOR, &J_DOWN, 3),
                TetronimoRotation::West => (&J_COLOR, &J_LEFT, 3),
            },
            TetronimoShape::LShape => match self.rotation {
                TetronimoRotation::North => (&L_COLOR, &L_UP, 3),
                TetronimoRotation::East => (&L_COLOR, &L_RIGHT, 3),
                TetronimoRotation::South => (&L_COLOR, &L_DOWN, 3),
                TetronimoRotation::West => (&L_COLOR, &L_LEFT, 3),
            },
            TetronimoShape::OShape => match self.rotation {
                TetronimoRotation::North => (&O_COLOR, &O_UP, 2),
                TetronimoRotation::East => (&O_COLOR, &O_UP, 2),
                TetronimoRotation::South => (&O_COLOR, &O_UP, 2),
                TetronimoRotation::West => (&O_COLOR, &O_UP, 2),
            },
            TetronimoShape::SShape => match self.rotation {
                TetronimoRotation::North => (&S_COLOR, &S_UP, 3),
                TetronimoRotation::East => (&S_COLOR, &S_RIGHT, 3),
                TetronimoRotation::South => (&S_COLOR, &S_DOWN, 3),
                TetronimoRotation::West => (&S_COLOR, &S_LEFT, 3),
            },
            TetronimoShape::TShape => match self.rotation {
                TetronimoRotation::North => (&T_COLOR, &T_UP, 3),
                TetronimoRotation::East => (&T_COLOR, &T_RIGHT, 3),
                TetronimoRotation::South => (&T_COLOR, &T_DOWN, 3),
                TetronimoRotation::West => (&T_COLOR, &T_LEFT, 3),
            },
            TetronimoShape::ZShape => match self.rotation {
                TetronimoRotation::North => (&Z_COLOR, &Z_UP, 3),
                TetronimoRotation::East => (&Z_COLOR, &Z_RIGHT, 3),
                TetronimoRotation::South => (&Z_COLOR, &Z_DOWN, 3),
                TetronimoRotation::West => (&Z_COLOR, &Z_LEFT, 3),
            },
        }
    }

    pub(crate) fn rotate(&self) {
        self.rotation.clockwise();
    }
}
