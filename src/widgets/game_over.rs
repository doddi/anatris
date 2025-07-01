use std::{fs::read_to_string, time::Duration};

use anathema::{
    component::Component,
    state::{State, Value},
};
use anathema::component::{Children, Context};
use smol::channel::Sender;

use crate::core::global_state::GlobalStateManagementMessage;

pub(crate) struct GameOverComponent {
    duration: Duration,
    tx: Sender<GlobalStateManagementMessage>,
}

impl GameOverComponent {
    pub(crate) fn new(tx: Sender<GlobalStateManagementMessage>) -> Self {
        Self {
            duration: Duration::ZERO,
            tx,
        }
    }
}

impl Component for GameOverComponent {
    type State = GameOverComponentState;

    type Message = GameOverComponentMessage;

    fn on_tick(
        &mut self,
        state: &mut Self::State,
        _children: Children<'_, '_>,
        _context: Context<'_, '_, Self::State>,
        dt: Duration) {
        if state.visible.copy_value() {
            self.duration += dt;
            if self.duration > Duration::new(5, 0) {
                let _ = self.tx.try_send(GlobalStateManagementMessage::MainMenu);
            }
        }
    }

    fn on_message(
        &mut self,
        message: Self::Message,
        state: &mut Self::State,
        _children: Children<'_, '_>,
        _context: Context<'_, '_, Self::State>) {
        match message {
            GameOverComponentMessage::Visible => {
                self.duration = Duration::ZERO;
                *state.visible.to_mut() = true
            }
            GameOverComponentMessage::Invisible => *state.visible.to_mut() = false,
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
