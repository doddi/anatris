use anathema::{component::Component, state::State};

pub(crate) struct LineCountComponent;

impl LineCountComponent {}

impl Component for LineCountComponent {
    type State = LineCountState;
    type Message = LineCountMessage;
}

#[derive(State)]
pub(crate) struct LineCountState {}

pub(crate) struct LineCountMessage {}
