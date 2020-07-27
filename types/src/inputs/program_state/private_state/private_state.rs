use crate::{Record, StateLeaf};

#[derive(Clone, PartialEq, Eq)]
pub struct PrivateState {
    record: Record,
    state_leaf: StateLeaf,
}

impl PrivateState {
    pub fn new() -> Self {
        Self {
            record: Record::new(),
            state_leaf: StateLeaf::new(),
        }
    }
}
