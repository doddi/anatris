use anathema::{
    component::Component,
    state::{State, Value},
};

pub(crate) struct StatisticsComponent;

impl StatisticsComponent {}

impl Component for StatisticsComponent {
    type State = StatisticsState;
    type Message = StatisticsMessage;

    fn message(
        &mut self,
        message: Self::Message,
        state: &mut Self::State,
        mut _elements: anathema::widgets::Elements<'_, '_>,
        mut _context: anathema::prelude::Context<'_, Self::State>,
    ) {
        *state.I.to_mut() = message.i_stat;
        *state.J.to_mut() = message.j_stat;
        *state.L.to_mut() = message.l_stat;
        *state.O.to_mut() = message.o_stat;
        *state.T.to_mut() = message.t_stat;
        *state.S.to_mut() = message.s_stat;
        *state.Z.to_mut() = message.z_stat;
    }
}

#[allow(non_snake_case)]
#[derive(State)]
pub(crate) struct StatisticsState {
    I: Value<u16>,
    J: Value<u16>,
    L: Value<u16>,
    O: Value<u16>,
    T: Value<u16>,
    S: Value<u16>,
    Z: Value<u16>,
}

impl StatisticsState {
    pub(crate) fn new() -> Self {
        Self {
            I: Value::new(0),
            J: Value::new(0),
            L: Value::new(0),
            O: Value::new(0),
            T: Value::new(0),
            S: Value::new(0),
            Z: Value::new(0),
        }
    }
}

pub(crate) struct StatisticsMessage {
    i_stat: u16,
    j_stat: u16,
    l_stat: u16,
    o_stat: u16,
    t_stat: u16,
    s_stat: u16,
    z_stat: u16,
}

impl StatisticsMessage {
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
