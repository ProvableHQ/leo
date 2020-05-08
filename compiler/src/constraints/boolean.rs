//! Methods to enforce constraints on booleans in a resolved Leo program.

use crate::{
    constraints::{new_variable_from_variable, ConstrainedProgram, ConstrainedValue},
    errors::BooleanError,
    types::{InputModel, InputValue, Variable},
};

use snarkos_errors::gadgets::SynthesisError;
use snarkos_models::{
    curves::{Field, PrimeField},
    gadgets::{
        r1cs::ConstraintSystem,
        utilities::{alloc::AllocGadget, boolean::Boolean, eq::EqGadget},
    },
};

impl<F: Field + PrimeField, CS: ConstraintSystem<F>> ConstrainedProgram<F, CS> {
    pub(crate) fn bool_from_parameter(
        &mut self,
        cs: &mut CS,
        scope: String,
        parameter_model: InputModel<F>,
        parameter_value: Option<InputValue<F>>,
    ) -> Result<Variable<F>, BooleanError> {
        // Check that the parameter value is the correct type
        let bool_value = match parameter_value {
            Some(parameter) => {
                if let InputValue::Boolean(bool) = parameter {
                    Some(bool)
                } else {
                    return Err(BooleanError::InvalidBoolean(parameter.to_string()));
                }
            }
            None => None,
        };

        // Check visibility of parameter
        let name = parameter_model.variable.name.clone();
        let number = if parameter_model.private {
            Boolean::alloc(cs.ns(|| name), || {
                bool_value.ok_or(SynthesisError::AssignmentMissing)
            })?
        } else {
            Boolean::alloc_input(cs.ns(|| name), || {
                bool_value.ok_or(SynthesisError::AssignmentMissing)
            })?
        };

        let parameter_variable = new_variable_from_variable(scope, &parameter_model.variable);

        // store each argument as variable in resolved program
        self.store_variable(
            parameter_variable.clone(),
            ConstrainedValue::Boolean(number),
        );

        Ok(parameter_variable)
    }

    pub(crate) fn boolean_array_from_parameter(
        &mut self,
        _cs: &mut CS,
        _scope: String,
        _parameter_model: InputModel<F>,
        _parameter_value: Option<InputValue<F>>,
    ) -> Result<Variable<F>, BooleanError> {
        unimplemented!("Cannot enforce boolean array as parameter")
        // // Check visibility of parameter
        // let mut array_value = vec![];
        // let name = parameter.variable.name.clone();
        // for argument in argument_array {
        //     let number = if parameter.private {
        //         Boolean::alloc(cs.ns(|| name), ||bool_value.ok_or(SynthesisError::AssignmentMissing).unwrap()
        //     } else {
        //         Boolean::alloc_input(cs.ns(|| name), || Ok(argument)).unwrap()
        //     };
        //
        //     array_value.push(number);
        // }
        //
        //
        // let parameter_variable = new_variable_from_variable(scope, &parameter.variable);
        //
        // // store array as variable in resolved program
        // self.store_variable(parameter_variable.clone(), ResolvedValue::BooleanArray(array_value));
        //
        // parameter_variable
    }

    pub(crate) fn get_boolean_constant(bool: Boolean) -> ConstrainedValue<F> {
        ConstrainedValue::Boolean(bool)
    }

    pub(crate) fn evaluate_not(
        value: ConstrainedValue<F>,
    ) -> Result<ConstrainedValue<F>, BooleanError> {
        match value {
            ConstrainedValue::Boolean(boolean) => Ok(ConstrainedValue::Boolean(boolean.not())),
            value => Err(BooleanError::CannotEvaluate(format!("!{}", value))),
        }
    }

    pub(crate) fn enforce_or(
        &mut self,
        cs: &mut CS,
        left: ConstrainedValue<F>,
        right: ConstrainedValue<F>,
    ) -> Result<ConstrainedValue<F>, BooleanError> {
        match (left, right) {
            (ConstrainedValue::Boolean(left_bool), ConstrainedValue::Boolean(right_bool)) => Ok(
                ConstrainedValue::Boolean(Boolean::or(cs, &left_bool, &right_bool)?),
            ),
            (left_value, right_value) => Err(BooleanError::CannotEnforce(format!(
                "{} || {}",
                left_value, right_value
            ))),
        }
    }

    pub(crate) fn enforce_and(
        &mut self,
        cs: &mut CS,
        left: ConstrainedValue<F>,
        right: ConstrainedValue<F>,
    ) -> Result<ConstrainedValue<F>, BooleanError> {
        match (left, right) {
            (ConstrainedValue::Boolean(left_bool), ConstrainedValue::Boolean(right_bool)) => Ok(
                ConstrainedValue::Boolean(Boolean::and(cs, &left_bool, &right_bool)?),
            ),
            (left_value, right_value) => Err(BooleanError::CannotEnforce(format!(
                "{} && {}",
                left_value, right_value
            ))),
        }
    }

    pub(crate) fn boolean_eq(left: Boolean, right: Boolean) -> ConstrainedValue<F> {
        ConstrainedValue::Boolean(Boolean::Constant(left.eq(&right)))
    }

    pub(crate) fn enforce_boolean_eq(
        &mut self,
        cs: &mut CS,
        left: Boolean,
        right: Boolean,
    ) -> Result<(), BooleanError> {
        Ok(left.enforce_equal(cs.ns(|| format!("enforce bool equal")), &right)?)
    }
}
