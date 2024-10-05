use anathema::{
    component::Component,
    state::{State, Value},
};

use crate::core::game_loop::ShapeStatistics;

pub(crate) struct StatisticsComponent;

impl StatisticsComponent {}

impl Component for StatisticsComponent {
    type State = StatisticsState;
    type Message = StatisticsComponentMessage;

    fn message(
        &mut self,
        message: Self::Message,
        state: &mut Self::State,
        mut _elements: anathema::widgets::Elements<'_, '_>,
        mut _context: anathema::prelude::Context<'_, Self::State>,
    ) {
        *state.i_shape.to_mut() = message.i_stat;
        *state.j_shape.to_mut() = message.j_stat;
        *state.l_shape.to_mut() = message.l_stat;
        *state.o_shape.to_mut() = message.o_stat;
        *state.t_shape.to_mut() = message.t_stat;
        *state.s_shape.to_mut() = message.s_stat;
        *state.z_shape.to_mut() = message.z_stat;
    }
}

#[allow(non_snake_case)]
#[derive(State, Debug)]
pub(crate) struct StatisticsState {
    i_shape: Value<u16>,
    j_shape: Value<u16>,
    l_shape: Value<u16>,
    o_shape: Value<u16>,
    t_shape: Value<u16>,
    s_shape: Value<u16>,
    z_shape: Value<u16>,
}

impl StatisticsState {
    pub(crate) fn new() -> Self {
        Self {
            i_shape: Value::new(0),
            j_shape: Value::new(0),
            l_shape: Value::new(0),
            o_shape: Value::new(0),
            t_shape: Value::new(0),
            s_shape: Value::new(0),
            z_shape: Value::new(0),
        }
    }
}

#[derive(Debug)]
pub(crate) struct StatisticsComponentMessage {
    i_stat: u16,
    j_stat: u16,
    l_stat: u16,
    o_stat: u16,
    t_stat: u16,
    s_stat: u16,
    z_stat: u16,
}

impl StatisticsComponentMessage {
    pub(crate) fn new(i: u16, j: u16, l: u16, o: u16, t: u16, s: u16, z: u16) -> Self {
        Self {
            i_stat: i,
            j_stat: j,
            l_stat: l,
            o_stat: o,
            t_stat: t,
            s_stat: s,
            z_stat: z,
        }
    }
}

impl From<ShapeStatistics> for StatisticsComponentMessage {
    fn from(value: ShapeStatistics) -> Self {
        StatisticsComponentMessage::new(
            value.i_count,
            value.j_count,
            value.l_count,
            value.o_count,
            value.t_count,
            value.s_count,
            value.z_count,
        )
    }
}
