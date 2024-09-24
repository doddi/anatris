use anathema::{component::Component, state::State};

pub(crate) struct GameTypeComponent;

impl GameTypeComponent {}

impl Component for GameTypeComponent {
    type State = GameTypeState;
    type Message = GameTypeMessage;
}

#[derive(State)]
pub(crate) struct GameTypeState {}

pub(crate) struct GameTypeMessage {}
