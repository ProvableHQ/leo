//! Methods to enforce constraints on booleans in a resolved aleo program.

use crate::constraints::{ResolvedProgram, ResolvedValue};
use crate::{new_variable_from_variable, Parameter, Variable};

use snarkos_models::curves::{Field, PrimeField};
use snarkos_models::gadgets::{
    r1cs::ConstraintSystem,
    utilities::{alloc::AllocGadget, boolean::Boolean, eq::EqGadget},
};

impl<F: Field + PrimeField, CS: ConstraintSystem<F>> ResolvedProgram<F, CS> {
    pub(crate) fn bool_from_parameter(
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
            .parse::<bool>()
            .expect(&format!(
                "expected main function parameter {} at index {}",
                parameter, index
            ));

        // Check visibility of parameter
        let name = parameter.variable.name.clone();
        let number = if parameter.private {
            Boolean::alloc(cs.ns(|| name), || Ok(argument)).unwrap()
        } else {
            Boolean::alloc_input(cs.ns(|| name), || Ok(argument)).unwrap()
        };

        let parameter_variable = new_variable_from_variable(scope, &parameter.variable);

        // store each argument as variable in resolved program
        self.store_variable(parameter_variable.clone(), ResolvedValue::Boolean(number));

        parameter_variable
    }

    pub(crate) fn boolean_array_from_parameter(
        &mut self,
        _cs: &mut CS,
        _scope: String,
        _index: usize,
        _parameter: Parameter<F>,
    ) -> Variable<F> {
        unimplemented!("Cannot enforce boolean array as parameter")
        // // Get command line argument for each parameter in program
        // let argument_array = std::env::args()
        //     .nth(index)
        //     .expect(&format!(
        //         "expected command line argument at index {}",
        //         index
        //     ))
        //     .parse::<Vec<bool>>()
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
        //         Boolean::alloc(cs.ns(|| name), || Ok(argument)).unwrap()
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

    pub(crate) fn get_boolean_constant(bool: bool) -> ResolvedValue<F> {
        ResolvedValue::Boolean(Boolean::Constant(bool))
    }

    pub(crate) fn enforce_not(value: ResolvedValue<F>) -> ResolvedValue<F> {
        match value {
            ResolvedValue::Boolean(boolean) => ResolvedValue::Boolean(boolean.not()),
            value => unimplemented!("cannot enforce not on non-boolean value {}", value),
        }
    }

    pub(crate) fn enforce_or(
        &mut self,
        cs: &mut CS,
        left: ResolvedValue<F>,
        right: ResolvedValue<F>,
    ) -> ResolvedValue<F> {
        match (left, right) {
            (ResolvedValue::Boolean(left_bool), ResolvedValue::Boolean(right_bool)) => {
                ResolvedValue::Boolean(Boolean::or(cs, &left_bool, &right_bool).unwrap())
            }
            (left_value, right_value) => unimplemented!(
                "cannot enforce or on non-boolean values {} || {}",
                left_value,
                right_value
            ),
        }
    }

    pub(crate) fn enforce_and(
        &mut self,
        cs: &mut CS,
        left: ResolvedValue<F>,
        right: ResolvedValue<F>,
    ) -> ResolvedValue<F> {
        match (left, right) {
            (ResolvedValue::Boolean(left_bool), ResolvedValue::Boolean(right_bool)) => {
                ResolvedValue::Boolean(Boolean::and(cs, &left_bool, &right_bool).unwrap())
            }
            (left_value, right_value) => unimplemented!(
                "cannot enforce and on non-boolean values {} && {}",
                left_value,
                right_value
            ),
        }
    }

    pub(crate) fn enforce_boolean_eq(
        &mut self,
        cs: &mut CS,
        left: Boolean,
        right: Boolean,
    ) -> ResolvedValue<F> {
        left.enforce_equal(cs.ns(|| format!("enforce bool equal")), &right)
            .unwrap();

        ResolvedValue::Boolean(Boolean::Constant(true))
    }
}
