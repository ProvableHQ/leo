use crate::{Input, PrivateState, PublicState};
use leo_inputs::{
    tables::{Table, Visibility},
    InputParserError,
};

#[derive(Clone, PartialEq, Eq)]
pub struct ProgramState {
    pub public: PublicState,
    pub private: PrivateState,
}

impl ProgramState {
    pub fn new() -> Self {
        Self {
            public: PublicState::new(),
            private: PrivateState::new(),
        }
    }

    pub fn store_definitions(&mut self, table: Table, expected_inputs: &Vec<Input>) -> Result<(), InputParserError> {
        match table.visibility {
            Visibility::Private(_private) => self.private.store_definitions(table.sections, &expected_inputs),
            Visibility::Public(_public) => self.public.store_definitions(table.sections, &expected_inputs),
        }
    }
}
