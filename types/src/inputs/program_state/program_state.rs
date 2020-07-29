use crate::{Input, PrivateState, PublicState};
use leo_inputs::{
    tables::{Table, Visibility},
    InputParserError,
};

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

    /// Returns an empty version of this struct with `None` values.
    /// Called during constraint synthesis to provide private inputs.
    pub fn empty(&self) -> Self {
        let public = self.public.empty();
        let private = self.private.empty();

        Self { public, private }
    }

    pub fn len(&self) -> usize {
        self.public.len() + self.private.len()
    }

    pub fn store_definitions(&mut self, table: Table) -> Result<(), InputParserError> {
        match table.visibility {
            Visibility::Private(_private) => self.private.store_definitions(table.sections),
            Visibility::Public(_public) => self.public.store_definitions(table.sections),
        }
    }
}
