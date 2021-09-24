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

use super::CoreCircuitFuncCall;
use crate::{ConstrainedValue, GroupType};
use leo_asg::Function;
use leo_errors::{Result, Span};

use snarkvm_fields::PrimeField;
use snarkvm_r1cs::ConstraintSystem;

macro_rules! scalar_type {
    ($scalar_type_name:ident) => {
        pub struct $scalar_type_name;

        impl<'a, F: PrimeField, G: GroupType<F>> CoreCircuitFuncCall<'a, F, G> for $scalar_type_name {
            fn call_function<CS: ConstraintSystem<F>>(
                &self,
                _cs: &mut CS,
                function: &'a Function<'a>,
                span: &Span,
                target: Option<ConstrainedValue<'a, F, G>>,
                mut arguments: Vec<ConstrainedValue<'a, F, G>>,
            ) -> Result<ConstrainedValue<'a, F, G>> {
                let function_ident = function.name.borrow();
                let function_name = function_ident.name.as_ref();
                match (function_name, arguments.len()) {
                    ("to_bits", 0) => {
                        assert_eq!(arguments.len(), 0);
                        assert!(target.is_some());

                        crate::common::bits::to_bits(target.unwrap(), span)
                    }
                    ("from_bits", 1) => {
                        assert_eq!(arguments.len(), 1);
                        assert!(target.is_none());
                        crate::common::bits::from_bits(arguments.remove(0), function.output.clone(), span)
                    }
                    ("to_bytes", 0) => {
                        assert_eq!(arguments.len(), 0);
                        assert!(target.is_some());

                        crate::common::bytes::to_bytes(target.unwrap(), span)
                    }
                    ("from_bytes", 1) => {
                        assert_eq!(arguments.len(), 1);
                        assert!(target.is_none());

                        crate::common::bytes::from_bytes(arguments.remove(0), function.output.clone(), span)
                    }
                    _ => unreachable!(),
                }
            }
        }
    };
}

scalar_type!(LeoAddress);
scalar_type!(LeoBool);
scalar_type!(LeoChar);
scalar_type!(LeoField);
scalar_type!(LeoGroup);
scalar_type!(LeoI8);
scalar_type!(LeoI16);
scalar_type!(LeoI32);
scalar_type!(LeoI64);
scalar_type!(LeoI128);
scalar_type!(LeoU8);
scalar_type!(LeoU16);
scalar_type!(LeoU32);
scalar_type!(LeoU64);
scalar_type!(LeoU128);
