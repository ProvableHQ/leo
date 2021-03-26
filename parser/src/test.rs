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

use std::{
    fs,
    path::{Path, PathBuf},
};

use crate::SyntaxError;

struct TestFailure {
    path: String,
    error: SyntaxError,
}

pub fn find_tests<T: AsRef<Path>>(path: T, out: &mut Vec<(String, String)>) {
    for entry in fs::read_dir(path).expect("fail to read tests").into_iter() {
        let entry = entry.expect("fail to read tests").path();
        if entry.is_dir() {
            find_tests(entry.as_path(), out);
            continue;
        } else if entry.extension().map(|x| x.to_str()).flatten().unwrap_or_default() != "leo" {
            continue;
        }
        let content = fs::read_to_string(entry.as_path()).expect("failed to read test");
        out.push((entry.as_path().to_str().unwrap_or_default().to_string(), content));
    }
}

#[test]
pub fn parser_pass_tests() {
    let mut pass = 0;
    let mut fail = Vec::new();
    let mut tests = Vec::new();
    let mut test_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    test_dir.push("../tests/pass/parse/");
    find_tests(&test_dir, &mut tests);
    for (path, content) in tests.into_iter() {
        match crate::parse(&path, &content) {
            Ok(_) => {
                pass += 1;
            }
            Err(e) => {
                fail.push(TestFailure { path, error: e });
            }
        }
    }
    if !fail.is_empty() {
        for (i, fail) in fail.iter().enumerate() {
            println!(
                "\n\n-----------------TEST #{} FAILED (and shouldn't have)-----------------",
                i + 1
            );
            println!("File: {}", fail.path);
            println!("{}", fail.error);
        }
        panic!("failed {}/{} tests", fail.len(), fail.len() + pass);
    } else {
        println!("passed {}/{} tests", pass, pass);
    }
}

#[test]
pub fn parser_fail_tests() {
    let mut pass = 0;
    let mut fail = Vec::new();
    let mut tests = Vec::new();
    let mut test_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    test_dir.push("../tests/fail/parse/");
    find_tests(&test_dir, &mut tests);
    for (path, content) in tests.into_iter() {
        match crate::parse(&path, &content) {
            Ok(_) => {
                fail.push(path);
            }
            Err(_e) => {
                pass += 1;
            }
        }
    }
    if !fail.is_empty() {
        for (i, fail) in fail.iter().enumerate() {
            println!(
                "\n\n-----------------TEST #{} PASSED (and shouldn't have)-----------------",
                i + 1
            );
            println!("File: {}", fail);
        }
        panic!("failed {}/{} tests", fail.len(), fail.len() + pass);
    } else {
        println!("passed {}/{} tests", pass, pass);
    }
}
