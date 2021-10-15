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

#![deny(clippy::all, clippy::missing_docs_in_private_items)]
#![doc = include_str!("../README.md")]

/// Contains traits and types for channels through which errors go.
pub mod emitter;

/// Contains the ASG error definitions.
pub mod asg;
pub use self::asg::*;

/// Contains the AST error definitions.
pub mod ast;
pub use self::ast::*;

/// Contains the CLI error definitions.
pub mod cli;
pub use self::cli::*;

/// Contains the common functionalities for defining errors..
#[macro_use]
pub mod common;
pub use self::common::*;

/// Contains the Compiler error definitions.
pub mod compiler;
pub use self::compiler::*;

/// Contains the Import error definitions.
pub mod import;
pub use self::import::*;

/// Contains the Package error definitions.
pub mod package;
pub use self::package::*;

/// Contains the Parser error definitions.
pub mod parser;
pub use self::parser::*;

/// Contains the SnarkVM error definitions.
pub mod snarkvm;
pub use self::snarkvm::*;

/// Contains the State error definitions.
pub mod state;
pub use self::state::*;

#[macro_use]
extern crate thiserror;

use leo_input::InputParserError;

/// The LeoError type that contains all sub error types.
/// This allows a unified error type throughout the Leo crates.
#[derive(Clone, Debug, Error)]
pub enum LeoError {
    /// Represents an ASG Error in a Leo Error.
    #[error(transparent)]
    AsgError(#[from] AsgError),

    /// Represents an AST Error in a Leo Error.
    #[error(transparent)]
    AstError(#[from] AstError),

    /// Represents an CLI Error in a Leo Error.
    #[error(transparent)]
    CliError(#[from] CliError),

    /// Represents an Compiler Error in a Leo Error.
    #[error(transparent)]
    CompilerError(#[from] CompilerError),

    /// Represents an Import Error in a Leo Error.
    #[error(transparent)]
    ImportError(#[from] ImportError),

    /// Represents an Input Error in a Leo Error.
    #[error(transparent)]
    InputError(#[from] InputParserError),

    /// Represents an Package Error in a Leo Error.
    #[error(transparent)]
    PackageError(#[from] PackageError),

    /// Represents an Parser Error in a Leo Error.
    #[error(transparent)]
    ParserError(#[from] ParserError),

    /// Represents an SnarkVM Error in a Leo Error.
    #[error(transparent)]
    SnarkVMError(#[from] SnarkVMError),

    /// Represents an State Error in a Leo Error.
    #[error(transparent)]
    StateError(#[from] StateError),
}

impl LeoError {
    /// Implement error code for each type of Error. For the unsupported use a default value.
    pub fn error_code(&self) -> String {
        use LeoError::*;

        match self {
            AsgError(error) => error.error_code(),
            AstError(error) => error.error_code(),
            CliError(error) => error.error_code(),
            CompilerError(error) => error.error_code(),
            ImportError(error) => error.error_code(),
            InputError(_error) => Default::default(), // TODO migrate me, or not cause we want inputs to have 0 deps.
            PackageError(error) => error.error_code(),
            ParserError(error) => error.error_code(),
            SnarkVMError(_error) => Default::default(), // TODO update once snarkvm implments a global top level error similar to LeoError.
            StateError(error) => error.error_code(),
        }
    }

    /// Implment exit code for each type of Error, even the ones that don't have one.
    pub fn exit_code(&self) -> i32 {
        use LeoError::*;

        match self {
            AsgError(error) => error.exit_code(),
            AstError(error) => error.exit_code(),
            CliError(error) => error.exit_code(),
            CompilerError(error) => error.exit_code(),
            ImportError(error) => error.exit_code(),
            InputError(_error) => 1, // TODO migrate me, or not cause we want inputs to have 0 deps.
            PackageError(error) => error.exit_code(),
            ParserError(error) => error.exit_code(),
            SnarkVMError(_error) => 1, // TODO update once snarkvm implments a global top level error similar to LeoError.
            StateError(error) => error.exit_code(),
        }
    }
}

/// A global result type for all Leo crates, that defaults the errors to be a LeoError.
pub type Result<T, E = LeoError> = core::result::Result<T, E>;
