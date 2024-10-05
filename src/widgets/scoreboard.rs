use anathema::{
    component::Component,
    state::{State, Value},
};

pub(crate) struct ScoreBoardComponent;

impl ScoreBoardComponent {}

impl Component for ScoreBoardComponent {
    type State = ScoreBoardComponentState;
    type Message = ScoreBoardComponentMessage;

    fn message(
        &mut self,
        message: Self::Message,
        state: &mut Self::State,
        mut _elements: anathema::widgets::Elements<'_, '_>,
        mut _context: anathema::prelude::Context<'_, Self::State>,
    ) {
        match message {
            ScoreBoardComponentMessage::Score(value) => *state.current_score.to_mut() = value,
        }
    }
}

#[derive(State)]
pub(crate) struct ScoreBoardComponentState {
    current_score: Value<u16>,
}

impl ScoreBoardComponentState {
    pub(crate) fn new() -> Self {
        Self {
            current_score: Value::new(0),
        }
    }
}

pub(crate) enum ScoreBoardComponentMessage {
    Score(u16),
}
