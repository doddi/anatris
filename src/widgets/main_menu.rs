use std::fs::read_to_string;

use anathema::{
    component::Component,
    state::{State, Value},
};
use smol::channel::Sender;

use crate::core::global_state::GlobalStateManagementMessage;

pub(crate) struct MainMenuComponent {
    tx: Sender<GlobalStateManagementMessage>,
}

impl MainMenuComponent {
    pub(crate) fn new(tx: Sender<GlobalStateManagementMessage>) -> Self {
        Self { tx }
    }

    fn handle_selection(&mut self, state: &mut MainMenuComponentState, selection: MainMenuAction) {
        match selection {
            MainMenuAction::Up => self.toggle_menu(state),
            MainMenuAction::Down => self.toggle_menu(state),
            MainMenuAction::Enter => match state.start_highlighted.to_bool() {
                true => {
                    let _ = self.tx.try_send(GlobalStateManagementMessage::Playing);
                }
                false => {
                    let _ = self.tx.try_send(GlobalStateManagementMessage::Exit);
                }
            },
        }
    }

    fn toggle_menu(&mut self, state: &mut MainMenuComponentState) {
        match state.start_highlighted.to_bool() {
            true => *state.start_highlighted.to_mut() = false,
            false => *state.start_highlighted.to_mut() = true,
        };
    }
}

impl Component for MainMenuComponent {
    type State = MainMenuComponentState;

    type Message = MainMenuComponentMessage;

    fn message(
        &mut self,
        message: Self::Message,
        state: &mut Self::State,
        mut _elements: anathema::widgets::Elements<'_, '_>,
        mut _context: anathema::prelude::Context<'_, Self::State>,
    ) {
        match message {
            MainMenuComponentMessage::Visible => *state.visible.to_mut() = true,
            MainMenuComponentMessage::Invisible => *state.visible.to_mut() = false,
            MainMenuComponentMessage::Change(selection) => self.handle_selection(state, selection),
        }
    }
}

#[derive(Debug)]
pub(crate) enum MainMenuAction {
    Up,
    Down,
    Enter,
}

#[derive(Debug)]
pub(crate) enum MainMenuComponentMessage {
    Visible,
    Invisible,
    Change(MainMenuAction),
}

#[derive(State)]
pub(crate) struct MainMenuComponentState {
    title: Value<String>,
    start_highlighted: Value<bool>,
    visible: Value<bool>,
}

impl MainMenuComponentState {
    pub(crate) fn new() -> Self {
        Self {
            title: Value::new(read_to_string("src/resources/title.txt").unwrap()),
            start_highlighted: Value::new(true),
            visible: Value::new(true),
        }
    }
}
