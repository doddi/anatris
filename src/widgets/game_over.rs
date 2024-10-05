use std::{fs::read_to_string, time::Duration};

use anathema::{
    component::Component,
    state::{State, Value},
};
use smol::channel::Sender;

use crate::GameStateManagementMessage;

pub(crate) struct GameOverComponent {
    duration: Duration,
    tx: Sender<GameStateManagementMessage>,
}

impl GameOverComponent {
    pub(crate) fn new(tx: Sender<GameStateManagementMessage>) -> Self {
        Self {
            duration: Duration::ZERO,
            tx,
        }
    }
}

impl Component for GameOverComponent {
    type State = GameOverComponentState;

    type Message = GameOverComponentMessage;

    fn message(
        &mut self,
        message: Self::Message,
        state: &mut Self::State,
        mut _elements: anathema::widgets::Elements<'_, '_>,
        mut _context: anathema::prelude::Context<'_, Self::State>,
    ) {
        match message {
            GameOverComponentMessage::Visible => {
                self.duration = Duration::ZERO;
                *state.visible.to_mut() = true
            }
            GameOverComponentMessage::Invisible => *state.visible.to_mut() = false,
        }
    }

    fn tick(
        &mut self,
        state: &mut Self::State,
        mut _elements: anathema::widgets::Elements<'_, '_>,
        _context: anathema::prelude::Context<'_, Self::State>,
        dt: std::time::Duration,
    ) {
        if state.visible.to_bool() {
            self.duration += dt;
            if self.duration > Duration::new(5, 0) {
                let _ = self.tx.try_send(GameStateManagementMessage::MainMenu);
            }
        }
    }
}

#[derive(State)]
pub(crate) struct GameOverComponentState {
    visible: Value<bool>,
    title: Value<String>,
}

impl GameOverComponentState {
    pub(crate) fn new() -> Self {
        Self {
            visible: Value::new(false),
            title: Value::new(read_to_string("src/resources/game-over.txt").unwrap()),
        }
    }
}

pub(crate) enum GameOverComponentMessage {
    Visible,
    Invisible,
}
