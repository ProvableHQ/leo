use crate::{Input, InputValue};
use leo_inputs::{definitions::Definition, InputParserError};

#[derive(Clone, PartialEq, Eq)]
pub struct MainInputs(pub Vec<Option<InputValue>>);

impl MainInputs {
    pub fn new() -> Self {
        Self(vec![])
    }

    /// Stores main input assignments that match expected main function inputs
    pub fn store_definitions(
        &mut self,
        definitions: Vec<Definition>,
        expected_inputs: &Vec<Input>,
    ) -> Result<(), InputParserError> {
        let mut program_inputs = vec![];

        for definition in definitions {
            // find input with matching name
            let matched_input = expected_inputs.clone().into_iter().find(|input| {
                // only look at program inputs
                match input {
                    Input::FunctionInput(function_input) => {
                        // name match
                        function_input.identifier.name.eq(&definition.parameter.variable.value)
                            // type match
                            && function_input.type_.to_string().eq(&definition.parameter.type_.to_string())
                    }
                    _ => false,
                }
            });

            match matched_input {
                Some(_) => {
                    let value = InputValue::from_expression(definition.parameter.type_, definition.expression)?;

                    // push value to main inputs
                    program_inputs.push(Some(value));
                }
                None => return Err(InputParserError::InputNotFound(definition.parameter.variable.value)),
            }
        }

        self.0 = program_inputs;

        Ok(())
    }
}
