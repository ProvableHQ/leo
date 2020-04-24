//! Methods to enforce constraints on integers in a resolved aleo program.
//!
//! @file integer.rs
//! @author Collin Chin <collin@aleo.org>
//! @date 2020

use crate::program::constraints::{ResolvedProgram, ResolvedValue};
use crate::program::{new_variable_from_variable, Integer, Parameter, Variable};

use snarkos_models::curves::{Field, PrimeField};
use snarkos_models::gadgets::{
    r1cs::ConstraintSystem,
    utilities::{boolean::Boolean, eq::ConditionalEqGadget, uint32::UInt32},
};

impl<F: Field + PrimeField, CS: ConstraintSystem<F>> ResolvedProgram<F, CS> {
    pub(crate) fn integer_from_parameter(
        &mut self,
        cs: &mut CS,
        scope: String,
        index: usize,
        parameter: Parameter<F>,
    ) -> Variable<F> {
        // Get command line argument for each parameter in program
        let argument = std::env::args()
            .nth(index)
            .expect(&format!(
                "expected command line argument at index {}",
                index
            ))
            .parse::<u32>()
            .expect(&format!(
                "expected main function parameter {} at index {}",
                parameter, index
            ));

        // Check visibility of parameter
        let name = parameter.variable.name.clone();
        let number = if parameter.private {
            UInt32::alloc(cs.ns(|| name), Some(argument)).unwrap()
        } else {
            UInt32::alloc_input(cs.ns(|| name), Some(argument)).unwrap()
        };

        let parameter_variable = new_variable_from_variable(scope, &parameter.variable);

        // store each argument as variable in resolved program
        self.store_variable(parameter_variable.clone(), ResolvedValue::U32(number));

        parameter_variable
    }

    pub(crate) fn integer_array_from_parameter(
        &mut self,
        _cs: &mut CS,
        _scope: String,
        _index: usize,
        _parameter: Parameter<F>,
    ) -> Variable<F> {
        unimplemented!("Cannot enforce integer array as parameter")
        // Get command line argument for each parameter in program
        // let argument_array = std::env::args()
        //     .nth(index)
        //     .expect(&format!(
        //         "expected command line argument at index {}",
        //         index
        //     ))
        //     .parse::<Vec<u32>>()
        //     .expect(&format!(
        //         "expected main function parameter {} at index {}",
        //         parameter, index
        //     ));
        //
        // // Check visibility of parameter
        // let mut array_value = vec![];
        // let name = parameter.variable.name.clone();
        // for argument in argument_array {
        //     let number = if parameter.private {
        //         UInt32::alloc(cs.ns(|| name), Some(argument)).unwrap()
        //     } else {
        //         UInt32::alloc_input(cs.ns(|| name), Some(argument)).unwrap()
        //     };
        //
        //     array_value.push(number);
        // }
        //
        //
        // let parameter_variable = new_variable_from_variable(scope, &parameter.variable);
        //
        // // store array as variable in resolved program
        // self.store_variable(parameter_variable.clone(), ResolvedValue::U32Array(array_value));
        //
        // parameter_variable
    }

    pub(crate) fn get_integer_constant(integer: Integer) -> ResolvedValue<F> {
        match integer {
            Integer::U32(u32_value) => ResolvedValue::U32(UInt32::constant(u32_value)),
        }
    }

    pub(crate) fn enforce_u32_eq(cs: &mut CS, left: UInt32, right: UInt32) -> ResolvedValue<F> {
        left.conditional_enforce_equal(
            cs.ns(|| format!("enforce field equal")),
            &right,
            &Boolean::Constant(true),
        )
        .unwrap();

        ResolvedValue::Boolean(Boolean::Constant(true))
    }

    pub(crate) fn enforce_u32_add(cs: &mut CS, left: UInt32, right: UInt32) -> ResolvedValue<F> {
        ResolvedValue::U32(
            UInt32::addmany(
                cs.ns(|| format!("enforce {} + {}", left.value.unwrap(), right.value.unwrap())),
                &[left, right],
            )
            .unwrap(),
        )
    }

    pub(crate) fn enforce_u32_sub(cs: &mut CS, left: UInt32, right: UInt32) -> ResolvedValue<F> {
        ResolvedValue::U32(
            left.sub(
                cs.ns(|| format!("enforce {} - {}", left.value.unwrap(), right.value.unwrap())),
                &right,
            )
            .unwrap(),
        )
    }

    pub(crate) fn enforce_u32_mul(cs: &mut CS, left: UInt32, right: UInt32) -> ResolvedValue<F> {
        ResolvedValue::U32(
            left.mul(
                cs.ns(|| format!("enforce {} * {}", left.value.unwrap(), right.value.unwrap())),
                &right,
            )
            .unwrap(),
        )
    }
    pub(crate) fn enforce_u32_div(cs: &mut CS, left: UInt32, right: UInt32) -> ResolvedValue<F> {
        ResolvedValue::U32(
            left.div(
                cs.ns(|| format!("enforce {} / {}", left.value.unwrap(), right.value.unwrap())),
                &right,
            )
            .unwrap(),
        )
    }
    pub(crate) fn enforce_u32_pow(cs: &mut CS, left: UInt32, right: UInt32) -> ResolvedValue<F> {
        ResolvedValue::U32(
            left.pow(
                cs.ns(|| {
                    format!(
                        "enforce {} ** {}",
                        left.value.unwrap(),
                        right.value.unwrap()
                    )
                }),
                &right,
            )
            .unwrap(),
        )
    }
}
