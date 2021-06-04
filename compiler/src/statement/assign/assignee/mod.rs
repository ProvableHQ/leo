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
use leo_asg::{AssignAccess, AssignOperation, AssignStatement, Span};

use snarkvm_fields::PrimeField;
use snarkvm_gadgets::utilities::boolean::Boolean;
use snarkvm_r1cs::ConstraintSystem;

mod array_index;
mod array_range_index;
mod member;
mod tuple;

struct ResolverContext<'a, 'b, F: PrimeField, G: GroupType<F>> {
    input: Vec<&'b mut ConstrainedValue<'a, F, G>>,
    span: Span,
    target_value: ConstrainedValue<'a, F, G>,
    remaining_accesses: Vec<&'b AssignAccess<'a>>,
    indicator: &'b Boolean,
    operation: AssignOperation,
}

// pub enum ConstrainedValueOrRef<'b, 'a, F: PrimeField, G: GroupType<F>> {
//     Value(&'b mut ConstrainedValue<'a, F, G>),
//     Ref(u32),
// }

// impl<'a, 'b, F: PrimeField, G: GroupType<F>> ConstrainedValueOrRef<'a, 'b, F, G> {
//     pub fn resolve_mut(self, program: &'b mut ConstrainedProgram<'a, F, G>) -> &'b mut ConstrainedValue<'a, F, G> {
//         match self {
//             ConstrainedValueOrRef::Value(x) => x,
//             ConstrainedValueOrRef::Ref(x) => program.get_mut(x).expect("missing var ref"),
//         }
//     }
// }

