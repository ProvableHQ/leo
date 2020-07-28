use crate::{Input, State};
use leo_inputs::{
    sections::{Header, Section},
    InputParserError,
};

#[derive(Clone, PartialEq, Eq)]
pub struct PublicState {
    state: State,
}

impl PublicState {
    pub fn new() -> Self {
        Self { state: State::new() }
    }

    pub fn store_definitions(
        &mut self,
        sections: Vec<Section>,
        expected_inputs: &Vec<Input>,
    ) -> Result<(), InputParserError> {
        for section in sections {
            match section.header {
                Header::State(_state) => self.state.store_definitions(section.definitions, expected_inputs)?,
                header => return Err(InputParserError::public_section(header)),
            }
        }

        Ok(())
    }
}
