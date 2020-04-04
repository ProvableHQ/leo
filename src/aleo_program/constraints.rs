use crate::aleo_program::{
    BooleanExpression, Expression, FieldExpression, Program, Statement, Variable,
};

use snarkos_models::curves::Field;
use snarkos_models::gadgets::utilities::eq::EqGadget;
use snarkos_models::gadgets::{
    r1cs::ConstraintSystem,
    utilities::{alloc::AllocGadget, boolean::Boolean, eq::ConditionalEqGadget, uint32::UInt32},
};
use std::collections::HashMap;

pub enum ResolvedValue {
    Boolean(Boolean),
    FieldElement(UInt32),
}

pub struct ResolvedProgram {
    pub resolved_variables: HashMap<Variable, ResolvedValue>,
}

impl ResolvedProgram {
    fn new() -> Self {
        Self {
            resolved_variables: HashMap::new(),
        }
    }

    fn insert(&mut self, variable: Variable, value: ResolvedValue) {
        self.resolved_variables.insert(variable, value);
    }

    fn bool_from_variable<F: Field, CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        variable: Variable,
    ) -> Boolean {
        if self.resolved_variables.contains_key(&variable) {
            match self.resolved_variables.get(&variable).unwrap() {
                ResolvedValue::Boolean(boolean) => boolean.clone(),
                _ => panic!("expected a boolean, got field"),
            };
            Boolean::Constant(true)
        } else {
            let argument = std::env::args()
                .nth(1)
                .unwrap_or("true".into())
                .parse::<bool>()
                .unwrap();
            println!(" argument passed to command line a = {:?}", argument);
            // let a = true;
            Boolean::alloc_input(cs.ns(|| variable.0), || Ok(argument)).unwrap()
        }
    }

    fn u32_from_variable<F: Field, CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        variable: Variable,
    ) -> UInt32 {
        if self.resolved_variables.contains_key(&variable) {
            match self.resolved_variables.get(&variable).unwrap() {
                ResolvedValue::FieldElement(field) => field.clone(),
                _ => panic!("expected a field, got boolean"),
            }
        } else {
            let argument = std::env::args()
                .nth(1)
                .unwrap_or("1".into())
                .parse::<u32>()
                .unwrap();

            println!(" argument passed to command line a = {:?}", argument);

            // let a = 1;
            UInt32::alloc(cs.ns(|| variable.0), Some(argument)).unwrap()
        }
    }

    fn get_bool_value<F: Field, CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        expression: BooleanExpression,
    ) -> Boolean {
        match expression {
            BooleanExpression::Variable(variable) => self.bool_from_variable(cs, variable),
            BooleanExpression::Value(value) => Boolean::Constant(value),
            expression => self.enforce_boolean_expression(cs, expression),
        }
    }

    fn get_u32_value<F: Field, CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        expression: FieldExpression,
    ) -> UInt32 {
        match expression {
            FieldExpression::Variable(variable) => self.u32_from_variable(cs, variable),
            FieldExpression::Number(number) => UInt32::constant(number),
            field => self.enforce_field_expression(cs, field),
        }
    }

    fn enforce_or<F: Field, CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        left: BooleanExpression,
        right: BooleanExpression,
    ) -> Boolean {
        let left = self.get_bool_value(cs, left);
        let right = self.get_bool_value(cs, right);

        Boolean::or(cs, &left, &right).unwrap()
    }

    fn enforce_and<F: Field, CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        left: BooleanExpression,
        right: BooleanExpression,
    ) -> Boolean {
        let left = self.get_bool_value(cs, left);
        let right = self.get_bool_value(cs, right);

        Boolean::and(cs, &left, &right).unwrap()
    }

    fn enforce_bool_equality<F: Field, CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        left: BooleanExpression,
        right: BooleanExpression,
    ) -> Boolean {
        let left = self.get_bool_value(cs, left);
        let right = self.get_bool_value(cs, right);

        left.enforce_equal(cs.ns(|| format!("enforce bool equal")), &right)
            .unwrap();

        Boolean::Constant(true)
    }

    fn enforce_field_equality<F: Field, CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        left: FieldExpression,
        right: FieldExpression,
    ) -> Boolean {
        let left = self.get_u32_value(cs, left);
        let right = self.get_u32_value(cs, right);

        left.conditional_enforce_equal(
            cs.ns(|| format!("enforce field equal")),
            &right,
            &Boolean::Constant(true),
        )
        .unwrap();

        Boolean::Constant(true)
    }

    fn enforce_boolean_expression<F: Field, CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        expression: BooleanExpression,
    ) -> Boolean {
        match expression {
            BooleanExpression::Or(left, right) => self.enforce_or(cs, *left, *right),
            BooleanExpression::And(left, right) => self.enforce_and(cs, *left, *right),
            BooleanExpression::BoolEq(left, right) => self.enforce_bool_equality(cs, *left, *right),
            BooleanExpression::FieldEq(left, right) => {
                self.enforce_field_equality(cs, *left, *right)
            }
            _ => unimplemented!(),
        }
    }

    fn enforce_add<F: Field, CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        left: FieldExpression,
        right: FieldExpression,
    ) -> UInt32 {
        let left = self.get_u32_value(cs, left);
        let right = self.get_u32_value(cs, right);

        println!("left: {:#?}", left.value.unwrap());
        println!("right: {:#?}", right.value.unwrap());
        // println!("expected: {:#?}", UInt32::alloc(cs.ns(|| format!("expected")), Some(3)));

        let res = left
            .add(
                cs.ns(|| format!("enforce {} + {}", left.value.unwrap(), right.value.unwrap())),
                &right,
            )
            .unwrap();

        println!("result: {:#?}", res.bits.to_vec());

        res
    }

    fn enforce_sub<F: Field, CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        left: FieldExpression,
        right: FieldExpression,
    ) -> UInt32 {
        let left = self.get_u32_value(cs, left);
        let right = self.get_u32_value(cs, right);

        left.sub(
            cs.ns(|| format!("enforce {} - {}", left.value.unwrap(), right.value.unwrap())),
            &right,
        )
        .unwrap()
    }

    fn enforce_mul<F: Field, CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        left: FieldExpression,
        right: FieldExpression,
    ) -> UInt32 {
        let left = self.get_u32_value(cs, left);
        let right = self.get_u32_value(cs, right);

        println!("left: {}", left.value.unwrap());
        println!("right: {}", right.value.unwrap());

        let res = left
            .mul(
                cs.ns(|| format!("enforce {} * {}", left.value.unwrap(), right.value.unwrap())),
                &right,
            )
            .unwrap();

        println!("result: {}", res.value.unwrap());

        res
    }

    fn enforce_div<F: Field, CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        left: FieldExpression,
        right: FieldExpression,
    ) -> UInt32 {
        let left = self.get_u32_value(cs, left);
        let right = self.get_u32_value(cs, right);

        left.div(
            cs.ns(|| format!("enforce {} / {}", left.value.unwrap(), right.value.unwrap())),
            &right,
        )
        .unwrap()
    }

    fn enforce_pow<F: Field, CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        left: FieldExpression,
        right: FieldExpression,
    ) -> UInt32 {
        let left = self.get_u32_value(cs, left);
        let right = self.get_u32_value(cs, right);

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
        .unwrap()
    }

    fn enforce_field_expression<F: Field, CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        expression: FieldExpression,
    ) -> UInt32 {
        println!("enforcing: {}", expression);
        match expression {
            FieldExpression::Add(left, right) => self.enforce_add(cs, *left, *right),
            FieldExpression::Sub(left, right) => self.enforce_sub(cs, *left, *right),
            FieldExpression::Mul(left, right) => self.enforce_mul(cs, *left, *right),
            FieldExpression::Div(left, right) => self.enforce_div(cs, *left, *right),
            FieldExpression::Pow(left, right) => self.enforce_pow(cs, *left, *right),
            _ => unimplemented!(),
        }
    }

    pub fn generate_constraints<F: Field, CS: ConstraintSystem<F>>(cs: &mut CS, program: Program) {
        let mut resolved_program = ResolvedProgram::new();

        program
            .statements
            .into_iter()
            .for_each(|statement| match statement {
                Statement::Definition(variable, expression) => match expression {
                    Expression::Boolean(boolean_expression) => {
                        let res =
                            resolved_program.enforce_boolean_expression(cs, boolean_expression);
                        // println!("variable boolean result: {}", res.get_value().unwrap());
                        resolved_program.insert(variable, ResolvedValue::Boolean(res));
                    }
                    Expression::FieldElement(field_expression) => {
                        let res = resolved_program.enforce_field_expression(cs, field_expression);
                        println!(
                            " variable field result: {} = {}",
                            variable.0,
                            res.value.unwrap()
                        );
                        resolved_program.insert(variable, ResolvedValue::FieldElement(res));
                    }
                    _ => unimplemented!(),
                },
                Statement::Return(statements) => {
                    statements
                        .into_iter()
                        .for_each(|expression| match expression {
                            Expression::Boolean(boolean_expression) => {
                                let res = resolved_program
                                    .enforce_boolean_expression(cs, boolean_expression);
                                println!("boolean result: {}", res.get_value().unwrap());
                            }
                            Expression::FieldElement(field_expression) => {
                                println!("expression {:?}", field_expression);
                                let res =
                                    resolved_program.enforce_field_expression(cs, field_expression);
                                println!("field result: {}", res.value.unwrap());
                            }
                            _ => unimplemented!(),
                        });
                }
                statement => unimplemented!("statement unimplemented: {}", statement),
            });
    }
}

// impl Program {
//     pub fn setup(&self) {
//         self.statements
//             .iter()
//             .for_each(|statement| {
//
//             })
//     }
// }
