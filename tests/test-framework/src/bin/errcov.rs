// // Copyright (C) 2019-2023 Aleo Systems Inc.
// // This file is part of the Leo library.

// // The Leo library is free software: you can redistribute it and/or modify
// // it under the terms of the GNU General Public License as published by
// // the Free Software Foundation, either version 3 of the License, or
// // (at your option) any later version.

// // The Leo library is distributed in the hope that it will be useful,
// // but WITHOUT ANY WARRANTY; without even the implied warranty of
// // MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// // GNU General Public License for more details.

// // You should have received a copy of the GNU General Public License
// // along with the Leo library. If not, see <https://www.gnu.org/licenses/>.

// use leo_errors::{AstError, InputError, LeoMessageCode, PackageError, ParserError};
// use leo_test_framework::{
//     fetch::find_tests,
//     output::TestExpectation,
//     test::{extract_test_config, TestExpectationMode as Expectation},
// };

// use regex::Regex;
// use serde_yaml::Value;
// use std::collections::{BTreeMap, HashSet};
// use std::{error::Error, fs, io, path::PathBuf};
// use clap::{clap::AppSettings, Parser};

// #[derive(Parser)]
// #[clap(name = "error-coverage", author = "The Aleo Team <hello@aleo.org>", setting = AppSettings::ColoredHelp)]
// struct Opt {
//     #[clap(
//         short,
//         long,
//         help = "Path to the output file, defaults to stdout.",
//         parse(from_os_str)
//     )]
//     output: Option<PathBuf>,
// }

// fn main() {
//     handle_error(run_with_args(Opt::from_args()));
// }

// fn run_with_args(opt: Opt) -> Result<(), Box<dyn Error>> {
//     // Variable that stores all the tests.
//     let mut tests = Vec::new();
//     let mut test_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
//     test_dir.push("../");

//     let mut expectation_dir = test_dir.clone();
//     expectation_dir.push("expectations");

//     find_tests(&test_dir, &mut tests);

//     // Store all covered error codes
//     let mut found_codes = BTreeMap::new();
//     let re = Regex::new(r"Error \[(?P<code>.*)\]:.*").unwrap();

//     for (path, content) in tests.into_iter() {
//         if let Some(config) = extract_test_config(&content) {
//             // Skip passing tests.
//             if config.expectation == Expectation::Pass {
//                 continue;
//             }

//             let mut expectation_path = expectation_dir.clone();

//             let path = PathBuf::from(path);
//             let relative_path = path.strip_prefix(&test_dir).expect("path error for test");

//             let mut relative_expectation_path = relative_path.to_str().unwrap().to_string();
//             relative_expectation_path += ".out";

//             // Add expectation category
//             if relative_expectation_path.starts_with("compiler") {
//                 expectation_path.push("compiler");
//             } else {
//                 expectation_path.push("parser");
//             }
//             expectation_path.push(&relative_expectation_path);

//             if expectation_path.exists() {
//                 let raw = std::fs::read_to_string(&expectation_path).expect("failed to read expectations file");
//                 let expectation: TestExpectation =
//                     serde_yaml::from_str(&raw).expect("invalid yaml in expectations file");

//                 for value in expectation.outputs {
//                     if let serde_yaml::Value::String(message) = value {
//                         if let Some(caps) = re.captures(&message) {
//                             if let Some(code) = caps.name("code") {
//                                 let files = found_codes
//                                     .entry(code.as_str().to_string())
//                                     .or_insert_with(HashSet::new);
//                                 let path = expectation_path
//                                     .strip_prefix(test_dir.clone())
//                                     .expect("invalid prefix for expectation path");
//                                 files.insert(PathBuf::from(path));
//                             }
//                         }
//                     }
//                 }
//             }
//         }
//     }

