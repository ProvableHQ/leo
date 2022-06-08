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

use walkdir::WalkDir;

pub fn find_tests<T: AsRef<Path> + Copy>(path: T, filter: T) -> Vec<(String, String)> {
    let count = WalkDir::new(path)
        .into_iter()
        .flatten()
        .filter_map(|f| {
            let path = f.path();
            if matches!(path.extension(), Some(s) if s == "leo") && !path.starts_with(filter) {
                let content = fs::read_to_string(path).expect("failed to read test");
                Some((path.to_str().unwrap_or_default().to_string(), content))
            } else {
                None
            }
        })
        .collect::<Vec<(String, String)>>();
    dbg!("find_tests count {}", count.len());
    count
}

pub fn split_tests_one_line(source: &str) -> Vec<&str> {
    source.lines().map(|x| x.trim()).filter(|x| !x.is_empty()).collect()
}

pub fn split_tests_two_line(source: &str) -> Vec<String> {
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
