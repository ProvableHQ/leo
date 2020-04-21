//! Methods to enforce constraints on integers in a resolved aleo program.
//!
//! @file integer.rs
//! @author Collin Chin <collin@aleo.org>
//! @date 2020

use crate::program::constraints::{new_scope_from_variable, ResolvedProgram, ResolvedValue};
use crate::program::{Integer, IntegerExpression, IntegerSpreadOrExpression, Variable};

use snarkos_models::curves::{Field, PrimeField};
use snarkos_models::gadgets::{
    r1cs::ConstraintSystem,
    utilities::{alloc::AllocGadget, boolean::Boolean, eq::ConditionalEqGadget, uint32::UInt32},
};

impl<F: Field + PrimeField, CS: ConstraintSystem<F>> ResolvedProgram<F, CS> {
    pub(crate) fn integer_from_variable(
        &mut self,
        cs: &mut CS,
        scope: String,
        variable: Variable<F>,
    ) -> ResolvedValue<F> {
        // Evaluate variable name in current function scope
        let variable_name = new_scope_from_variable(scope, &variable);

        if self.contains_name(&variable_name) {
            // TODO: return synthesis error: "assignment missing" here
            self.get(&variable_name).unwrap().clone()
        } else {
            // TODO: remove this after resolving arguments
            let argument = std::env::args()
                .nth(1)
                .unwrap_or("1".into())
                .parse::<u32>()
                .unwrap();

            println!(" argument passed to command line a = {:?}\n", argument);

            // let a = 1;
            ResolvedValue::U32(UInt32::alloc(cs.ns(|| variable.name), Some(argument)).unwrap())
        }
    }

    fn get_integer_constant(integer: Integer) -> ResolvedValue<F> {
        match integer {
            Integer::U32(u32_value) => ResolvedValue::U32(UInt32::constant(u32_value)),
        }
    }

    fn get_integer_value(
        &mut self,
        cs: &mut CS,
        scope: String,
        expression: IntegerExpression<F>,
    ) -> ResolvedValue<F> {
        match expression {
            IntegerExpression::Variable(variable) => {
                self.integer_from_variable(cs, scope, variable)
            }
            IntegerExpression::Number(number) => Self::get_integer_constant(number),
            expression => self.enforce_integer_expression(cs, scope, expression),
        }
    }

    fn enforce_u32_equality(cs: &mut CS, left: UInt32, right: UInt32) -> Boolean {
        left.conditional_enforce_equal(
            cs.ns(|| format!("enforce field equal")),
            &right,
            &Boolean::Constant(true),
        )
        .unwrap();

        Boolean::Constant(true)
    }

    pub(crate) fn enforce_integer_equality(
        &mut self,
        cs: &mut CS,
        scope: String,
        left: IntegerExpression<F>,
        right: IntegerExpression<F>,
    ) -> Boolean {
        let left = self.get_integer_value(cs, scope.clone(), left);
        let right = self.get_integer_value(cs, scope.clone(), right);

        match (left, right) {
            (ResolvedValue::U32(left_u32), ResolvedValue::U32(right_u32)) => {
                Self::enforce_u32_equality(cs, left_u32, right_u32)
            }
            (left_int, right_int) => {
                unimplemented!("equality not impl between {} == {}", left_int, right_int)
            }
        }
    }

    fn enforce_u32_add(cs: &mut CS, left: UInt32, right: UInt32) -> ResolvedValue<F> {
        ResolvedValue::U32(
            UInt32::addmany(
                cs.ns(|| format!("enforce {} + {}", left.value.unwrap(), right.value.unwrap())),
                &[left, right],
            )
            .unwrap(),
        )
    }

    fn enforce_integer_add(
        &mut self,
        cs: &mut CS,
        scope: String,
        left: IntegerExpression<F>,
        right: IntegerExpression<F>,
    ) -> ResolvedValue<F> {
        let left = self.get_integer_value(cs, scope.clone(), left);
        let right = self.get_integer_value(cs, scope.clone(), right);

        match (left, right) {
            (ResolvedValue::U32(left_u32), ResolvedValue::U32(right_u32)) => {
                Self::enforce_u32_add(cs, left_u32, right_u32)
            }
            (left_int, right_int) => {
                unimplemented!("add not impl between {} + {}", left_int, right_int)
            }
        }
    }

    fn enforce_u32_sub(cs: &mut CS, left: UInt32, right: UInt32) -> ResolvedValue<F> {
        ResolvedValue::U32(
            left.sub(
                cs.ns(|| format!("enforce {} - {}", left.value.unwrap(), right.value.unwrap())),
                &right,
            )
            .unwrap(),
        )
    }

    fn enforce_integer_sub(
        &mut self,
        cs: &mut CS,
        scope: String,
        left: IntegerExpression<F>,
        right: IntegerExpression<F>,
    ) -> ResolvedValue<F> {
        let left = self.get_integer_value(cs, scope.clone(), left);
        let right = self.get_integer_value(cs, scope.clone(), right);

        match (left, right) {
            (ResolvedValue::U32(left_u32), ResolvedValue::U32(right_u32)) => {
                Self::enforce_u32_sub(cs, left_u32, right_u32)
            }
            (left_int, right_int) => {
                unimplemented!("add not impl between {} + {}", left_int, right_int)
            }
        }
    }

