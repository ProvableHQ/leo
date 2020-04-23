//! Methods to enforce constraints on booleans in a resolved aleo program.
//!
//! @file boolean.rs
//! @author Collin Chin <collin@aleo.org>
//! @date 2020

use crate::program::constraints::{ResolvedProgram, ResolvedValue};
use crate::program::{new_variable_from_variable, Parameter, Variable};

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

    // pub(crate) fn bool_from_variable(&mut self, scope: String, variable: Variable<F>) -> Boolean {
    //     // Evaluate variable name in current function scope
    //     let variable_name = new_scope_from_variable(scope, &variable);
    //
    //     match self.get(&variable_name) {
    //         Some(value) => match value {
    //             ResolvedValue::Boolean(boolean) => boolean.clone(),
    //             value => unimplemented!(
    //                 "expected boolean for variable {}, got {}",
    //                 variable_name,
    //                 value
    //             ),
    //         },
    //         None => unimplemented!("cannot resolve variable {} in program", variable_name),
    //     }
    // }

    // fn get_bool_value(
    //     &mut self,
    //     cs: &mut CS,
    //     scope: String,
    //     expression: BooleanExpression<F>,
    // ) -> Boolean {
    //     match expression {
    //         BooleanExpression::Variable(variable) => self.bool_from_variable(scope, variable),
    //         BooleanExpression::Value(value) => Boolean::Constant(value),
    //         expression => match self.enforce_boolean_expression(cs, scope, expression) {
    //             ResolvedValue::Boolean(value) => value,
    //             _ => unimplemented!("boolean expression did not resolve to boolean"),
    //         },
    //     }
    // }

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
    //
    // pub(crate) fn enforce_boolean_expression(
    //     &mut self,
    //     cs: &mut CS,
    //     scope: String,
    //     expression: BooleanExpression<F>,
    // ) -> ResolvedValue<F> {
    //     match expression {
    //         BooleanExpression::Variable(variable) => {
    //             ResolvedValue::Boolean(self.bool_from_variable(cs, scope, variable))
    //         }
    //         BooleanExpression::Value(value) => ResolvedValue::Boolean(Boolean::Constant(value)),
    //         BooleanExpression::Not(expression) => {
    //             ResolvedValue::Boolean(self.enforce_not(cs, scope, *expression))
    //         }
    //         BooleanExpression::Or(left, right) => {
    //             ResolvedValue::Boolean(self.enforce_or(cs, scope, *left, *right))
    //         }
    //         BooleanExpression::And(left, right) => {
    //             ResolvedValue::Boolean(self.enforce_and(cs, scope, *left, *right))
    //         }
    //         BooleanExpression::IntegerEq(left, right) => {
    //             ResolvedValue::Boolean(self.enforce_integer_equality(cs, scope, *left, *right))
    //         }
    //         BooleanExpression::FieldEq(left, right) => {
    //             ResolvedValue::Boolean(self.enforce_field_equality(cs, scope, *left, *right))
    //         }
    //         BooleanExpression::BoolEq(left, right) => {
    //             ResolvedValue::Boolean(self.enforce_bool_equality(cs, scope, *left, *right))
    //         }
    //         BooleanExpression::IfElse(first, second, third) => {
    //             let resolved_first =
    //                 match self.enforce_boolean_expression(cs, scope.clone(), *first) {
    //                     ResolvedValue::Boolean(resolved) => resolved,
    //                     _ => unimplemented!("if else conditional must resolve to boolean"),
    //                 };
    //             if resolved_first.eq(&Boolean::Constant(true)) {
    //                 self.enforce_boolean_expression(cs, scope, *second)
    //             } else {
    //                 self.enforce_boolean_expression(cs, scope, *third)
    //             }
    //         }
    //         BooleanExpression::Array(array) => {
    //             let mut result = vec![];
    //             array.into_iter().for_each(|element| match *element {
    //                 BooleanSpreadOrExpression::Spread(spread) => match spread {
    //                     BooleanExpression::Variable(variable) => {
    //                         let array_name = new_scope_from_variable(scope.clone(), &variable);
    //                         match self.get(&array_name) {
    //                             Some(value) => match value {
    //                                 ResolvedValue::BooleanArray(array) => {
    //                                     result.extend(array.clone())
    //                                 }
    //                                 value => unimplemented!(
    //                                     "spreads only implemented for arrays, got {}",
    //                                     value
    //                                 ),
    //                             },
    //                             None => unimplemented!(
    //                                 "cannot copy elements from array that does not exist {}",
    //                                 variable.name
    //                             ),
    //                         }
    //                     }
    //                     value => {
    //                         unimplemented!("spreads only implemented for arrays, got {}", value)
    //                     }
    //                 },
    //                 BooleanSpreadOrExpression::Expression(expression) => {
    //                     match self.enforce_boolean_expression(cs, scope.clone(), expression) {
    //                         ResolvedValue::Boolean(value) => result.push(value),
    //                         value => {
    //                             unimplemented!("expected boolean for boolean array, got {}", value)
    //                         }
    //                     }
    //                 }
    //             });
    //             ResolvedValue::BooleanArray(result)
    //         }
    //         expression => unimplemented!("boolean expression {}", expression),
    //     }
    // }
}
