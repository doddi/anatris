use anathema::{
    component::Component,
    state::{State, Value},
};

pub(crate) struct LineCountComponent {}

impl LineCountComponent {}

impl Component for LineCountComponent {
    type State = LineCountState;
    type Message = LineCountComponentMessage;

    fn on_message(
        &mut self,
        message: Self::Message,
        state: &mut Self::State,
        _children: anathema::component::Children<'_, '_>,
        _context: anathema::component::Context<'_, '_, Self::State>,
    ) {
        match message {
            LineCountComponentMessage::Count(value) => *state.count.to_mut() = value,
        }
    }
}

#[derive(State)]
pub(crate) struct LineCountState {
    count: Value<u16>,
}

impl LineCountState {
    pub(crate) fn new() -> Self {
        Self {
            count: Value::new(0),
        }
    }
}

#[derive(Debug)]
pub(crate) enum LineCountComponentMessage {
    Count(u16),
}
