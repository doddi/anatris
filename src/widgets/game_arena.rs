use std::time::Duration;

use anathema::{
    backend::tui::Style,
    component::{Component, Context},
    default_widgets::Canvas,
    geometry::LocalPos,
    state::{List, State, Value},
    widgets::Elements,
};

use crate::core::game_loop::{GameAction, GameLoop, MoveActionType};

// TODO: Gameplay logic should be moved to core module
const GLYPH_WIDTH: u16 = 2;
const CANVAS_WIDTH: u16 = 10;
const CANVAS_HEIGHT: u16 = 20;

#[derive(State)]
pub(crate) struct GameArenaComponentState {
    paused: Value<bool>,
    debug: Value<List<String>>,
}

impl GameArenaComponentState {
    pub(crate) fn new() -> Self {
        Self {
            paused: Value::new(false),
            debug: List::empty(),
        }
    }
}

impl From<GameArenaComponentMessage> for MoveActionType {
    fn from(value: GameArenaComponentMessage) -> Self {
        match value {
            GameArenaComponentMessage::Rotate => MoveActionType::Rotate,
            GameArenaComponentMessage::Drop => MoveActionType::Drop,
            GameArenaComponentMessage::MoveLeft => MoveActionType::MoveLeft,
            GameArenaComponentMessage::MoveRight => MoveActionType::MoveRight,
        }
    }
}

#[derive(Debug)]
pub(crate) enum GameArenaComponentMessage {
    Rotate,
    Drop,
    MoveLeft,
    MoveRight,
}

pub(crate) struct GameArenaComponent {
    last_fall_update: Duration,
    last_move_update: Duration,

    move_requested: MoveActionType,
    game_loop: GameLoop,
}

impl GameArenaComponent {
    pub(crate) fn new() -> Self {
        Self {
            last_fall_update: Duration::ZERO,
            last_move_update: Duration::ZERO,

            move_requested: MoveActionType::None,
            game_loop: GameLoop::new(CANVAS_WIDTH as usize, CANVAS_HEIGHT as usize),
        }
    }

    fn draw_tetronimo(&self, canvas: &mut Canvas) {
        self.game_loop.draw_piece(|character, position| {
            let position: LocalPos = LocalPos::new(position.x * GLYPH_WIDTH, position.y);
            canvas.put(character, Style::reset(), position)
        });
    }

    fn clear_tetronimo(&self, canvas: &mut Canvas) {
        self.game_loop.clear_piece(|position| {
            let position: LocalPos = LocalPos::new(position.x * GLYPH_WIDTH, position.y);
            canvas.erase(position);
            let position: LocalPos = LocalPos::new(position.x * GLYPH_WIDTH, position.y + 1);
            canvas.erase(position);
        });
    }

    fn draw_arena(&self, canvas: &mut Canvas) {
        self.game_loop
            .draw_arena(|character, position| canvas.put(character, Style::reset(), position));
    }

    fn handle_moving_state(
        &mut self,
        dt: Duration,
        mut elements: Elements,
        state: &mut GameArenaComponentState,
    ) {
        self.last_fall_update += dt;
        self.last_move_update += dt;

        if self.last_move_update >= Duration::from_millis(200) {
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

        if self.last_fall_update >= Duration::from_secs(1) {
            self.last_fall_update = Duration::ZERO;
            self.game_loop.fall_tick();
        }

        self.game_loop.do_state_machine();
        state.debug.insert(
            0,
            format!("position:     {:?}", self.game_loop.get_position()),
        );
        state.debug.insert(
            1,
            format!("old_position: {:?}", self.game_loop.get_old_position()),
        );

        elements.by_tag("canvas").first(|el, _| {
            let canvas = el.to::<Canvas>();

            self.draw_arena(canvas);
            self.clear_tetronimo(canvas);
            self.draw_tetronimo(canvas);
        });
    }
}

impl Component for GameArenaComponent {
    type State = GameArenaComponentState;
    type Message = GameArenaComponentMessage;

    fn tick(
        &mut self,
        state: &mut Self::State,
        mut elements: Elements<'_, '_>,
        context: Context<'_, Self::State>,
        dt: Duration,
    ) {
        let is_paused = extract_bool_attribute(context, "paused");

        match is_paused {
            Some(true) => self.game_loop.handle_input(GameAction::Pause),
            _ => self.handle_moving_state(dt, elements, state),
        }
    }

    fn message(
        &mut self,
        message: Self::Message,
        _state: &mut Self::State,
        mut _elements: Elements<'_, '_>,
        mut _context: Context<'_, Self::State>,
    ) {
        if self.move_requested == MoveActionType::None {
            self.move_requested = message.into();
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
