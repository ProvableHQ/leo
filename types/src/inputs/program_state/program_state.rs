use crate::{PrivateState, PublicState};

#[derive(Clone, PartialEq, Eq)]
pub struct ProgramState {
    public: PublicState,
    private: PrivateState,
}

impl ProgramState {
    pub fn new() -> Self {
        Self {
            public: PublicState::new(),
            private: PrivateState::new(),
        }
    }
}
