use crate::{Input, InputValue};
use leo_inputs::{definitions::Definition, InputParserError};
use std::collections::HashMap;

#[derive(Clone, PartialEq, Eq)]
pub struct MainInputs {
    inputs: HashMap<String, Option<InputValue>>,
}

impl MainInputs {
    pub fn new() -> Self {
        Self { inputs: HashMap::new() }
    }

    pub fn len(&self) -> usize {
        self.inputs.len()
    }

    /// Returns an empty version of this struct with `None` values.
    /// Called during constraint synthesis to provide private inputs.
    pub fn empty(&self) -> Self {
        let mut inputs = self.inputs.clone();

        inputs.iter_mut().for_each(|(_name, value)| {
            *value = None;
        });

        Self { inputs }
    }

    /// Stores main input assignments that match expected main function inputs
    pub fn store_definitions(&mut self, definitions: Vec<Definition>) -> Result<(), InputParserError> {
        // for definition in definitions {
        //
        // }

        // let mut program_inputs = vec![];
        //
        // for definition in definitions {
        //     // find input with matching name
        //     let matched_input = expected_inputs.clone().into_iter().find(|input| {
        //         // only look at program inputs
        //         match input {
        //             Input::FunctionInput(function_input) => {
        //                 // name match
        //                 function_input.identifier.name.eq(&definition.parameter.variable.value)
        //                     // type match
        //                     && function_input.type_.to_string().eq(&definition.parameter.type_.to_string())
        //             }
        //             _ => false,
        //         }
        //     });
        //
        //     match matched_input {
        //         Some(_) => {
        //             let value = InputValue::from_expression(definition.parameter.type_, definition.expression)?;
        //
        //             // push value to main inputs
        //             program_inputs.push(Some(value));
        //         }
        //         None => return Err(InputParserError::InputNotFound(definition.parameter.variable.value)),
        //     }
        // }
        //
        // self.0 = program_inputs;

        Ok(())
    }

    /// Returns main function input at `index`
    pub fn get(&self, name: &String) -> Option<InputValue> {
        self.inputs.get(name).unwrap().clone()
    }
}
