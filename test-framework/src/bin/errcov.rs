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
use leo_ast::AstPass;
use leo_compiler::{compiler::thread_leaked_context, TypeInferencePhase};
use leo_imports::ImportParser;
use leo_test_framework::{
    fetch::find_tests,
    output::TestExpectation,
    test::{extract_test_config, TestExpectationMode as Expectation},
};

use regex::Regex;
use std::{collections::HashSet, error::Error, fs, path::PathBuf};
use structopt::{clap::AppSettings, StructOpt};

#[derive(StructOpt)]
#[structopt(name = "error-coverage", author = "The Aleo Team <hello@aleo.org>", setting = AppSettings::ColoredHelp)]
struct Opt {
    #[structopt(short, long, help = "Path to the output file", parse(from_os_str))]
    output: PathBuf,
}

fn main() {
    handle_error(run_with_args(Opt::from_args()));
}

fn run_with_args(opt: Opt) -> Result<(), Box<dyn Error>> {
    // Variable that stores all the tests.
    let mut tests = Vec::new();
    let mut test_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    test_dir.push("../tests/");

    let mut expectation_dir = test_dir.clone();
    expectation_dir.push("expectations");

    find_tests(&test_dir, &mut tests);

    // Store all covered error codes
    let mut codes = HashSet::new();
    let re = Regex::new(r"Error \[(?P<code>.*)\]:.*").unwrap();

    for (path, content) in tests.into_iter() {
        if let Some(config) = extract_test_config(&content) {
            // Skip passing tests.
            if config.expectation == Expectation::Pass {
                continue;
            }

            let mut expectation_path = expectation_dir.clone();

            let path = PathBuf::from(path);
            let relative_path = path.strip_prefix(&test_dir).expect("path error for test");

            let mut relative_expectation_path = relative_path.to_str().unwrap().to_string();
            relative_expectation_path += ".out";

            // Add expectation category
            if relative_expectation_path.starts_with("compiler") {
                expectation_path.push("compiler");
            } else {
                expectation_path.push("parser");
            }
            expectation_path.push(&relative_expectation_path);

            if expectation_path.exists() {
                let raw = std::fs::read_to_string(&expectation_path).expect("failed to read expectations file");
                let expectation: TestExpectation =
                    serde_yaml::from_str(&raw).expect("invalid yaml in expectations file");

                for value in expectation.outputs {
                    if let serde_yaml::Value::String(message) = value {
                        if let Some(caps) = re.captures(&message) {
                            if let Some(code) = caps.name("code") {
                                codes.insert(code.as_str().to_string());
                            }
                        }
                    }
                }
            }
        }
    }

    let mut sorted: Vec<_> = codes.iter().collect();
    sorted.sort();

    println!("Found the following error codes");
    for code in sorted {
        println!("{}", code)
    }

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
