use anathema::{
    component::Component,
    state::{State, Value},
};

pub(crate) struct StatisticComponent {}

impl Component for StatisticComponent {
    type State = StatisticComponentState;

    type Message = ();
}

#[derive(State)]
pub(crate) struct StatisticComponentState {
    count: Value<u16>,
}

impl StatisticComponentState {
    pub(crate) fn new() -> Self {
        Self {
            count: Value::new(0),
        }
    }
}
