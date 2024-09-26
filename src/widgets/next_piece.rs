use crate::core::tetronimo::TetronimoShape;
use anathema::{
    component::Component,
    state::{State, Value},
};

pub(crate) struct NextPieceComponent;

impl NextPieceComponent {}

impl Component for NextPieceComponent {
    type State = NextPieceState;
    type Message = NextPieceMessage;

    fn message(
        &mut self,
        message: Self::Message,
        state: &mut Self::State,
        mut _elements: anathema::widgets::Elements<'_, '_>,
        mut _context: anathema::prelude::Context<'_, Self::State>,
    ) {
        *state.shape.to_mut() = message.shape.into();
    }
}

impl From<TetronimoShape> for String {
    fn from(value: TetronimoShape) -> Self {
        match value {
            TetronimoShape::IShape => 'I'.to_string(),
            TetronimoShape::JShape => 'J'.to_string(),
            TetronimoShape::LShape => 'L'.to_string(),
            TetronimoShape::OShape => 'O'.to_string(),
            TetronimoShape::SShape => 'S'.to_string(),
            TetronimoShape::TShape => 'T'.to_string(),
            TetronimoShape::ZShape => 'Z'.to_string(),
        }
    }
}

#[derive(State)]
pub(crate) struct NextPieceState {
    shape: Value<String>,
}

impl NextPieceState {
    pub(crate) fn new() -> Self {
        Self {
            shape: Value::new('I'.to_string()),
        }
    }
}

pub(crate) struct NextPieceMessage {
    shape: TetronimoShape,
}

impl NextPieceMessage {
    pub(crate) fn new(shape: TetronimoShape) -> Self {
        Self { shape }
    }
}
