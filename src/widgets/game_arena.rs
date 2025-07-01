use std::time::Duration;

use anathema::{
    backend::tui::Style,
    component::{Component, Context},
    default_widgets::Canvas,
    geometry::LocalPos,
    state::{List, State, Value},
};
use anathema::component::Children;
use smol::channel::Sender;

use crate::core::{
    game_loop::{GameAction, GameLoop, MoveActionType},
    global_state::GlobalStateManagementMessage,
    tetronimo::TetronimoShape,
};

const GLYPH_WIDTH: u16 = 2;
const MOVE_TICK_DURATION: u64 = 200;
const FALL_TICK_DURATION: u64 = 200;

#[derive(State)]
pub(crate) struct GameArenaComponentState {
    paused: Value<bool>,
    debug: Value<List<String>>,
}

impl GameArenaComponentState {
    pub(crate) fn new() -> Self {
        Self {
            paused: Value::new(false),
            debug: List::empty().into(),
        }
    }
}

impl From<GameArenaComponentMessage> for MoveActionType {
    fn from(value: GameArenaComponentMessage) -> Self {
        match value {
            GameArenaComponentMessage::Initialise => todo!(),
            GameArenaComponentMessage::Rotate => MoveActionType::Rotate,
            GameArenaComponentMessage::Drop => MoveActionType::Drop,
            GameArenaComponentMessage::MoveLeft => MoveActionType::MoveLeft,
            GameArenaComponentMessage::MoveRight => MoveActionType::MoveRight,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum GameArenaComponentMessage {
    Initialise,
    Rotate,
    Drop,
    MoveLeft,
    MoveRight,
}

pub(crate) struct GameArenaComponent {
    tx: Sender<GlobalStateManagementMessage>,
    last_fall_update: Duration,
    last_move_update: Duration,

    move_requested: MoveActionType,
    game_loop: GameLoop,
}

impl From<&TetronimoShape> for char {
    fn from(value: &TetronimoShape) -> Self {
        match value {
            TetronimoShape::IShape => '🟦',
            TetronimoShape::JShape => '🟪',
            TetronimoShape::LShape => '🟥',
            TetronimoShape::OShape => '🟨',
            TetronimoShape::SShape => '🟩',
            TetronimoShape::TShape => '🟫',
            TetronimoShape::ZShape => '🟧',
        }
    }
}

impl GameArenaComponent {
    pub(crate) fn new(tx: Sender<GlobalStateManagementMessage>, game_loop: GameLoop) -> Self {
        Self {
            tx,
            last_fall_update: Duration::ZERO,
            last_move_update: Duration::ZERO,

            move_requested: MoveActionType::None,
            game_loop,
        }
    }

    fn draw_tetronimo(&self, canvas: &mut Canvas) {
        self.game_loop.draw_piece(|character, position| {
            let position: LocalPos = LocalPos::new(position.x * GLYPH_WIDTH, position.y);
            canvas.put(character.into(), Style::reset(), position)
        });
    }

    fn draw_arena(&self, canvas: &mut Canvas) {
        self.game_loop
            .draw_arena(|character, position| match character {
                Some(char) => {
                    let position = LocalPos::new(position.x * GLYPH_WIDTH, position.y);
                    canvas.put(char.into(), Style::reset(), position)
                }
                None => {
                    let position = LocalPos::new(position.x * GLYPH_WIDTH, position.y);
                    canvas.erase(position)
                }
            });
    }

    fn handle_moving_state(
        &mut self,
        _state: &mut GameArenaComponentState,
        mut children: Children<'_, '_>,
        dt: Duration,
    ) {
        self.last_fall_update += dt;
        self.last_move_update += dt;

        if self.last_move_update >= Duration::from_millis(MOVE_TICK_DURATION) {
            self.last_move_update = Duration::ZERO;
            match self.move_requested {
                MoveActionType::MoveLeft => self
                    .game_loop
                    .handle_input(GameAction::Move(MoveActionType::MoveLeft)),
                MoveActionType::MoveRight => self
                    .game_loop
                    .handle_input(GameAction::Move(MoveActionType::MoveRight)),
                MoveActionType::Rotate => self
                    .game_loop
                    .handle_input(GameAction::Move(MoveActionType::Rotate)),
                _ => (),
            }
            self.move_requested = MoveActionType::None;
        }

        if self.last_fall_update >= Duration::from_millis(FALL_TICK_DURATION) {
            self.last_fall_update = Duration::ZERO;
            self.game_loop.fall_tick();
        }

        self.game_loop.do_state_machine(
            |score| {
                let _ = self
                    .tx
                    .try_send(GlobalStateManagementMessage::UpdateScore(score));
            },
            |score| {
                let _ = self
                    .tx
                    .try_send(GlobalStateManagementMessage::UpdateLines(score));
            },
            |shape| {
                let _ = self
                    .tx
                    .try_send(GlobalStateManagementMessage::UpdateNextTetronimo(shape));
            },
            |statistics| {
                let _ = self
                    .tx
                    .try_send(GlobalStateManagementMessage::UpdateStatistics(
                        statistics.into(),
                    ));
            },
        );

        children.elements().by_tag("canvas").first(|el, _| {
            let canvas = el.to::<Canvas>();

            self.draw_arena(canvas);
            self.draw_tetronimo(canvas);
        });
    }
}

impl Component for GameArenaComponent {
    type State = GameArenaComponentState;
    type Message = GameArenaComponentMessage;

    fn on_tick(
        &mut self,
        state: &mut Self::State,
        children: Children<'_, '_>,
        context: Context<'_, '_, Self::State>,
        dt: Duration,
    ) {
        let is_paused = context.attributes.get_as::<bool>("paused");

        match is_paused {
            Some(true) => self.game_loop.handle_input(GameAction::Pause),
            _ => self.handle_moving_state(state, children, dt),
        }
    }

    fn on_message(
        &mut self,
        message: Self::Message,
        _state: &mut Self::State,
        _children: Children<'_, '_>,
        _context: Context<'_, '_, Self::State>,
    ) {
        if message == GameArenaComponentMessage::Initialise {
            self.game_loop.initialise();
        } else if self.move_requested == MoveActionType::None {
            self.move_requested = message.into();
        }
    }
}
