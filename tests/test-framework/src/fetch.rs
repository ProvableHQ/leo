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

use std::{fs, path::Path};

pub fn find_tests<T: AsRef<Path>>(path: T, out: &mut Vec<(String, String)>) {
    for entry in fs::read_dir(path).expect("fail to read tests") {
        let entry = entry.expect("fail to read tests").path();
        if entry.is_dir() {
            find_tests(entry.as_path(), out);
            continue;
        } else if entry.extension().and_then(|x| x.to_str()).unwrap_or_default() != "leo" {
            continue;
        }
        let content = fs::read_to_string(entry.as_path()).expect("failed to read test");
        out.push((entry.as_path().to_str().unwrap_or_default().to_string(), content));
    }
}

pub fn split_tests_oneline(source: &str) -> Vec<&str> {
    source.lines().map(|x| x.trim()).filter(|x| !x.is_empty()).collect()
}

pub fn split_tests_twoline(source: &str) -> Vec<String> {
    let mut out = vec![];
    let mut lines = vec![];
    for line in source.lines() {
        let line = line.trim();
        if line.is_empty() {
            if !lines.is_empty() {
                out.push(lines.join("\n"));
            }
            lines.clear();
            continue;
        }
        lines.push(line);
    }
    let last_test = lines.join("\n");
    if !last_test.trim().is_empty() {
        out.push(last_test.trim().to_string());
    }
    out
}
