//! Methods to enforce constraints on uint64s in a resolved Leo program.

use crate::{
    constraints::{new_variable_from_variable, ConstrainedProgram, ConstrainedValue},
    errors::IntegerError,
    types::{InputModel, Integer, Variable},
};

use snarkos_errors::gadgets::SynthesisError;
use snarkos_models::{
    curves::{Field, PrimeField},
    gadgets::{
        r1cs::ConstraintSystem,
        utilities::{alloc::AllocGadget, eq::EqGadget, uint64::UInt64},
    },
};

impl<F: Field + PrimeField, CS: ConstraintSystem<F>> ConstrainedProgram<F, CS> {
    pub(crate) fn u64_from_parameter(
        &mut self,
        cs: &mut CS,
        scope: String,
        parameter_model: InputModel<F>,
        integer_option: Option<usize>,
    ) -> Result<Variable<F>, IntegerError> {
        // Type cast to u64 in rust.
        // If this fails should we return our own error?
        let u64_option = integer_option.map(|integer| integer as u64);

        // Check visibility of parameter
        let name = parameter_model.variable.name.clone();
        let integer = if parameter_model.private {
            UInt64::alloc(cs.ns(|| name), || {
                u64_option.ok_or(SynthesisError::AssignmentMissing)
            })?
        } else {
            UInt64::alloc_input(cs.ns(|| name), || {
                u64_option.ok_or(SynthesisError::AssignmentMissing)
            })?
        };

        let parameter_variable = new_variable_from_variable(scope, &parameter_model.variable);

        // store each argument as variable in resolved program
        self.store_variable(
            parameter_variable.clone(),
            ConstrainedValue::Integer(Integer::U64(integer)),
        );

        Ok(parameter_variable)
    }

    // pub(crate) fn u64_array_from_parameter(
    //     &mut self,
    //     _cs: &mut CS,
    //     _scope: String,
    //     _parameter_model: ParameterModel<F>,
    //     _parameter_value: Option<ParameterValue<F>>,
    // ) -> Result<Variable<F>, IntegerError> {
    //     unimplemented!("Cannot enforce integer array as parameter")
    //     // // Check visibility of parameter
    //     // let mut array_value = vec![];
    //     // let name = parameter.variable.name.clone();
    //     // for argument in argument_array {
    //     //     let number = if parameter.private {
    //     //         UInt32::alloc(cs.ns(|| name), Some(argument)).unwrap()
    //     //     } else {
    //     //         UInt32::alloc_input(cs.ns(|| name), Some(argument)).unwrap()
    //     //     };
    //     //
    //     //     array_value.push(number);
    //     // }
    //     //
    //     //
    //     // let parameter_variable = new_variable_from_variable(scope, &parameter.variable);
    //     //
    //     // // store array as variable in resolved program
    //     // self.store_variable(parameter_variable.clone(), ResolvedValue::U32Array(array_value));
    //     //
    //     // parameter_variable
    // }

    pub(crate) fn enforce_u64_eq(
        cs: &mut CS,
        left: UInt64,
        right: UInt64,
    ) -> Result<(), IntegerError> {
        Ok(left.enforce_equal(cs.ns(|| format!("enforce u64 equal")), &right)?)
    }

    pub(crate) fn enforce_u64_add(
        cs: &mut CS,
        left: UInt64,
        right: UInt64,
    ) -> Result<UInt64, IntegerError> {
        Ok(UInt64::addmany(
            cs.ns(|| format!("enforce {} + {}", left.value.unwrap(), right.value.unwrap())),
            &[left, right],
        )?)
    }

    pub(crate) fn enforce_u64_sub(
        cs: &mut CS,
        left: UInt64,
        right: UInt64,
    ) -> Result<UInt64, IntegerError> {
        Ok(left.sub(
            cs.ns(|| format!("enforce {} - {}", left.value.unwrap(), right.value.unwrap())),
            &right,
        )?)
    }

    pub(crate) fn enforce_u64_mul(
        cs: &mut CS,
        left: UInt64,
        right: UInt64,
    ) -> Result<UInt64, IntegerError> {
        Ok(left.mul(
            cs.ns(|| format!("enforce {} * {}", left.value.unwrap(), right.value.unwrap())),
            &right,
        )?)
    }
    pub(crate) fn enforce_u64_div(
        cs: &mut CS,
        left: UInt64,
        right: UInt64,
    ) -> Result<UInt64, IntegerError> {
        Ok(left.div(
            cs.ns(|| format!("enforce {} / {}", left.value.unwrap(), right.value.unwrap())),
            &right,
        )?)
    }
    pub(crate) fn enforce_u64_pow(
        cs: &mut CS,
        left: UInt64,
        right: UInt64,
    ) -> Result<UInt64, IntegerError> {
        Ok(left.pow(
            cs.ns(|| {
                format!(
                    "enforce {} ** {}",
                    left.value.unwrap(),
                    right.value.unwrap()
                )
            }),
            &right,
        )?)
    }
}
