use language::*;

use from_pest::FromPest;
use std::fs;

fn main() {
    // use snarkos_gadgets::curves::edwards_bls12::FqGadget;
    // use snarkos_models::gadgets::{
    //     r1cs::{ConstraintSystem, TestConstraintSystem, Fr},
    //     utilities::{
    //         alloc::{AllocGadget},
    //         boolean::Boolean,
    //         uint32::UInt32,
    //     }
    // };

    // Read in file as string
    let unparsed_file = fs::read_to_string("simple.program").expect("cannot read file");

    // Parse the file using langauge.pest
    let mut file = ast::parse(&unparsed_file).expect("unsuccessful parse");

    // Build the abstract syntax tree
    let syntax_tree = ast::File::from_pest(&mut file).expect("infallible");

    let program = program::Program::from(syntax_tree);

    println!("{:?}", program);

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

    // // Constraint testing
    // let bool = Boolean::alloc(cs.ns(|| format!("boolean")), || Ok(true)).unwrap();
    // let a_bit = UInt32::alloc(cs.ns(|| "a_bit"), Some(4u32)).unwrap();
    // let b_bit = UInt32::constant(5u32);
    //
    // a_bit.conditional_enforce_equal(cs.ns(|| format!("enforce equal")), &b_bit, &bool).unwrap();

    // println!("satisfied: {:?}", cs.is_satisfied());

    // println!("\n\n number of constraints for input: {}", cs.num_constraints());

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
