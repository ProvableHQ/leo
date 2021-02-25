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

//! Resolves assignees in a compiled Leo program.

use crate::{
    errors::StatementError,
    program::ConstrainedProgram,
    value::ConstrainedValue,
    GroupType,
    ResolvedAssigneeAccess,
};
use leo_asg::{
    ArrayAccessExpression,
    ArrayRangeAccessExpression,
    CircuitAccessExpression,
    Expression,
    Node,
    Span,
    TupleAccessExpression,
    Variable,
};

use snarkvm_models::{curves::PrimeField, gadgets::r1cs::ConstraintSystem};

impl<'a, F: PrimeField, G: GroupType<F>> ConstrainedProgram<'a, F, G> {
    fn prepare_mut_access<CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        expr: &'a Expression<'a>,
        span: &Span,
        output: &mut Vec<ResolvedAssigneeAccess>,
    ) -> Result<Option<Variable<'a>>, StatementError> {
        match expr {
            Expression::ArrayRangeAccess(ArrayRangeAccessExpression { array, left, right, .. }) => {
                let inner = self.prepare_mut_access(cs, array.get(), span, output)?;
                let start_index = left
                    .get()
                    .map(|start| self.enforce_index(cs, start, &span))
                    .transpose()?;
                let stop_index = right
                    .get()
                    .map(|stop| self.enforce_index(cs, stop, &span))
                    .transpose()?;

                output.push(ResolvedAssigneeAccess::ArrayRange(start_index, stop_index));
                Ok(inner)
            }
            Expression::ArrayAccess(ArrayAccessExpression { array, index, .. }) => {
                let inner = self.prepare_mut_access(cs, array.get(), span, output)?;
                let index = self.enforce_index(cs, index.get(), &span)?;

                output.push(ResolvedAssigneeAccess::ArrayIndex(index));
                Ok(inner)
            }
            Expression::TupleAccess(TupleAccessExpression { tuple_ref, index, .. }) => {
                let inner = self.prepare_mut_access(cs, tuple_ref.get(), span, output)?;

                output.push(ResolvedAssigneeAccess::Tuple(*index, span.clone()));
                Ok(inner)
            }
            Expression::CircuitAccess(CircuitAccessExpression { target, member, .. }) => {
                if let Some(target) = target.get() {
                    let inner = self.prepare_mut_access(cs, target, span, output)?;

                    output.push(ResolvedAssigneeAccess::Member(member.clone()));
                    Ok(inner)
                } else {
                    Ok(None)
                }
            }
            Expression::VariableRef(variable_ref) => Ok(Some(variable_ref.variable.clone())),
            _ => Ok(None), // not a valid reference to mutable variable, we copy
        }
    }

    // resolve a mutable reference from an expression
    // return Ok(None) if no valid mutable reference, or Err(_) on more critical error
    pub fn resolve_mut_ref<CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        assignee: &'a Expression<'a>,
    ) -> Result<Option<Vec<&mut ConstrainedValue<'a, F, G>>>, StatementError> {
        let span = assignee.span().cloned().unwrap_or_default();

        let mut accesses = vec![];
        let target = self.prepare_mut_access(cs, assignee, &span, &mut accesses)?;
        if target.is_none() {
            return Ok(None);
        }
        let variable = target.unwrap();
        let variable = variable.borrow();

        let mut result = vec![match self.get_mut(variable.id) {
            Some(value) => value,
            None => return Err(StatementError::undefined_variable(variable.name.to_string(), span)),
        }];

        for access in accesses {
            result = Self::resolve_assignee_access(access, &span, result)?;
        }
        Ok(Some(result))
    }
}
