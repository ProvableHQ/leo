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

//! Resolves assignees in a compiled Leo program.

use crate::{
    errors::StatementError,
    new_scope,
    parse_index,
    program::ConstrainedProgram,
    value::ConstrainedValue,
    GroupType,
};
use leo_ast::{Assignee, AssigneeAccess, Identifier, PositiveNumber, Span};

use snarkvm_models::{
    curves::{Field, PrimeField},
    gadgets::r1cs::ConstraintSystem,
};

enum ResolvedAssigneeAccess {
    ArrayRange(Option<usize>, Option<usize>),
    ArrayIndex(usize),
    Tuple(PositiveNumber, Span),
    Member(Identifier),
}

impl<F: Field + PrimeField, G: GroupType<F>> ConstrainedProgram<F, G> {
    pub fn resolve_assignee<CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        file_scope: &str,
        function_scope: &str,
        declared_circuit_reference: &str,
        mut_self: bool,
        assignee: Assignee,
    ) -> Result<Vec<&mut ConstrainedValue<F, G>>, StatementError> {
        let value_ref = if assignee.identifier.is_self() {
            if !mut_self {
                return Err(StatementError::immutable_assign("self".to_string(), assignee.span));
            }
            declared_circuit_reference.to_string()
        } else {
            new_scope(&function_scope, &assignee.identifier().to_string())
        };

        let span = assignee.span.clone();
        let identifier_string = assignee.identifier.to_string();

        let resolved_accesses = assignee
            .accesses
            .into_iter()
            .map(|access| match access {
                AssigneeAccess::ArrayRange(start, stop) => {
                    let start_index = start
                        .map(|start| self.enforce_index(cs, file_scope, function_scope, start, &span))
                        .transpose()?;
                    let stop_index = stop
                        .map(|stop| self.enforce_index(cs, file_scope, function_scope, stop, &span))
                        .transpose()?;
                    Ok(ResolvedAssigneeAccess::ArrayRange(start_index, stop_index))
                }
                AssigneeAccess::ArrayIndex(index) => {
                    let index = self.enforce_index(cs, file_scope, function_scope, index, &span)?;

                    Ok(ResolvedAssigneeAccess::ArrayIndex(index))
                }
                AssigneeAccess::Tuple(index, span) => Ok(ResolvedAssigneeAccess::Tuple(index, span)),
                AssigneeAccess::Member(identifier) => Ok(ResolvedAssigneeAccess::Member(identifier)),
            })
            .collect::<Result<Vec<_>, crate::errors::ExpressionError>>()?;

        let mut result = vec![match self.get_mut(&value_ref) {
            Some(value) => match value {
                ConstrainedValue::Mutable(mutable) => &mut **mutable,
                _ => return Err(StatementError::immutable_assign(identifier_string, span)),
            },
            None => return Err(StatementError::undefined_variable(identifier_string, span)),
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

    // discards unnecessary mutable wrappers
    fn unwrap_mutable(input: &mut ConstrainedValue<F, G>) -> &mut ConstrainedValue<F, G> {
        match input {
            ConstrainedValue::Mutable(x) => &mut **x,
            x => x,
        }
    }

    fn resolve_assignee_access<'a>(
        access: ResolvedAssigneeAccess,
        span: &Span,
        mut value: Vec<&'a mut ConstrainedValue<F, G>>,
    ) -> Result<Vec<&'a mut ConstrainedValue<F, G>>, StatementError> {
        match access {
            ResolvedAssigneeAccess::ArrayIndex(index) => {
                if value.len() != 1 {
                    return Err(StatementError::array_assign_interior_index(span.clone()));
                }
                match Self::unwrap_mutable(value.remove(0)) {
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
                    match Self::unwrap_mutable(value.remove(0)) {
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

                    Ok(value.drain(start_index..stop_index).map(Self::unwrap_mutable).collect())
                }
            }
            ResolvedAssigneeAccess::Tuple(index, span) => {
                let index = parse_index(&index, &span)?;

                if value.len() != 1 {
                    return Err(StatementError::array_assign_interior_index(span));
                }
                match Self::unwrap_mutable(value.remove(0)) {
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
                match Self::unwrap_mutable(value.remove(0)) {
                    ConstrainedValue::CircuitExpression(_variable, members) => {
                        // Modify the circuit variable in place
                        let matched_variable = members.iter_mut().find(|member| member.0 == name);

                        match matched_variable {
                            Some(member) => match &mut member.1 {
                                ConstrainedValue::Function(_circuit_identifier, function) => {
                                    // Throw an error if we try to mutate a circuit function
                                    Err(StatementError::immutable_circuit_function(
                                        function.identifier.to_string(),
                                        span.to_owned(),
                                    ))
                                }
                                ConstrainedValue::Static(_circuit_function) => {
                                    // Throw an error if we try to mutate a static circuit function
                                    Err(StatementError::immutable_circuit_function(
                                        "static".into(),
                                        span.to_owned(),
                                    ))
                                }
                                value => Ok(vec![value]),
                            },
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
                    _ => Err(StatementError::undefined_circuit(name.to_string(), span.to_owned())),
                }
            }
        }
    }

    pub fn get_mutable_assignee(
        &mut self,
        name: &str,
        span: &Span,
    ) -> Result<&mut ConstrainedValue<F, G>, StatementError> {
        // Check that assignee exists and is mutable
        Ok(match self.get_mut(name) {
            Some(value) => match value {
                ConstrainedValue::Mutable(mutable_value) => {
                    // Get the mutable value.
                    mutable_value.get_inner_mut();

                    // Return the mutable value.
                    mutable_value
                }
                _ => return Err(StatementError::immutable_assign(name.to_owned(), span.to_owned())),
            },
            None => return Err(StatementError::undefined_variable(name.to_owned(), span.to_owned())),
        })
    }
}
