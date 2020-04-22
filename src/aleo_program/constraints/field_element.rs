//! Methods to enforce constraints on field elements in a resolved aleo program.
//!
//! @file field_element.rs
//! @author Collin Chin <collin@aleo.org>
//! @date 2020

use crate::aleo_program::constraints::{new_scope_from_variable, ResolvedProgram, ResolvedValue};
use crate::aleo_program::{
    new_variable_from_variable, FieldExpression, FieldSpreadOrExpression, Parameter, Variable,
};

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

    fn field_element_from_variable(&mut self, scope: String, variable: Variable<F>) -> F {
        // Evaluate variable name in current function scope
        let variable_name = new_scope_from_variable(scope, &variable);

        if self.contains_name(&variable_name) {
            // TODO: return synthesis error: "assignment missing" here
            match self.get(&variable_name).unwrap().clone() {
                ResolvedValue::FieldElement(fe) => fe,
                value => unimplemented!(
                    "expected field element for variable {}, got {}",
                    variable_name,
                    value
                ),
            }
        } else {
            unimplemented!("cannot resolve variable {} in program", variable_name)
        }
    }

    fn get_field_value(&mut self, cs: &mut CS, scope: String, expression: FieldExpression<F>) -> F {
        match expression {
            FieldExpression::Variable(variable) => {
                self.field_element_from_variable(scope, variable)
            }
            FieldExpression::Number(element) => element,
            expression => match self.enforce_field_expression(cs, scope, expression) {
                ResolvedValue::FieldElement(element) => element,
                value => unimplemented!("expected field element, got {}", value),
            },
        }
    }

    pub(crate) fn enforce_field_equality(
        &mut self,
        cs: &mut CS,
        scope: String,
        left: FieldExpression<F>,
        right: FieldExpression<F>,
    ) -> Boolean {
        let left = self.get_field_value(cs, scope.clone(), left);
        let right = self.get_field_value(cs, scope.clone(), right);

        Boolean::Constant(left.eq(&right))
    }

    fn enforce_field_add(
        &mut self,
        cs: &mut CS,
        scope: String,
        left: FieldExpression<F>,
        right: FieldExpression<F>,
    ) -> ResolvedValue<F> {
        let left = self.get_field_value(cs, scope.clone(), left);
        let right = self.get_field_value(cs, scope.clone(), right);

        ResolvedValue::FieldElement(left.add(&right))
    }

    fn enforce_field_sub(
        &mut self,
        cs: &mut CS,
        scope: String,
        left: FieldExpression<F>,
        right: FieldExpression<F>,
    ) -> ResolvedValue<F> {
        let left = self.get_field_value(cs, scope.clone(), left);
        let right = self.get_field_value(cs, scope.clone(), right);

        ResolvedValue::FieldElement(left.sub(&right))
    }

    fn enforce_field_mul(
        &mut self,
        cs: &mut CS,
        scope: String,
        left: FieldExpression<F>,
        right: FieldExpression<F>,
    ) -> ResolvedValue<F> {
        let left = self.get_field_value(cs, scope.clone(), left);
        let right = self.get_field_value(cs, scope.clone(), right);

        ResolvedValue::FieldElement(left.mul(&right))
    }

    fn enforce_field_div(
        &mut self,
        cs: &mut CS,
        scope: String,
        left: FieldExpression<F>,
        right: FieldExpression<F>,
    ) -> ResolvedValue<F> {
        let left = self.get_field_value(cs, scope.clone(), left);
        let right = self.get_field_value(cs, scope.clone(), right);

        ResolvedValue::FieldElement(left.div(&right))
    }

    fn enforce_field_pow(
        &mut self,
        _cs: &mut CS,
        _scope: String,
        _left: FieldExpression<F>,
        _right: FieldExpression<F>,
    ) -> ResolvedValue<F> {
        unimplemented!("field element exponentiation not supported")
        // let left = self.get_field_value(cs, scope.clone(), left);
        // let right = self.get_field_value(cs, scope.clone(), right);
        //
        // ResolvedValue::FieldElement(left.pow(&right))
    }

    pub(crate) fn enforce_field_expression(
        &mut self,
        cs: &mut CS,
        scope: String,
        expression: FieldExpression<F>,
    ) -> ResolvedValue<F> {
        match expression {
            FieldExpression::Variable(variable) => {
                ResolvedValue::FieldElement(self.field_element_from_variable(scope, variable))
            }
            FieldExpression::Number(field) => ResolvedValue::FieldElement(field),
            FieldExpression::Add(left, right) => self.enforce_field_add(cs, scope, *left, *right),
            FieldExpression::Sub(left, right) => self.enforce_field_sub(cs, scope, *left, *right),
            FieldExpression::Mul(left, right) => self.enforce_field_mul(cs, scope, *left, *right),
            FieldExpression::Div(left, right) => self.enforce_field_div(cs, scope, *left, *right),
            FieldExpression::Pow(left, right) => self.enforce_field_pow(cs, scope, *left, *right),
            FieldExpression::IfElse(first, second, third) => {
                let resolved_first =
                    match self.enforce_boolean_expression(cs, scope.clone(), *first) {
                        ResolvedValue::Boolean(resolved) => resolved,
                        _ => unimplemented!("if else conditional must resolve to boolean"),
                    };

                if resolved_first.eq(&Boolean::Constant(true)) {
                    self.enforce_field_expression(cs, scope, *second)
                } else {
                    self.enforce_field_expression(cs, scope, *third)
                }
            }
            FieldExpression::Array(array) => {
                let mut result = vec![];
                array.into_iter().for_each(|element| match *element {
                    FieldSpreadOrExpression::Spread(spread) => match spread {
                        FieldExpression::Variable(variable) => {
                            let array_name = new_scope_from_variable(scope.clone(), &variable);
                            match self.get(&array_name) {
                                Some(value) => match value {
                                    ResolvedValue::FieldElementArray(array) => {
                                        result.extend(array.clone())
                                    }
                                    value => unimplemented!(
                                        "spreads only implemented for arrays, got {}",
                                        value
                                    ),
                                },
                                None => unimplemented!(
                                    "cannot copy elements from array that does not exist {}",
                                    variable.name
                                ),
                            }
                        }
                        value => {
                            unimplemented!("spreads only implemented for arrays, got {}", value)
                        }
                    },
                    FieldSpreadOrExpression::Expression(expression) => {
                        match self.enforce_field_expression(cs, scope.clone(), expression) {
                            ResolvedValue::FieldElement(value) => result.push(value),
                            _ => unimplemented!("cannot resolve field"),
                        }
                    }
                });
                ResolvedValue::FieldElementArray(result)
            }
        }
    }
}
