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

pub mod asg;
pub use self::asg::*;

pub mod ast;
pub use self::ast::*;

pub mod cli;
pub use self::cli::*;

#[macro_use]
pub mod common;
pub use self::common::*;

pub mod compiler;
pub use self::compiler::*;

pub mod import;
pub use self::import::*;

pub mod package;
pub use self::package::*;

pub mod parser;
pub use self::parser::*;

pub mod snarkvm;
pub use self::snarkvm::*;

pub mod state;
pub use self::state::*;

#[macro_use]
extern crate thiserror;

use leo_input::InputParserError;

use backtrace::Backtrace;

#[derive(Debug, Error)]
pub enum LeoError {
    #[error(transparent)]
    AsgError(#[from] AsgError),

    #[error(transparent)]
    AstError(#[from] AstError),

    #[error(transparent)]
    CliError(#[from] CliError),

    #[error(transparent)]
    CompilerError(#[from] CompilerError),

    #[error(transparent)]
    ImportError(#[from] ImportError),

    #[error(transparent)]
    InputError(#[from] InputParserError),

    #[error(transparent)]
    PackageError(#[from] PackageError),

    #[error(transparent)]
    ParserError(#[from] ParserError),

    #[error(transparent)]
    SnarkVMError(#[from] SnarkVMError),

    #[error(transparent)]
    StateError(#[from] StateError),
}

impl LeoError {
    pub fn exit_code(&self) -> u32 {
        use LeoError::*;

        match self {
            AsgError(error) => error.exit_code(),
            AstError(error) => error.exit_code(),
            CliError(error) => error.exit_code(),
            CompilerError(error) => error.exit_code(),
            ImportError(error) => error.exit_code(),
            InputError(_error) => 0, // TODO migrate me.
            PackageError(error) => error.exit_code(),
            ParserError(error) => error.exit_code(),
            SnarkVMError(_error) => 0, // TODO update once snarkvm implments a global top level error similar to LeoError.
            StateError(error) => error.exit_code(),
        }
    }
}

pub type Result<T, E = LeoError> = core::result::Result<T, E>;

#[inline(always)]
pub fn new_backtrace() -> Backtrace {
    Backtrace::new()
}

// #[test]
// fn test_error() {
//     let err = FormattedError {
//         path: std::sync::Arc::new("file.leo".to_string()),
//         line_start: 2,
//         line_stop: 2,
//         col_start: 9,
//         col_stop: 10,
//         content: "let a = x;".into(),
//         message: "undefined value `x`".to_string(),
//     };

//     assert_eq!(
//         err.to_string(),
//         vec![
//             "    --> file.leo:2:9",
//             "     |",
//             "   2 | let a = x;",
//             "     |         ^",
//             "     |",
//             "     = undefined value `x`",
//         ]
//         .join("\n")
//     );
// }
