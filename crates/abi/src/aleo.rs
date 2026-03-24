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
/// interface (functions, mappings).
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

    let functions = aleo
        .functions
        .iter()
        .filter(|(_, f)| f.variant.is_entry())
        .map(|(_, f)| convert_function_stub(f, &record_names))
        .collect();

    let mut program = abi::Program { program, structs, records, mappings, storage_variables, functions };

    // Prune types not used in the public interface.
    prune_non_interface_types(&mut program);

    program
}

/// Converts a function stub to an ABI function.
fn convert_function_stub(function: &ast::FunctionStub, record_names: &HashSet<Symbol>) -> abi::Function {
    let name = function.identifier.name.to_string();
    let is_final = function.has_final_output();
    let inputs = function.input.iter().map(|i| convert_input(i, record_names)).collect();
    let outputs = function.output.iter().map(|o| convert_output(o, record_names)).collect();
    abi::Function { name, is_final, inputs, outputs }
}

fn convert_input(input: &ast::Input, record_names: &HashSet<Symbol>) -> abi::Input {
    abi::Input {
        name: input.identifier.name.to_string(),
        ty: convert_function_input(&input.type_, record_names),
        mode: convert_mode(input.mode),
    }
}

fn convert_output(output: &ast::Output, record_names: &HashSet<Symbol>) -> abi::Output {
    abi::Output { ty: convert_function_output(&output.type_, record_names), mode: convert_mode(output.mode) }
}

fn convert_function_input(ty: &ast::Type, record_names: &HashSet<Symbol>) -> abi::FunctionInput {
    if let ast::Type::Composite(comp_ty) = ty {
        let name = comp_ty.path.identifier().name;
        if record_names.contains(&name) {
            return abi::FunctionInput::Record(abi::RecordRef {
                path: comp_ty.path.segments_iter().map(|s| s.to_string()).collect(),
                program: comp_ty.path.program().map(|s| s.to_string()),
            });
        }
    }
    abi::FunctionInput::Plaintext(convert_plaintext(ty))
}

fn convert_function_output(ty: &ast::Type, record_names: &HashSet<Symbol>) -> abi::FunctionOutput {
    match ty {
        ast::Type::Future(_) => abi::FunctionOutput::Final,
        ast::Type::Composite(comp_ty) => {
            let name = comp_ty.path.identifier().name;
            if record_names.contains(&name) {
                return abi::FunctionOutput::Record(abi::RecordRef {
                    path: comp_ty.path.segments_iter().map(|s| s.to_string()).collect(),
                    program: comp_ty.path.program().map(|s| s.to_string()),
                });
            }
            abi::FunctionOutput::Plaintext(convert_plaintext(ty))
        }
        _ => abi::FunctionOutput::Plaintext(convert_plaintext(ty)),
    }
}
