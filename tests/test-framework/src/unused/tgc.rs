// Copyright (C) 2019-2023 Aleo Systems Inc.
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
use leo_ast::AstPass;
use leo_compiler::{compiler::thread_leaked_context, TypeInferencePhase};
use leo_imports::ImportParser;
use leo_test_framework::{
    fetch::find_tests,
    test::{extract_test_config, TestExpectationMode as Expectation},
};

use std::{error::Error, fs, path::PathBuf};
use structopt::{clap::AppSettings, Parser};

#[derive(Parser)]
#[clap(name = "ast-stages-generator", author = "The Aleo Team <hello@aleo.org>", setting = AppSettings::ColoredHelp)]
struct Opt {
    #[clap(short, long, help = "Path to the output folder (auto generated)", default_value = "tmp/tgc")]
    path: PathBuf,

    #[clap(short, long, help = "Run only for test that match pattern")]
    filter: Option<String>,

    #[clap(short, long, help = "Skip tests matching pattern")]
    skip: Option<Vec<String>>,
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
    'main_loop: for (index, (path, text)) in tests.iter().enumerate() {
        if let Some(config) = extract_test_config(text) {
            // Skip namespaces that we don't need; also skip failure tests.
            if config.namespace != "Compile" || config.expectation == Expectation::Fail {
                continue;
            }

            let mut test_name = path.split("tests/").last().unwrap().replace(std::path::MAIN_SEPARATOR, "_");

            // Filter out the tests that do not match pattern, if pattern is set.
            if let Some(filter) = &opt.filter {
                if !test_name.contains(filter) {
                    continue;
                }
            }

            // If skip flag is used, don't run tests matching the pattern.
            if let Some(skips) = &opt.skip {
                for skip_pattern in skips {
                    if test_name.contains(skip_pattern) {
                        println!("Skipping: {} because of {}", test_name, skip_pattern);
                        continue 'main_loop;
                    }
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
                .unwrap_or_else(|| PathBuf::from(path));

            let end_of_header = text.find("*/").expect("failed to find header block in test");
            // Do this to match test-framework ast bc of spans
            let text = &text[end_of_header + 2..];
            // Write all files into the directory.
            generate_asts(cwd, target, text)?;
        }
    }

    Ok(())
}

/// Do what Compiler does - prepare 3 stages of AST: initial, imports_resolved, canonicalized and type_inferenced
/// Write these ASTs without Spans to JSON
fn generate_asts(src_path: PathBuf, target_path: PathBuf, text: &str) -> Result<(), Box<dyn Error>> {
    std::env::set_var("LEO_TESTFRAMEWORK", "true");

    let mut ast = leo_parser::parse_ast(src_path.clone().into_os_string().into_string().unwrap(), text)?;

    ast.to_json_file_without_keys(target_path.clone(), "initial_ast.json", &["span"])?;

    ast = leo_ast_passes::Importer::do_pass(ast.into_repr(), &mut ImportParser::new(src_path, Default::default()))?;

    ast.to_json_file_without_keys(target_path.clone(), "imports_resolved_ast.json", &["span"])?;

    ast = leo_ast_passes::Canonicalizer::do_pass(ast.into_repr())?;

    ast.to_json_file_without_keys(target_path.clone(), "canonicalization_ast.json", &["span"])?;

    let mut ti_ast = ast.into_repr();
    ti_ast.name = "test".to_string(); // Do this to match test-framework ast

    let asg = Asg::new(thread_leaked_context(), &ti_ast)?;

    let type_inferenced = TypeInferencePhase::default()
        .phase_ast(&ti_ast, &asg.clone().into_repr())
        .expect("Failed to produce type inference ast.");

    type_inferenced.to_json_file_without_keys(target_path, "type_inferenced_ast.json", &["span"])?;

    std::env::remove_var("LEO_TESTFRAMEWORK");

    Ok(())
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
