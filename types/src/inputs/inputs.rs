use crate::InputValue;
use leo_inputs::{common::visibility::Visibility, files::File, InputParserError};

#[derive(Clone)]
pub struct Inputs {
    private: Vec<Option<InputValue>>,
    //public: Vec<_>
}

impl Inputs {
    pub fn new() -> Self {
        Self { private: vec![] }
    }

    pub fn get_private_inputs(&self) -> Vec<Option<InputValue>> {
        return self.private.clone();
    }

    pub fn set_private_inputs(&mut self, inputs: Vec<Option<InputValue>>) {
        self.private = inputs;
    }

    pub fn set_private_inputs_size(&mut self, size: usize) {
        self.private = vec![None; size];
    }

    pub fn from_inputs_file(file: File) -> Result<Self, InputParserError> {
        let mut private = vec![];

        for section in file.sections.into_iter() {
            for assignment in section.assignments.into_iter() {
                if let Some(Visibility::Public(_)) = assignment.parameter.visibility {
                    // Collect public parameters here
                } else {
                    // parameter is private by default

                    // evaluate expression
                    let value = InputValue::from_expression(assignment.parameter.type_, assignment.expression)?;

                    // push value to vector
                    private.push(Some(value));
                }
            }
        }

        Ok(Self { private })
    }
}
