use crate::errors::IntegerError;
use snarkos_models::{
    curves::PrimeField,
    gadgets::{
        r1cs::ConstraintSystem,
        utilities::boolean::{AllocatedBit, Boolean},
    },
};

use std::{cmp::Ordering, fmt::Debug};

int_impl!(Int8, i8, 8);
int_impl!(Int16, i16, 16);
int_impl!(Int32, i32, 32);
int_impl!(Int64, i64, 64);
int_impl!(Int128, i128, 128);

/// A signed two's complement integer object
pub trait Int: Debug + Clone + PartialOrd + Eq + PartialEq {
    /// Returns true if all bits in this `Int` are constant
    fn is_constant(&self) -> bool;

    /// Returns true if both `Int` objects have constant bits
    fn result_is_constant(first: &Self, second: &Self) -> bool {
        first.is_constant() && second.is_constant()
    }

    /// Add two `Int` objects
    fn add<F: PrimeField, CS: ConstraintSystem<F>>(&self, cs: CS, other: &Self) -> Result<Self, IntegerError>;

    /// Subtract two `Int` objects
    fn sub<F: PrimeField, CS: ConstraintSystem<F>>(&self, cs: CS, other: &Self) -> Result<Self, IntegerError>;

    /// Multiply two `Int` objects
    fn mul<F: PrimeField, CS: ConstraintSystem<F>>(&self, cs: CS, other: &Self) -> Result<Self, IntegerError>;

    /// Divide two `Int` objects
    fn div<F: PrimeField, CS: ConstraintSystem<F>>(&self, cs: CS, other: &Self) -> Result<Self, IntegerError>;

    /// Exponentiation between two `Int` objects
    fn pow<F: PrimeField, CS: ConstraintSystem<F>>(&self, cs: CS, other: &Self) -> Result<Self, IntegerError>;
}
