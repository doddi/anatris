use anathema::{
    component::Component,
    state::{AnyState, State, Value},
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
            MainMenuComponentMessage::KeyUp => toggle_highlight(state),
            MainMenuComponentMessage::KeyDown => toggle_highlight(state),
        }
    }
}

fn toggle_highlight(state: &mut MainMenuComponentState) {
    let current = *state.start_highlighted.to_ref();
    *state.start_highlighted.to_mut() = !current;
}

#[derive(Debug)]
pub(crate) enum MainMenuComponentMessage {
    Visible,
    Invisible,
    KeyUp,
    KeyDown,
}

#[derive(State)]
pub(crate) struct MainMenuComponentState {
    start_highlighted: Value<bool>,
    visible: Value<bool>,
}

impl MainMenuComponentState {
    pub(crate) fn new() -> Self {
        Self {
            start_highlighted: Value::new(true),
            visible: Value::new(true),
        }
    }
}
