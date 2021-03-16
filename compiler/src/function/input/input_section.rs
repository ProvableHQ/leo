// Copyright (C) 2019-2021 Aleo Systems Inc.
// This file is part of the Leo library.

// The Leo library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The Leo library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the Leo library. If not, see <https://www.gnu.org/licenses/>.

use crate::{errors::FunctionError, ConstrainedCircuitMember, ConstrainedProgram, ConstrainedValue, GroupType};
use leo_asg::{AsgConvertError, Circuit, CircuitMember};
use leo_ast::{Identifier, InputValue, Parameter};

use snarkvm_fields::PrimeField;
use snarkvm_r1cs::ConstraintSystem;

use indexmap::IndexMap;

impl<'a, F: PrimeField, G: GroupType<F>> ConstrainedProgram<'a, F, G> {
    pub fn allocate_input_section<CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        identifier: Identifier,
        expected_type: &'a Circuit<'a>,
        section: IndexMap<Parameter, Option<InputValue>>,
        circuit_name: Option<&leo_ast::Identifier>,
    ) -> Result<ConstrainedValue<'a, F, G>, FunctionError> {
        let mut members = Vec::with_capacity(section.len());

        // Allocate each section definition as a circuit member value
        for (parameter, option) in section.into_iter() {
            let section_members = expected_type.members.borrow();
            let expected_type = match section_members.get(&parameter.variable.name) {
                Some(CircuitMember::Variable(inner)) => inner,
                _ => continue, // present, but unused
            };
            let declared_type = self.asg.scope.resolve_ast_type(&parameter.type_, circuit_name)?;
            if !expected_type.is_assignable_from(&declared_type) {
                return Err(AsgConvertError::unexpected_type(
                    &expected_type.to_string(),
                    Some(&declared_type.to_string()),
                    &identifier.span,
                )
                .into());
            }
            let member_name = parameter.variable.clone();
            let member_value = self.allocate_main_function_input(
                cs,
                &declared_type,
                &parameter.variable.name,
                option,
                &parameter.span,
            )?;
            let member = ConstrainedCircuitMember(member_name, member_value);

            members.push(member)
        }

        // Return section as circuit expression

        Ok(ConstrainedValue::CircuitExpression(expected_type, members))
    }
}
