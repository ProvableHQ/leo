//! Methods to enforce constraints on field elements in a resolved aleo program.
//!
//! @file field_element.rs
//! @author Collin Chin <collin@aleo.org>
//! @date 2020

use crate::program::constraints::{ResolvedProgram, ResolvedValue};
use crate::program::{new_variable_from_variable, Parameter, Variable};

use snarkos_models::curves::{Field, PrimeField};
use snarkos_models::gadgets::{r1cs::ConstraintSystem, utilities::boolean::Boolean};
// use std::ops::{Add, Div, Mul, Neg, Sub};

impl<F: Field + PrimeField, CS: ConstraintSystem<F>> ResolvedProgram<F, CS> {
    pub(crate) fn field_element_from_parameter(
        &mut self,
        cs: &mut CS,
        scope: String,
        index: usize,
        parameter: Parameter<F>,
    ) -> Variable<F> {
        // Get command line argument for each parameter in program
        let argument: F = std::env::args()
            .nth(index)
            .expect(&format!(
                "expected command line argument at index {}",
                index
            ))
            .parse::<F>()
            .unwrap_or_default();

        // Check visibility of parameter
        let name = parameter.variable.name.clone();
        if parameter.private {
            cs.alloc(|| name, || Ok(argument.clone())).unwrap();
        } else {
            cs.alloc_input(|| name, || Ok(argument.clone())).unwrap();
        }

        let parameter_variable = new_variable_from_variable(scope, &parameter.variable);

        // store each argument as variable in resolved program
        self.store_variable(
            parameter_variable.clone(),
            ResolvedValue::FieldElement(argument),
        );

        parameter_variable
    }

    pub(crate) fn field_element_array_from_parameter(
        &mut self,
        _cs: &mut CS,
        _scope: String,
        _index: usize,
        _parameter: Parameter<F>,
    ) -> Variable<F> {
        unimplemented!("Cannot enforce field element array as parameter")

        // // Get command line argument for each parameter in program
        // let argument_array = std::env::args()
        //     .nth(index)
        //     .expect(&format!(
        //         "expected command line argument at index {}",
        //         index
        //     ))
        //     .parse::<Vec<F>>()
        //     .expect(&format!(
        //         "expected main function parameter {} at index {}",
        //         parameter, index
        //     ));
        //
        // // Check visibility of parameter
        // let mut array_value = vec![];
        // let name = parameter.variable.name.clone();
        // for argument in argument_array {
        //     if parameter.private {
        //         cs.alloc(|| name, || Ok(argument.clone())).unwrap();
        //     } else {
        //         cs.alloc_input(|| name, || Ok(argument.clone())).unwrap();
        //     };
        // }
        //
        //
        // let parameter_variable = new_variable_from_variable(scope, &parameter.variable);
        //
        // // store array as variable in resolved program
        // self.store_variable(parameter_variable.clone(), ResolvedValue::FieldElementArray(argument_array));
        //
        // parameter_variable
    }

    pub(crate) fn enforce_field_eq(&mut self, fe1: F, fe2: F) -> ResolvedValue<F> {
        ResolvedValue::Boolean(Boolean::Constant(fe1.eq(&fe2)))
    }

    pub(crate) fn enforce_field_add(&mut self, fe1: F, fe2: F) -> ResolvedValue<F> {
        ResolvedValue::FieldElement(fe1.add(&fe2))
    }

    pub(crate) fn enforce_field_sub(&mut self, fe1: F, fe2: F) -> ResolvedValue<F> {
        ResolvedValue::FieldElement(fe1.sub(&fe2))
    }

    pub(crate) fn enforce_field_mul(&mut self, fe1: F, fe2: F) -> ResolvedValue<F> {
        ResolvedValue::FieldElement(fe1.mul(&fe2))
    }

    pub(crate) fn enforce_field_div(&mut self, fe1: F, fe2: F) -> ResolvedValue<F> {
        ResolvedValue::FieldElement(fe1.div(&fe2))
    }

    pub(crate) fn enforce_field_pow(&mut self, _fe1: F, _fe2: F) -> ResolvedValue<F> {
        unimplemented!("field element exponentiation not supported")

        // ResolvedValue::FieldElement(fe1.pow(&fe2))
    }
}
