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
            GameComponentMessage::Visible => {
                *state.visible.to_mut() = true;
                *state.paused.to_mut() = false;
            }
            GameComponentMessage::Invisible => {
                *state.visible.to_mut() = false;
                *state.paused.to_mut() = true;
            }
            GameComponentMessage::Paused => *state.paused.to_mut() = true,
            GameComponentMessage::Running => *state.paused.to_mut() = false,
        }

        // TODO: How to pass state down to children. It would be nice to do this
        // or even better would be to set attributes/state on children from
        // template that the widget code can then use.
        // Pushing
    }
}

#[derive(State)]
pub(crate) struct GameComponentState {
    paused: Value<bool>,
    visible: Value<bool>,
}

impl GameComponentState {
    pub(crate) fn new() -> Self {
        Self {
            visible: Value::new(false),
            paused: Value::new(true),
        }
    }
}

#[derive(Debug)]
pub(crate) enum GameComponentMessage {
    Visible,
    Invisible,
    Paused,
    Running,
}
