use crate::InputValue;

#[derive(Clone, PartialEq, Eq)]
pub struct State(Vec<Option<InputValue>>);

impl State {
    pub fn new() -> Self {
        Self(vec![])
    }
}
