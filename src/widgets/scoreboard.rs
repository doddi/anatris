use anathema::{
    component::Component,
    state::{State, Value},
};

pub(crate) struct ScoreBoardComponent;

impl ScoreBoardComponent {}

impl Component for ScoreBoardComponent {
    type State = ScoreBoardState;
    type Message = ScoreBoardMessage;

    fn message(
        &mut self,
        message: Self::Message,
        state: &mut Self::State,
        mut _elements: anathema::widgets::Elements<'_, '_>,
        mut _context: anathema::prelude::Context<'_, Self::State>,
    ) {
        match message {
            ScoreBoardMessage::Score(value) => *state.current_score.to_mut() = value,
        }
    }
}

#[derive(State)]
pub(crate) struct ScoreBoardState {
    current_score: Value<u16>,
}

impl ScoreBoardState {
    pub(crate) fn new() -> Self {
        Self {
            current_score: Value::new(0),
        }
    }
}

pub(crate) enum ScoreBoardMessage {
    Score(u16),
}
