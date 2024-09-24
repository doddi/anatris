use anathema::{component::Component, state::State};

pub(crate) struct GameComponent;

impl GameComponent {}

impl Component for GameComponent {
    type State = GameState;
    type Message = GameMessage;
}

#[derive(State)]
pub(crate) struct GameState {}

pub(crate) struct GameMessage {}
