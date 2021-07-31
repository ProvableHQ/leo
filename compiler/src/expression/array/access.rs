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

//! Enforces array access in a compiled Leo program.

use std::convert::TryInto;

use crate::{
    arithmetic::*,
    program::ConstrainedProgram,
    relational::*,
    value::{ConstrainedValue, Integer},
    GroupType,
};
use leo_asg::{ConstInt, Expression};
use leo_errors::{CompilerError, LeoError, Span};


use snarkvm_fields::PrimeField;
use snarkvm_gadgets::{
    boolean::Boolean,
    traits::{
        eq::{EqGadget, EvaluateEqGadget},
        select::CondSelectGadget,
    },
};
use snarkvm_r1cs::ConstraintSystem;

impl<'a, F: PrimeField, G: GroupType<F>> ConstrainedProgram<'a, F, G> {
    pub fn array_bounds_check<CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        index_resolved: &Integer,
        array_len: u32,
        span: &Span,
    ) -> Result<(), LeoError> {
        let bounds_check = evaluate_lt::<F, G, CS>(
            cs,
            ConstrainedValue::Integer(index_resolved.clone()),
            ConstrainedValue::Integer(Integer::new(
                &ConstInt::U32(array_len).cast_to(&index_resolved.get_type()),
            )),
            span,
        )?;
        let bounds_check = match bounds_check {
            ConstrainedValue::Boolean(b) => b,
            _ => unimplemented!("illegal non-Integer returned from lt"),
        };
        let namespace_string = format!("evaluate array access bounds {}:{}", span.line_start, span.col_start);
        let mut unique_namespace = cs.ns(|| namespace_string);
        bounds_check
            .enforce_equal(&mut unique_namespace, &Boolean::Constant(true))
            .map_err(|e| LeoError::from(CompilerError::cannot_enforce("array bounds check".to_string(), e, span)))?;
        Ok(())
    }

    #[allow(clippy::too_many_arguments)]
    pub fn enforce_array_access<CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        array: &'a Expression<'a>,
        index: &'a Expression<'a>,
        span: &Span,
    ) -> Result<ConstrainedValue<'a, F, G>, LeoError> {
        let mut array = match self.enforce_expression(cs, array)? {
            ConstrainedValue::Array(array) => array,
            value => return Err(LeoError::from(CompilerError::undefined_array(value.to_string(), span))),
        };

        let index_resolved = self.enforce_index(cs, index, span)?;
        if let Some(resolved) = index_resolved.to_usize() {
            if resolved >= array.len() {
                return Err(LeoError::from(CompilerError::array_index_out_of_bounds(resolved, span));)
            }
            Ok(array[resolved].to_owned())
        } else {
            if array.is_empty() {
                return Err(LeoError::from(CompilerError::array_index_out_of_bounds(0, span));)
            }
            {
                let array_len: u32 = array
                    .len()
                    .try_into()
                    .map_err(|_| LeoError::from(CompilerError::array_length_out_of_bounds(span)))?;
                self.array_bounds_check(cs, &&index_resolved, array_len, span)?;
            }

            let mut current_value = array.pop().unwrap();
            for (i, item) in array.into_iter().enumerate() {
                let namespace_string = format!("evaluate array access eq {} {}:{}", i, span.line_start, span.col_start);
                let eq_namespace = cs.ns(|| namespace_string);

                let index_bounded = i
                    .try_into()
                    .map_err(|_| LeoError::from(CompilerError::array_index_out_of_legal_bounds(span)))?;
                let const_index = ConstInt::U32(index_bounded).cast_to(&index_resolved.get_type());
                let index_comparison = index_resolved
                    .evaluate_equal(eq_namespace, &Integer::new(&const_index))
                    .map_err(|_| LeoError::from(CompilerError::cannot_evaluate("==".to_string(), span)))?;

                let unique_namespace =
                    cs.ns(|| format!("select array access {} {}:{}", i, span.line_start, span.col_start));
                let value =
                    ConstrainedValue::conditionally_select(unique_namespace, &index_comparison, &item, &current_value)
                        .map_err(|e| LeoError::from(CompilerError::cannot_enforce("conditional select".to_string(), e, span)))?;
                current_value = value;
            }
            Ok(current_value)
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn enforce_array_range_access<CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        array: &'a Expression<'a>,
        left: Option<&'a Expression<'a>>,
        right: Option<&'a Expression<'a>>,
        length: usize,
        span: &Span,
    ) -> Result<ConstrainedValue<'a, F, G>, LeoError> {
        let array = match self.enforce_expression(cs, array)? {
            ConstrainedValue::Array(array) => array,
            value => return Err(LeoError::from(CompilerError::undefined_array(value.to_string(), span))),
        };

        let from_resolved = match left {
            Some(from_index) => self.enforce_index(cs, from_index, span)?,
            None => Integer::new(&ConstInt::U32(0)), // Array slice starts at index 0
        };
        let to_resolved = match right {
            Some(to_index) => self.enforce_index(cs, to_index, span)?,
            None => {
                let index_bounded: u32 = array
                    .len()
                    .try_into()
                    .map_err(|_| LeoError::from(CompilerError::array_length_out_of_bounds(span)))?;
                Integer::new(&ConstInt::U32(index_bounded))
            } // Array slice ends at array length
        };
        let const_dimensions = match (from_resolved.to_usize(), to_resolved.to_usize()) {
            (Some(from), Some(to)) => Some((from, to)),
            (Some(from), None) => Some((from, from + length)),
            (None, Some(to)) => Some((to - length, to)),
            (None, None) => None,
        };
        Ok(if let Some((left, right)) = const_dimensions {
            if right - left != length {
                return Err(LeoError::from(CompilerError::array_invalid_slice_length(span)));
            }
            if right > array.len() {
                return Err(LeoError::from(CompilerError::array_index_out_of_bounds(right, span)));
            }
            ConstrainedValue::Array(array[left..right].to_owned())
        } else {
            {
                let calc_len = enforce_sub::<F, G, _>(
                    cs,
                    ConstrainedValue::Integer(to_resolved.clone()),
                    ConstrainedValue::Integer(from_resolved.clone()),
                    span,
                )?;
                let calc_len = match calc_len {
                    ConstrainedValue::Integer(i) => i,
                    _ => unimplemented!("illegal non-Integer returned from sub"),
                };
                let namespace_string = format!(
                    "evaluate array range access length check {}:{}",
                    span.line_start, span.col_start
                );
                let mut unique_namespace = cs.ns(|| namespace_string);
                calc_len
                    .enforce_equal(&mut unique_namespace, &Integer::new(&ConstInt::U32(length as u32)))
                    .map_err(|e| LeoError::from(CompilerError::cannot_enforce("array length check".to_string(), e, span)))?;
            }
            {
                let bounds_check = evaluate_le::<F, G, _>(
                    cs,
                    ConstrainedValue::Integer(to_resolved),
                    ConstrainedValue::Integer(Integer::new(&ConstInt::U32(array.len() as u32))),
                    span,
                )?;
                let bounds_check = match bounds_check {
                    ConstrainedValue::Boolean(b) => b,
                    _ => unimplemented!("illegal non-Integer returned from le"),
                };
                let namespace_string = format!(
                    "evaluate array range access bounds {}:{}",
                    span.line_start, span.col_start
                );
                let mut unique_namespace = cs.ns(|| namespace_string);
                bounds_check
                    .enforce_equal(&mut unique_namespace, &Boolean::Constant(true))
                    .map_err(|e| LeoError::from(CompilerError::cannot_enforce("array bounds check".to_string(), e, span)))?;
            }
            let mut windows = array.windows(length);
            let mut result = ConstrainedValue::Array(vec![]);

            for i in 0..length {
                let window = if let Some(window) = windows.next() {
                    window
                } else {
                    break;
                };
                let array_value = ConstrainedValue::Array(window.to_vec());
                let mut unique_namespace =
                    cs.ns(|| format!("array index eq-check {} {}:{}", i, span.line_start, span.col_start));

                let equality = evaluate_eq::<F, G, _>(
                    &mut unique_namespace,
                    ConstrainedValue::Integer(from_resolved.clone()),
                    ConstrainedValue::Integer(Integer::new(&ConstInt::U32(i as u32))),
                    span,
                )?;
                let equality = match equality {
                    ConstrainedValue::Boolean(b) => b,
                    _ => unimplemented!("unexpected non-Boolean for evaluate_eq"),
                };

                let unique_namespace =
                    unique_namespace.ns(|| format!("array index {} {}:{}", i, span.line_start, span.col_start));
                result = ConstrainedValue::conditionally_select(unique_namespace, &equality, &array_value, &result)
                    .map_err(|e| LeoError::from(CompilerError::cannot_enforce("conditional select".to_string(), e, span)))?;
            }
            result
        })
    }
}
