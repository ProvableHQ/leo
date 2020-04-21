use crate::aleo_program::constraints::{new_scope_from_variable, ResolvedProgram, ResolvedValue};
use crate::aleo_program::{FieldExpression, FieldSpreadOrExpression, Variable};

use snarkos_models::curves::{Field, PrimeField};
use snarkos_models::gadgets::{r1cs::ConstraintSystem, utilities::boolean::Boolean};
// use std::ops::{Add, Div, Mul, Neg, Sub};

impl<F: Field + PrimeField, CS: ConstraintSystem<F>> ResolvedProgram<F, CS> {
    /// Constrain field elements

    fn field_from_variable(&mut self, _cs: &mut CS, scope: String, variable: Variable<F>) -> F {
        // Evaluate variable name in current function scope
        let variable_name = new_scope_from_variable(scope, &variable);

        if self.contains_name(&variable_name) {
            // TODO: return synthesis error: "assignment missing" here
            match self.get(&variable_name).unwrap() {
                ResolvedValue::FieldElement(field) => field.clone(),
                value => unimplemented!("expected field element, got {}", value),
            }
        } else {
            // TODO: remove this after resolving arguments
            let argument = std::env::args()
                .nth(1)
                .unwrap_or("1".into())
                .parse::<u32>()
                .unwrap();

            println!(" argument passed to command line a = {:?}\n", argument);

            // let a = 1;
            F::default()
        }
    }

    fn get_field_value(&mut self, cs: &mut CS, scope: String, expression: FieldExpression<F>) -> F {
        match expression {
            FieldExpression::Variable(variable) => self.field_from_variable(cs, scope, variable),
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
                ResolvedValue::FieldElement(self.field_from_variable(cs, scope, variable))
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
