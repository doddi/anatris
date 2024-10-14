use anathema::geometry::LocalPos;
use smol::channel::Sender;

use crate::GlobalStateManagementMessage;

use super::tetronimo::{Tetronimo, TetronimoShape};

pub(crate) struct GameLoop {
    arena: Vec<Option<TetronimoShape>>,

    next_piece: Option<TetronimoShape>,
    piece: Tetronimo,

    position: Position,
    old_position: Position,

    arena_size: Position,

    game_state: GameLoopState,

    current_score: u16,
    current_lines: u16,
    shapes_statistics: ShapeStatistics,

    tx: Sender<GlobalStateManagementMessage>,
}

#[derive(Clone, Copy, Default)]
pub(crate) struct ShapeStatistics {
    pub(crate) i_count: u16,
    pub(crate) j_count: u16,
    pub(crate) l_count: u16,
    pub(crate) o_count: u16,
    pub(crate) t_count: u16,
    pub(crate) s_count: u16,
    pub(crate) z_count: u16,
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
pub(crate) enum GameLoopState {
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
    pub(crate) fn new(
        arena_width: usize,
        arena_height: usize,
        tx: Sender<GlobalStateManagementMessage>,
    ) -> Self {
        Self {
            arena: vec![None; arena_width * arena_height],

            next_piece: None,
            piece: Tetronimo::random(),
            position: Position::new(0, 0),
            old_position: Position::new(0, 0),
            arena_size: Position::new(arena_width, arena_height),
            game_state: GameLoopState::Start,

            current_score: 0,
            current_lines: 0,
            shapes_statistics: ShapeStatistics::default(),

            tx,
        }
    }

    pub(crate) fn handle_input(&mut self, game_action: GameAction) {
        match game_action {
            GameAction::Pause => {
                if self.game_state == GameLoopState::Paused {
                    self.game_state = GameLoopState::Running;
                }
            }
            GameAction::Move(move_action) => {
                if self.game_state == GameLoopState::Running {
                    self.game_state = GameLoopState::Moving(move_action)
                }
            }
        }
    }

    pub(crate) fn fall_tick(&mut self) {
        if self.game_state == GameLoopState::Running {
            self.game_state = GameLoopState::Falling
        }
    }

    pub(crate) fn do_state_machine<S, L, T, N>(
        &mut self,
        update_score: S,
        update_line: L,
        update_next: N,
        update_statistics: T,
    ) where
        S: FnMut(u16),
        L: FnMut(u16),
        N: FnMut(TetronimoShape),
        T: FnMut(ShapeStatistics),
    {
        self.old_position = self.position.clone();
        match self.game_state {
            GameLoopState::Paused => (),
            GameLoopState::Start => {
                self.handle_start(update_score, update_line, update_next, update_statistics)
            }
            GameLoopState::Running => (),
            GameLoopState::Falling => self.handle_falling(),
            GameLoopState::Moving(game_move_type) => {
                self.handle_movement_state(&game_move_type);
            }
            GameLoopState::PieceBlocked => self.handle_piece_blocked(),
            GameLoopState::CheckRows => {
                self.handle_check_rows(update_score, update_line, update_statistics)
            }
            GameLoopState::CheckGameOver => {
                self.handle_check_game_over(update_next, update_statistics)
            }
            GameLoopState::GameOver => self.handle_game_over(),
        }
    }

    fn handle_start<S, L, P, T>(
        &mut self,
        mut update_score: S,
        mut update_lines: L,
        update_next_piece: P,
        update_statistics: T,
    ) where
        S: FnMut(u16),
        L: FnMut(u16),
        P: FnMut(TetronimoShape),
        T: FnMut(ShapeStatistics),
    {
        self.create_new_piece(update_next_piece, update_statistics);
        self.create_new_arena();
        self.current_score = 0;
        self.current_lines = 0;
        update_score(self.current_score);
        update_lines(self.current_lines);
        self.game_state = GameLoopState::Running;
    }

    fn handle_falling(&mut self) {
        let (shape, width) = self.piece.get_chars();
        if shape
            .iter()
            .enumerate()
            .filter(|(offset, present)| {
                let x = self.position.x + offset % width;
                let y = ((self.position.y) + offset / width) + 1;
                let array_pos = (self.arena_size.x * y) + x;
                **present && (y >= self.arena_size.y || self.arena[array_pos].is_some())
            })
            .count()
            == 0
        {
            self.position.y += 1;
            self.game_state = GameLoopState::Running;
        } else {
            self.game_state = GameLoopState::PieceBlocked;
        }
    }

