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

use crate::{assert_satisfied, parse_program};
#[allow(unused)]
use leo_asg::{new_context, Asg, AsgContext};
use leo_ast::Ast;
use leo_compiler::TypeInferenceStage;
use leo_imports::ImportParser;
use leo_parser::parser;

thread_local! {
    static THREAD_GLOBAL_CONTEXT: AsgContext<'static> = {
        let leaked = Box::leak(Box::new(leo_asg::new_alloc_context()));
        leo_asg::new_context(leaked)
    }
}

pub fn thread_leaked_context() -> AsgContext<'static> {
    THREAD_GLOBAL_CONTEXT.with(|f| *f)
}

pub fn parse_program_ast(file_string: &str) -> Ast {
    const TEST_PROGRAM_PATH: &str = "";
    let test_program_file_path = std::path::PathBuf::from(TEST_PROGRAM_PATH);

    let mut ast = Ast::new(
        parser::parse(test_program_file_path.to_str().expect("unwrap fail"), &file_string)
            .expect("Failed to parse file."),
    );
    ast.canonicalize().expect("Failed to canonicalize program.");

    let program = ast.clone().into_repr();
    let asg = Asg::new(thread_leaked_context(), &program, &mut ImportParser::default())
        .expect("Failed to create ASG from AST");

    let new_ast = TypeInferenceStage::default()
        .stage_ast(&program, &asg.into_repr())
        .expect("Failed to produce type inference ast.");

    new_ast
}

#[test]
fn test_basic() {
    // Check program is valid.
    let program_string = include_str!("basic.leo");
    let program = parse_program(program_string).unwrap();
    assert_satisfied(program);

    // Check we get expected ast.
    let ast = parse_program_ast(program_string);
    let expected_json = include_str!("basic.json");
    let expected_ast: Ast = Ast::from_json_string(expected_json).expect("Unable to parse json.");

    assert_eq!(expected_ast, ast);
}

#[test]
fn test_for_loop_and_compound() {
    // Check program is valid.
    let program_string = include_str!("for_loop_and_compound.leo");
    let program = parse_program(program_string).unwrap();
    assert_satisfied(program);

    // Check we get expected ast.
    let ast = parse_program_ast(program_string);
    let expected_json = include_str!("for_loop_and_compound.json");
    let expected_ast: Ast = Ast::from_json_string(expected_json).expect("Unable to parse json.");

    assert_eq!(expected_ast, ast);
}

// #[test]
// fn test_big_self_outside_circuit_fail() {
//     // Check program is invalid.
//     let program_string = include_str!("big_self_outside_circuit_fail.leo");
//     let program = parse_program(program_string);
//     assert!(program.is_err());
// }
