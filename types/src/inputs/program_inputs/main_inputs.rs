use crate::InputValue;

#[derive(Clone, PartialEq, Eq)]
pub struct MainInputs(Vec<Option<InputValue>>);

impl MainInputs {
    pub fn new() -> Self {
        Self(vec![])
    }
}
