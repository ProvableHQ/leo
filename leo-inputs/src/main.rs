use leo_inputs::{self, LeoInputsParser};

use std::env::current_dir;

fn main() {
    let mut path = current_dir().unwrap();
    path.push("input.leo");

    let input_file = &LeoInputsParser::load_file(&path).expect("cannot read file");
    let syntax_tree = LeoInputsParser::parse_file(&path, input_file).unwrap();

    println!("tree: {:#?}", syntax_tree);
}
