// Copyright (C) 2019-2025 Provable Inc.
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

use super::visitor::StaticAnalyzingVisitor;

use leo_ast::{
    Constructor,
    Node,
    NodeBuilder,
    leo_admin_constructor,
    leo_checksum_constructor,
    leo_noupgrade_constructor,
};
use leo_errors::{BufferEmitter, Handler, StaticAnalyzerError};
use leo_package::{MappingTarget, UpgradeConfig};

use snarkvm::prelude::Network;

use std::fmt::Display;

impl<N: Network> StaticAnalyzingVisitor<'_, N> {
    /// Checks that the declared constructor matches the configuration.
    pub fn check_constructor_matches(&self, constructor: &Constructor) {
        if let Some(config) = &self.state.upgrade_config {
            match config {
                UpgradeConfig::Admin { address } => self.check_admin_constructor(constructor, address),
                UpgradeConfig::Checksum { mapping, key } => self.check_checksum_constructor(constructor, mapping, key),
                UpgradeConfig::Custom => self.check_custom_constructor(constructor),
                UpgradeConfig::NoUpgrade => self.check_noupgrade_constructor(constructor),
            }
        }
    }

    // Checks that an `Admin` constructor is well formed.
    fn check_admin_constructor(&self, constructor: &Constructor, address: &str) {
        // Construct the expected constructor.
        let expected = parse_constructor::<N>(&leo_admin_constructor(address));
        // Check that the expected constructor matches the given constructor.
        if constructor != &expected {
            self.state.handler.emit_err(StaticAnalyzerError::custom_error(
                constructor_error_message("admin", expected),
                constructor.span(),
            ));
        }
    }

    // Checks that a `Checksum` constructor is well formed.
    fn check_checksum_constructor(&self, constructor: &Constructor, mapping: &MappingTarget, key: &str) {
        // Construct the expected constructor.
        let expected = parse_constructor::<N>(&leo_checksum_constructor(mapping, key));
        // Check that the expected constructor matches the given constructor.
        if constructor != &expected {
            self.state.handler.emit_err(StaticAnalyzerError::custom_error(
                constructor_error_message("checksum", expected),
                constructor.span(),
            ));
        }
    }

    // Checks that a `Custom` constructor is well formed.
    fn check_custom_constructor(&self, _constructor: &Constructor) {
        // No checks are performed for custom constructors for now.
        // In the future, we may want to run more powerful static analyses to rule out unsafe constructors.
    }

    // Checks that a `NoUpgrade` constructor is well formed.
    fn check_noupgrade_constructor(&self, constructor: &Constructor) {
        // Construct the expected constructor.
        let expected = parse_constructor::<N>(&leo_noupgrade_constructor());
        // Check that the expected constructor matches the given constructor.
        if constructor != &expected {
            self.state.handler.emit_err(StaticAnalyzerError::custom_error(
                constructor_error_message("no upgrade", expected),
                constructor.span(),
            ));
        }
    }
}

// A helper function to parse a constructor string into a Leo constructor.
fn parse_constructor<N: Network>(constructor_string: &str) -> Constructor {
    // Initialize a new handler.
    let handler = Handler::new(BufferEmitter::new());
    // Initialize a node builder.
    let node_builder = NodeBuilder::new(0);
    // Parse the constructor string.
    leo_parser::parse_constructor::<N>(handler, &node_builder, constructor_string, 0)
        .expect("The default constructor should be well-formed")
}

// A helper function to provide an error message if the constructor is not well formed.
fn constructor_error_message(mode: impl Display, expected: impl Display) -> String {
    format!(
        r"
The constructor is not well formed for the '{mode}' upgrade configuration.

The expected constructor is:
```
{expected}
```

For more information, please refer to the documentation: TODO
    "
    )
}
