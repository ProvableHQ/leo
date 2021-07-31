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

#[macro_use]
extern crate thiserror;

use eyre::ErrReport;

#[derive(Debug, Error)]
pub enum LeoError {
    #[error(transparent)]
    AsgError(#[from] AsgError),

    #[error(transparent)]
    AstError(#[from] AstError),

    #[error(transparent)]
    CompilerError(#[from] CompilerError),
    
    #[error(transparent)]
    ImportError(#[from] ImportError),

    #[error(transparent)]
    ParserError(#[from] ParserError),

    #[error(transparent)]
    RustError(#[from] ErrReport),

    #[error(transparent)]
    SnarkVMError(#[from] SnarkVMError),
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
