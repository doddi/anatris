use anathema::{
    component::Component,
    state::{State, Value},
};

pub(crate) struct GameComponent;

impl GameComponent {}

impl Component for GameComponent {
    type State = GameComponentState;
    type Message = GameComponentMessage;

    fn message(
        &mut self,
        message: Self::Message,
        state: &mut Self::State,
        mut _elements: anathema::widgets::Elements<'_, '_>,
        mut _context: anathema::prelude::Context<'_, Self::State>,
    ) {
        match message {
            GameComponentMessage::Visible => *state.visible.to_mut() = true,
            GameComponentMessage::Invisible => *state.visible.to_mut() = false,
        }
    }
}

#[derive(State)]
pub(crate) struct GameComponentState {
    visible: Value<bool>,
}

impl GameComponentState {
    pub(crate) fn new() -> Self {
        Self {
            visible: Value::new(false),
        }
    }
}

#[derive(Debug)]
pub(crate) enum GameComponentMessage {
    Visible,
    Invisible,
}
