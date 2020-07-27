use crate::InputValue;

#[derive(Clone, PartialEq, Eq)]
pub struct Registers(Vec<Option<InputValue>>);

impl Registers {
    pub fn new() -> Self {
        Self(vec![])
    }
}
