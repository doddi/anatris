use std::fs::read_to_string;

use anathema::{
    component::Component,
    state::{State, Value},
};

pub(crate) struct MainMenuComponent;

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
            MainMenuComponentMessage::ChangeTo(selection) => toggle_highlight(state, selection),
        }
    }
}

fn toggle_highlight(state: &mut MainMenuComponentState, selection: MainMenuComponentSelection) {
    match selection {
        MainMenuComponentSelection::Start => *state.start_highlighted.to_mut() = true,
        MainMenuComponentSelection::Exit => *state.start_highlighted.to_mut() = false,
    }
}

#[derive(Debug)]
pub(crate) enum MainMenuComponentSelection {
    Start,
    Exit,
}

#[derive(Debug)]
pub(crate) enum MainMenuComponentMessage {
    Visible,
    Invisible,
    ChangeTo(MainMenuComponentSelection),
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
