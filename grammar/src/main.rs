// Copyright (C) 2019-2021 Aleo Systems Inc.
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

use leo_grammar::Grammar;
use leo_grammar::ParserError;
use std::env;
use std::fs;
use std::path::Path;

fn to_grammar(filepath: &Path) -> Result<String, ParserError> {
    // Loads the Leo code as a string from the given file path.
    let program_filepath = filepath.to_path_buf();
    let program_string = Grammar::load_file(&program_filepath)?;

    // Parses the Leo file and constructs an abstract syntax tree.
    let ast = Grammar::new(&program_filepath, &program_string)?;

    // Serializes the abstract syntax tree into JSON format.
    let serialized_ast = Grammar::to_json_string(&ast)?;

    Ok(serialized_ast)
}

fn main() -> Result<(), ParserError> {
    // Parse the command-line arguments as strings.
    let cli_arguments = env::args().collect::<Vec<String>>();

    // Check that the correct number of command-line arguments were passed in.
    if cli_arguments.len() < 2 || cli_arguments.len() > 3 {
        eprintln!("Warning - an invalid number of command-line arguments were provided.");
        println!(
            "\nCommand-line usage:\n\n\tleo_grammar {{PATH/TO/INPUT_FILENAME}}.leo {{PATH/TO/OUTPUT_DIRECTORY (optional)}}\n"
        );
        return Ok(()); // Exit innocently
    }

    // Construct the input filepath.
    let input_filepath = Path::new(&cli_arguments[1]);

    // Construct the serialized abstract syntax tree.
    let serialized_ast = to_grammar(&input_filepath)?;
    println!("{}", serialized_ast);

    // Determine the output directory.
    let output_directory = match cli_arguments.len() == 3 {
        true => format!(
            "{}/{}.json",
            cli_arguments[2],
            input_filepath.file_stem().unwrap().to_str().unwrap()
        ),
        false => format!("./{}.json", input_filepath.file_stem().unwrap().to_str().unwrap()),
    };

    // Write the serialized abstract syntax tree to the output directory.
    fs::write(Path::new(&output_directory), serialized_ast)?;

    Ok(())
}
