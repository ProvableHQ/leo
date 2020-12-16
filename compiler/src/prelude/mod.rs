pub mod blake2s;
use std::sync::Arc;

pub use blake2s::*;

use crate::{errors::ExpressionError, ConstrainedValue, GroupType};
use leo_asg::{FunctionBody, Span};
use snarkvm_models::{
    curves::{Field, PrimeField},
    gadgets::r1cs::ConstraintSystem,
};

pub trait CoreCircuit<F: Field + PrimeField, G: GroupType<F>>: Send + Sync {
    fn call_function<CS: ConstraintSystem<F>>(
        &self,
        cs: &mut CS,
        function: Arc<FunctionBody>,
        span: &Span,
        target: Option<ConstrainedValue<F, G>>,
        arguments: Vec<ConstrainedValue<F, G>>,
    ) -> Result<ConstrainedValue<F, G>, ExpressionError>;
}

pub fn resolve_core_circuit<F: Field + PrimeField, G: GroupType<F>>(name: &str) -> impl CoreCircuit<F, G> {
    match name {
        "blake2s" => Blake2s,
        _ => unimplemented!("invalid core circuit: {}", name),
    }
}