    fn enforce_u32_mul(cs: &mut CS, left: UInt32, right: UInt32) -> ResolvedValue<F> {
        ResolvedValue::U32(
            left.mul(
                cs.ns(|| format!("enforce {} * {}", left.value.unwrap(), right.value.unwrap())),
                &right,
            )
            .unwrap(),
        )
    }

    fn enforce_integer_mul(
        &mut self,
        cs: &mut CS,
        scope: String,
        left: IntegerExpression<F>,
        right: IntegerExpression<F>,
    ) -> ResolvedValue<F> {
        let left = self.get_integer_value(cs, scope.clone(), left);
        let right = self.get_integer_value(cs, scope.clone(), right);

        match (left, right) {
            (ResolvedValue::U32(left_u32), ResolvedValue::U32(right_u32)) => {
                Self::enforce_u32_mul(cs, left_u32, right_u32)
            }
            (left_int, right_int) => {
                unimplemented!("add not impl between {} + {}", left_int, right_int)
            }
        }
    }

    fn enforce_u32_div(cs: &mut CS, left: UInt32, right: UInt32) -> ResolvedValue<F> {
        ResolvedValue::U32(
            left.div(
                cs.ns(|| format!("enforce {} / {}", left.value.unwrap(), right.value.unwrap())),
                &right,
            )
            .unwrap(),
        )
    }

    fn enforce_integer_div(
        &mut self,
        cs: &mut CS,
        scope: String,
        left: IntegerExpression<F>,
        right: IntegerExpression<F>,
    ) -> ResolvedValue<F> {
        let left = self.get_integer_value(cs, scope.clone(), left);
        let right = self.get_integer_value(cs, scope.clone(), right);

        match (left, right) {
            (ResolvedValue::U32(left_u32), ResolvedValue::U32(right_u32)) => {
                Self::enforce_u32_div(cs, left_u32, right_u32)
            }
            (left_int, right_int) => {
                unimplemented!("add not impl between {} + {}", left_int, right_int)
            }
        }
    }

    fn enforce_u32_pow(cs: &mut CS, left: UInt32, right: UInt32) -> ResolvedValue<F> {
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

    fn enforce_integer_pow(
        &mut self,
        cs: &mut CS,
        scope: String,
        left: IntegerExpression<F>,
        right: IntegerExpression<F>,
    ) -> ResolvedValue<F> {
        let left = self.get_integer_value(cs, scope.clone(), left);
        let right = self.get_integer_value(cs, scope.clone(), right);

        match (left, right) {
            (ResolvedValue::U32(left_u32), ResolvedValue::U32(right_u32)) => {
                Self::enforce_u32_pow(cs, left_u32, right_u32)
            }
            (left_int, right_int) => {
                unimplemented!("add not impl between {} + {}", left_int, right_int)
            }
        }
    }

    pub(crate) fn enforce_integer_expression(
        &mut self,
        cs: &mut CS,
        scope: String,
        expression: IntegerExpression<F>,
    ) -> ResolvedValue<F> {
        match expression {
            IntegerExpression::Variable(variable) => {
                self.integer_from_variable(cs, scope, variable)
            }
            IntegerExpression::Number(number) => Self::get_integer_constant(number),
            IntegerExpression::Add(left, right) => {
                self.enforce_integer_add(cs, scope, *left, *right)
            }
            IntegerExpression::Sub(left, right) => {
                self.enforce_integer_sub(cs, scope, *left, *right)
            }
            IntegerExpression::Mul(left, right) => {
                self.enforce_integer_mul(cs, scope, *left, *right)
            }
            IntegerExpression::Div(left, right) => {
                self.enforce_integer_div(cs, scope, *left, *right)
            }
            IntegerExpression::Pow(left, right) => {
                self.enforce_integer_pow(cs, scope, *left, *right)
            }
            IntegerExpression::IfElse(first, second, third) => {
                let resolved_first =
                    match self.enforce_boolean_expression(cs, scope.clone(), *first) {
                        ResolvedValue::Boolean(resolved) => resolved,
                        _ => unimplemented!("if else conditional must resolve to boolean"),
                    };

                if resolved_first.eq(&Boolean::Constant(true)) {
                    self.enforce_integer_expression(cs, scope, *second)
                } else {
                    self.enforce_integer_expression(cs, scope, *third)
                }
            }
            IntegerExpression::Array(array) => {
                let mut result = vec![];
                array.into_iter().for_each(|element| match *element {
                    IntegerSpreadOrExpression::Spread(spread) => match spread {
                        IntegerExpression::Variable(variable) => {
                            let array_name = new_scope_from_variable(scope.clone(), &variable);
                            match self.get(&array_name) {
                                Some(value) => match value {
                                    ResolvedValue::U32Array(array) => result.extend(array.clone()),
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
                    IntegerSpreadOrExpression::Expression(expression) => {
                        match self.enforce_integer_expression(cs, scope.clone(), expression) {
                            ResolvedValue::U32(value) => result.push(value),
                            _ => unimplemented!("cannot resolve field"),
                        }
                    }
                });
                ResolvedValue::U32Array(result)
            }
        }
    }
}
