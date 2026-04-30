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

#[cfg(feature = "aleo-bytecode")]
use leo_span::create_session_if_not_set_then;
#[cfg(feature = "aleo-bytecode")]
use std::fmt;

/// Error returned when generating ABI data from Aleo bytecode fails.
#[cfg(feature = "aleo-bytecode")]
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct BytecodeAbiError(String);

#[cfg(feature = "aleo-bytecode")]
impl BytecodeAbiError {
    fn new(error: impl ToString) -> Self {
        Self(error.to_string())
    }
}

#[cfg(feature = "aleo-bytecode")]
impl fmt::Display for BytecodeAbiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

#[cfg(feature = "aleo-bytecode")]
impl std::error::Error for BytecodeAbiError {}

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

    let mut program =
        abi::Program { program, implements: vec![], structs, records, mappings, storage_variables, functions };

    // Prune types not used in the public interface.
    prune_non_interface_types(&mut program);

    program
}

/// Generates ABI from Aleo bytecode for the requested network.
#[cfg(feature = "aleo-bytecode")]
pub fn generate_from_bytecode(
    name: impl fmt::Display,
    bytecode: &str,
    network: ast::NetworkName,
) -> Result<abi::Program, BytecodeAbiError> {
    create_session_if_not_set_then(|_| {
        let aleo = leo_disassembler::disassemble_from_str_for_network(name, bytecode, network)
            .map_err(BytecodeAbiError::new)?;
        Ok(generate(&aleo))
    })
}

/// Converts a function stub to an ABI function.
fn convert_function_stub(function: &ast::FunctionStub, record_names: &HashSet<Symbol>) -> abi::Function {
    let name = function.identifier.name.to_string();
    let is_final = function.has_final_output();
    let inputs = function.input.iter().map(|i| convert_input(i, record_names)).collect();
    let outputs = function.output.iter().map(|o| convert_output(o, record_names)).collect();
    abi::Function { name, is_final, const_parameters: vec![], inputs, outputs }
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
    if let ast::Type::DynRecord = ty {
        return abi::FunctionInput::DynamicRecord;
    }
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
        ast::Type::DynRecord => abi::FunctionOutput::DynamicRecord,
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

#[cfg(all(test, feature = "aleo-bytecode"))]
mod tests {
    use super::*;

    use serde_json::Value;

    const SIMPLE_ALEO: &str = include_str!("../../../tests/tests/cli/test_abi_from_aleo/contents/simple.aleo");
    const SIMPLE_ABI: &str =
        include_str!("../../../tests/expectations/cli/test_abi_from_aleo/contents/simple.abi.json");

    #[test]
    fn generate_from_bytecode_matches_existing_cli_fixture() {
        let abi = generate_from_bytecode("simple.aleo", SIMPLE_ALEO, ast::NetworkName::TestnetV0)
            .expect("expected ABI generation to succeed");
        let abi = serde_json::to_value(&abi).expect("expected generated ABI to serialize");
        let expected: Value = serde_json::from_str(SIMPLE_ABI).expect("expected fixture ABI JSON to parse");
        assert_eq!(abi, expected);
    }
}
