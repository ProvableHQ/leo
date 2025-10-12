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

// Unit tests for individual compiler passes.
// This module provides test infrastructure to verify the behavior of individual
// compiler passes in isolation. Each pass can be tested by providing Leo source
// code and verifying the AST output after the pass runs.

use crate::*;
use leo_ast::{Ast, NetworkName, NodeBuilder};
use leo_errors::{BufferEmitter, Handler};
use leo_parser::parse_ast;
use leo_span::{create_session_if_not_set_then, source_map::FileName, with_session_globals};
use serial_test::serial;

// =============================================================================
// Compiler Pass Enumeration
// =============================================================================

/// Enum representing all compiler passes in their canonical order.
/// This matches the order in `compiler/compiler/src/compiler.rs::intermediate_passes()`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompilerPass {
    PathResolution,
    SymbolTableCreation,
    TypeChecking,
    ProcessingAsync,
    StaticAnalyzing,
    ConstPropUnrollAndMorphing,
    OptionLowering,
    ProcessingScript,
    SsaForming1, // rename_defs: true
    Destructuring,
    SsaForming2, // rename_defs: false
    WriteTransforming,
    SsaForming3, // rename_defs: false
    Flattening,
    FunctionInlining,
    SsaForming4, // rename_defs: true
    CommonSubexpressionEliminating,
    DeadCodeEliminating,
}

/// Helper function to parse Leo source code into an AST.
fn parse_program(source: &str, handler: &Handler) -> Result<Ast, ()> {
    let node_builder = NodeBuilder::default();
    let filename = FileName::Custom("test".into());
    let source_file = with_session_globals(|s| s.source_map.new_source(source, filename));

    handler.extend_if_error(parse_ast(handler.clone(), &node_builder, &source_file, &[], NetworkName::TestnetV0))
}

/// Helper to initialize a CompilerState with a parsed AST.
fn initialize_state(ast: Ast, handler: Handler) -> CompilerState {
    CompilerState {
        ast,
        handler,
        type_table: TypeTable::default(),
        node_builder: NodeBuilder::default(),
        assigner: Assigner::default(),
        symbol_table: SymbolTable::default(),
        struct_graph: Default::default(),
        call_graph: Default::default(),
        warnings: Default::default(),
        is_test: false,
        network: NetworkName::TestnetV0,
    }
}

/// Runs compiler passes up to and including the specified target pass.
fn run_passes_until(source: &str, target_pass: CompilerPass, handler: &Handler) -> Result<String, ()> {
    let ast = parse_program(source, handler)?;
    let mut state = initialize_state(ast, handler.clone());

    // Belo wis almost exactly the same as the intermediate_passes function in compiler.rs.
    let type_check_input = TypeCheckingInput::new(state.network);
    let all_passes = [
        CompilerPass::PathResolution,
        CompilerPass::SymbolTableCreation,
        CompilerPass::TypeChecking,
        CompilerPass::ProcessingAsync,
        CompilerPass::StaticAnalyzing,
        CompilerPass::ConstPropUnrollAndMorphing,
        CompilerPass::OptionLowering,
        CompilerPass::ProcessingScript,
        CompilerPass::SsaForming1,
        CompilerPass::Destructuring,
        CompilerPass::SsaForming2,
        CompilerPass::WriteTransforming,
        CompilerPass::SsaForming3,
        CompilerPass::Flattening,
        CompilerPass::FunctionInlining,
        CompilerPass::SsaForming4,
        CompilerPass::CommonSubexpressionEliminating,
        CompilerPass::DeadCodeEliminating,
    ];

    for pass in all_passes {
        match pass {
            CompilerPass::PathResolution => {
                handler.extend_if_error(PathResolution::do_pass((), &mut state))?;
            }
            CompilerPass::SymbolTableCreation => {
                handler.extend_if_error(SymbolTableCreation::do_pass((), &mut state))?;
            }
            CompilerPass::TypeChecking => {
                handler.extend_if_error(TypeChecking::do_pass(type_check_input.clone(), &mut state))?;
            }
            CompilerPass::ProcessingAsync => {
                handler.extend_if_error(ProcessingAsync::do_pass(type_check_input.clone(), &mut state))?;
            }
            CompilerPass::StaticAnalyzing => {
                handler.extend_if_error(StaticAnalyzing::do_pass((), &mut state))?;
            }
            CompilerPass::ConstPropUnrollAndMorphing => {
                handler.extend_if_error(ConstPropUnrollAndMorphing::do_pass(type_check_input.clone(), &mut state))?;
            }
            CompilerPass::OptionLowering => {
                handler.extend_if_error(OptionLowering::do_pass(type_check_input.clone(), &mut state))?;
            }
            CompilerPass::ProcessingScript => {
                handler.extend_if_error(ProcessingScript::do_pass((), &mut state))?;
            }
            CompilerPass::SsaForming1 => {
                handler.extend_if_error(SsaForming::do_pass(SsaFormingInput { rename_defs: true }, &mut state))?;
            }
            CompilerPass::Destructuring => {
                handler.extend_if_error(Destructuring::do_pass((), &mut state))?;
            }
            CompilerPass::SsaForming2 => {
                handler.extend_if_error(SsaForming::do_pass(SsaFormingInput { rename_defs: false }, &mut state))?;
            }
            CompilerPass::WriteTransforming => {
                handler.extend_if_error(WriteTransforming::do_pass((), &mut state))?;
            }
            CompilerPass::SsaForming3 => {
                handler.extend_if_error(SsaForming::do_pass(SsaFormingInput { rename_defs: false }, &mut state))?;
            }
            CompilerPass::Flattening => {
                handler.extend_if_error(Flattening::do_pass((), &mut state))?;
            }
            CompilerPass::FunctionInlining => {
                handler.extend_if_error(FunctionInlining::do_pass((), &mut state))?;
            }
            CompilerPass::SsaForming4 => {
                handler.extend_if_error(SsaForming::do_pass(SsaFormingInput { rename_defs: true }, &mut state))?;
            }
            CompilerPass::CommonSubexpressionEliminating => {
                handler.extend_if_error(CommonSubexpressionEliminating::do_pass((), &mut state))?;
            }
            CompilerPass::DeadCodeEliminating => {
                handler.extend_if_error(DeadCodeEliminating::do_pass((), &mut state))?;
            }
        }

        // Stop on the targeted pass.
        if pass == target_pass {
            break;
        }
    }

    if handler.err_count() != 0 {
        return Err(());
    }

    // Get the AST  up to the targeted pass.
    Ok(format!("{}", state.ast.ast))
}