//     // Collect all defined error codes.
//     let mut all_codes = HashSet::new();
//     collect_error_codes(
//         &mut all_codes,
//         AstError::message_type(),
//         AstError::code_identifier(),
//         AstError::code_mask(),
//         AstError::num_exit_codes(),
//     );
//     collect_error_codes(
//         &mut all_codes,
//         InputError::message_type(),
//         InputError::code_identifier(),
//         InputError::code_mask(),
//         InputError::num_exit_codes(),
//     );
//     collect_error_codes(
//         &mut all_codes,
//         PackageError::message_type(),
//         PackageError::code_identifier(),
//         PackageError::code_mask(),
//         PackageError::num_exit_codes(),
//     );
//     collect_error_codes(
//         &mut all_codes,
//         ParserError::message_type(),
//         ParserError::code_identifier(),
//         ParserError::code_mask(),
//         ParserError::num_exit_codes(),
//     );

//     // Repackage data into values compatible with serde_yaml
//     let mut covered_errors = serde_yaml::Mapping::new();
//     let mut unknown_errors = serde_yaml::Mapping::new();

//     for (code, paths) in found_codes.iter() {
//         let mut yaml_paths = Vec::with_capacity(paths.len());
//         for path in paths {
//             yaml_paths.push(path.to_str().unwrap());
//         }
//         yaml_paths.sort_unstable();
//         let yaml_paths = yaml_paths.iter().map(|s| Value::String(s.to_string())).collect();

//         if all_codes.contains(code) {
//             covered_errors.insert(Value::String(code.to_owned()), Value::Sequence(yaml_paths));
//         } else {
//             unknown_errors.insert(Value::String(code.to_owned()), Value::Sequence(yaml_paths));
//         }
//         all_codes.remove(code);
//     }

//     let mut codes: Vec<String> = all_codes.drain().collect();
//     codes.sort();

//     let mut uncovered_errors = Vec::new();
//     for code in codes {
//         uncovered_errors.push(Value::String(code))
//     }

//     let mut uncovered_information = serde_yaml::Mapping::new();
//     uncovered_information.insert(
//         Value::String(String::from("count")),
//         Value::Number(serde_yaml::Number::from(uncovered_errors.len())),
//     );
//     uncovered_information.insert(Value::String(String::from("codes")), Value::Sequence(uncovered_errors));

//     let mut covered_information = serde_yaml::Mapping::new();
//     covered_information.insert(
//         Value::String(String::from("count")),
//         Value::Number(serde_yaml::Number::from(covered_errors.len())),
//     );
//     covered_information.insert(Value::String(String::from("codes")), Value::Mapping(covered_errors));

//     let mut unknown_information = serde_yaml::Mapping::new();
//     unknown_information.insert(
//         Value::String(String::from("count")),
//         Value::Number(serde_yaml::Number::from(unknown_errors.len())),
//     );
//     unknown_information.insert(Value::String(String::from("codes")), Value::Mapping(unknown_errors));

//     let mut results = serde_yaml::Mapping::new();
//     results.insert(
//         Value::String(String::from("uncovered")),
//         Value::Mapping(uncovered_information),
//     );

//     results.insert(
//         Value::String(String::from("covered")),
//         Value::Mapping(covered_information),
//     );
//     results.insert(
//         Value::String(String::from("unknown")),
//         Value::Mapping(unknown_information),
//     );

//     // Output error coverage results
//     if let Some(pathbuf) = opt.output {
//         let file = fs::File::create(pathbuf).expect("error creating output file");
//         serde_yaml::to_writer(file, &results).expect("serialization failed for error coverage report");
//     } else {
//         serde_yaml::to_writer(io::stdout(), &results).expect("serialization failed for error coverage report");
//     }

//     Ok(())
// }

// fn collect_error_codes(
//     codes: &mut HashSet<String>,
//     error_type: String,
//     code_identifier: i8,
//     exit_code_mask: i32,
//     num_exit_codes: i32,
// ) {
//     for exit_code in 0..num_exit_codes {
//         codes.insert(format!(
//             "E{}{:0>3}{:0>4}",
//             error_type,
//             code_identifier,
//             exit_code_mask + exit_code,
//         ));
//     }
// }

// fn handle_error(res: Result<(), Box<dyn Error>>) {
//     match res {
//         Ok(_) => (),
//         Err(err) => {
//             eprintln!("Error: {}", err);
//             std::process::exit(1);
//         }
//     }
// }

fn main() {}
