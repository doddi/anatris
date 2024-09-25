use anathema::{component::Component, state::State};

pub(crate) struct StaticPieceComponent;

impl StaticPieceComponent {}

impl Component for StaticPieceComponent {
    type State = StaticPieceState;
    type Message = StaticPieceMessage;
}

#[derive(State)]
pub(crate) struct StaticPieceState {}

pub(crate) struct StaticPieceMessage {}
