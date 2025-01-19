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

use super::*;
use snarkvm::prelude::{Itertools, PrivateKey};
use std::str::FromStr;

use leo_ast::{ExpressionVisitor, Function, ProgramId, ProgramScope, StatementVisitor, TestManifest, TestMetadata};
use leo_errors::TestError;
use leo_span::{Symbol, sym};

pub struct TestManifestGenerator<'a, N: Network> {
    // The error handler.
    handler: &'a Handler,
    // The manifest we are currently generating.
    pub manifest: Option<TestManifest<N>>,
}

impl<'a, N: Network> TestManifestGenerator<'a, N> {
    /// Initial a new instance of the test manifest generator.
    pub fn new(handler: &'a Handler) -> Self {
        Self { handler, manifest: None }
    }

    /// Initialize the manifest.
    pub fn initialize_manifest(&mut self, program_id: &ProgramId) {
        self.manifest = Some(TestManifest::new(program_id));
    }
}

impl<'a, N: Network> ProgramVisitor<'a> for TestManifestGenerator<'a, N> {
    fn visit_program_scope(&mut self, input: &'a ProgramScope) {
        // Initialize a new manifest.
        self.initialize_manifest(&input.program_id);
        // Visit the functions in the program scope.
        input.functions.iter().for_each(|(_, c)| (self.visit_function(c)));
    }

    fn visit_function(&mut self, input: &'a Function) {
        // Find all of the test annotations.
        let test_annotations = input.annotations.iter().filter(|a| a.identifier.name == sym::test).collect_vec();

        // Validate the number and usage of test annotations.
        match test_annotations.len() {
            0 => return,
            1 => {
                // Check that the function is a transition.
                if !input.variant.is_transition() {
                    self.handler.emit_err(TestError::non_transition_test(input.span));
                }
            }
            _ => {
                self.handler.emit_err(TestError::multiple_test_annotations(input.span));
                return;
            }
        }

        // Get the test annotation.
        let test_annotation = test_annotations[0];

        // Initialize the private key.
        let mut private_key = None;
        // Initialize the seed.
        let mut seed = None;
        // Initialize the should fail flag.
        let mut should_fail = false;

        // Check the annotation body.
        for (key, value) in test_annotation.data.iter() {
            // Check that the key and associated value is valid.
            if key.name == Symbol::intern("private_key") {
                // Attempt to parse the value as a private key.
                match value {
                    None => self.handler.emit_err(TestError::missing_annotation_value(
                        test_annotation.identifier,
                        key,
                        key.span,
                    )),
                    Some(string) => match PrivateKey::<N>::from_str(string) {
                        Ok(pk) => private_key = Some(pk),
                        Err(err) => self.handler.emit_err(TestError::invalid_annotation_value(
                            test_annotation.identifier,
                            key,
                            string,
                            err,
                            key.span,
                        )),
                    },
                }
            } else if key.name == Symbol::intern("seed") {
                // Attempt to parse the value as a u64.
                match value {
                    None => self.handler.emit_err(TestError::missing_annotation_value(
                        test_annotation.identifier,
                        key,
                        key.span,
                    )),
                    Some(string) => match string.parse::<u64>() {
                        Ok(s) => seed = Some(s),
                        Err(err) => self.handler.emit_err(TestError::invalid_annotation_value(
                            test_annotation.identifier,
                            key,
                            string,
                            err,
                            key.span,
                        )),
                    },
                }
            } else if key.name == Symbol::intern("should_fail") {
                // Check that there is no value associated with the key.
                if let Some(string) = value {
                    self.handler.emit_err(TestError::unexpected_annotation_value(
                        test_annotation.identifier,
                        key,
                        string,
                        key.span,
                    ));
                }
                should_fail = true;
            } else {
                self.handler.emit_err(TestError::unknown_annotation_key(test_annotation.identifier, key, key.span))
            }
        }

        // Add the test to the manifest.
        self.manifest.as_mut().unwrap().add_test(TestMetadata {
            function_name: input.identifier.to_string(),
            private_key,
            seed,
            should_fail,
        });
    }
}

impl<'a, N: Network> StatementVisitor<'a> for TestManifestGenerator<'a, N> {}

impl<'a, N: Network> ExpressionVisitor<'a> for TestManifestGenerator<'a, N> {
    type AdditionalInput = ();
    type Output = ();
}