// =============================================================================
// Test Runner Macro
// =============================================================================
// This macro generates test runner functions for compiler passes.
// Each runner follows the same pattern:
// Setup error handling, call run_passess_until to get specified pass, and then return
// the resulting AST or errors
// Usage: make_runner!(function_name, CompilerPass::Variant);

macro_rules! make_runner {
    ($runner_name:ident, $pass:expr) => {
        fn $runner_name(source: &str) -> String {
            let buf = BufferEmitter::new();
            let handler = Handler::new(buf.clone());

            create_session_if_not_set_then(|_| match run_passes_until(source, $pass, &handler) {
                Ok(ast) => format!("{}{}", buf.extract_warnings(), ast),
                Err(()) => format!("{}{}", buf.extract_errs(), buf.extract_warnings()),
            })
        }
    };
}

make_runner!(function_inlining_runner, CompilerPass::FunctionInlining);
make_runner!(dead_code_elimination_runner, CompilerPass::DeadCodeEliminating);
make_runner!(static_single_assignment_runner, CompilerPass::SsaForming1);
make_runner!(flattening_runner, CompilerPass::Flattening);
make_runner!(destructuring_runner, CompilerPass::Destructuring);
make_runner!(common_subexpression_elimination_runner, CompilerPass::CommonSubexpressionEliminating);
make_runner!(processing_async_runner, CompilerPass::ProcessingAsync);
make_runner!(static_analyzing_runner, CompilerPass::StaticAnalyzing);
make_runner!(const_prop_unroll_and_morphing_runner, CompilerPass::ConstPropUnrollAndMorphing);
make_runner!(option_lowering_runner, CompilerPass::OptionLowering);
make_runner!(processing_script_runner, CompilerPass::ProcessingScript);
make_runner!(write_transforming_runner, CompilerPass::WriteTransforming);

// =============================================================================
// Test Functions
// =============================================================================

#[test]
#[serial]
fn test_function_inlining() {
    leo_test_framework::run_tests("passes/function_inlining", function_inlining_runner);
}

#[test]
#[serial]
fn test_dead_code_elimination() {
    leo_test_framework::run_tests("passes/dead_code_elimination", dead_code_elimination_runner);
}

#[test]
#[serial]
fn test_static_single_assignment() {
    leo_test_framework::run_tests("passes/static_single_assignment", static_single_assignment_runner);
}

#[test]
#[serial]
fn test_flattening() {
    leo_test_framework::run_tests("passes/flattening", flattening_runner);
}

#[test]
#[serial]
fn test_destructuring() {
    leo_test_framework::run_tests("passes/destructuring", destructuring_runner);
}

#[test]
#[serial]
fn test_common_subexpression_elimination() {
    leo_test_framework::run_tests("passes/common_subexpression_elimination", common_subexpression_elimination_runner);
}

#[test]
#[serial]
fn test_processing_async() {
    leo_test_framework::run_tests("passes/processing_async", processing_async_runner);
}

#[test]
#[serial]
fn test_static_analyzing() {
    leo_test_framework::run_tests("passes/static_analyzing", static_analyzing_runner);
}

#[test]
#[serial]
fn test_const_prop_unroll_and_morphing() {
    leo_test_framework::run_tests("passes/const_prop_unroll_and_morphing", const_prop_unroll_and_morphing_runner);
}

#[test]
#[serial]
fn test_option_lowering() {
    leo_test_framework::run_tests("passes/option_lowering", option_lowering_runner);
}

#[test]
#[serial]
fn test_processing_script() {
    leo_test_framework::run_tests("passes/processing_script", processing_script_runner);
}

#[test]
#[serial]
fn test_write_transforming() {
    leo_test_framework::run_tests("passes/write_transforming", write_transforming_runner);
}