    fn handle_movement_state(&mut self, game_move_type: &MoveActionType) {
        match game_move_type {
            MoveActionType::None => (),
            MoveActionType::Rotate => self.handle_rotate(),
            MoveActionType::Drop => self.handle_drop(),
            MoveActionType::MoveLeft => self.handle_move_left(),
            MoveActionType::MoveRight => self.handle_move_right(),
        }
        self.game_state = GameLoopState::Running;
    }

    fn handle_rotate(&mut self) {
        self.piece.rotate();
    }

    fn handle_drop(&mut self) {}

    fn handle_move_left(&mut self) {
        let (shape, width) = self.piece.get_chars();
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
        let (shape, width) = self.piece.get_chars();
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

    fn handle_piece_blocked(&mut self) {
        self.add_piece_to_arena();
        self.game_state = GameLoopState::CheckRows;
    }

    fn handle_check_rows<S, L, T>(
        &mut self,
        mut update_score: S,
        mut update_line: L,
        _update_statistics: T,
    ) where
        S: FnMut(u16),
        L: FnMut(u16),
        T: FnMut(ShapeStatistics),
    {
        let complete_row = self.remove_complete_rows();

        if complete_row > 0 {
            self.current_score += complete_row * complete_row;
            update_score(self.current_score);

            self.current_lines += complete_row;
            update_line(self.current_lines);
        } else {
            self.game_state = GameLoopState::CheckGameOver;
        }
    }

    fn remove_complete_rows(&mut self) -> u16 {
        let mut complete_row = 0;
        for y in 0..self.arena_size.y {
            let mut complete = true;
            for x in 0..self.arena_size.x {
                if self.arena[(self.arena_size.x * y) + x].is_none() {
                    complete = false;
                }
            }

            if complete {
                self.drop_rows(y);
                self.drop_blocks();
                complete_row += 1;
            }
        }
        complete_row
    }

    fn handle_check_game_over<P, T>(&mut self, update_next_piece: P, update_statistics: T)
    where
        P: FnMut(TetronimoShape),
        T: FnMut(ShapeStatistics),
    {
        self.create_new_piece(update_next_piece, update_statistics);
        let (blocks, width) = self.piece.get_chars();
        let mut overlap = false;
        blocks.iter().enumerate().for_each(|(offset, present)| {
            let x = self.position.x + offset % width;
            let y = self.position.y + offset / width;
            if *present && self.arena[x + (self.arena_size.x * y)].is_some() {
                overlap = true;
            }
        });

        if overlap {
            self.game_state = GameLoopState::GameOver;
        } else {
            self.game_state = GameLoopState::Running;
        }
    }

    fn handle_game_over(&mut self) {
        self.game_state = GameLoopState::Start;
        let _ = self.tx.try_send(GlobalStateManagementMessage::GameOver);
    }

    fn add_piece_to_arena(&mut self) {
        let (blocks, width) = self.piece.get_chars();
        blocks.iter().enumerate().for_each(|(offset, present)| {
            let x = self.position.x + offset % width;
            let y = self.position.y + offset / width;
            if *present {
                self.arena[x + (self.arena_size.x * y)] = Some(self.piece.shape.clone());
            }
        });
    }

    fn create_new_piece<P, T>(&mut self, mut update_next_piece: P, mut update_statistics: T)
    where
        P: FnMut(TetronimoShape),
        T: FnMut(ShapeStatistics),
    {
        self.piece = match &self.next_piece {
            Some(piece) => Tetronimo::new(piece.clone()),
            None => Tetronimo::random(),
        };

        match self.piece.shape {
            TetronimoShape::IShape => self.shapes_statistics.i_count += 1,
            TetronimoShape::JShape => self.shapes_statistics.j_count += 1,
            TetronimoShape::LShape => self.shapes_statistics.l_count += 1,
            TetronimoShape::OShape => self.shapes_statistics.o_count += 1,
            TetronimoShape::SShape => self.shapes_statistics.s_count += 1,
            TetronimoShape::TShape => self.shapes_statistics.t_count += 1,
            TetronimoShape::ZShape => self.shapes_statistics.z_count += 1,
        };

        update_statistics(self.shapes_statistics);
        self.next_piece = Some(rand::random());
        self.position = Position::new(self.arena_size.x / 2, 0);

        if let Some(next_piece) = &self.next_piece {
            update_next_piece(next_piece.clone());
        }
    }

    fn create_new_arena(&mut self) {
        self.arena = vec![None; self.arena_size.x * self.arena_size.y];
    }

    pub(crate) fn draw_piece<F>(&self, mut func: F)
    where
        F: FnMut(&TetronimoShape, LocalPos),
    {
        let (shape, width) = self.piece.get_chars();
        shape.iter().enumerate().for_each(|(offset, present)| {
            if *present {
                let x = (offset % width) as u16;
                let y = (offset / width) as u16;
                let local_pos = LocalPos::new(x, y);
                let pos: LocalPos = self.position.clone().into();
                func(&self.piece.shape, local_pos + pos);
            }
        });
    }

    pub(crate) fn draw_arena<D>(&self, mut draw: D)
    where
        D: FnMut(Option<&TetronimoShape>, LocalPos),
    {
        self.arena.iter().enumerate().for_each(|(offset, piece)| {
            let x = offset % self.arena_size.x;
            let y = offset / self.arena_size.x;
            let local_pos = LocalPos::new(x as u16, y as u16);
            match piece {
                Some(piece) => draw(Some(piece), local_pos),
                None => draw(None, local_pos),
            }
        });
    }

    // A completed row has been removed now it is time to drop all the blocks
    // into place.
    fn drop_rows(&mut self, row: usize) {
        // Remove row from the arena
        let row_offset = row * self.arena_size.x;
        for col_offset in 0..self.arena_size.x {
            self.arena[row_offset + col_offset] = None;
        }
    }

    fn drop_blocks(&mut self) {
        loop {
            let mut changed = false;
            for block in 0..(self.arena_size.x * self.arena_size.y) {
                if self.arena[block].is_none() {
                    // Cant drop an empty block
                    continue;
                } else if (block + self.arena_size.x) >= (self.arena_size.x * self.arena_size.y) {
                    // Reached end of the board
                    continue;
                } else if self.arena[block + self.arena_size.x].is_none() {
                    self.arena[block + self.arena_size.x] = self.arena[block].clone();
                    self.arena[block] = None;
                    changed = true;
                }
            }

            if changed {
                self.drop_blocks();
            } else {
                break;
            }
        }
    }

    pub(crate) fn initialise(&mut self) {
        self.game_state = GameLoopState::Start;
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

#[cfg(test)]
mod test {
    use crate::core::tetronimo::TetronimoShape;

    use super::GameLoop;

    #[test]
    fn calculate_rows_when_empty() {
        let (tx, _rx) = smol::channel::unbounded();
        let mut under_test = GameLoop::new(2, 2, tx);

        assert_eq!(0, under_test.remove_complete_rows());
    }

    #[test]
    fn calculate_single_row_at_bottom() {
        let (tx, _rx) = smol::channel::unbounded();
        let mut under_test = GameLoop::new(3, 2, tx);
        under_test.arena[3] = Some(TetronimoShape::IShape);
        under_test.arena[4] = Some(TetronimoShape::IShape);
        under_test.arena[5] = Some(TetronimoShape::IShape);

        assert_eq!(1, under_test.remove_complete_rows());
    }

    #[test]
    fn calculate_all_rows() {
        let (tx, _rx) = smol::channel::unbounded();
        let mut under_test = GameLoop::new(3, 2, tx);
        under_test.arena[0] = Some(TetronimoShape::ZShape);
        under_test.arena[1] = Some(TetronimoShape::ZShape);
        under_test.arena[2] = Some(TetronimoShape::ZShape);
        under_test.arena[3] = Some(TetronimoShape::ZShape);
        under_test.arena[4] = Some(TetronimoShape::ZShape);
        under_test.arena[5] = Some(TetronimoShape::ZShape);

        assert_eq!(2, under_test.remove_complete_rows());
    }

    #[test]
    fn drop_single_block() {
        let (tx, _rx) = smol::channel::unbounded();
        let mut under_test = GameLoop::new(2, 2, tx);
        under_test.arena[0] = Some(TetronimoShape::JShape);

        under_test.drop_blocks();

        assert_eq!(None, under_test.arena[0]);
        assert_eq!(None, under_test.arena[1]);
        assert_eq!(Some(TetronimoShape::JShape), under_test.arena[2]);
        assert_eq!(None, under_test.arena[3]);
    }
}
