extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;
use std::fs;

#[derive(Parser)]
#[grammar = "language.pest"]
pub struct LanguageParser;

// pub struct Statement

fn main() {
    let unparsed_file = fs::read_to_string("simple.program").expect("cannot read file");

    let file = LanguageParser::parse(Rule::file, &unparsed_file)
        .expect("unsuccessful parse") // unwrap the parse result
        .next().unwrap(); // get and unwrap the `file` rule; never fails

    for token in file.into_inner() {
        match token.as_rule() {
            Rule::statement => println!("{:?}", token.into_inner()),
            Rule::EOI => println!("END"),
            _ => println!("{:?}", token.into_inner()),
        }
        // println!("{:?}", token);
    }

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

    let successful_parse = LanguageParser::parse(Rule::value, "-273");
    println!("{:?}", successful_parse);

    // let unsuccessful_parse = CSVParser::parse(Rule::field, "this is not a number");
    // println!("{:?}", unsuccessful_parse);
}
