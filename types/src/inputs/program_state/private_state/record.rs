use crate::InputValue;

#[derive(Clone, PartialEq, Eq)]
pub struct Record(Vec<Option<InputValue>>);

impl Record {
    pub fn new() -> Self {
        Self(vec![])
    }
}
