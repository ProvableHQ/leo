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

use super::CoreCircuit;
use crate::errors::ExpressionError;
use crate::ConstrainedValue;
use crate::GroupType;
use crate::Integer;
use leo_asg::Function;
use leo_asg::Span;
use snarkvm_gadgets::algorithms::prf::Blake2sGadget;
use snarkvm_gadgets::traits::uint::UInt8;
use snarkvm_gadgets::traits::ToBytesGadget;
use snarkvm_models::curves::PrimeField;
use snarkvm_models::gadgets::algorithms::PRFGadget;
use snarkvm_r1cs::ConstraintSystem;

pub struct Blake2s;

fn unwrap_argument<F: PrimeField, G: GroupType<F>>(arg: ConstrainedValue<F, G>) -> Vec<UInt8> {
    if let ConstrainedValue::Array(args) = arg {
        assert_eq!(args.len(), 32); // asg enforced
        args.into_iter()
            .map(|item| {
                if let ConstrainedValue::Integer(Integer::U8(item)) = item {
                    item
                } else {
                    panic!("illegal non-u8 type in blake2s call");
                }
            })
            .collect()
    } else {
        panic!("illegal non-array type in blake2s call");
    }
}

impl<'a, F: PrimeField, G: GroupType<F>> CoreCircuit<'a, F, G> for Blake2s {
    fn call_function<CS: ConstraintSystem<F>>(
        &self,
        cs: &mut CS,
        function: &'a Function<'a>,
        span: &Span,
        target: Option<ConstrainedValue<'a, F, G>>,
        mut arguments: Vec<ConstrainedValue<'a, F, G>>,
    ) -> Result<ConstrainedValue<'a, F, G>, ExpressionError> {
        assert_eq!(arguments.len(), 2); // asg enforced
        assert!(function.name.borrow().name == "hash"); // asg enforced
        assert!(target.is_none()); // asg enforced
        let input = unwrap_argument(arguments.remove(1));
        let seed = unwrap_argument(arguments.remove(0));

        let digest =
            Blake2sGadget::check_evaluation_gadget(cs.ns(|| "blake2s hash"), &seed[..], &input[..]).map_err(|e| {
                ExpressionError::cannot_enforce("Blake2s check evaluation gadget".to_owned(), e, span.clone())
            })?;

        Ok(ConstrainedValue::Array(
            digest
                .to_bytes(cs)
                .map_err(|e| ExpressionError::cannot_enforce("Vec<UInt8> ToBytes".to_owned(), e, span.clone()))?
                .into_iter()
                .map(Integer::U8)
                .map(ConstrainedValue::Integer)
                .collect(),
        ))
    }
}
