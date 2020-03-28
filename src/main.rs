use language::*;

use from_pest::FromPest;
use std::fs;
// use std::env;

fn main() {
    // use snarkos_gadgets::curves::edwards_bls12::FqGadget;
    use snarkos_models::gadgets::{
        r1cs::{ConstraintSystem, Fr, TestConstraintSystem},
        utilities::{
            alloc::AllocGadget, boolean::Boolean, eq::ConditionalEqGadget, uint32::UInt32,
        },
    };

    // Read in file as string
    let unparsed_file = fs::read_to_string("simple.program").expect("cannot read file");

    // Parse the file using langauge.pest
    let mut file = ast::parse(&unparsed_file).expect("unsuccessful parse");

    // Build the abstract syntax tree
    let syntax_tree = ast::File::from_pest(&mut file).expect("infallible");
    // println!("{:#?}", syntax_tree);

    let program = aleo_program::Program::from(syntax_tree);
    println!(" compiled: {:#?}", program);

    let mut cs = TestConstraintSystem::<Fr>::new();
    let argument = std::env::args()
        .nth(1)
        .unwrap_or("1".into())
        .parse::<u32>()
        .unwrap();

    println!(" argument passed to command line a = {:?}", argument);

    program
        .statements
        .into_iter()
        .for_each(|statement| match statement {
            aleo_program::Statement::Return(statements) => {
                statements
                    .into_iter()
                    .for_each(|expression| match expression {
                        aleo_program::Expression::Boolean(operation) => match operation {
                            aleo_program::BooleanExpression::FieldEq(lhs, rhs) => match *lhs {
                                aleo_program::FieldExpression::Variable(variable) => {
                                    let left_variable =
                                        UInt32::alloc(cs.ns(|| variable.0), Some(argument))
                                            .unwrap();
                                    match *rhs {
                                        aleo_program::FieldExpression::Number(number) => {
                                            let right_number = UInt32::constant(number);

                                            let bool = Boolean::alloc(
                                                cs.ns(|| format!("boolean")),
                                                || Ok(true),
                                            )
                                            .unwrap();
                                            left_variable
                                                .conditional_enforce_equal(
                                                    cs.ns(|| format!("enforce equal")),
                                                    &right_number,
                                                    &bool,
                                                )
                                                .unwrap();
                                        }
                                        _ => unimplemented!(),
                                    }
                                }
                                _ => unimplemented!(),
                            },
                            _ => unimplemented!(),
                        },
                        _ => unimplemented!(),
                    });
            }
            _ => unimplemented!(),
        });

    // program
    //     .statements
    //     .into_iter()
    //     .for_each(|statement| {
    //         match statement {
    //             aleo_program::Statement::Constraint(quadratic, linear) => {
    //                 let a_var = (quadratic.0).0[0].clone();
    //                 assert!(a_var.id > 0);
    //
    //                 let c_var = linear.0[0].clone();
    //                 let c_value = c_var.value.parse::<u32>().unwrap();
    //
    //                 let a_bit = UInt32::alloc(cs.ns(|| "a_bit"), Some(argument)).unwrap();
    //                 let c_bit = UInt32::constant(c_value);
    //                 let bool = Boolean::alloc(cs.ns(|| format!("boolean")), || Ok(true)).unwrap();
    //
    //                 a_bit.conditional_enforce_equal(cs.ns(|| format!("enforce equal")), &c_bit, &bool).unwrap();
    //             }
    //         }
    //     });

    // let program = zokrates_program::Program::from(syntax_tree);

    // println!("{:#?}", program);

    // // Use this code when proving
    // let left_u32 = left_string.parse::<u32>().unwrap();
    // let right_u32 = right_string.parse::<u32>().unwrap();
    //
    // println!("left u32 value: {:#?}", left_u32);
    // println!("right u32 value: {:#?}", right_u32);
    //
    // let left_constraint = UInt32::alloc(cs.ns(|| "left variable"), Some(left_u32)).unwrap();
    // let right_constraint = UInt32::constant(right_u32);
    //
    // let bool = Boolean::alloc(cs.ns(|| format!("boolean")), || Ok(true)).unwrap();
    //
    // left_constraint.conditional_enforce_equal(cs.ns(|| format!("enforce left == right")), &right_constraint, &bool).unwrap();

    // Constraint testing
    // let bool = Boolean::alloc(cs.ns(|| format!("boolean")), || Ok(true)).unwrap();
    // let a_bit = UInt32::alloc(cs.ns(|| "a_bit"), Some(4u32)).unwrap();
    // let b_bit = UInt32::constant(4u32);
    //
    // a_bit.conditional_enforce_equal(cs.ns(|| format!("enforce equal")), &b_bit, &bool).unwrap();

    println!("\n satisfied: {:?}", cs.is_satisfied());

    println!(
        "\n number of constraints for input: {}",
        cs.num_constraints()
    );

    // for token in file.into_inner() {
    //     match token.as_rule() {
    //         Rule::statement => println!("{:?}", token.into_inner()),
    //         Rule::EOI => println!("END"),
    //         _ => println!("{:?}", token.into_inner()),
    //     }
    //     // println!("{:?}", token);
    // }

    // let mut field_sum: f64 = 0.0;
    // let mut record_count: u64 = 0;
    //
    // for record in file.into_inner() {
    //     match record.as_rule() {
    //         Rule::record => {
    //             record_count += 1;
    //
    //             for field in record.into_inner() {
    //                 field_sum += field.as_str().parse::<f64>().unwrap();
    //             }
    //         }
    //         Rule::EOI => (),
    //         _ => unreachable!(),
    //     }
    // }

    // println!("Sum of fields: {}", field_sum);
    // println!("Number of records: {}", record_count);

    // let successful_parse = LanguageParser::parse(Rule::value, "-273");
    // println!("{:?}", successful_parse);

    // let unsuccessful_parse = CSVParser::parse(Rule::field, "this is not a number");
    // println!("{:?}", unsuccessful_parse);
}
