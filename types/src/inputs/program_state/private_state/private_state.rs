use crate::{Input, Record, StateLeaf};
use leo_inputs::{
    sections::{Header, Section},
    InputParserError,
};

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

    /// Returns an empty version of this struct with `None` values.
    /// Called during constraint synthesis to provide private inputs.
    pub fn empty(&self) -> Self {
        let record = self.record.empty();
        let state_leaf = self.state_leaf.empty();

        Self { record, state_leaf }
    }

    pub fn len(&self) -> usize {
        let mut len = 0;

        // add record variable
        if self.record.is_present() {
            len += 1;
        }

        // add state_leaf variable
        if self.state_leaf.is_present() {
            len += 1;
        }

        len
    }

    /// Parse all inputs included in a file and store them in `self`.
    pub fn parse(&mut self, sections: Vec<Section>) -> Result<(), InputParserError> {
        for section in sections {
            match section.header {
                Header::Record(_state) => self.record.parse(section.definitions)?,
                Header::StateLeaf(_state_leaf) => self.state_leaf.parse(section.definitions)?,
                header => return Err(InputParserError::private_section(header)),
            }
        }

        Ok(())
    }
}
