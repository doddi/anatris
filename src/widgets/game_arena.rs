use std::time::Duration;

use anathema::{
    backend::tui::Style,
    component::{Component, Context},
    default_widgets::Canvas,
    geometry::LocalPos,
    state::{State, Value},
    widgets::Elements,
};
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

const GLYPH_WIDTH: u16 = 2;

#[derive(State)]
pub(crate) struct GameArenaComponentState {
    paused: Value<bool>,
}

impl GameArenaComponentState {
    pub(crate) fn new() -> Self {
        Self {
            paused: Value::new(false),
        }
    }
}

#[derive(PartialEq, Eq)]
pub(crate) enum MoveType {
    None,
    Left,
    Right,
}

pub(crate) enum GameArenaComponentMessage {
    Rotate,
    Drop,
    Move(MoveType),
}

pub(crate) struct GameArenaComponent {
    last_fall_update: Duration,
    last_move_update: Duration,

    move_type: MoveType,
    position: LocalPos,
    new_position: LocalPos,

    rotate: bool,

    piece: Tetronimo,
    pieces: Vec<char>,
}

impl GameArenaComponent {
    pub(crate) fn new() -> Self {
        Self {
            last_fall_update: Duration::ZERO,
            last_move_update: Duration::ZERO,

            move_type: MoveType::None,
            position: LocalPos::ZERO,
            new_position: LocalPos::ZERO,

            rotate: false,

            piece: Tetronimo::new(),
            // pieces: Vec::new(),
            pieces: GameArenaComponent::initialise_arena(),
        }
    }

    fn initialise_arena() -> Vec<char> {
        let mut board = Vec::with_capacity(20 * 20);

        for _x in 0..20 {
            for _y in 0..20 {
                let num = rand::thread_rng().gen_range(0..10);
                if num == 2 {
                    board.push('🟧');
                } else {
                    board.push(' ');
                }
            }
        }
        board
    }

    fn draw_tetronimo(&self, canvas: &mut Canvas) {
        let (character, shape, width) = self.piece.get_chars();
        shape.iter().enumerate().for_each(|(offset, present)| {
            if *present {
                let x = ((offset % width) as u16) * 2;
                let y = (offset / width) as u16;
                let local_pos = LocalPos::new(x, y);
                canvas.put(*character, Style::reset(), local_pos + self.position)
            }
        });
    }

    fn clear_tetronimo(&self, canvas: &mut Canvas) {
        let (_character, shape, width) = self.piece.get_chars();
        shape.iter().enumerate().for_each(|(offset, present)| {
            if *present {
                let x = ((offset % width) as u16) * GLYPH_WIDTH;
                let y = (offset / width) as u16;
                let mut local_pos = LocalPos::new(x, y);
                canvas.erase(local_pos + self.position);
                local_pos = LocalPos::new(x + 1, y);
                canvas.erase(local_pos + self.position);
            }
        });
    }

    fn draw_arena(&self, canvas: &mut Canvas) {
        self.pieces.iter().enumerate().for_each(|(offset, piece)| {
            let x = ((offset % 20) * GLYPH_WIDTH as usize) as u16;
            let y = (offset / 2) as u16;
            let local_pos = LocalPos::new(x, y);
            canvas.put(*piece, Style::reset(), local_pos);
        });
    }

    fn move_left(&mut self) {
        if self.position.x >= GLYPH_WIDTH {
            self.new_position.x -= GLYPH_WIDTH;
        }
    }

    fn move_right(&mut self) {
        self.new_position.x += GLYPH_WIDTH;
    }
}

impl Component for GameArenaComponent {
    type State = GameArenaComponentState;
    type Message = GameArenaComponentMessage;

    fn tick(
        &mut self,
        _state: &mut Self::State,
        mut elements: Elements<'_, '_>,
        _context: Context<'_, Self::State>,
        dt: Duration,
    ) {
        let is_paused = extract_bool_attribute(_context, "paused");
        match is_paused {
            Some(true) => (),
            _ => {
                let mut should_rotate = false;
                self.last_fall_update += dt;
                self.last_move_update += dt;

                if self.last_move_update >= Duration::from_millis(200) {
                    self.last_move_update = Duration::ZERO;
                    self.new_position = self.position;
                    match self.move_type {
                        MoveType::None => (),
                        MoveType::Left => self.move_left(),
                        MoveType::Right => self.move_right(),
                    }
                    self.move_type = MoveType::None;

                    if self.rotate {
                        self.rotate = false;
                        should_rotate = true;
                    }
                }

                if self.last_fall_update >= Duration::from_secs(1) {
                    self.last_fall_update = Duration::ZERO;
                    self.new_position.y += 1;
                }

                elements.by_tag("canvas").first(|el, _| {
                    let canvas = el.to::<Canvas>();

                    self.draw_arena(canvas);
                    self.clear_tetronimo(canvas);
                    self.position = self.new_position;
                    if should_rotate {
                        self.piece.rotation = self.piece.rotation.clockwise();
                    }
                    self.draw_tetronimo(canvas);
                });
            }
        }
    }

    fn message(
        &mut self,
        message: Self::Message,
        _state: &mut Self::State,
        mut _elements: Elements<'_, '_>,
        mut _context: Context<'_, Self::State>,
    ) {
        match message {
            GameArenaComponentMessage::Move(move_type) => {
                if self.move_type == MoveType::None {
                    match move_type {
                        MoveType::Left => self.move_type = MoveType::Left,
                        MoveType::Right => self.move_type = MoveType::Right,
                        MoveType::None => todo!(),
                    }
                }
            }
            GameArenaComponentMessage::Rotate => self.rotate = true,
            GameArenaComponentMessage::Drop => todo!(),
        }
    }
}

fn extract_bool_attribute(
    context: Context<GameArenaComponentState>,
    attribute: &str,
) -> Option<bool> {
    let either = context.get_external(attribute);
    match either {
        Some(either) => match either {
            anathema::widgets::expressions::Either::Static(value) => Some(value.to_bool()),
            anathema::widgets::expressions::Either::Dyn(value) => match value.to_common() {
                Some(value) => Some(value.to_bool()),
                _ => None,
            },
        },
        _ => None,
    }
}

////////////////////// TETRONIMO /////////////////////////
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

struct Tetronimo {
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
    fn new() -> Self {
        Self {
            shape: rand::random(),
            rotation: TetronimoRotation::East,
        }
    }

    fn get_chars(&self) -> (&char, &[bool], usize) {
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
}
