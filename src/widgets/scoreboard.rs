use anathema::{component::Component, state::State};

pub(crate) struct ScoreBoardComponent;

impl ScoreBoardComponent {}

impl Component for ScoreBoardComponent {
    type State = ScoreBoardState;
    type Message = ScoreBoardMessage;
}

#[derive(State)]
pub(crate) struct ScoreBoardState {}

pub(crate) struct ScoreBoardMessage {}
