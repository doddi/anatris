use anathema::{component::Component, state::State};

pub(crate) struct TetronimoComponent;

impl TetronimoComponent {}

impl Component for TetronimoComponent {
    type State = TetronimoState;
    type Message = TetronimoMessage;
}

#[derive(State)]
pub(crate) struct TetronimoState {}

pub(crate) struct TetronimoMessage {}
