// Copyright (C) 2019-2026 Provable Inc.
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

//! This is a simple test framework for the Leo compiler.

#[cfg(not(feature = "no_parallel"))]
use rayon::prelude::*;

use std::{fs, path::PathBuf};
use walkdir::WalkDir;

enum TestFailure {
    Panicked(String),
    Mismatch { got: String, expected: String },
}

/// Pulls tests from `category`, running them through the `runner` and
/// comparing them against expectations in previous runs.
///
/// The tests are `.leo` files in `tests/{category}`, and the
/// runner receives the contents of each of them as a `&str`,
/// returning a `String` result. A test is considered to have failed
/// if it panics or if results differ from the previous run.
///
///
/// If no corresponding `.out` file is found in `expecations/{category}`,
/// or if the environment variable `REWRITE_EXPECTATIONS` is set, no
/// comparison to a previous result is done and the result of the current
/// run is written to the file.
pub fn run_tests(category: &str, runner: fn(&str) -> String) {
    // This ensures error output doesn't try to display colors.
    unsafe {
        // SAFETY: Safety issues around `set_var` are surprisingly complicated.
        // For now, I think marking tests as `serial` may be sufficient to
        // address this, and in the future we'll try to think of an alternative for
        // error output.
        std::env::set_var("NOCOLOR", "x");
    }

    let base_tests_dir: PathBuf = [env!("CARGO_MANIFEST_DIR"), "..", "tests"].iter().collect();

    let base_tests_dir = base_tests_dir.canonicalize().unwrap();
    let tests_dir = base_tests_dir.join("tests").join(category);
    let expectations_dir = base_tests_dir.join("expectations").join(category);

    let filter_string = std::env::var("TEST_FILTER").unwrap_or_default();
    let rewrite_expectations = std::env::var("REWRITE_EXPECTATIONS").is_ok();

    struct TestResult {
        failure: Option<TestFailure>,
        name: PathBuf,
        wrote: bool,
    }

    let paths: Vec<PathBuf> = WalkDir::new(&tests_dir)
        .into_iter()
        .flatten()
        .filter_map(|entry| {
            let path = entry.path();

            if path.to_str().is_none() {
                panic!("Path not unicode: {}.", path.display());
            };

            let path_str = path.to_str().unwrap();

            if !path_str.contains(&filter_string) || !path_str.ends_with(".leo") {
                return None;
            }

            Some(path.into())
        })
        .collect();

    let run_test = |path: &PathBuf| -> TestResult {
        let contents =
            fs::read_to_string(path).unwrap_or_else(|e| panic!("Failed to read file {}: {e}.", path.display()));
        let result_output = std::panic::catch_unwind(|| runner(&contents));
        if let Err(payload) = result_output {
            let s1 = payload.downcast_ref::<&str>().map(|s| s.to_string());
            let s2 = payload.downcast_ref::<String>().cloned();
            let s = s1.or(s2).unwrap_or_else(|| "Unknown panic payload".to_string());

            return TestResult { failure: Some(TestFailure::Panicked(s)), name: path.clone(), wrote: false };
        }
        let output = result_output.unwrap();

        let mut expectation_path: PathBuf = expectations_dir.join(path.strip_prefix(&tests_dir).unwrap());
        expectation_path.set_extension("out");

        // It may not be ideal to the the IO below in parallel, but I'm thinking it likely won't matter.
        if rewrite_expectations || !expectation_path.exists() {
            fs::write(&expectation_path, &output)
                .unwrap_or_else(|e| panic!("Failed to write file {}: {e}.", expectation_path.display()));
            TestResult { failure: None, name: path.clone(), wrote: true }
        } else {
            let expected = fs::read_to_string(&expectation_path)
                .unwrap_or_else(|e| panic!("Failed to read file {}: {e}.", expectation_path.display()));
            if output == expected {
                TestResult { failure: None, name: path.clone(), wrote: false }
            } else {
                TestResult {
                    failure: Some(TestFailure::Mismatch { got: output, expected }),
                    name: path.clone(),
                    wrote: false,
                }
            }
        }
    };

    #[cfg(feature = "no_parallel")]
    let results: Vec<TestResult> = paths.iter().map(run_test).collect();

    #[cfg(not(feature = "no_parallel"))]
    let results: Vec<TestResult> = paths.par_iter().map(run_test).collect();

    println!("Ran {} tests.", results.len());

    let failure_count = results.iter().filter(|test_result| test_result.failure.is_some()).count();

    if failure_count != 0 {
        eprintln!("{failure_count}/{} tests failed.", results.len());
    }

    let writes = results.iter().filter(|test_result| test_result.wrote).count();

    for test_result in results.iter() {
        if let Some(test_failure) = &test_result.failure {
            eprintln!("FAILURE: {}:", test_result.name.display());
            match test_failure {
                TestFailure::Panicked(s) => eprintln!("Rust panic:\n{s}"),
                TestFailure::Mismatch { got, expected } => {
                    eprintln!("\ngot:\n{got}\nexpected:\n{expected}\n")
                }
            }
        }
    }

    if writes != 0 {
        println!("Wrote {}/{} expectation files for tests:", writes, results.len());
    }

    for test_result in results.iter() {
        if test_result.wrote {
            println!("{}", test_result.name.display());
        }
    }

    assert!(failure_count == 0);
}
