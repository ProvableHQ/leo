//! Methods to enforce constraints on booleans in a resolved aleo program.
//!
//! @file boolean.rs
//! @author Collin Chin <collin@aleo.org>
//! @date 2020

use crate::aleo_program::constraints::{new_scope_from_variable, ResolvedProgram, ResolvedValue};
use crate::aleo_program::{BooleanExpression, BooleanSpreadOrExpression, Variable};

use snarkos_models::curves::{Field, PrimeField};
use snarkos_models::gadgets::{
    r1cs::ConstraintSystem,
    utilities::{alloc::AllocGadget, boolean::Boolean, eq::EqGadget},
};

impl<F: Field + PrimeField, CS: ConstraintSystem<F>> ResolvedProgram<F, CS> {
    pub(crate) fn bool_from_variable(
        &mut self,
        cs: &mut CS,
        scope: String,
        variable: Variable<F>,
    ) -> Boolean {
        // Evaluate variable name in current function scope
        let variable_name = new_scope_from_variable(scope, &variable);

        if self.contains_name(&variable_name) {
            // TODO: return synthesis error: "assignment missing" here
            match self.get(&variable_name).unwrap() {
                ResolvedValue::Boolean(boolean) => boolean.clone(),
                _ => panic!("expected a boolean, got field"),
            }
        } else {
            let argument = std::env::args()
                .nth(1)
                .unwrap_or("true".into())
                .parse::<bool>()
                .unwrap();
            println!(" argument passed to command line a = {:?}\n", argument);
            // let a = true;
            Boolean::alloc(cs.ns(|| variable.name), || Ok(argument)).unwrap()
        }
    }

    fn get_bool_value(
        &mut self,
        cs: &mut CS,
        scope: String,
        expression: BooleanExpression<F>,
    ) -> Boolean {
        match expression {
            BooleanExpression::Variable(variable) => self.bool_from_variable(cs, scope, variable),
            BooleanExpression::Value(value) => Boolean::Constant(value),
            expression => match self.enforce_boolean_expression(cs, scope, expression) {
                ResolvedValue::Boolean(value) => value,
                _ => unimplemented!("boolean expression did not resolve to boolean"),
            },
        }
    }

    fn enforce_not(
        &mut self,
        cs: &mut CS,
        scope: String,
        expression: BooleanExpression<F>,
    ) -> Boolean {
        let expression = self.get_bool_value(cs, scope, expression);

        expression.not()
    }

    fn enforce_or(
        &mut self,
        cs: &mut CS,
        scope: String,
        left: BooleanExpression<F>,
        right: BooleanExpression<F>,
    ) -> Boolean {
        let left = self.get_bool_value(cs, scope.clone(), left);
        let right = self.get_bool_value(cs, scope.clone(), right);

        Boolean::or(cs, &left, &right).unwrap()
    }

    fn enforce_and(
        &mut self,
        cs: &mut CS,
        scope: String,
        left: BooleanExpression<F>,
        right: BooleanExpression<F>,
    ) -> Boolean {
        let left = self.get_bool_value(cs, scope.clone(), left);
        let right = self.get_bool_value(cs, scope.clone(), right);

        Boolean::and(cs, &left, &right).unwrap()
    }

    fn enforce_bool_equality(
        &mut self,
        cs: &mut CS,
        scope: String,
        left: BooleanExpression<F>,
        right: BooleanExpression<F>,
    ) -> Boolean {
        let left = self.get_bool_value(cs, scope.clone(), left);
        let right = self.get_bool_value(cs, scope.clone(), right);

        left.enforce_equal(cs.ns(|| format!("enforce bool equal")), &right)
            .unwrap();

        Boolean::Constant(true)
    }

    pub(crate) fn enforce_boolean_expression(
        &mut self,
        cs: &mut CS,
        scope: String,
        expression: BooleanExpression<F>,
    ) -> ResolvedValue<F> {
        match expression {
            BooleanExpression::Variable(variable) => {
                ResolvedValue::Boolean(self.bool_from_variable(cs, scope, variable))
            }
            BooleanExpression::Value(value) => ResolvedValue::Boolean(Boolean::Constant(value)),
            BooleanExpression::Not(expression) => {
                ResolvedValue::Boolean(self.enforce_not(cs, scope, *expression))
            }
            BooleanExpression::Or(left, right) => {
                ResolvedValue::Boolean(self.enforce_or(cs, scope, *left, *right))
            }
            BooleanExpression::And(left, right) => {
                ResolvedValue::Boolean(self.enforce_and(cs, scope, *left, *right))
            }
            BooleanExpression::IntegerEq(left, right) => {
                ResolvedValue::Boolean(self.enforce_integer_equality(cs, scope, *left, *right))
            }
            BooleanExpression::FieldEq(left, right) => {
                ResolvedValue::Boolean(self.enforce_field_equality(cs, scope, *left, *right))
            }
            BooleanExpression::BoolEq(left, right) => {
                ResolvedValue::Boolean(self.enforce_bool_equality(cs, scope, *left, *right))
            }
            BooleanExpression::IfElse(first, second, third) => {
                let resolved_first =
                    match self.enforce_boolean_expression(cs, scope.clone(), *first) {
                        ResolvedValue::Boolean(resolved) => resolved,
                        _ => unimplemented!("if else conditional must resolve to boolean"),
                    };
                if resolved_first.eq(&Boolean::Constant(true)) {
                    self.enforce_boolean_expression(cs, scope, *second)
                } else {
                    self.enforce_boolean_expression(cs, scope, *third)
                }
            }
            BooleanExpression::Array(array) => {
                let mut result = vec![];
                array.into_iter().for_each(|element| match *element {
                    BooleanSpreadOrExpression::Spread(spread) => match spread {
                        BooleanExpression::Variable(variable) => {
                            let array_name = new_scope_from_variable(scope.clone(), &variable);
                            match self.get(&array_name) {
                                Some(value) => match value {
                                    ResolvedValue::BooleanArray(array) => {
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
                    BooleanSpreadOrExpression::Expression(expression) => {
                        match self.enforce_boolean_expression(cs, scope.clone(), expression) {
                            ResolvedValue::Boolean(value) => result.push(value),
                            value => {
                                unimplemented!("expected boolean for boolean array, got {}", value)
                            }
                        }
                    }
                });
                ResolvedValue::BooleanArray(result)
            }
            expression => unimplemented!("boolean expression {}", expression),
        }
    }
}
