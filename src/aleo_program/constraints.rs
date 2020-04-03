use crate::aleo_program::{
    BooleanExpression, Expression, FieldExpression, Program, Statement, Variable,
};

use snarkos_models::curves::Field;
use snarkos_models::gadgets::utilities::eq::EqGadget;
use snarkos_models::gadgets::{
    r1cs::ConstraintSystem,
    utilities::{alloc::AllocGadget, boolean::Boolean, eq::ConditionalEqGadget, uint32::UInt32},
};

fn bool_from_variable<F: Field, CS: ConstraintSystem<F>>(
    cs: &mut CS,
    variable: Variable,
) -> Boolean {
    let argument = std::env::args()
        .nth(1)
        .unwrap_or("true".into())
        .parse::<bool>()
        .unwrap();

    println!(" argument passed to command line a = {:?}", argument);
    // let a = true;
    Boolean::alloc_input(cs.ns(|| variable.0), || Ok(argument)).unwrap()
}

fn u32_from_variable<F: Field, CS: ConstraintSystem<F>>(cs: &mut CS, variable: Variable) -> UInt32 {
    let argument = std::env::args()
        .nth(1)
        .unwrap_or("1".into())
        .parse::<u32>()
        .unwrap();

    println!(" argument passed to command line a = {:?}", argument);

    // let a = 1;
    UInt32::alloc(cs.ns(|| variable.0), Some(argument)).unwrap()
}

fn get_bool_value<F: Field, CS: ConstraintSystem<F>>(
    cs: &mut CS,
    expression: BooleanExpression,
) -> Boolean {
    match expression {
        BooleanExpression::Variable(variable) => bool_from_variable(cs, variable),
        BooleanExpression::Value(value) => Boolean::Constant(value),
        expression => enforce_boolean_expression(cs, expression),
    }
}

fn get_u32_value<F: Field, CS: ConstraintSystem<F>>(
    cs: &mut CS,
    expression: FieldExpression,
) -> UInt32 {
    match expression {
        FieldExpression::Variable(variable) => u32_from_variable(cs, variable),
        FieldExpression::Number(number) => UInt32::constant(number),
        field => enforce_field_expression(cs, field),
    }
}

fn enforce_or<F: Field, CS: ConstraintSystem<F>>(
    cs: &mut CS,
    left: BooleanExpression,
    right: BooleanExpression,
) -> Boolean {
    let left = get_bool_value(cs, left);
    let right = get_bool_value(cs, right);

    Boolean::or(cs, &left, &right).unwrap()
}

fn enforce_and<F: Field, CS: ConstraintSystem<F>>(
    cs: &mut CS,
    left: BooleanExpression,
    right: BooleanExpression,
) -> Boolean {
    let left = get_bool_value(cs, left);
    let right = get_bool_value(cs, right);

    Boolean::and(cs, &left, &right).unwrap()
}

fn enforce_bool_equality<F: Field, CS: ConstraintSystem<F>>(
    cs: &mut CS,
    left: BooleanExpression,
    right: BooleanExpression,
) -> Boolean {
    let left = get_bool_value(cs, left);
    let right = get_bool_value(cs, right);

    left.enforce_equal(cs.ns(|| format!("enforce bool equal")), &right)
        .unwrap();

    Boolean::Constant(true)
}

fn enforce_field_equality<F: Field, CS: ConstraintSystem<F>>(
    cs: &mut CS,
    left: FieldExpression,
    right: FieldExpression,
) -> Boolean {
    let left = get_u32_value(cs, left);
    let right = get_u32_value(cs, right);

    left.conditional_enforce_equal(
        cs.ns(|| format!("enforce field equal")),
        &right,
        &Boolean::Constant(true),
    )
    .unwrap();

    Boolean::Constant(true)
}

fn enforce_boolean_expression<F: Field, CS: ConstraintSystem<F>>(
    cs: &mut CS,
    expression: BooleanExpression,
) -> Boolean {
    match expression {
        BooleanExpression::Or(left, right) => enforce_or(cs, *left, *right),
        BooleanExpression::And(left, right) => enforce_and(cs, *left, *right),
        BooleanExpression::BoolEq(left, right) => enforce_bool_equality(cs, *left, *right),
        BooleanExpression::FieldEq(left, right) => enforce_field_equality(cs, *left, *right),
        _ => unimplemented!(),
    }
}

