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

//! Enforces a return statement in a compiled Leo program.

use crate::program::Program;
use leo_asg::{FunctionQualifier, ReturnStatement};
use leo_errors::Result;
use leo_span::sym;
use snarkvm_ir::{Instruction, PredicateData, Value};

impl<'a> Program<'a> {
    pub fn enforce_return_statement(&mut self, statement: &ReturnStatement<'a>) -> Result<()> {
        let function = self.current_function.expect("return in non-function");
        let is_mut_self = matches!(function.qualifier, FunctionQualifier::MutSelfRef);
        let result = self.enforce_expression(statement.expression.get())?;
        let mut output = result;
        if is_mut_self {
            let self_var = function
                .scope
                .resolve_variable(sym::SelfLower)
                .expect("missing self in mut self function");
            let self_var_register = self.resolve_var(self_var);
            output = Value::Tuple(vec![Value::Ref(self_var_register), output]);
        }
        self.emit(Instruction::Return(PredicateData { values: vec![output] }));
        Ok(())
    }
}
