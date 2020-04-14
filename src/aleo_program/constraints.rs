use crate::aleo_program::{
    BooleanExpression, BooleanSpreadOrExpression, Expression, FieldExpression,
    FieldSpreadOrExpression, Function, Program, Statement, Struct, Variable,
};

use snarkos_models::curves::{Field, PrimeField};
use snarkos_models::gadgets::utilities::eq::EqGadget;
use snarkos_models::gadgets::{
    r1cs::ConstraintSystem,
    utilities::{alloc::AllocGadget, boolean::Boolean, eq::ConditionalEqGadget, uint32::UInt32},
};
use std::collections::HashMap;
use std::fmt;

#[derive(Clone)]
pub enum ResolvedValue {
    Boolean(Boolean),
    BooleanArray(Vec<Boolean>),
    FieldElement(UInt32),
    FieldElementArray(Vec<UInt32>),
    Struct(Struct),
    Function(Function),
}

impl fmt::Display for ResolvedValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ResolvedValue::FieldElement(ref value) => write!(f, "{}", value.value.unwrap()),
            ResolvedValue::FieldElementArray(ref array) => {
                write!(f, "[")?;
                for (i, e) in array.iter().enumerate() {
                    write!(f, "{}", e.value.unwrap())?;
                    if i < array.len() - 1 {
                        write!(f, ", ")?;
                    }
                }
                write!(f, "]")
            }
            _ => unimplemented!("resolve values not finished"),
        }
    }
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

    fn bool_from_variable<F: Field + PrimeField, CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        variable: Variable,
    ) -> Boolean {
        if self.resolved_variables.contains_key(&variable) {
            // TODO: return synthesis error: "assignment missing" here
            match self.resolved_variables.get(&variable).unwrap() {
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
            Boolean::alloc(cs.ns(|| variable.0), || Ok(argument)).unwrap()
        }
    }

    fn u32_from_variable<F: Field + PrimeField, CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        variable: Variable,
    ) -> UInt32 {
        if self.resolved_variables.contains_key(&variable) {
            // TODO: return synthesis error: "assignment missing" here
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

            println!(" argument passed to command line a = {:?}\n", argument);

            // let a = 1;
            UInt32::alloc(cs.ns(|| variable.0), Some(argument)).unwrap()
        }
    }

    fn get_bool_value<F: Field + PrimeField, CS: ConstraintSystem<F>>(
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

    fn get_u32_value<F: Field + PrimeField + PrimeField, CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        expression: FieldExpression,
    ) -> UInt32 {
        match expression {
            FieldExpression::Variable(variable) => self.u32_from_variable(cs, variable),
            FieldExpression::Number(number) => UInt32::constant(number),
            field => match self.enforce_field_expression(cs, field) {
                ResolvedValue::FieldElement(value) => value,
                _ => unimplemented!("value not resolved"),
            },
        }
    }

    fn enforce_not<F: Field + PrimeField, CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        expression: BooleanExpression,
    ) -> Boolean {
        let expression = self.get_bool_value(cs, expression);

        expression.not()
    }

    fn enforce_or<F: Field + PrimeField, CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        left: BooleanExpression,
        right: BooleanExpression,
    ) -> Boolean {
        let left = self.get_bool_value(cs, left);
        let right = self.get_bool_value(cs, right);

        Boolean::or(cs, &left, &right).unwrap()
    }

    fn enforce_and<F: Field + PrimeField, CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        left: BooleanExpression,
        right: BooleanExpression,
    ) -> Boolean {
        let left = self.get_bool_value(cs, left);
        let right = self.get_bool_value(cs, right);

        Boolean::and(cs, &left, &right).unwrap()
    }

    fn enforce_bool_equality<F: Field + PrimeField, CS: ConstraintSystem<F>>(
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

    fn enforce_field_equality<F: Field + PrimeField, CS: ConstraintSystem<F>>(
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

    fn enforce_boolean_expression<F: Field + PrimeField, CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        expression: BooleanExpression,
    ) -> Boolean {
        match expression {
            BooleanExpression::Variable(variable) => self.bool_from_variable(cs, variable),
            BooleanExpression::Value(value) => Boolean::Constant(value),
            BooleanExpression::Not(expression) => self.enforce_not(cs, *expression),
            BooleanExpression::Or(left, right) => self.enforce_or(cs, *left, *right),
            BooleanExpression::And(left, right) => self.enforce_and(cs, *left, *right),
            BooleanExpression::BoolEq(left, right) => self.enforce_bool_equality(cs, *left, *right),
            BooleanExpression::FieldEq(left, right) => {
                self.enforce_field_equality(cs, *left, *right)
            }
            BooleanExpression::IfElse(first, second, third) => {
                if self
                    .enforce_boolean_expression(cs, *first)
                    .eq(&Boolean::Constant(true))
                {
                    self.enforce_boolean_expression(cs, *second)
                } else {
                    self.enforce_boolean_expression(cs, *third)
                }
            }
            _ => unimplemented!(),
        }
    }

    fn enforce_add<F: Field + PrimeField, CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        left: FieldExpression,
        right: FieldExpression,
    ) -> UInt32 {
        let left = self.get_u32_value(cs, left);
        let right = self.get_u32_value(cs, right);

        UInt32::addmany(
            cs.ns(|| format!("enforce {} + {}", left.value.unwrap(), right.value.unwrap())),
            &[left, right],
        )
        .unwrap()
    }

    fn enforce_sub<F: Field + PrimeField, CS: ConstraintSystem<F>>(
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

    fn enforce_mul<F: Field + PrimeField, CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        left: FieldExpression,
        right: FieldExpression,
    ) -> UInt32 {
        let left = self.get_u32_value(cs, left);
        let right = self.get_u32_value(cs, right);

        let res = left
            .mul(
                cs.ns(|| format!("enforce {} * {}", left.value.unwrap(), right.value.unwrap())),
                &right,
            )
            .unwrap();

        res
    }

    fn enforce_div<F: Field + PrimeField, CS: ConstraintSystem<F>>(
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

    fn enforce_pow<F: Field + PrimeField, CS: ConstraintSystem<F>>(
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

    fn enforce_field_expression<F: Field + PrimeField, CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        expression: FieldExpression,
    ) -> ResolvedValue {
        match expression {
            FieldExpression::Variable(variable) => {
                ResolvedValue::FieldElement(self.u32_from_variable(cs, variable))
            }
            FieldExpression::Number(number) => {
                ResolvedValue::FieldElement(UInt32::constant(number))
            }
            FieldExpression::Add(left, right) => {
                ResolvedValue::FieldElement(self.enforce_add(cs, *left, *right))
            }
            FieldExpression::Sub(left, right) => {
                ResolvedValue::FieldElement(self.enforce_sub(cs, *left, *right))
            }
            FieldExpression::Mul(left, right) => {
                ResolvedValue::FieldElement(self.enforce_mul(cs, *left, *right))
            }
            FieldExpression::Div(left, right) => {
                ResolvedValue::FieldElement(self.enforce_div(cs, *left, *right))
            }
            FieldExpression::Pow(left, right) => {
                ResolvedValue::FieldElement(self.enforce_pow(cs, *left, *right))
            }
            FieldExpression::IfElse(first, second, third) => {
                if self
                    .enforce_boolean_expression(cs, *first)
                    .eq(&Boolean::Constant(true))
                {
                    self.enforce_field_expression(cs, *second)
                } else {
                    self.enforce_field_expression(cs, *third)
                }
            }
            FieldExpression::Array(array) => ResolvedValue::FieldElementArray(
                array
                    .into_iter()
                    .map(|element| match *element {
                        FieldSpreadOrExpression::Spread(spread) => {
                            unimplemented!("spreads not enforced yet")
                        }
                        FieldSpreadOrExpression::FieldExpression(expression) => {
                            let resolved = self.enforce_field_expression(cs, expression);
                            match resolved {
                                ResolvedValue::FieldElement(value) => value,
                                _ => unimplemented!("cannot resolve field"),
                            }
                        }
                    })
                    .collect::<Vec<UInt32>>(),
            ),
        }
    }

    fn enforce_statement<F: Field + PrimeField, CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        statement: Statement,
    ) {
        match statement {
            Statement::Definition(variable, expression) => match expression {
                Expression::Boolean(boolean_expression) => {
                    let res = self.enforce_boolean_expression(cs, boolean_expression);
                    println!(
                        " variable boolean result: {} = {}",
                        variable.0,
                        res.get_value().unwrap()
                    );
                    self.insert(variable, ResolvedValue::Boolean(res));
                }
                Expression::FieldElement(field_expression) => {
                    let res = self.enforce_field_expression(cs, field_expression);
                    println!(" variable field result: {} = {}", variable.0, res);
                    self.insert(variable, res);
                }
                Expression::Variable(unresolved_variable) => {
                    if self.resolved_variables.contains_key(&unresolved_variable) {
                        // Reassigning variable to another variable
                        let already_assigned = self
                            .resolved_variables
                            .get_mut(&unresolved_variable)
                            .unwrap()
                            .clone();
                        self.insert(variable, already_assigned);
                    } else {
                        // The type of the unassigned variable depends on what is passed in
                        if std::env::args()
                            .nth(1)
                            .expect("variable declaration not passed in")
                            .parse::<bool>()
                            .is_ok()
                        {
                            let resolved_boolean = self.bool_from_variable(cs, unresolved_variable);
                            println!(
                                "variable boolean result: {} = {}",
                                variable.0,
                                resolved_boolean.get_value().unwrap()
                            );
                            self.insert(variable, ResolvedValue::Boolean(resolved_boolean));
                        } else {
                            let resolved_field_element =
                                self.u32_from_variable(cs, unresolved_variable);
                            println!(
                                " variable field result: {} = {}",
                                variable.0,
                                resolved_field_element.value.unwrap()
                            );
                            self.insert(
                                variable,
                                ResolvedValue::FieldElement(resolved_field_element),
                            );
                        }
                    }
                }
            },
            Statement::Return(statements) => {
                statements
                    .into_iter()
                    .for_each(|expression| match expression {
                        Expression::Boolean(boolean_expression) => {
                            let res = self.enforce_boolean_expression(cs, boolean_expression);
                            println!("\n  Boolean result = {}", res.get_value().unwrap());
                        }
                        Expression::FieldElement(field_expression) => {
                            let res = self.enforce_field_expression(cs, field_expression);
                            println!("\n  Field result = {}", res);
                        }
                        Expression::Variable(variable) => {
                            match self.resolved_variables.get_mut(&variable).unwrap().clone() {
                                ResolvedValue::Boolean(boolean) => println!(
                                    "\n  Variable result = {}",
                                    boolean.get_value().unwrap()
                                ),
                                ResolvedValue::FieldElement(field_element) => println!(
                                    "\n  Variable field result = {}",
                                    field_element.value.unwrap()
                                ),
                                _ => {}
                            }
                        }
                    });
            }
        };
    }

    pub fn generate_constraints<F: Field + PrimeField, CS: ConstraintSystem<F>>(
        cs: &mut CS,
        program: Program,
    ) {
        let mut resolved_program = ResolvedProgram::new();

        program
            .structs
            .into_iter()
            .for_each(|(variable, struct_def)| {
                resolved_program
                    .resolved_variables
                    .insert(variable, ResolvedValue::Struct(struct_def));
            });
        program
            .functions
            .into_iter()
            .for_each(|(variable, function)| {
                resolved_program
                    .resolved_variables
                    .insert(variable, ResolvedValue::Function(function));
            });

        // let main = resolved_program
        //     .resolved_variables
        //     .get_mut(&Variable("main".into()))
        //     .expect("main function not defined");
        //
        // match main {
        //     ResolvedValue::Function(function) => function
        //         .statements
        //         .clone()
        //         .into_iter()
        //         .for_each(|statement| resolved_program.enforce_statement(cs, statement)),
        //     _ => unimplemented!("main must be a function"),
        // }

        program
            .statements
            .into_iter()
            .for_each(|statement| resolved_program.enforce_statement(cs, statement));
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
