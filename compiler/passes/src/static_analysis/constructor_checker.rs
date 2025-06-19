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

use crate::Normalizer;

use super::visitor::StaticAnalyzingVisitor;

use leo_ast::{
    ArrayType,
    Constructor,
    Expression,
    IntegerType,
    Location,
    NetworkName,
    Node,
    NodeBuilder,
    ProgramId,
    ProgramReconstructor,
    Type,
    leo_admin_constructor,
    leo_checksum_constructor,
    leo_noupgrade_constructor,
};
use leo_errors::{BufferEmitter, Handler, StaticAnalyzerError};
use leo_package::{MappingTarget, UpgradeConfig};
use leo_span::Symbol;

use snarkvm::prelude::{Address, CanaryV0, Literal, MainnetV0, Network, ProgramID, TestnetV0};

use std::{fmt::Display, str::FromStr};

impl StaticAnalyzingVisitor<'_> {
    /// Checks that the declared constructor matches the configuration.
    pub fn check_constructor_matches(&self, constructor: &Constructor) {
        if let Some(config) = &self.state.upgrade_config {
            match config {
                UpgradeConfig::Admin { address } => self.check_admin_constructor(constructor, address),
                UpgradeConfig::Checksum { mapping, key } => self.check_checksum_constructor(constructor, mapping, key),
                UpgradeConfig::Custom => self.check_custom_constructor(constructor),
                UpgradeConfig::NoUpgrade => self.check_noupgrade_constructor(constructor),
            }
        } else {
            self.state.handler.emit_err(StaticAnalyzerError::custom_error(
                "A constructor was specified in the program but no upgrade configuration was provided in the `program.json`.",
                Option::<String>::None,
                constructor.span(),
            ));
        }
    }

    // Checks that an `Admin` constructor is well-formed.
    fn check_admin_constructor(&self, constructor: &Constructor, address: &str) {
        // Verify that the address is valid.
        if match self.state.network {
            NetworkName::MainnetV0 => Address::<MainnetV0>::from_str(address).is_err(),
            NetworkName::TestnetV0 => Address::<TestnetV0>::from_str(address).is_err(),
            NetworkName::CanaryV0 => Address::<CanaryV0>::from_str(address).is_err(),
        } {
            self.state.handler.emit_err(StaticAnalyzerError::custom_error(
                format!("'{address}' is not a valid address for the current network: {}", self.state.network),
                Option::<String>::None,
                constructor.span(),
            ));
        }
        // Construct the expected constructor.
        let expected = self.parse_constructor(&leo_admin_constructor(address));
        // Check that the expected constructor matches the given constructor.
        if !constructors_match(constructor, &expected) {
            self.state.handler.emit_err(StaticAnalyzerError::custom_error(
                constructor_error_message("admin"),
                Some(constructor_help_message(expected)),
                constructor.span(),
            ));
        }
    }

    // Checks that a `Checksum` constructor is well-formed.
    fn check_checksum_constructor(&self, constructor: &Constructor, mapping: &MappingTarget, key: &str) {
        // Get the location of the mapping.
        let location = match mapping {
            MappingTarget::Local(name) => Location::new(self.current_program, Symbol::intern(name)),
            MappingTarget::External { program_id, name } => {
                // Parse the program ID.
                let result = match self.state.network {
                    NetworkName::MainnetV0 => ProgramID::<MainnetV0>::from_str(program_id).map(|p| (&p).into()),
                    NetworkName::TestnetV0 => ProgramID::<TestnetV0>::from_str(program_id).map(|p| (&p).into()),
                    NetworkName::CanaryV0 => ProgramID::<CanaryV0>::from_str(program_id).map(|p| (&p).into()),
                };
                let program_id: ProgramId = match result {
                    Ok(program_id) => program_id,
                    Err(_) => {
                        self.state.handler.emit_err(StaticAnalyzerError::custom_error(
                            format!("The program ID '{program_id}' is not a valid program ID."),
                            Option::<String>::None,
                            constructor.span(),
                        ));
                        return;
                    }
                };
                Location::new(program_id.name.name, Symbol::intern(name))
            }
        };
        // Get the type of the key used to index the mapping.
        let result = match self.state.network {
            NetworkName::MainnetV0 => Literal::<MainnetV0>::from_str(key).map(|l| get_type_from_snarkvm_literal(&l)),
            NetworkName::TestnetV0 => Literal::<TestnetV0>::from_str(key).map(|l| get_type_from_snarkvm_literal(&l)),
            NetworkName::CanaryV0 => Literal::<CanaryV0>::from_str(key).map(|l| get_type_from_snarkvm_literal(&l)),
        };
        let key_type: Type = match result {
            Ok(type_) => type_,
            Err(_) => {
                self.state.handler.emit_err(StaticAnalyzerError::custom_error(
                    format!("The key '{key}' is not a valid."),
                    Some("Use a valid literal expression for the key. The literal cannot be a struct or array."),
                    constructor.span(),
                ));
                return;
            }
        };
        // Check that the mapping exists, the key type is correct, and the value type is `[u8; 32]`.
        let is_valid = self
            .state
            .symbol_table
            .lookup_global(location)
            .map(|variable| match variable.type_ {
                Type::Mapping(ref mapping_type) => {
                    mapping_type.key.as_ref().eq_flat_relaxed(&key_type)
                        && mapping_type.value.as_ref().eq_flat_relaxed(&Type::Array(ArrayType::new(
                            Type::Integer(IntegerType::U8),
                            Expression::Literal(leo_ast::Literal::integer(
                                IntegerType::U8,
                                "32".to_string(),
                                Default::default(),
                                Default::default(),
                            )),
                        )))
                }
                _ => false,
            })
            .unwrap_or(false);
        // Report an error if the mapping is not valid.
        if !is_valid {
            self.state.handler.emit_err(StaticAnalyzerError::custom_error(
                format!(
                    "Could not find a mapping `{}/{}` with key type `{}` and value type `[u8; 32]` for the 'checksum' upgrade configuration.",
                    location.program, location.name, key_type
                ),
                Some("Ensure that the correct program is imported and that the mapping exists."),
                constructor.span(),
            ));
        }
        // Construct the expected constructor.
        let expected = self.parse_constructor(&leo_checksum_constructor(mapping, key));
        // Check that the expected constructor matches the given constructor.
        if !constructors_match(constructor, &expected) {
            self.state.handler.emit_err(StaticAnalyzerError::custom_error(
                constructor_error_message("checksum"),
                Some(constructor_help_message(expected)),
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
        let expected = self.parse_constructor(&leo_noupgrade_constructor());
        // Check that the expected constructor matches the given constructor.
        if !constructors_match(constructor, &expected) {
            self.state.handler.emit_err(StaticAnalyzerError::custom_error(
                constructor_error_message("noupgrade"),
                Some(constructor_help_message(expected)),
                constructor.span(),
            ));
        }
    }

    // A helper function to parse a constructor string into a Leo constructor.
    fn parse_constructor(&self, constructor_string: &str) -> Constructor {
        // Initialize a new handler.
        let handler = Handler::new(BufferEmitter::new());
        // Initialize a node builder.
        let node_builder = NodeBuilder::new(0);
        // Parse the constructor string.
        leo_parser::parse_constructor(handler, &node_builder, constructor_string, 0, self.state.network)
            .expect("The default constructor should be well-formed")
    }
}

// A helper function to provide an error message if the constructor is not well formed.
fn constructor_error_message(mode: impl Display) -> String {
    format!("The constructor is not well-formed for the '{mode}' upgrade configuration.")
}

// A helper function to provide a help message if the constructor is not well formed.
fn constructor_help_message(expected: impl Display) -> String {
    format!(
        r"
The expected constructor is:
```
{expected}
```

For more information, please refer to the documentation: TODO
    "
    )
}

// A helper function to get the type from a snarkVM literal.
fn get_type_from_snarkvm_literal<N: Network>(literal: &Literal<N>) -> Type {
    match literal {
        Literal::Field(_) => Type::Field,
        Literal::Group(_) => Type::Group,
        Literal::Address(_) => Type::Address,
        Literal::Scalar(_) => Type::Scalar,
        Literal::Boolean(_) => Type::Boolean,
        Literal::String(_) => Type::String,
        Literal::I8(_) => Type::Integer(IntegerType::I8),
        Literal::I16(_) => Type::Integer(IntegerType::I16),
        Literal::I32(_) => Type::Integer(IntegerType::I32),
        Literal::I64(_) => Type::Integer(IntegerType::I64),
        Literal::I128(_) => Type::Integer(IntegerType::I128),
        Literal::U8(_) => Type::Integer(IntegerType::U8),
        Literal::U16(_) => Type::Integer(IntegerType::U16),
        Literal::U32(_) => Type::Integer(IntegerType::U32),
        Literal::U64(_) => Type::Integer(IntegerType::U64),
        Literal::U128(_) => Type::Integer(IntegerType::U128),
        Literal::Signature(_) => Type::Signature,
    }
}

// A helper function to determine if two constructors match.
fn constructors_match(constructor1: &Constructor, constructor2: &Constructor) -> bool {
    // Clone the two constructors.
    let constructor1 = constructor1.clone();
    let constructor2 = constructor2.clone();
    // Normalize them by removing the spans and NodeIDs.
    let mut normalizer = Normalizer;
    let constructor1 = normalizer.reconstruct_constructor(constructor1);
    let constructor2 = normalizer.reconstruct_constructor(constructor2);
    // Check if the two constructors are equal.
    constructor1 == constructor2
}
