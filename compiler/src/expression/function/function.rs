// Copyright (C) 2019-2020 Aleo Systems Inc.
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

//! Enforce a function call expression in a compiled Leo program.

use crate::{errors::ExpressionError, program::ConstrainedProgram, value::ConstrainedValue, GroupType};
use leo_asg::{Expression, Span, Function};
use std::sync::Arc;

use snarkvm_models::{
    curves::{Field, PrimeField},
    gadgets::r1cs::ConstraintSystem,
};

impl<F: Field + PrimeField, G: GroupType<F>> ConstrainedProgram<F, G> {
    #[allow(clippy::too_many_arguments)]
    pub fn enforce_function_call_expression<CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        file_scope: &str,
        function_scope: &str,
        function: &Arc<Function>,
        target: Option<&Arc<Expression>>,
        arguments: &Vec<Arc<Expression>>,
        span: &Span,
    ) -> Result<ConstrainedValue<F, G>, ExpressionError> {

        let name_unique = || {
            format!(
                "function call {} {}:{}",
                function.name.borrow().clone(),
                span.line,
                span.start,
            )
        };
        let function = function.body.borrow().upgrade().expect("stale function in call expression");

        let target = if let Some(target) = target {
            Some(self.enforce_expression(cs, file_scope, function_scope, target)?)
        } else {
            None
        };

        let old_self_alias = self.self_alias.take();
        self.self_alias = if let Some(target) = &target {
            let self_var = function.scope.borrow().resolve_variable("self").expect("attempted to call static function from non-static context");
            match target {
                ConstrainedValue::CircuitExpression(circuit, id, values) => {
                    assert!(Some(&circuit.circuit) == function.function.circuit.borrow().map(|x| x.upgrade()).flatten().as_ref());
                    Some((self_var.borrow().id.clone(), id.clone()))
                },
                _ => panic!("attempted to pass non-circuit as target"),
            }
        } else {
            None
        };
        
        let return_value = self.enforce_function(
            &mut cs.ns(name_unique),
            file_scope,
            function_scope,
            &function,
            target,
            arguments,
        )
        .map_err(|error| ExpressionError::from(Box::new(error)))?;

        Ok(return_value)
    }
}
