use crate::{PrivateState, PublicState, Record, State, StateLeaf};
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

    /// Parse all inputs included in a file and store them in `self`.
    pub fn parse(&mut self, table: Table) -> Result<(), InputParserError> {
        match table.visibility {
            Visibility::Private(_private) => self.private.parse(table.sections),
            Visibility::Public(_public) => self.public.parse(table.sections),
        }
    }

    /// Returns the runtime record input values
    pub fn get_record(&self) -> &Record {
        self.private.get_record()
    }

    /// Returns the runtime state input values
    pub fn get_state(&self) -> &State {
        self.public.get_state()
    }

    /// Returns the runtime state leaf input values
    pub fn get_state_leaf(&self) -> &StateLeaf {
        self.private.get_state_leaf()
    }
}
