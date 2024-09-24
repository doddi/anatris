use anathema::{component::Component, state::State};

pub(crate) struct NextPieceComponent;

impl NextPieceComponent {}

impl Component for NextPieceComponent {
    type State = NextPieceState;
    type Message = NextPieceMessage;
}

#[derive(State)]
pub(crate) struct NextPieceState {}

pub(crate) struct NextPieceMessage {}
