// Copyright (C) 2019-2020 Aleo Systems Inc.
// This file is part of the Leo library.

// The Leo library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The Leo library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the Leo library. If not, see <https://www.gnu.org/licenses/>.

use leo_ast::{Ast, AstError};
use leo_grammar::Grammar;
use std::{env, fs, path::Path};

fn to_leo_tree(filepath: &Path) -> Result<String, AstError> {
    // Loads the Leo code as a string from the given file path.
    let program_filepath = filepath.to_path_buf();
    let program_string = Grammar::load_file(&program_filepath)?;

    // Parses the Leo file and constructs a pest ast.
    let ast = Grammar::new(&program_filepath, &program_string)?;

    // Parse the pest ast and constructs a ast.
    let leo_ast = Ast::new("leo_tree", &ast)?;

    let serialized_leo_ast = Ast::to_json_string(&leo_ast)?;

    Ok(serialized_leo_ast)
}

fn main() -> Result<(), AstError> {
    // Parse the command-line arguments as strings.
    let cli_arguments = env::args().collect::<Vec<String>>();

    // Check that the correct number of command-line arguments were passed in.
    if cli_arguments.len() < 2 || cli_arguments.len() > 3 {
        eprintln!("Warning - an invalid number of command-line arguments were provided.");
        println!(
            "\nCommand-line usage:\n\n\tleo_ast {{PATH/TO/INPUT_FILENAME}}.leo {{PATH/TO/OUTPUT_DIRECTORY (optional)}}\n"
        );
        return Ok(()); // Exit innocently
    }

    // Construct the input filepath.
    let input_filepath = Path::new(&cli_arguments[1]);

    // Construct the serialized syntax tree.
    let serialized_leo_tree = to_leo_tree(&input_filepath)?;
    println!("{}", serialized_leo_tree);

    // Determine the output directory.
    let output_directory = match cli_arguments.len() == 3 {
        true => format!(
            "{}/{}.json",
            cli_arguments[2],
            input_filepath.file_stem().unwrap().to_str().unwrap()
        ),
        false => format!("./{}.json", input_filepath.file_stem().unwrap().to_str().unwrap()),
    };

    // Write the serialized syntax tree to the output directory.
    fs::write(Path::new(&output_directory), serialized_leo_tree)?;

    Ok(())
}
