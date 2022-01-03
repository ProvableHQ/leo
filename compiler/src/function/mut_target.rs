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

//! Resolves assignees in a compiled Leo program.

use std::cell::Cell;

use crate::program::Program;
use leo_asg::{
    AccessExpression, ArrayAccess, ArrayRangeAccess, AssignAccess, AssignOperation, AssignStatement, CircuitAccess,
    Expression, Node, TupleAccess, Variable,
};
use leo_errors::Result;
use snarkvm_ir::Value;

impl<'a> Program<'a> {
    fn prepare_mut_access(
        out: &mut Vec<AssignAccess<'a>>,
        expr: &'a Expression<'a>,
    ) -> Result<Option<&'a Variable<'a>>> {
        match expr {
            Expression::Access(access) => match access {
                AccessExpression::ArrayRange(ArrayRangeAccess { array, left, right, .. }) => {
                    let inner = Self::prepare_mut_access(out, array.get())?;

                    out.push(AssignAccess::ArrayRange(left.clone(), right.clone()));

                    Ok(inner)
                }
                AccessExpression::Array(ArrayAccess { array, index, .. }) => {
                    let inner = Self::prepare_mut_access(out, array.get())?;

                    out.push(AssignAccess::ArrayIndex(index.clone()));
                    Ok(inner)
                }
                AccessExpression::Circuit(CircuitAccess { target, member, .. }) => {
                    if let Some(target) = target.get() {
                        let inner = Self::prepare_mut_access(out, target)?;

                        out.push(AssignAccess::Member(member.clone()));
                        Ok(inner)
                    } else {
                        Ok(None)
                    }
                }
                AccessExpression::Tuple(TupleAccess { tuple_ref, index, .. }) => {
                    let inner = Self::prepare_mut_access(out, tuple_ref.get())?;

                    out.push(AssignAccess::Tuple(*index));
                    Ok(inner)
                }
            },
            Expression::VariableRef(variable_ref) => Ok(Some(variable_ref.variable)),
            _ => Ok(None), // not a valid reference to mutable variable, we copy
        }
    }

    // resolve a mutable reference from an expression
    // return false if no valid mutable reference, or Err(_) on more critical error
    pub fn resolve_mut_ref(&mut self, assignee: &'a Expression<'a>, target_value: Value) -> Result<bool> {
        let mut accesses = vec![];
        let target = Self::prepare_mut_access(&mut accesses, assignee)?;
        if target.is_none() {
            return Ok(false);
        }
        let variable = target.unwrap();

        self.resolve_assign(
            &AssignStatement {
                id: 0u32.into(), // This is okay, since constraints are compiled after ASG passes
                parent: Cell::new(None),
                span: assignee.span().cloned(),
                operation: AssignOperation::Assign,
                target_variable: Cell::new(variable),
                target_accesses: accesses,
                value: Cell::new(assignee),
            },
            target_value,
        )?;

        Ok(true)
    }
}