impl<'a, F: PrimeField, G: GroupType<F>> ConstrainedProgram<'a, F, G> {
    fn enforce_assign_context<'b, CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        context: &ResolverContext<'a, 'b, F, G>,
        target: &mut ConstrainedValue<'a, F, G>,
    ) -> Result<(), StatementError> {
        Self::enforce_assign_operation(
            cs,
            context.indicator,
            format!("select_assign {}:{}", &context.span.line_start, &context.span.col_start),
            &context.operation,
            target,
            context.target_value.clone(),
            &context.span,
        )
    }

    fn resolve_target_access<'b, CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        mut context: ResolverContext<'a, 'b, F, G>,
    ) -> Result<(), StatementError> {
        if context.remaining_accesses.is_empty() {
            if context.input.len() != 1 {
                panic!("invalid non-array-context multi-value assignment");
            }
            let input = context.input.remove(0);
            self.enforce_assign_context(cs, &context, input)?;
            return Ok(());
        }
        match context.remaining_accesses.pop().unwrap() {
            AssignAccess::ArrayRange(start, stop) => {
                self.resolve_target_access_array_range(cs, context, start.get(), stop.get())
            }
            AssignAccess::ArrayIndex(index) => self.resolve_target_access_array_index(cs, context, index.get()),
            AssignAccess::Tuple(index) => self.resolve_target_access_tuple(cs, context, *index),
            AssignAccess::Member(identifier) => self.resolve_target_access_member(cs, context, identifier),
        }
    }

    pub fn resolve_assign<CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        assignee: &AssignStatement<'a>,
        target_value: ConstrainedValue<'a, F, G>,
        indicator: &Boolean,
    ) -> Result<(), StatementError> {
        let span = assignee.span.clone().unwrap_or_default();
        let variable = assignee.target_variable.get().borrow();

        let mut target = self.get(variable.id).unwrap().clone();
        self.resolve_target_access(cs, ResolverContext {
            input: vec![&mut target],
            span,
            target_value,
            remaining_accesses: assignee.target_accesses.iter().rev().collect(),
            indicator,
            operation: assignee.operation,
        })?;
        *self.get_mut(variable.id).unwrap() = target;
        Ok(())
    }

    pub(crate) fn check_range_index(
        start_index: usize,
        stop_index: usize,
        len: usize,
        span: &Span,
    ) -> Result<(), StatementError> {
        if stop_index < start_index {
            Err(StatementError::array_assign_range_order(
                start_index,
                stop_index,
                len,
                span,
            ))
        } else if start_index > len {
            Err(StatementError::array_assign_index_bounds(start_index, len, span))
        } else if stop_index > len {
            Err(StatementError::array_assign_index_bounds(stop_index, len, span))
        } else {
            Ok(())
        }
    }

    // // todo: this can prob have most of its error checking removed
    // pub(crate) fn resolve_assignee_access<'b, CS: ConstraintSystem<F>>(
    //     cs: &mut CS,
    //     access: ResolvedAssigneeAccess,
    //     span: &Span,
    //     mut value: Vec<&'b mut ConstrainedValue<'a, F, G>>,
    // ) -> Result<Vec<&'b mut ConstrainedValue<'a, F, G>>, StatementError> {
    //     match access {
    //         ResolvedAssigneeAccess::ArrayIndex(index) => {
    //             if value.len() != 1 {
    //                 return Err(StatementError::array_assign_interior_index(span));
    //             }
    //             match value.remove(0) {
    //                 ConstrainedValue::Array(old) => {
    //                     if index > old.len() {
    //                         Err(StatementError::array_assign_index_bounds(index, old.len(), span))
    //                     } else {
    //                         Ok(vec![old.get_mut(index).unwrap()])
    //                     }
    //                 }
    //                 _ => Err(StatementError::array_assign_index(span)),
    //             }
    //         },
    //         ResolvedAssigneeAccess::DynArrayIndex(index_resolved) => {
    //             if value.len() != 1 {
    //                 return Err(StatementError::array_assign_interior_index(span));
    //             }
    //             match value.remove(0) {
    //                 ConstrainedValue::Array(old) => {
    //                     for (i, item) in old.into_iter().enumerate() {
    //                         let namespace_string = format!("evaluate dyn array assignment eq {} {}:{}", i, span.line_start, span.col_start);
    //                         let eq_namespace = cs.ns(|| namespace_string);

    //                         let index_bounded = i
    //                             .try_into()
    //                             .map_err(|_| ExpressionError::array_index_out_of_legal_bounds(span))?;
    //                         let const_index = ConstInt::U32(index_bounded).cast_to(&index_resolved.get_type());
    //                         let index_comparison = index_resolved
    //                             .evaluate_equal(eq_namespace, &Integer::new(&const_index))
    //                             .map_err(|_| ExpressionError::cannot_evaluate("==".to_string(), span))?;

    //                         let unique_namespace =
    //                             cs.ns(|| format!("select array dyn assignment {} {}:{}", i, span.line_start, span.col_start));
    //                         let mut_container = ConstrainedValue::Illegal;
    //                         let value =
    //                             ConstrainedValue::conditionally_select(unique_namespace, &index_comparison, &mut_container, &item)
    //                                 .map_err(|e| ExpressionError::cannot_enforce("conditional select".to_string(), e, span))?;

    //                     }
    //                     if index > old.len() {
    //                         Err(StatementError::array_assign_index_bounds(index, old.len(), span))
    //                     } else {
    //                         Ok(vec![old.get_mut(index).unwrap()])
    //                     }
    //                 }
    //                 _ => Err(StatementError::array_assign_index(span)),
    //             }
    //         },
    //         ResolvedAssigneeAccess::ArrayRange(start_index, stop_index) => {
    //             let start_index = start_index.unwrap_or(0);

    //             if value.len() == 1 {
    //                 // not a range of a range
    //                 match value.remove(0) {
    //                     ConstrainedValue::Array(old) => {
    //                         let stop_index = stop_index.unwrap_or(old.len());
    //                         Self::check_range_index(start_index, stop_index, old.len(), span)?;

    //                         Ok(old[start_index..stop_index].iter_mut().collect())
    //                     }
    //                     _ => Err(StatementError::array_assign_index(span)),
    //                 }
    //             } else {
    //                 // range of a range
    //                 let stop_index = stop_index.unwrap_or(value.len());
    //                 Self::check_range_index(start_index, stop_index, value.len(), span)?;

    //                 Ok(value.drain(start_index..stop_index).collect())
    //             }
    //         }
    //         ResolvedAssigneeAccess::Tuple(index, span) => {
    //             if value.len() != 1 {
    //                 return Err(StatementError::array_assign_interior_index(&span));
    //             }
    //             match value.remove(0) {
    //                 ConstrainedValue::Tuple(old) => {
    //                     if index > old.len() {
    //                         Err(StatementError::tuple_assign_index_bounds(index, old.len(), &span))
    //                     } else {
    //                         Ok(vec![&mut old[index]])
    //                     }
    //                 }
    //                 _ => Err(StatementError::tuple_assign_index(&span)),
    //             }
    //         }
    //         ResolvedAssigneeAccess::Member(name) => {
    //             if value.len() != 1 {
    //                 return Err(StatementError::array_assign_interior_index(span));
    //             }
    //             match value.remove(0) {
    //                 ConstrainedValue::CircuitExpression(_variable, members) => {
    //                     // Modify the circuit variable in place
    //                     let matched_variable = members.iter_mut().find(|member| member.0 == name);

    //                     match matched_variable {
    //                         Some(member) => Ok(vec![&mut member.1]),
    //                         None => {
    //                             // Throw an error if the circuit variable does not exist in the circuit
    //                             Err(StatementError::undefined_circuit_variable(name.to_string(), span))
    //                         }
    //                     }
    //                 }
    //                 // Throw an error if the circuit definition does not exist in the file
    //                 x => Err(StatementError::undefined_circuit(x.to_string(), span)),
    //             }
    //         }
    //     }
    // }
}
