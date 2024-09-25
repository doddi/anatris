use anathema::{
    component::Component,
    state::{State, Value},
};

pub(crate) struct MainMenuComponent {}

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
        }
    }
}

#[derive(Debug)]
pub(crate) enum MainMenuComponentMessage {
    Visible,
    Invisible,
}

#[derive(State)]
pub(crate) struct MainMenuComponentState {
    visible: Value<bool>,
}

impl MainMenuComponentState {
    pub(crate) fn new() -> Self {
        Self {
            visible: Value::new(true),
        }
    }
}
