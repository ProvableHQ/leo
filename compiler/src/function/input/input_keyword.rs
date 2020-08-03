use crate::{errors::FunctionError, ConstrainedCircuitMember, ConstrainedProgram, ConstrainedValue, GroupType};
use leo_types::{Identifier, Input};

use snarkos_models::{
    curves::{Field, PrimeField},
    gadgets::r1cs::ConstraintSystem,
};

pub const RECORD_VARIABLE_NAME: &str = "record";
pub const REGISTERS_VARIABLE_NAME: &str = "registers";
pub const STATE_VARIABLE_NAME: &str = "state";
pub const STATE_LEAF_VARIABLE_NAME: &str = "state_leaf";

impl<F: Field + PrimeField, G: GroupType<F>> ConstrainedProgram<F, G> {
    pub fn allocate_input_keyword<CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        identifier: Identifier,
        input: &Input,
    ) -> Result<ConstrainedValue<F, G>, FunctionError> {
        // Create an identifier for each input variable

        let registers_name = Identifier {
            name: REGISTERS_VARIABLE_NAME.to_string(),
            span: identifier.span.clone(),
        };
        let record_name = Identifier {
            name: RECORD_VARIABLE_NAME.to_string(),
            span: identifier.span.clone(),
        };
        let state_name = Identifier {
            name: STATE_VARIABLE_NAME.to_string(),
            span: identifier.span.clone(),
        };
        let state_leaf_name = Identifier {
            name: STATE_LEAF_VARIABLE_NAME.to_string(),
            span: identifier.span.clone(),
        };

        // Fetch each input variable's definitions

        let registers_values = input.get_registers().values();
        let record_values = input.get_record().values();
        let state_values = input.get_state().values();
        let state_leaf_values = input.get_state_leaf().values();

        // Allocate each input variable as a circuit expression

        let mut sections = vec![];

        sections.push((registers_name, registers_values));
        sections.push((record_name, record_values));
        sections.push((state_name, state_values));
        sections.push((state_leaf_name, state_leaf_values));

        let mut members = vec![];

        for (name, values) in sections {
            let member_name = name.clone();
            let member_value = self.allocate_input_section(cs, name, values)?;

            let member = ConstrainedCircuitMember(member_name, member_value);

            members.push(member)
        }

        // Return input variable keyword as circuit expression

        Ok(ConstrainedValue::CircuitExpression(identifier, members))
    }
}
