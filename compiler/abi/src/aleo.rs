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

//! ABI generation for Aleo programs (pre-compiled bytecode dependencies).

use crate::{
    convert_mapping,
    convert_mode,
    convert_plaintext,
    convert_record,
    convert_struct,
    prune_non_interface_types,
};

use leo_abi_types as abi;
use leo_ast as ast;
use leo_span::Symbol;

use std::collections::HashSet;

/// Generates ABI from an Aleo program (pre-compiled bytecode dependency).
///
/// Unlike Leo programs, Aleo programs do not have modules, so type paths in the
/// generated ABI are always single-element (just the type name).
///
/// The returned ABI is pruned to only include types that appear in the public
/// interface (transitions, mappings).
pub fn generate(aleo: &ast::AleoProgram) -> abi::Program {
    let program = aleo.stub_id.to_string();

    // Collect composites. Aleo programs have no modules, so path = [name].
    let structs: Vec<abi::Struct> =
        aleo.composites.iter().filter(|(_, c)| !c.is_record).map(|(_, c)| convert_struct(c, &[])).collect();

    let records: Vec<abi::Record> =
        aleo.composites.iter().filter(|(_, c)| c.is_record).map(|(_, c)| convert_record(c, &[])).collect();

    let mappings = aleo.mappings.iter().map(|(_, m)| convert_mapping(m)).collect();

    // Aleo programs don't have storage variables.
    let storage_variables = vec![];

    // Create a lookup set for record names.
    let record_names: HashSet<Symbol> =
        aleo.composites.iter().filter(|(_, c)| c.is_record).map(|(name, _)| *name).collect();

    let transitions = aleo
        .functions
        .iter()
        .filter(|(_, f)| f.variant.is_transition())
        .map(|(_, f)| convert_function_stub(f, &record_names))
        .collect();

    let mut program = abi::Program { program, structs, records, mappings, storage_variables, transitions };

    // Prune types not used in the public interface.
    prune_non_interface_types(&mut program);

    program
}

/// Converts a function stub to a transition for ABI.
fn convert_function_stub(function: &ast::FunctionStub, record_names: &HashSet<Symbol>) -> abi::Transition {
    let name = function.identifier.name.to_string();
    let is_async = function.variant.is_async();
    let inputs = function.input.iter().map(|i| convert_input(i, record_names)).collect();
    let outputs = function.output.iter().map(|o| convert_output(o, record_names)).collect();
    abi::Transition { name, is_async, inputs, outputs }
}

fn convert_input(input: &ast::Input, record_names: &HashSet<Symbol>) -> abi::Input {
    abi::Input {
        name: input.identifier.name.to_string(),
        ty: convert_transition_input(&input.type_, record_names),
        mode: convert_mode(input.mode),
    }
}

fn convert_output(output: &ast::Output, record_names: &HashSet<Symbol>) -> abi::Output {
    abi::Output { ty: convert_transition_output(&output.type_, record_names), mode: convert_mode(output.mode) }
}

fn convert_transition_input(ty: &ast::Type, record_names: &HashSet<Symbol>) -> abi::TransitionInput {
    if let ast::Type::Composite(comp_ty) = ty {
        let name = comp_ty.path.identifier().name;
        if record_names.contains(&name) {
            return abi::TransitionInput::Record(abi::RecordRef {
                path: comp_ty.path.segments_iter().map(|s| s.to_string()).collect(),
                program: comp_ty.path.program().map(|s| s.to_string()),
            });
        }
    }
    abi::TransitionInput::Plaintext(convert_plaintext(ty))
}

fn convert_transition_output(ty: &ast::Type, record_names: &HashSet<Symbol>) -> abi::TransitionOutput {
    match ty {
        ast::Type::Future(_) => abi::TransitionOutput::Future,
        ast::Type::Composite(comp_ty) => {
            let name = comp_ty.path.identifier().name;
            if record_names.contains(&name) {
                return abi::TransitionOutput::Record(abi::RecordRef {
                    path: comp_ty.path.segments_iter().map(|s| s.to_string()).collect(),
                    program: comp_ty.path.program().map(|s| s.to_string()),
                });
            }
            abi::TransitionOutput::Plaintext(convert_plaintext(ty))
        }
        _ => abi::TransitionOutput::Plaintext(convert_plaintext(ty)),
    }
}
