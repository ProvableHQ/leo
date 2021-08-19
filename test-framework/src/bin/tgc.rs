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

use leo_asg::Asg;
use leo_compiler::{compiler::thread_leaked_context, TypeInferencePhase};
use leo_imports::ImportParser;
use leo_test_framework::{
    fetch::find_tests,
    test::{extract_test_config, TestExpectationMode as Expectation},
};

use std::{error::Error, fs, path::PathBuf};
use structopt::{clap::AppSettings, StructOpt};

#[derive(StructOpt)]
#[structopt(name = "ast-stages-generator", author = "The Aleo Team <hello@aleo.org>", setting = AppSettings::ColoredHelp)]
struct Opt {
    #[structopt(
        short,
        long,
        help = "Path to the output folder (auto generated)",
        default_value = "tmp/tgc"
    )]
    path: PathBuf,

    #[structopt(short, long, help = "Run only for test that match pattern")]
    filter: Option<String>,

    #[structopt(short, long, help = "Skip tests matching pattern")]
    skip: Option<String>,
}

fn main() {
    handle_error(run_with_args(Opt::from_args()));
}

fn run_with_args(opt: Opt) -> Result<(), Box<dyn Error>> {
    // Variable that stores all the tests.
    let mut tests = Vec::new();
    let mut test_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    test_dir.push("../tests/");
    find_tests(&test_dir, &mut tests);

    if !opt.path.exists() {
        fs::create_dir_all(&opt.path)?;
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

            // Filter out the tests that do not match pattern, if pattern is set.
            if let Some(filter) = &opt.filter {
                if !test_name.contains(filter) {
                    continue;
                }
            }

            // If skip flag is used, don't run tests matching the pattern.
            if let Some(skip_name) = &opt.skip {
                if test_name.contains(skip_name) {
                    continue;
                }
            }

            test_name.push_str(&format!("_{}", index));

            // Create directory for this file.
            let mut target = PathBuf::from("tmp/tgc");
            target.push(test_name);

            if !target.exists() {
                fs::create_dir_all(target.clone())?;
            }

            let cwd = config
                .extra
                .get("cwd")
                .map(|val| {
                    let mut cwd = PathBuf::from(path);
                    cwd.pop();
                    cwd.join(&val.as_str().unwrap())
                })
                .unwrap_or(PathBuf::from(path));

            // Write all files into the directory.
            let (initial, canonicalized, type_inferenced) = generate_asts(cwd, text)?;

            target.push("initial_ast.json");
            fs::write(target.clone(), initial)?;
            target.pop();

            target.push("canonicalization_ast.json");
            fs::write(target.clone(), canonicalized)?;
            target.pop();

            target.push("type_inferenced_ast.json");
            fs::write(target.clone(), type_inferenced)?;
        }
    }

    Ok(())
}

/// Do what Compiler does - prepare 3 stages of AST: initial, canonicalized and type_inferenced
fn generate_asts(path: PathBuf, text: &String) -> Result<(String, String, String), Box<dyn Error>> {
    let mut ast = leo_parser::parse_ast(path.clone().into_os_string().into_string().unwrap(), text)?;
    let initial = ast.to_json_string()?;

    ast.canonicalize()?;
    let canonicalized = ast.to_json_string()?;

    let asg = Asg::new(thread_leaked_context(), &ast, &mut ImportParser::new(path))?;

    let type_inferenced = TypeInferencePhase::default()
        .phase_ast(&ast.into_repr(), &asg.clone().into_repr())
        .expect("Failed to produce type inference ast.")
        .to_json_string()?;

    Ok((initial, canonicalized, type_inferenced))
}

fn handle_error(res: Result<(), Box<dyn Error>>) {
    match res {
        Ok(_) => (),
        Err(err) => {
            eprintln!("Error: {}", err);
            std::process::exit(1);
        }
    }
}
