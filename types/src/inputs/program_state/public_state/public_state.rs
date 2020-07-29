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

    pub fn len(&self) -> usize {
        if self.state.is_present() { 1usize } else { 0usize }
    }

    /// Returns an empty version of this struct with `None` values.
    /// Called during constraint synthesis to provide private inputs.
    pub fn empty(&self) -> Self {
        let state = self.state.empty();

        Self { state }
    }

    pub fn parse(&mut self, sections: Vec<Section>) -> Result<(), InputParserError> {
        for section in sections {
            match section.header {
                Header::State(_state) => self.state.parse(section.definitions)?,
                header => return Err(InputParserError::public_section(header)),
            }
        }

        Ok(())
    }
}
