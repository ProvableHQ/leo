use crate::errors::AddressError;
use leo_types::Span;

use snarkos_errors::gadgets::SynthesisError;
use snarkos_models::{
    curves::{Field, PrimeField},
    gadgets::r1cs::ConstraintSystem,
};

#[derive(Clone, Debug)]
pub struct Address(pub String);

impl Address {
    pub(crate) fn constant(address: String, span: Span) -> Result<Self, AddressError> {
        Ok(Self(address))
    }

    // pub(crate) fn allocate<F: Field + PrimeField, CS: ConstraintSystem<F>>(
    //     cs: &mut CS,
    //     name: String,
    //     option: Option<bool>,
    //     span: Span,
    // ) -> Result<Self, AddressError> {
    //     let boolean_name = format!("{}: bool", name);
    //     let boolean_name_unique = format!("`{}` {}:{}", boolean_name, span.line, span.start);
    //
    //     Self::alloc(cs.ns(|| boolean_name_unique), || {
    //         option.ok_or(SynthesisError::AssignmentMissing)
    //     })
    //         .map_err(|_| AddressError::missing_boolean(boolean_name, span))
    // }
}
