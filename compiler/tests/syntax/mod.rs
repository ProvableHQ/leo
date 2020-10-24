// Copyright (C) 2019-2020 Aleo Systems Inc.
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

use crate::{expect_compiler_error, parse_input, parse_program};
use leo_ast::ParserError;
use leo_compiler::errors::{CompilerError, ExpressionError, FunctionError, StatementError};
use leo_dynamic_check::errors::{DynamicCheckError, FrameError, TypeAssertionError};
use leo_input::InputParserError;

pub mod identifiers;

#[test]
#[ignore]
fn test_semicolon() {
    let bytes = include_bytes!("semicolon.leo");
    let error = parse_program(bytes).err().unwrap();

    match error {
        CompilerError::ParserError(ParserError::SyntaxError(_)) => {}
        _ => panic!("test_semicolon failed the wrong expected error, should be a ParserError"),
    }
}

#[test]
fn test_undefined() {
    let bytes = include_bytes!("undefined.leo");
    let program = parse_program(bytes).unwrap();

    let error = expect_compiler_error(program);

    match error {
        CompilerError::FunctionError(FunctionError::StatementError(StatementError::ExpressionError(
            ExpressionError::Error(error),
        ))) => {
            assert_eq!(
                format!("{}", error),
                vec![
                    "    --> \"/test/src/main.leo\": 2:12",
                    "     |",
                    "   2 |      return a",
                    "     |             ^",
                    "     |",
                    "     = cannot find value `a` in this scope",
                ]
                .join("\n")
            );
        }
        _ => panic!("expected an undefined identifier error"),
    }
}

#[test]
#[ignore]
fn input_syntax_error() {
    let bytes = include_bytes!("input_semicolon.leo");
    let error = parse_input(bytes).err().unwrap();

    // Expect an input parser error.
    match error {
        CompilerError::InputParserError(InputParserError::SyntaxError(_)) => {}
        _ => panic!("input syntax error should be a ParserError"),
    }
}

#[test]
fn test_compare_mismatched_types() {
    let bytes = include_bytes!("compare_mismatched_types.leo");
    let error = parse_program(bytes).err().unwrap();

    // Expect a dynamic check error.
    match error {
        CompilerError::DynamicCheckError(DynamicCheckError::FrameError(FrameError::TypeAssertionError(
            TypeAssertionError::Error(_),
        ))) => {}
        error => panic!("Expected dynamic check error, found {}", error),
    }
}
