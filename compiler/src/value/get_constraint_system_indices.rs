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

use crate::{Address, FieldType, GroupType, Integer};

use snarkos_models::{
    curves::{Field, PrimeField},
    gadgets::{
        r1cs::{ConstraintSystem, Index},
        utilities::{boolean::Boolean, ToBitsGadget},
    },
};

///
/// Return the constraint system indices that this boolean is located at.
///
pub fn get_constraint_system_indices_boolean(boolean: &Boolean) -> Vec<Index> {
    match boolean {
        Boolean::Constant(_) => vec![],
        Boolean::Is(allocated_bit) => vec![allocated_bit.get_variable().get_unchecked()],
        Boolean::Not(allocated_bit) => vec![allocated_bit.get_variable().get_unchecked()],
    }
}

///
/// Return the constraint system indices for a vector of boolean bits.
///
pub fn get_constraint_system_indices_bits(bits: Vec<Boolean>) -> Vec<Index> {
    // Lookup the index of each boolean in the constraint system.
    let mut indices = vec![];

    for boolean in bits {
        let mut boolean_indices = get_constraint_system_indices_boolean(&boolean);

        // Append the index of the boolean to the list of indices.
        indices.append(&mut boolean_indices);
    }

    indices
}

///
/// Return the constraint system indices that this integer is located at.
///
pub fn get_constraint_system_indices_integer(integer: &Integer) -> Vec<Index> {
    // Represent this integer as a vector of booleans.
    let bits = integer.get_bits();

    get_constraint_system_indices_bits(bits)
}

///
/// Return the constraint system indices that this field is located at.
///
pub fn get_constraint_system_indices_field<F: Field + PrimeField, CS: ConstraintSystem<F>>(
    cs: CS,
    field: &FieldType<F>,
) -> Vec<Index> {
    // Represent this field as a vector of booleans.
    let bits = field.to_bits(cs).unwrap();

    get_constraint_system_indices_bits(bits)
}

///
/// Return the constraint system indices that this group is located at.
///
pub fn get_constraint_system_indices_group<F: Field + PrimeField, G: GroupType<F>, CS: ConstraintSystem<F>>(
    cs: CS,
    group: &G,
) -> Vec<Index> {
    let bits = group.to_bits(cs).unwrap();

    get_constraint_system_indices_bits(bits)
}

///
/// Return the constraint system indices that this address is located at.
///
pub fn get_constraint_system_indices_address<F: Field + PrimeField, CS: ConstraintSystem<F>>(
    cs: CS,
    address: &Address,
) -> Vec<Index> {
    let bits = address.bytes.to_bits(cs).unwrap();

    get_constraint_system_indices_bits(bits)
}
