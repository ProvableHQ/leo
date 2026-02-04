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

/// Contains the ASG error definitions.
use crate::LeoMessageCode;

/// Contains the AST error definitions.
mod ast;
pub use self::ast::*;

/// Contains the CLI error definitions.
mod cli;
pub use self::cli::*;

/// Contains the Compiler error definitions.
mod compiler;
pub use self::compiler::*;

/// Contains the Flattener error definitions.
mod flattener;
pub use self::flattener::*;

/// Contains the Loop Unroller error definitions.
mod loop_unroller;
pub use self::loop_unroller::*;

mod interpreter_halt;
pub use self::interpreter_halt::*;

/// Contains the Package error definitions.
mod package;
pub use self::package::*;

/// Contains the Parser error definitions.
mod parser;
pub use self::parser::*;

/// Contains the Static Analyzer error definitions.
mod static_analyzer;
pub use self::static_analyzer::*;

/// Contains the Type Checker error definitions.
mod type_checker;
pub use self::type_checker::*;

/// Contains the Name Validation error definitions.
mod name_validation;
pub use self::name_validation::*;

/// Contains the Utils error definitions.
mod utils;
pub use self::utils::*;

/// The LeoError type that contains all sub error types.
/// This allows a unified error type throughout the Leo crates.
#[derive(Debug, Error)]
pub enum LeoError {
    /// Represents an AST Error in a Leo Error.
    #[error(transparent)]
    AstError(#[from] AstError),
    /// Represents a CLI Error in a Leo Error.
    #[error(transparent)]
    CliError(#[from] CliError),
    /// Represents a Compiler Error in a Leo Error.
    #[error(transparent)]
    CompilerError(#[from] CompilerError),
    #[error(transparent)]
    InterpreterHalt(#[from] InterpreterHalt),
    /// Represents a Package Error in a Leo Error.
    #[error(transparent)]
    PackageError(#[from] PackageError),
    /// Represents a Parser Error in a Leo Error.
    #[error(transparent)]
    ParserError(#[from] ParserError),
    /// Represents a Static Analyzer Error in a Leo Error.
    #[error(transparent)]
    StaticAnalyzerError(#[from] StaticAnalyzerError),
    /// Represents a Type Checker Error in a Leo Error.
    #[error(transparent)]
    TypeCheckerError(#[from] TypeCheckerError),
    /// Represents a Name Validation Error in a Leo Error.
    #[error(transparent)]
    NameValidationError(#[from] NameValidationError),
    /// Represents a Loop Unroller Error in a Leo Error.
    #[error(transparent)]
    LoopUnrollerError(#[from] LoopUnrollerError),
    /// Represents a Flatten Error in a Leo Error.
    #[error(transparent)]
    FlattenError(#[from] FlattenError),
    /// Purely for just exiting with the correct status code and
    /// not re-displaying an error.
    #[error("")]
    LastErrorCode(i32),
    /// Represents a Utils Error in a Leo Error.
    #[error(transparent)]
    UtilError(#[from] UtilError),
    /// Anyhow errors.
    #[error(transparent)]
    Anyhow(#[from] anyhow::Error),
}

impl LeoError {
    /// Implement error code for each type of Error.
    pub fn error_code(&self) -> String {
        use LeoError::*;

        match self {
            AstError(error) => error.error_code(),
            CompilerError(error) => error.error_code(),
            CliError(error) => error.error_code(),
            ParserError(error) => error.error_code(),
            PackageError(error) => error.error_code(),
            StaticAnalyzerError(error) => error.error_code(),
            TypeCheckerError(error) => error.error_code(),
            NameValidationError(error) => error.error_code(),
            LoopUnrollerError(error) => error.error_code(),
            FlattenError(error) => error.error_code(),
            UtilError(error) => error.error_code(),
            LastErrorCode(_) => unreachable!(),
            Anyhow(_) => "SnarkVM Error".to_string(), // todo: implement error codes for snarkvm errors.
            InterpreterHalt(_) => "Interpreter Halt".to_string(),
        }
    }

    /// Implement exit code for each type of Error.
    pub fn exit_code(&self) -> i32 {
        use LeoError::*;

        match self {
            AstError(error) => error.exit_code(),
            CompilerError(error) => error.exit_code(),
            CliError(error) => error.exit_code(),
            ParserError(error) => error.exit_code(),
            PackageError(error) => error.exit_code(),
            StaticAnalyzerError(error) => error.exit_code(),
            TypeCheckerError(error) => error.exit_code(),
            NameValidationError(error) => error.exit_code(),
            LoopUnrollerError(error) => error.exit_code(),
            FlattenError(error) => error.exit_code(),
            UtilError(error) => error.exit_code(),
            LastErrorCode(code) => *code,
            Anyhow(_) => 11000, // todo: implement exit codes for snarkvm errors.
            InterpreterHalt(_) => 1,
        }
    }
}

/// The LeoWarning type that contains all sub warning types.
/// This allows a unified warning type throughout the Leo crates.
#[derive(Debug, Error, Hash, PartialEq, Eq)]
pub enum LeoWarning {
    /// Represents an Parser Warning in a Leo Warning.
    #[error(transparent)]
    ParserWarning(#[from] ParserWarning),
    /// Represents a Static Analyzer Warning in a Leo Warning.
    #[error(transparent)]
    StaticAnalyzerWarning(#[from] StaticAnalyzerWarning),
    /// Represents a Type Checker Warning in a Leo Warning.
    #[error(transparent)]
    TypeCheckerWarning(#[from] TypeCheckerWarning),
}

impl LeoWarning {
    /// Implement warning code for each type of Warning.
    pub fn error_code(&self) -> String {
        use LeoWarning::*;

        match self {
            ParserWarning(warning) => warning.warning_code(),
            TypeCheckerWarning(warning) => warning.warning_code(),
            StaticAnalyzerWarning(warning) => warning.warning_code(),
        }
    }
}

/// A global result type for all Leo crates, that defaults the errors to be a LeoError.
pub type Result<T, E = LeoError> = core::result::Result<T, E>;
