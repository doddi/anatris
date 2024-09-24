use anathema::{component::Component, state::State};

pub(crate) struct StatisticsComponent;

impl StatisticsComponent {}

impl Component for StatisticsComponent {
    type State = StatisticsState;
    type Message = StatisticsMessage;
}

#[derive(State)]
pub(crate) struct StatisticsState {}

pub(crate) struct StatisticsMessage {}
