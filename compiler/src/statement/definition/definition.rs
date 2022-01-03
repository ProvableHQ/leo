// Copyright (C) 2019-2022 Aleo Systems Inc.
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

//! Enforces a definition statement in a compiled Leo program.

use crate::program::Program;
use leo_asg::{DefinitionStatement, Variable};
use leo_errors::Result;
use snarkvm_ir::{Instruction, Integer, QueryData, Value};

impl<'a> Program<'a> {
    fn enforce_multiple_definition(&mut self, variable_names: &[&'a Variable<'a>], values: Value) -> Result<()> {
        for (i, variable) in variable_names.iter().enumerate() {
            let target = self.alloc_var(variable);
            self.emit(Instruction::TupleIndexGet(QueryData {
                destination: target,
                values: vec![values.clone(), Value::Integer(Integer::U32(i as u32))],
            }));
        }

        Ok(())
    }

    #[allow(clippy::too_many_arguments)]
    pub fn enforce_definition_statement(&mut self, statement: &DefinitionStatement<'a>) -> Result<()> {
        let num_variables = statement.variables.len();
        let expression = self.enforce_expression(statement.value.get())?;

        if num_variables == 1 {
            let variable = statement.variables.get(0).unwrap();
            // Define a single variable with a single value
            self.alloc_var(variable);
            self.store(variable, expression);
            Ok(())
        } else {
            self.enforce_multiple_definition(&statement.variables[..], expression)
        }
    }
}