fn enforce_add<F: Field, CS: ConstraintSystem<F>>(
    cs: &mut CS,
    left: FieldExpression,
    right: FieldExpression,
) -> UInt32 {
    let left = get_u32_value(cs, left);
    let right = get_u32_value(cs, right);

    left.add(
        cs.ns(|| format!("enforce {} + {}", left.value.unwrap(), right.value.unwrap())),
        &right,
    )
    .unwrap()
}

fn enforce_sub<F: Field, CS: ConstraintSystem<F>>(
    cs: &mut CS,
    left: FieldExpression,
    right: FieldExpression,
) -> UInt32 {
    let left = get_u32_value(cs, left);
    let right = get_u32_value(cs, right);

    left.sub(
        cs.ns(|| format!("enforce {} - {}", left.value.unwrap(), right.value.unwrap())),
        &right,
    )
    .unwrap()
}

fn enforce_mul<F: Field, CS: ConstraintSystem<F>>(
    cs: &mut CS,
    left: FieldExpression,
    right: FieldExpression,
) -> UInt32 {
    let left = get_u32_value(cs, left);
    let right = get_u32_value(cs, right);

    left.mul(
        cs.ns(|| format!("enforce {} * {}", left.value.unwrap(), right.value.unwrap())),
        &right,
    )
    .unwrap()
}

fn enforce_div<F: Field, CS: ConstraintSystem<F>>(
    cs: &mut CS,
    left: FieldExpression,
    right: FieldExpression,
) -> UInt32 {
    let left = get_u32_value(cs, left);
    let right = get_u32_value(cs, right);

    left.div(
        cs.ns(|| format!("enforce {} / {}", left.value.unwrap(), right.value.unwrap())),
        &right,
    )
    .unwrap()
}

fn enforce_pow<F: Field, CS: ConstraintSystem<F>>(
    cs: &mut CS,
    left: FieldExpression,
    right: FieldExpression,
) -> UInt32 {
    let left = get_u32_value(cs, left);
    let right = get_u32_value(cs, right);

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
    cs: &mut CS,
    expression: FieldExpression,
) -> UInt32 {
    match expression {
        FieldExpression::Add(left, right) => enforce_add(cs, *left, *right),
        FieldExpression::Sub(left, right) => enforce_sub(cs, *left, *right),
        FieldExpression::Mul(left, right) => enforce_mul(cs, *left, *right),
        FieldExpression::Div(left, right) => enforce_div(cs, *left, *right),
        FieldExpression::Pow(left, right) => enforce_pow(cs, *left, *right),
        _ => unimplemented!(),
    }
}

pub fn generate_constraints<F: Field, CS: ConstraintSystem<F>>(cs: &mut CS, program: Program) {
    program
        .statements
        .into_iter()
        .for_each(|statement| match statement {
            Statement::Definition(variable, expression) => match expression {
                Expression::Boolean(boolean_expression) => {
                    let res = enforce_boolean_expression(cs, boolean_expression);
                    println!("boolean result: {}", res.get_value().unwrap());
                }
                Expression::FieldElement(field_expression) => {
                    let res = enforce_field_expression(cs, field_expression);
                    println!("field result: {}", res.value.unwrap());
                }
                _ => unimplemented!(),
            },
            Statement::Return(statements) => {
                statements
                    .into_iter()
                    .for_each(|expression| match expression {
                        Expression::Boolean(boolean_expression) => {
                            let res = enforce_boolean_expression(cs, boolean_expression);
                            println!("boolean result: {}", res.get_value().unwrap());
                        }
                        Expression::FieldElement(field_expression) => {
                            let res = enforce_field_expression(cs, field_expression);
                            println!("field result: {}", res.value.unwrap());
                        }
                        _ => unimplemented!(),
                    });
            }
            _ => unimplemented!(),
        });
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
