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

use super::CoreFunctionCall;
use crate::{ConstrainedValue, GroupType};
use leo_asg::{Function, Type};
use leo_errors::{Result, Span};

use snarkvm_fields::PrimeField;
use snarkvm_gadgets::bits::Boolean;
use snarkvm_r1cs::ConstraintSystem;

use std::{cell::RefCell, rc::Rc};

pub struct ToBits;

impl<'a, F: PrimeField, G: GroupType<F>> CoreFunctionCall<'a, F, G> for ToBits {
    fn call_function<CS: ConstraintSystem<F>>(
        &self,
        _cs: &mut CS,
        function: Rc<RefCell<Function<'a>>>,
        span: &Span,
        target: Option<ConstrainedValue<'a, F, G>>,
        arguments: Vec<ConstrainedValue<'a, F, G>>,
    ) -> Result<ConstrainedValue<'a, F, G>> {
        assert_eq!(arguments.len(), 0); // asg enforced
        assert!(function.borrow().name.borrow().name.as_ref() == "to_bits"); // asg enforced
        assert!(target.is_some()); // asg enforced

        let bits = target.unwrap().to_bits_le(span)?;

        Ok(ConstrainedValue::Array(
            bits.into_iter().map(ConstrainedValue::Boolean).collect(),
        ))
    }
}

pub struct FromBits;

fn unwrap_argument<F: PrimeField, G: GroupType<F>>(arg: ConstrainedValue<F, G>, expected_len: usize) -> Vec<Boolean> {
    if let ConstrainedValue::Array(args) = arg {
        assert_eq!(args.len(), expected_len); // asg enforced
        args.into_iter()
            .map(|item| {
                if let ConstrainedValue::Boolean(boolean) = item {
                    boolean
                } else {
                    panic!("illegal non-boolean type in from_bits call");
                }
            })
            .collect()
    } else {
        panic!("illegal non-array type in blake2s call");
    }
}

impl<'a, F: PrimeField, G: GroupType<F>> CoreFunctionCall<'a, F, G> for FromBits {
    fn call_function<CS: ConstraintSystem<F>>(
        &self,
        _cs: &mut CS,
        function: Rc<RefCell<Function<'a>>>,
        span: &Span,
        target: Option<ConstrainedValue<'a, F, G>>,
        mut arguments: Vec<ConstrainedValue<'a, F, G>>,
    ) -> Result<ConstrainedValue<'a, F, G>> {
        assert_eq!(arguments.len(), 1); // asg enforced
                                        // assert!(function.borrow().name.borrow().name.as_ref() == "from_bits"); // asg enforced
        assert!(target.is_some()); // asg enforced

        let type_ = match target {
            Some(ConstrainedValue::Named(name)) => name.to_string(),
            _ => unimplemented!("only named values implement to_bits gadget"),
        };

        let (expected_number_of_bits, output_type): (usize, leo_asg::Type) = match type_.as_str() {
            "u8" => (8, Type::Integer(leo_ast::IntegerType::U8)),
            "u16" => (16, Type::Integer(leo_ast::IntegerType::U16)),
            "u32" => (32, Type::Integer(leo_ast::IntegerType::U32)),
            "u64" => (64, Type::Integer(leo_ast::IntegerType::U64)),
            "u128" => (128, Type::Integer(leo_ast::IntegerType::U128)),
            "i8" => (8, Type::Integer(leo_ast::IntegerType::I8)),
            "i16" => (16, Type::Integer(leo_ast::IntegerType::I16)),
            "i32" => (32, Type::Integer(leo_ast::IntegerType::I32)),
            "i64" => (64, Type::Integer(leo_ast::IntegerType::I64)),
            "i128" => (128, Type::Integer(leo_ast::IntegerType::I128)),
            _ => unimplemented!(),
        };

        let bits = unwrap_argument(arguments.remove(0), expected_number_of_bits);

        // function.borrow_mut().output = output_type;

        ConstrainedValue::from_bits_le(&type_, &bits, span)
    }
}
