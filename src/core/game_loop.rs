use anathema::geometry::LocalPos;

use super::tetronimo::{Tetronimo, TetronimoShape};

pub(crate) struct GameLoop {
    arena: Vec<Option<TetronimoShape>>,

    next_piece: Option<TetronimoShape>,
    piece: Tetronimo,
    old_piece: Option<Tetronimo>,

    position: Position,
    old_position: Position,

    arena_size: Position,

    game_state: GameState,

    current_score: u16,
    current_lines: u16,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum MoveActionType {
    None,
    Rotate,
    Drop,
    MoveLeft,
    MoveRight,
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum GameState {
    Paused,
    Start,
    Running,
    Falling,
    Moving(MoveActionType),
    PieceBlocked,
    CheckRows,
    CheckGameOver,
    GameOver,
}

pub(crate) enum GameAction {
    Pause,
    Move(MoveActionType),
}

impl GameLoop {
    pub(crate) fn new(arena_width: usize, arena_height: usize) -> Self {
        Self {
            arena: Vec::with_capacity(arena_width * arena_height),

            next_piece: None,
            piece: Tetronimo::random(),
            old_piece: None,
            position: Position::new(0, 0),
            old_position: Position::new(0, 0),
            arena_size: Position::new(arena_width, arena_height),
            game_state: GameState::Start,

            current_score: 0,
            current_lines: 0,
        }
    }

    pub(crate) fn handle_input(&mut self, game_action: GameAction) {
        match game_action {
            GameAction::Pause => {
                if self.game_state == GameState::Paused {
                    self.game_state = GameState::Running;
                }
            }
            GameAction::Move(move_action) => {
                if self.game_state == GameState::Running {
                    self.game_state = GameState::Moving(move_action)
                }
            }
        }
    }

    pub(crate) fn fall_tick(&mut self) {
        if self.game_state == GameState::Running {
            self.game_state = GameState::Falling
        }
    }

    pub(crate) fn do_state_machine<S, L, T, N>(
        &mut self,
        update_score: S,
        update_line: L,
        update_next: N,
        _update_statistics: T,
    ) where
        S: FnMut(u16),
        L: FnMut(u16),
        N: FnMut(TetronimoShape),
        T: FnMut(u16, u16, u16, u16, u16, u16, u16),
    {
        self.old_position = self.position.clone();
        self.old_piece = Some(self.piece.clone());
        match self.game_state {
            GameState::Paused => (),
            GameState::Start => self.handle_start(update_score, update_line, update_next),
            GameState::Running => (),
            GameState::Falling => self.handle_falling(),
            GameState::Moving(game_move_type) => {
                self.handle_movement_state(&game_move_type);
            }
            GameState::PieceBlocked => self.handle_piece_blocked(update_next),
            GameState::CheckRows => self.handle_check_rows(update_score, update_line),
            GameState::CheckGameOver => self.handle_check_game_over(),
            GameState::GameOver => self.handle_game_over(),
        }
    }

    fn handle_falling(&mut self) {
        let (_, shape, width) = self.piece.get_chars();
        if shape
            .iter()
            .enumerate()
            .filter(|(offset, present)| {
                let y = offset / width;
                let cube_position_on_arena = self.position.y + y;
                **present && (cube_position_on_arena + 1) >= self.arena_size.y
            })
            .count()
            == 0
        {
            self.position.y += 1;
            self.game_state = GameState::Running;
        } else {
            self.game_state = GameState::PieceBlocked;
        }

        // TODO: Need to do the same test but checking against the arena pieces
    }

    fn handle_movement_state(&mut self, game_move_type: &MoveActionType) {
        match game_move_type {
            MoveActionType::None => (),
            MoveActionType::Rotate => self.handle_rotate(),
            MoveActionType::Drop => self.handle_drop(),
            MoveActionType::MoveLeft => self.handle_move_left(),
            MoveActionType::MoveRight => self.handle_move_right(),
        }
        self.game_state = GameState::Running;
    }

    fn handle_rotate(&mut self) {
        self.piece.rotate();
    }

    fn handle_drop(&mut self) {}

    fn handle_move_left(&mut self) {
        let (_, shape, width) = self.piece.get_chars();
        if shape
            .iter()
            .enumerate()
            .filter(|(offset, present)| {
                let x = offset % width;
                let cube_position_on_arena = self.position.x + x;
                **present && cube_position_on_arena == 0
            })
            .count()
            == 0
        {
            self.position.x -= 1;
        }
    }

    fn handle_move_right(&mut self) {
        let (_, shape, width) = self.piece.get_chars();
        if shape
            .iter()
            .enumerate()
            .filter(|(offset, present)| {
                let x = offset % width;
                let cube_position_on_arena = self.position.x + x;
                **present && (cube_position_on_arena + 1) >= self.arena_size.x
            })
            .count()
            == 0
        {
            self.position.x += 1;
        }
    }

    fn handle_piece_blocked<P>(&mut self, update_next_piece: P)
    where
        P: FnMut(TetronimoShape),
    {
        self.create_new_piece(update_next_piece);
        self.game_state = GameState::CheckRows;
    }

    fn handle_check_rows<S: FnMut(u16), L: FnMut(u16)>(
        &mut self,
        mut update_score: S,
        _update_line: L,
    ) {
        // TODO: if rows removed update lines and score
        update_score(5);
        self.game_state = GameState::CheckGameOver;
    }

    fn handle_check_game_over(&mut self) {
        self.game_state = GameState::Running;
    }

    fn handle_start<S, L, P>(
        &mut self,
        mut update_score: S,
        mut update_lines: L,
        update_next_piece: P,
    ) where
        S: FnMut(u16),
        L: FnMut(u16),
        P: FnMut(TetronimoShape),
    {
        self.create_new_piece(update_next_piece);
        self.create_new_arena();
        self.current_score = 0;
        self.current_lines = 0;
        update_score(self.current_score);
        update_lines(self.current_lines);
        self.game_state = GameState::Running;
    }

    fn handle_game_over(&self) {
        todo!()
    }

    fn create_new_piece<P>(&mut self, mut update_next_piece: P)
    where
        P: FnMut(TetronimoShape),
    {
        self.piece = match &self.next_piece {
            Some(piece) => Tetronimo::new(piece.clone()),
            None => Tetronimo::random(),
        };
        self.next_piece = Some(rand::random());
        self.position = Position::new(self.arena_size.x / 2, 0);

        if let Some(next_piece) = &self.next_piece {
            update_next_piece(next_piece.clone());
        }
    }

    fn create_new_arena(&mut self) {
        self.arena.fill(None);
    }

    pub(crate) fn draw_piece<F>(&self, mut func: F)
    where
        F: FnMut(char, LocalPos),
    {
        let (character, shape, width) = self.piece.get_chars();
        shape.iter().enumerate().for_each(|(offset, present)| {
            if *present {
                let x = (offset % width) as u16;
                let y = (offset / width) as u16;
                let local_pos = LocalPos::new(x, y);
                let pos: LocalPos = self.position.clone().into();
                func(*character, local_pos + pos);
            }
        });
    }

    pub(crate) fn clear_piece<F>(&self, mut func: F)
    where
        F: FnMut(LocalPos),
    {
        if let Some(piece) = &self.old_piece {
            let (_, shape, width) = piece.get_chars();
            shape.iter().enumerate().for_each(|(offset, present)| {
                if *present {
                    let x = (offset % width) as u16;
                    let y = (offset / width) as u16;
                    let local_pos = LocalPos::new(x, y);
                    let pos: LocalPos = self.old_position.clone().into();
                    func(local_pos + pos);
                }
            });
        }
    }

    pub(crate) fn draw_arena<F>(&self, mut func: F)
    where
        F: FnMut(char, LocalPos),
    {
        self.arena.iter().enumerate().for_each(|(offset, piece)| {
            let x = offset % self.arena_size.x;
            let y = offset / self.arena_size.y;
            let local_pos = LocalPos::new(x as u16, y as u16);
            if let Some(piece) = piece {
                func(piece.into(), local_pos);
            }
        });
    }
}

#[derive(Debug, Clone)]
struct Position {
    pub x: usize,
    pub y: usize,
}

impl Position {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

impl From<Position> for LocalPos {
    fn from(value: Position) -> Self {
        LocalPos::new(value.x as u16, value.y as u16)
    }
}
