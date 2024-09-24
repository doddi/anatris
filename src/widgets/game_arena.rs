use anathema::{component::Component, state::State};

pub(crate) struct GameArenaComponent;

impl GameArenaComponent {}

impl Component for GameArenaComponent {
    type State = GameArenaState;
    type Message = GameArenaMessage;
}

#[derive(State)]
pub(crate) struct GameArenaState {}

pub(crate) struct GameArenaMessage {}
