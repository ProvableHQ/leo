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

use crate::ProgramId;
use snarkvm::prelude::{Network, PrivateKey};

use serde::{Deserialize, Serialize};

/// A manifest describing the tests to be run and their associated metadata.
#[derive(Debug, Serialize, Deserialize)]
#[serde(bound = "")]
pub struct TestManifest<N: Network> {
    /// The program ID.
    pub program_id: String,
    /// The tests to be run.
    pub tests: Vec<TestMetadata<N>>,
}

impl<N: Network> TestManifest<N> {
    /// Create a new test manifest.
    pub fn new(program_id: &ProgramId) -> Self {
        Self { program_id: program_id.to_string(), tests: Vec::new() }
    }

    /// Add a test to the manifest.
    pub fn add_test(&mut self, test: TestMetadata<N>) {
        self.tests.push(test);
    }
}

/// Metadata associated with a test.
#[derive(Debug, Serialize, Deserialize)]
#[serde(bound = "")]
pub struct TestMetadata<N: Network> {
    /// The name of the function.
    pub function_name: String,
    /// The private key to run the test with.
    pub private_key: Option<PrivateKey<N>>,
    /// The seed for the RNG.
    pub seed: Option<u64>,
    /// Whether or not the test is expected to fail.
    pub should_fail: bool,
}
