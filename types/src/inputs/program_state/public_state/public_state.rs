use crate::State;

#[derive(Clone, PartialEq, Eq)]
pub struct PublicState {
    state: State,
}

impl PublicState {
    pub fn new() -> Self {
        Self { state: State::new() }
    }
}
