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

//! A field value in a compiled Leo program.

pub mod input;

pub mod field_type;
pub use self::field_type::*;

use snark_std::{field::Field as FieldStd, ops::Equal, traits::CircuitBuilder};
use snarkvm_errors::gadgets::SynthesisError;
use snarkvm_models::{
    curves::{Field, PrimeField},
    gadgets::{curves::FpGadget, r1cs::ConstraintSystem, utilities::boolean::Boolean},
};

use serde::__private::PhantomData;
use std::{
    cell::{RefCell, RefMut},
    rc::Rc,
};

pub struct FieldCircuitBuilder<F: Field + PrimeField, CS: ConstraintSystem<F>>(Rc<RefCell<CS>>, PhantomData<F>);

impl<F: Field + PrimeField, CS: ConstraintSystem<F>> CircuitBuilder<F> for FieldCircuitBuilder<F, CS> {
    type CS = CS;

    fn borrow_mut(&self) -> RefMut<Self::CS> {
        self.0.borrow_mut()
    }
}

impl<F: Field + PrimeField, CS: ConstraintSystem<F>> Clone for FieldCircuitBuilder<F, CS> {
    fn clone(&self) -> Self {
        Self(self.0.clone(), PhantomData)
    }
}

pub fn evaluate_eq_fp_gadget<F: Field + PrimeField, CS: ConstraintSystem<F>>(
    cs: CS,
    a: &FpGadget<F>,
    b: &FpGadget<F>,
) -> Result<Boolean, SynthesisError> {
    let builder = FieldCircuitBuilder {
        0: Rc::new(RefCell::new(cs)),
        1: Default::default(),
    };
    let a_std = FieldStd::from((a.clone(), builder.clone()));
    let b_std = FieldStd::from((b.clone(), builder.clone()));

    let result_std = a_std.eq(&b_std).map_err(|_| SynthesisError::Unsatisfiable)?;
    let result_option = result_std.to_gadget_unsafe();

    result_option.ok_or(SynthesisError::Unsatisfiable)
}
