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
use crate::{ConstrainedValue, GroupType, Integer};
use leo_asg::Function;
use leo_errors::{Result, Span};

use snarkvm_fields::PrimeField;
use snarkvm_gadgets::integers::uint::UInt8;
use snarkvm_r1cs::ConstraintSystem;

use std::{cell::RefCell, rc::Rc};

pub struct ToBytes;

impl<'a, F: PrimeField, G: GroupType<F>> CoreFunctionCall<'a, F, G> for ToBytes {
    fn call_function<CS: ConstraintSystem<F>>(
        &self,
        _cs: &mut CS,
        function: Rc<RefCell<Function<'a>>>,
        span: &Span,
        target: Option<ConstrainedValue<'a, F, G>>,
        arguments: Vec<ConstrainedValue<'a, F, G>>,
    ) -> Result<ConstrainedValue<'a, F, G>> {
        assert_eq!(arguments.len(), 0); // asg enforced
        assert!((*function).borrow().name.borrow().name.as_ref() == "to_bytes"); // asg enforced
        assert!(target.is_some()); // asg enforced

        let bytes = target.unwrap().to_bytes(span)?;

        Ok(ConstrainedValue::Array(
            bytes
                .into_iter()
                .map(Integer::U8)
                .map(ConstrainedValue::Integer)
                .collect(),
        ))
    }
}

pub struct FromBytes;

fn unwrap_argument<F: PrimeField, G: GroupType<F>>(arg: ConstrainedValue<F, G>, _expected_len: usize) -> Vec<UInt8> {
    if let ConstrainedValue::Array(args) = arg {
        // assert_eq!(args.len(), 32); // asg enforced
        args.into_iter()
            .map(|item| {
                if let ConstrainedValue::Integer(Integer::U8(u8int)) = item {
                    u8int
                } else {
                    panic!("illegal non-u8 type in from_bits call");
                }
            })
            .collect()
    } else {
        panic!("illegal non-array type in blake2s call");
    }
}

impl<'a, F: PrimeField, G: GroupType<F>> CoreFunctionCall<'a, F, G> for FromBytes {
    fn call_function<CS: ConstraintSystem<F>>(
        &self,
        _cs: &mut CS,
        function: Rc<RefCell<Function<'a>>>,
        span: &Span,
        target: Option<ConstrainedValue<'a, F, G>>,
        mut arguments: Vec<ConstrainedValue<'a, F, G>>,
    ) -> Result<ConstrainedValue<'a, F, G>> {
        assert_eq!(arguments.len(), 1); // asg enforced
        assert!((*function).borrow().name.borrow().name.as_ref() == "from_bytes"); // asg enforced
        assert!(target.is_some()); // asg enforced

        let type_ = match target {
            Some(ConstrainedValue::Named(name)) => name.to_string(),
            _ => unimplemented!("only named values implement to_bits gadget"),
        };

        let expected_number_of_bytes: usize = match type_.as_str() {
            "u8" => 8,
            "u16" => 16,
            "u32" => 32,
            "u64" => 64,
            "u128" => 128,
            "i8" => 8,
            "i16" => 16,
            "i32" => 32,
            "i64" => 64,
            "i128" => 128,
            _ => unimplemented!(),
        };

        let bytes = unwrap_argument(arguments.remove(0), expected_number_of_bytes);

        ConstrainedValue::from_bytes(&type_, &bytes, span)
    }
}
