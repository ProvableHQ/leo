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

use leo_errors::{
    AsgError, AstError, CliError, CompilerError, ImportError, LeoErrorCode, PackageError, ParserError, StateError,
};
use leo_test_framework::{
    fetch::find_tests,
    output::TestExpectation,
    test::{extract_test_config, TestExpectationMode as Expectation},
};

use regex::Regex;
use serde_yaml::Value;
use std::collections::{BTreeMap, HashSet};
use std::{
    error::Error,
    fs,
    io::Write,
    path::{Path, PathBuf},
};
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
    let mut found_codes = BTreeMap::new();
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
                                let files = found_codes.entry(code.as_str().to_string()).or_insert(HashSet::new());
                                let path = expectation_path
                                    .strip_prefix(test_dir.clone())
                                    .expect("invalid prefix for expectation path");
                                files.insert(PathBuf::from(path));
                            }
                        }
                    }
                }
            }
        }
    }

    // Collect all defined error codes.
    let mut all_codes = HashSet::new();
    collect_error_codes(
        &mut all_codes,
        AsgError::error_type(),
        AsgError::code_identifier(),
        AsgError::exit_code_mask(),
        AsgError::num_exit_codes(),
    );
    collect_error_codes(
        &mut all_codes,
        AstError::error_type(),
        AstError::code_identifier(),
        AstError::exit_code_mask(),
        AstError::num_exit_codes(),
    );
    collect_error_codes(
        &mut all_codes,
        CliError::error_type(),
        CliError::code_identifier(),
        CliError::exit_code_mask(),
        CliError::num_exit_codes(),
    );
    collect_error_codes(
        &mut all_codes,
        CompilerError::error_type(),
        CompilerError::code_identifier(),
        CompilerError::exit_code_mask(),
        CompilerError::num_exit_codes(),
    );
    collect_error_codes(
        &mut all_codes,
        ImportError::error_type(),
        ImportError::code_identifier(),
        ImportError::exit_code_mask(),
        ImportError::num_exit_codes(),
    );
    collect_error_codes(
        &mut all_codes,
        PackageError::error_type(),
        PackageError::code_identifier(),
        PackageError::exit_code_mask(),
        PackageError::num_exit_codes(),
    );
    collect_error_codes(
        &mut all_codes,
        ParserError::error_type(),
        ParserError::code_identifier(),
        ParserError::exit_code_mask(),
        ParserError::num_exit_codes(),
    );
    collect_error_codes(
        &mut all_codes,
        StateError::error_type(),
        StateError::code_identifier(),
        StateError::exit_code_mask(),
        StateError::num_exit_codes(),
    );

    let mut covered_errors = serde_yaml::Mapping::new();
    let mut unknown_errors = serde_yaml::Mapping::new();

    for (code, paths) in found_codes.iter() {
        let mut yaml_paths = Vec::new();
        for path in paths {
            yaml_paths.push(path.to_str().unwrap());
        }
        yaml_paths.sort();
        let yaml_paths = yaml_paths.iter().map(|s| Value::String(s.to_string())).collect();

        if all_codes.contains(code) {
            covered_errors.insert(Value::String(code.to_owned()), Value::Sequence(yaml_paths));
            all_codes.remove(code);
        } else {
            unknown_errors.insert(Value::String(code.to_owned()), Value::Sequence(yaml_paths));
        }
    }

    let mut codes: Vec<String> = all_codes.drain().collect();
    codes.sort();

    let mut uncovered_errors = Vec::new();
    for code in codes {
        uncovered_errors.push(Value::String(code))
    }

    let mut results = serde_yaml::Mapping::new();
    results.insert(
        Value::String(String::from("uncovered")),
        Value::Sequence(uncovered_errors),
    );
    results.insert(Value::String(String::from("covered")), Value::Mapping(covered_errors));
    results.insert(Value::String(String::from("unknown")), Value::Mapping(unknown_errors));
    //let results_str = serde_yaml::to_string(&results).expect("serialization failed for error coverage report");

    let mut file = fs::File::create(opt.output)?;
    serde_yaml::to_writer(file, &results).expect("serialization failed for error coverage report");

    Ok(())
}

fn collect_error_codes(
    codes: &mut HashSet<String>,
    error_type: String,
    code_identifier: i8,
    exit_code_mask: i32,
    num_exit_codes: i32,
) {
    for exit_code in 0..num_exit_codes {
        codes.insert(format!(
            "E{}{:0>3}{:0>4}",
            error_type,
            code_identifier,
            exit_code_mask + exit_code,
        ));
    }
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
