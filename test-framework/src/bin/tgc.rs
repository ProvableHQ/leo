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

use leo_test_framework::{
    fetch::find_tests,
    test::{extract_test_config, TestExpectationMode as Expectation},
};
// use leo_test_framework::runner::{Runner, Namespace};

use leo_asg::Asg;
use leo_compiler::{compiler::thread_leaked_context, TypeInferencePhase};
use leo_imports::ImportParser;

use std::{
    error::Error,
    path::{Path, PathBuf},
};

use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let mut tests = Vec::new();
    let mut test_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    test_dir.push("../tests/");

    find_tests(&test_dir, &mut tests);

    if !Path::new("tmp").exists() {
        fs::create_dir("tmp")?;
    }

    if !Path::new("tmp/tgc").exists() {
        fs::create_dir("tmp/tgc")?;
    }

    // Prepare directory for placing results.
    for (index, (path, text)) in tests.iter().enumerate() {
        if let Some(config) = extract_test_config(text) {
            // Skip namespaces that we don't need; also skip failure tests.
            if config.namespace != "Compile" || config.expectation == Expectation::Fail {
                continue;
            }

            let mut test_name = path
                .split(std::path::MAIN_SEPARATOR)
                .last()
                .unwrap()
                .replace(".leo", "");

            test_name.push_str(&format!("_{}", index));

            // Create directory for this file.
            let mut target = PathBuf::from("tmp/tgc");
            target.push(test_name);
            fs::create_dir(target.clone())?;

            // Write all files into the directory.
            let (initial, canonicalized, _type_inferenced) = generate_asts(path, text)?;

            target.push("initial_ast.json");
            fs::write(target.clone(), initial)?;
            target.pop();

            target.push("canonicalization_ast.json");
            fs::write(target.clone(), canonicalized)?;
        }
    }

    Ok(())
}

/// Do what Compiler does - prepare 3 stages of AST: initial, canonicalized and type_inferenced
fn generate_asts(path: &String, text: &String) -> Result<(String, String, String), Box<dyn Error>> {
    let mut ast = leo_parser::parse_ast(path, text)?;
    let initial = ast.to_json_string()?;

    ast.canonicalize()?;
    let canonicalized = ast.to_json_string()?;

    let asg = Asg::new(
        thread_leaked_context(),
        &ast,
        &mut ImportParser::new(PathBuf::from(path)),
    )?;

    let type_inferenced = TypeInferencePhase::default()
        .phase_ast(&ast.into_repr(), &asg.clone().into_repr())
        .expect("Failed to produce type inference ast.")
        .to_json_string()?;

    Ok((initial, canonicalized, type_inferenced))
}
