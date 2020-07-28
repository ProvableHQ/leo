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

    pub fn store_definitions(
        &mut self,
        sections: Vec<Section>,
        expected_inputs: &Vec<Input>,
    ) -> Result<(), InputParserError> {
        for section in sections {
            match section.header {
                Header::Record(_state) => self.record.store_definitions(section.definitions, expected_inputs)?,
                Header::StateLeaf(_state_leaf) => self
                    .state_leaf
                    .store_definitions(section.definitions, expected_inputs)?,
                header => return Err(InputParserError::private_section(header)),
            }
        }

        Ok(())
    }
}
