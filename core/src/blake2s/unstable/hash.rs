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

use crate::CoreFunctionArgument;

use snarkos_gadgets::algorithms::prf::Blake2sGadget;
use snarkos_models::{
    curves::{Field, PrimeField},
    gadgets::{
        algorithms::PRFGadget,
        r1cs::ConstraintSystem,
        utilities::{uint::UInt8, ToBytesGadget},
    },
};

#[derive(Clone, PartialEq, Eq)]
pub struct Blake2sFunction {}

impl Blake2sFunction {
    pub fn hash<F: Field + PrimeField, CS: ConstraintSystem<F>>(
        mut cs: CS,
        arguments: Vec<CoreFunctionArgument>,
        //_span: Span // todo: return errors using `leo-typed` span
    ) -> Vec<UInt8> {
        // The check evaluation gadget should have two arguments: seed and input
        if arguments.len() != 2 {
            println!("incorrect number of arguments")
        }

        let seed = &arguments[0].0[..];
        let input = &arguments[1].0[..];

        let res = Blake2sGadget::check_evaluation_gadget(cs.ns(|| "blake2s hash"), seed, input).unwrap();
        let bytes = res.to_bytes(cs).unwrap();
        // println!("output {:?}", res.to_bytes(cs).unwrap().len());
        bytes
    }
}
