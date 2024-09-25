use std::time::Duration;

use anathema::{
    backend::tui::Style,
    component::{Component, Context},
    default_widgets::Canvas,
    geometry::LocalPos,
    state::{State, Value},
    widgets::Elements,
};

use crate::core::tetronimo::Tetronimo;

// TODO: Gameplay logic should be moved to core module
const GLYPH_WIDTH: u16 = 2;
const CANVAS_WIDTH: u16 = 20;
const CANVAS_HEIGHT: u16 = 20;
const CANVAS_WIDTH_IN_GLYPHS: u16 = CANVAS_WIDTH / 2;
const CANVAS_HEIGHT_IN_GLYPHS: u16 = CANVAS_HEIGHT / 2;

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

enum GameState {
    Falling,
    PieceBlocked,
    CheckRows,
    CheckGameOver,
    GameOver,
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

    game_state: GameState,
}

impl GameArenaComponent {
    pub(crate) fn new() -> Self {
        Self {
            last_fall_update: Duration::ZERO,
            last_move_update: Duration::ZERO,

            move_type: MoveType::None,
            position: LocalPos::new(CANVAS_WIDTH / 2, 0),
            new_position: LocalPos::new(CANVAS_WIDTH / 2, 0),

            rotate: false,

            piece: Tetronimo::new(),
            pieces: Vec::new(),

            game_state: GameState::Falling,
        }
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
            let x = (offset as u16 % CANVAS_WIDTH_IN_GLYPHS) * GLYPH_WIDTH;
            let y = offset as u16 / CANVAS_HEIGHT_IN_GLYPHS;
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
        let (_, shape, width) = self.piece.get_chars();
        if shape
            .iter()
            .enumerate()
            .filter(|(offset, present)| {
                let x: u16 = ((offset % width) as u16) * GLYPH_WIDTH;
                let cube_position_on_arena = self.position.x + x;
                **present && (cube_position_on_arena + GLYPH_WIDTH) >= CANVAS_WIDTH
            })
            .count()
            == 0
        {
            self.new_position.x += GLYPH_WIDTH;
        }
    }

    fn perform_fall(&mut self) {
        let (_, shape, width) = self.piece.get_chars();
        if shape
            .iter()
            .enumerate()
            .filter(|(offset, present)| {
                let y: u16 = (offset / width) as u16;
                let cube_position_on_arena = self.position.y + y;
                **present && (cube_position_on_arena + 1) >= CANVAS_HEIGHT
            })
            .count()
            == 0
        {
            self.new_position.y += 1;
        } else {
            self.game_state = GameState::PieceBlocked;
        }
    }

    fn handle_moving_state(&mut self, dt: Duration, mut elements: Elements) {
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
            self.perform_fall();
        }

        elements.by_tag("canvas").first(|el, _| {
            let canvas = el.to::<Canvas>();

            self.draw_arena(canvas);
            self.clear_tetronimo(canvas);
            self.position = self.new_position;
            if should_rotate {
                self.piece.rotate();
            }
            self.draw_tetronimo(canvas);
        });
    }

    fn handle_piece_blocked_state(&mut self) {
        println!("handle_piece_blocked_state");
        self.piece = Tetronimo::new();
        self.position = LocalPos::new(CANVAS_WIDTH / 2, 0);
        self.new_position = LocalPos::new(CANVAS_WIDTH / 2, 0);
        self.game_state = GameState::CheckRows
    }

    fn check_rows(&mut self) {
        self.game_state = GameState::CheckGameOver;
    }

    fn check_game_over(&mut self) {
        self.game_state = GameState::Falling;
    }
}

impl Component for GameArenaComponent {
    type State = GameArenaComponentState;
    type Message = GameArenaComponentMessage;

    fn tick(
        &mut self,
        _state: &mut Self::State,
        mut elements: Elements<'_, '_>,
        context: Context<'_, Self::State>,
        dt: Duration,
    ) {
        let is_paused = extract_bool_attribute(context, "paused");

        match is_paused {
            Some(true) => (),
            _ => match self.game_state {
                GameState::Falling => self.handle_moving_state(dt, elements),
                GameState::PieceBlocked => self.handle_piece_blocked_state(),
                GameState::CheckRows => self.check_rows(),
                GameState::CheckGameOver => self.check_game_over(),
                GameState::GameOver => todo!(),
            },
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
            anathema::widgets::expressions::Either::Dyn(value) => {
                value.to_common().map(|value| value.to_bool())
            }
        },
        _ => None,
    }
}
