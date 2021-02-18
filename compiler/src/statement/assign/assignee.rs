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

use crate::{errors::StatementError, program::ConstrainedProgram, value::ConstrainedValue, GroupType};
use leo_asg::{AssignAccess, AssignStatement, Identifier, Span};

use snarkvm_models::{curves::PrimeField, gadgets::r1cs::ConstraintSystem};

pub(crate) enum ResolvedAssigneeAccess {
    ArrayRange(Option<usize>, Option<usize>),
    ArrayIndex(usize),
    Tuple(usize, Span),
    Member(Identifier),
}

impl<'a, F: PrimeField, G: GroupType<F>> ConstrainedProgram<'a, F, G> {
    pub fn resolve_assign<CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        assignee: &AssignStatement<'a>,
    ) -> Result<Vec<&mut ConstrainedValue<'a, F, G>>, StatementError> {
        let span = assignee.span.clone().unwrap_or_default();

        let resolved_accesses = assignee
            .target_accesses
            .iter()
            .map(|access| match access {
                AssignAccess::ArrayRange(start, stop) => {
                    let start_index = start
                        .get()
                        .map(|start| self.enforce_index(cs, start, &span))
                        .transpose()?;
                    let stop_index = stop.get().map(|stop| self.enforce_index(cs, stop, &span)).transpose()?;
                    Ok(ResolvedAssigneeAccess::ArrayRange(start_index, stop_index))
                }
                AssignAccess::ArrayIndex(index) => {
                    let index = self.enforce_index(cs, index.get(), &span)?;

                    Ok(ResolvedAssigneeAccess::ArrayIndex(index))
                }
                AssignAccess::Tuple(index) => Ok(ResolvedAssigneeAccess::Tuple(*index, span.clone())),
                AssignAccess::Member(identifier) => Ok(ResolvedAssigneeAccess::Member(identifier.clone())),
            })
            .collect::<Result<Vec<_>, crate::errors::ExpressionError>>()?;

        let variable = assignee.target_variable.get().borrow();

        let mut result = vec![match self.get_mut(&variable.id) {
            Some(value) => value,
            None => return Err(StatementError::undefined_variable(variable.name.to_string(), span)),
        }];

        for access in resolved_accesses {
            result = Self::resolve_assignee_access(access, &span, result)?;
        }
        Ok(result)
    }

    fn check_range_index(start_index: usize, stop_index: usize, len: usize, span: &Span) -> Result<(), StatementError> {
        if stop_index < start_index {
            Err(StatementError::array_assign_range_order(
                start_index,
                stop_index,
                len,
                span.clone(),
            ))
        } else if start_index > len {
            Err(StatementError::array_assign_index_bounds(
                start_index,
                len,
                span.clone(),
            ))
        } else if stop_index > len {
            Err(StatementError::array_assign_index_bounds(stop_index, len, span.clone()))
        } else {
            Ok(())
        }
    }

    // todo: this can prob have most of its error checking removed
    pub(crate) fn resolve_assignee_access<'b>(
        access: ResolvedAssigneeAccess,
        span: &Span,
        mut value: Vec<&'b mut ConstrainedValue<'a, F, G>>,
    ) -> Result<Vec<&'b mut ConstrainedValue<'a, F, G>>, StatementError> {
        match access {
            ResolvedAssigneeAccess::ArrayIndex(index) => {
                if value.len() != 1 {
                    return Err(StatementError::array_assign_interior_index(span.clone()));
                }
                match value.remove(0) {
                    ConstrainedValue::Array(old) => {
                        if index > old.len() {
                            Err(StatementError::array_assign_index_bounds(
                                index,
                                old.len(),
                                span.clone(),
                            ))
                        } else {
                            Ok(vec![old.get_mut(index).unwrap()])
                        }
                    }
                    _ => Err(StatementError::array_assign_index(span.clone())),
                }
            }
            ResolvedAssigneeAccess::ArrayRange(start_index, stop_index) => {
                let start_index = start_index.unwrap_or(0);

                if value.len() == 1 {
                    // not a range of a range
                    match value.remove(0) {
                        ConstrainedValue::Array(old) => {
                            let stop_index = stop_index.unwrap_or(old.len());
                            Self::check_range_index(start_index, stop_index, old.len(), &span)?;

                            Ok(old[start_index..stop_index].iter_mut().collect())
                        }
                        _ => Err(StatementError::array_assign_index(span.clone())),
                    }
                } else {
                    // range of a range
                    let stop_index = stop_index.unwrap_or(value.len());
                    Self::check_range_index(start_index, stop_index, value.len(), &span)?;

                    Ok(value.drain(start_index..stop_index).collect())
                }
            }
            ResolvedAssigneeAccess::Tuple(index, span) => {
                if value.len() != 1 {
                    return Err(StatementError::array_assign_interior_index(span));
                }
                match value.remove(0) {
                    ConstrainedValue::Tuple(old) => {
                        if index > old.len() {
                            Err(StatementError::tuple_assign_index_bounds(index, old.len(), span))
                        } else {
                            Ok(vec![&mut old[index]])
                        }
                    }
                    _ => Err(StatementError::tuple_assign_index(span)),
                }
            }
            ResolvedAssigneeAccess::Member(name) => {
                if value.len() != 1 {
                    return Err(StatementError::array_assign_interior_index(span.clone()));
                }
                match value.remove(0) {
                    ConstrainedValue::CircuitExpression(_variable, members) => {
                        // Modify the circuit variable in place
                        let matched_variable = members.iter_mut().find(|member| member.0 == name);

                        match matched_variable {
                            Some(member) => Ok(vec![&mut member.1]),
                            None => {
                                // Throw an error if the circuit variable does not exist in the circuit
                                Err(StatementError::undefined_circuit_variable(
                                    name.to_string(),
                                    span.to_owned(),
                                ))
                            }
                        }
                    }
                    // Throw an error if the circuit definition does not exist in the file
                    x => Err(StatementError::undefined_circuit(x.to_string(), span.to_owned())),
                }
            }
        }
    }
}
