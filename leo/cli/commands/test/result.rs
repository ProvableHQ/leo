// Copyright (C) 2019-2024 Aleo Systems Inc.
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

/// The results of running a test program.
pub struct TestResults {
    /// The name of the test program.
    program_name: String,
    /// The ran test functions and the associated result.
    results: Vec<(String, String)>,
    /// The skipped tests.
    skipped: Vec<String>,
}

impl TestResults {
    /// Initialize a new `TestResults` object.
    pub fn new(program_name: String) -> Self {
        Self { program_name, results: Default::default(), skipped: Default::default() }
    }

    /// Add a function name and result to the results.
    pub fn add_result(&mut self, function: String, result: String) {
        self.results.push((function, result))
    }

    /// Add a skipped test to the results.
    pub fn skip(&mut self, function: String) {
        self.skipped.push(function)
    }
}

impl std::fmt::Display for TestResults {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Test results for program '{}':", self.program_name)?;
        for (function, result) in &self.results {
            writeln!(f, "   Ran '{function}' | {result}")?;
        }
        for function in &self.skipped {
            writeln!(f, "   Skipped '{function}'")?;
        }
        Ok(())
    }
}
