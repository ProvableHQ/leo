use crate::InputValue;

#[derive(Clone, PartialEq, Eq)]
pub struct StateLeaf(Vec<Option<InputValue>>);

impl StateLeaf {
    pub fn new() -> Self {
        Self(vec![])
    }
}
