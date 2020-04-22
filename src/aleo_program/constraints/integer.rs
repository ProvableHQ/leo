//! Methods to enforce constraints on integers in a resolved aleo program.
//!
//! @file integer.rs
//! @author Collin Chin <collin@aleo.org>
//! @date 2020

use crate::aleo_program::constraints::{new_scope_from_variable, ResolvedProgram, ResolvedValue};
use crate::aleo_program::{
    new_variable_from_variable, Integer, IntegerExpression, IntegerSpreadOrExpression, Parameter,
    Variable,
};

use snarkos_models::curves::{Field, PrimeField};
use snarkos_models::gadgets::{
    r1cs::ConstraintSystem,
    utilities::{ boolean::Boolean, eq::ConditionalEqGadget, uint32::UInt32},
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

    pub(crate) fn integer_from_variable(
        &mut self,
        scope: String,
        variable: Variable<F>,
    ) -> ResolvedValue<F> {
        // Evaluate variable name in current function scope
        let variable_name = new_scope_from_variable(scope, &variable);

        if self.contains_name(&variable_name) {
            // TODO: return synthesis error: "assignment missing" here
            self.get(&variable_name).unwrap().clone()
        } else {
            unimplemented!("cannot resolve variable {} in program", variable_name)
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
            IntegerExpression::Variable(variable) => self.integer_from_variable(scope, variable),
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
            IntegerExpression::Variable(variable) => self.integer_from_variable(scope, variable),
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
