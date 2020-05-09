//! Methods to enforce constraints on uint32s in a resolved Leo program.

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
        utilities::{alloc::AllocGadget, eq::EqGadget, uint32::UInt32},
    },
};

impl<F: Field + PrimeField, CS: ConstraintSystem<F>> ConstrainedProgram<F, CS> {
    pub(crate) fn u32_from_parameter(
        &mut self,
        cs: &mut CS,
        scope: String,
        parameter_model: InputModel<F>,
        integer_option: Option<usize>,
    ) -> Result<Variable<F>, IntegerError> {
        // Type cast to u32 in rust.
        // If this fails should we return our own error?
        let u32_option = integer_option.map(|integer| integer as u32);

        // Check visibility of parameter
        let name = parameter_model.variable.name.clone();
        let integer = if parameter_model.private {
            UInt32::alloc(cs.ns(|| name), || {
                u32_option.ok_or(SynthesisError::AssignmentMissing)
            })?
        } else {
            UInt32::alloc_input(cs.ns(|| name), || {
                u32_option.ok_or(SynthesisError::AssignmentMissing)
            })?
        };

        let parameter_variable = new_variable_from_variable(scope, &parameter_model.variable);

        // store each argument as variable in resolved program
        self.store_variable(
            parameter_variable.clone(),
            ConstrainedValue::Integer(Integer::U32(integer)),
        );

        Ok(parameter_variable)
    }

    // pub(crate) fn u32_array_from_parameter(
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

    pub(crate) fn enforce_u32_eq(
        cs: &mut CS,
        left: UInt32,
        right: UInt32,
    ) -> Result<(), IntegerError> {
        Ok(left.enforce_equal(cs.ns(|| format!("enforce u32 equal")), &right)?)
    }

    pub(crate) fn enforce_u32_add(
        cs: &mut CS,
        left: UInt32,
        right: UInt32,
    ) -> Result<UInt32, IntegerError> {
        Ok(UInt32::addmany(
            cs.ns(|| format!("enforce {} + {}", left.value.unwrap(), right.value.unwrap())),
            &[left, right],
        )?)
    }

    pub(crate) fn enforce_u32_sub(
        cs: &mut CS,
        left: UInt32,
        right: UInt32,
    ) -> Result<UInt32, IntegerError> {
        Ok(left.sub(
            cs.ns(|| format!("enforce {} - {}", left.value.unwrap(), right.value.unwrap())),
            &right,
        )?)
    }

    pub(crate) fn enforce_u32_mul(
        cs: &mut CS,
        left: UInt32,
        right: UInt32,
    ) -> Result<UInt32, IntegerError> {
        Ok(left.mul(
            cs.ns(|| format!("enforce {} * {}", left.value.unwrap(), right.value.unwrap())),
            &right,
        )?)
    }
    pub(crate) fn enforce_u32_div(
        cs: &mut CS,
        left: UInt32,
        right: UInt32,
    ) -> Result<UInt32, IntegerError> {
        Ok(left.div(
            cs.ns(|| format!("enforce {} / {}", left.value.unwrap(), right.value.unwrap())),
            &right,
        )?)
    }
    pub(crate) fn enforce_u32_pow(
        cs: &mut CS,
        left: UInt32,
        right: UInt32,
    ) -> Result<UInt32, IntegerError> {
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
