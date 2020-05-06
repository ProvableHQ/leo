use leo_inputs::{self, inputs_ast};

use from_pest::FromPest;
use std::fs;

fn main() {
    // Read in file as string
    let unparsed_file = fs::read_to_string("input.leo").expect("cannot read file");

    // Parse the file using leo.pest
    let mut file = inputs_ast::parse(&unparsed_file).expect("unsuccessful parse");

    // Build the abstract syntax tree
    let syntax_tree = inputs_ast::File::from_pest(&mut file).expect("infallible");

    println!("tree: {}", syntax_tree);
}
