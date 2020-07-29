use leo_ast::{LeoParser, ParserError};
use std::{env, fs, path::Path};

fn to_leo_ast(filepath: &Path) -> Result<String, ParserError> {
    // Loads the Leo code as a string from the given file path.
    let program_filepath = filepath.to_path_buf();
    let program_string = LeoParser::load_file(&program_filepath)?;

    // Parses the Leo program string and constructs an abstract syntax tree.
    let abstract_syntax_tree = LeoParser::parse_file(&program_filepath, &program_string)?;

    // Serializes the abstract syntax tree into JSON format.
    let serialized_ast = LeoParser::to_json_string(&abstract_syntax_tree)?;

    Ok(serialized_ast)
}

fn main() -> Result<(), ParserError> {
    // Parse the command-line arguments as strings.
    let cli_arguments = env::args().collect::<Vec<String>>();

    // Check that the correct number of command-line arguments were passed in.
    if cli_arguments.len() < 2 || cli_arguments.len() > 3 {
        eprintln!("Error - invalid number of command-line arguments");
        println!("\nCommand-line usage:\n\n\tleo_ast {{input_filename}}.leo {{optional_output_filepath}}\n");
        return Ok(()); // Exit innocently
    }

    // Create the input filepath.
    let input_filepath = Path::new(&cli_arguments[1]);

    // Create the serialized abstract syntax tree.
    let serialized_ast = to_leo_ast(&input_filepath)?;
    println!("{}", serialized_ast);

    // Create the output filepath.
    let output_filepath = match cli_arguments.len() == 3 {
        true => format!(
            "{}/{}.json",
            cli_arguments[2],
            input_filepath.file_stem().unwrap().to_str().unwrap()
        ),
        false => format!("./{}.json", input_filepath.file_stem().unwrap().to_str().unwrap()),
    };

    // Write the serialized abstract syntax tree to the output filepath.
    fs::write(Path::new(&output_filepath), serialized_ast)?;

    Ok(())
}
