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

//! ABI generation for Leo programs.
//!
//! This crate generates ABI definitions from the Leo AST. The ABI should be
//! generated after type checking to ensure all types are resolved.

pub mod aleo;

pub use leo_abi_types::*;

use leo_abi_types as abi;
use leo_ast::{self as ast, Expression, Literal, LiteralVariant};
use leo_span::Symbol;

use indexmap::IndexMap;
use std::collections::HashSet;

struct Ctx<'a> {
    scope: &'a ast::ProgramScope,
    stubs: &'a IndexMap<Symbol, ast::Stub>,
    modules: &'a IndexMap<Vec<Symbol>, ast::Module>,
}

/// Generates the ABI for a Leo program.
///
/// The returned ABI is pruned to only include types that appear in the public
/// interface (transitions, mappings, storage variables).
pub fn generate(ast: &ast::Program) -> abi::Program {
    let scope = ast.program_scopes.values().next().unwrap();
    let ctx = Ctx { scope, stubs: &ast.stubs, modules: &ast.modules };

    let program = scope.program_id.to_string();

    // Collect program-scope composites (path = [name])
    let mut structs: Vec<abi::Struct> =
        scope.composites.iter().filter(|(_, c)| !c.is_record).map(|(_, c)| convert_struct(c, &[])).collect();

    let mut records: Vec<abi::Record> =
        scope.composites.iter().filter(|(_, c)| c.is_record).map(|(_, c)| convert_record(c, &[])).collect();

    // Collect module composites (path = module_path + [name])
    for (module_path, module) in &ast.modules {
        for (_, composite) in &module.composites {
            if composite.is_record {
                records.push(convert_record(composite, module_path));
            } else {
                structs.push(convert_struct(composite, module_path));
            }
        }
    }

    let mappings = scope.mappings.iter().map(|(_, m)| convert_mapping(m)).collect();

    let storage_variables = scope.storage_variables.iter().map(|(_, sv)| convert_storage_variable(sv)).collect();

    let transitions = scope
        .functions
        .iter()
        .filter(|(_, f)| f.variant.is_entry())
        .map(|(_, f)| convert_transition(f, &ctx))
        .collect();

    let mut program = abi::Program { program, structs, records, mappings, storage_variables, transitions };

    // Prune types not used in the public interface.
    prune_non_interface_types(&mut program);

    program
}

fn convert_struct(composite: &ast::Composite, module_path: &[Symbol]) -> abi::Struct {
    let mut path: Vec<String> = module_path.iter().map(|s| s.to_string()).collect();
    path.push(composite.identifier.name.to_string());

    abi::Struct { path, fields: composite.members.iter().map(convert_field).collect() }
}

fn convert_record(composite: &ast::Composite, module_path: &[Symbol]) -> abi::Record {
    let mut path: Vec<String> = module_path.iter().map(|s| s.to_string()).collect();
    path.push(composite.identifier.name.to_string());

    abi::Record { path, fields: composite.members.iter().map(convert_record_field).collect() }
}

fn convert_field(member: &ast::Member) -> abi::StructField {
    abi::StructField { name: member.identifier.name.to_string(), ty: convert_plaintext(&member.type_) }
}

fn convert_record_field(member: &ast::Member) -> abi::RecordField {
    abi::RecordField {
        name: member.identifier.name.to_string(),
        ty: convert_plaintext(&member.type_),
        mode: convert_mode(member.mode),
    }
}

fn convert_mapping(mapping: &ast::Mapping) -> abi::Mapping {
    abi::Mapping {
        name: mapping.identifier.name.to_string(),
        key: convert_plaintext(&mapping.key_type),
        value: convert_plaintext(&mapping.value_type),
    }
}

fn convert_storage_variable(sv: &ast::StorageVariable) -> abi::StorageVariable {
    abi::StorageVariable { name: sv.identifier.name.to_string(), ty: convert_storage_type(&sv.type_) }
}

fn convert_storage_type(ty: &ast::Type) -> abi::StorageType {
    match ty {
        ast::Type::Vector(v) => abi::StorageType::Vector(Box::new(convert_storage_type(&v.element_type))),
        other => abi::StorageType::Plaintext(convert_plaintext(other)),
    }
}

fn convert_transition(function: &ast::Function, ctx: &Ctx) -> abi::Transition {
    let name = function.identifier.name.to_string();
    let is_async = function.has_final_output();
    let inputs = function.input.iter().map(|i| convert_input(i, ctx)).collect();
    let outputs = function.output.iter().map(|o| convert_output(o, ctx)).collect();
    abi::Transition { name, is_async, inputs, outputs }
}

fn convert_input(input: &ast::Input, ctx: &Ctx) -> abi::Input {
    abi::Input {
        name: input.identifier.name.to_string(),
        ty: convert_transition_input(&input.type_, ctx),
        mode: convert_mode(input.mode),
    }
}

fn convert_output(output: &ast::Output, ctx: &Ctx) -> abi::Output {
    abi::Output { ty: convert_transition_output(&output.type_, ctx), mode: convert_mode(output.mode) }
}

fn convert_mode(mode: ast::Mode) -> abi::Mode {
    match mode {
        ast::Mode::None => abi::Mode::None,
        ast::Mode::Constant => abi::Mode::Constant,
        ast::Mode::Private => abi::Mode::Private,
        ast::Mode::Public => abi::Mode::Public,
    }
}

fn convert_plaintext(ty: &ast::Type) -> abi::Plaintext {
    match ty {
        ast::Type::Address => abi::Plaintext::Primitive(abi::Primitive::Address),
        ast::Type::Boolean => abi::Plaintext::Primitive(abi::Primitive::Boolean),
        ast::Type::Field => abi::Plaintext::Primitive(abi::Primitive::Field),
        ast::Type::Group => abi::Plaintext::Primitive(abi::Primitive::Group),
        ast::Type::Scalar => abi::Plaintext::Primitive(abi::Primitive::Scalar),
        ast::Type::Signature => abi::Plaintext::Primitive(abi::Primitive::Signature),
        ast::Type::Integer(int_ty) => abi::Plaintext::Primitive(convert_integer(*int_ty)),
        ast::Type::Array(arr_ty) => abi::Plaintext::Array(abi::Array {
            element: Box::new(convert_plaintext(arr_ty.element_type())),
            length: extract_array_length(&arr_ty.length),
        }),
        ast::Type::Composite(comp_ty) => abi::Plaintext::Struct(abi::StructRef {
            path: comp_ty.path.segments_iter().map(|s| s.to_string()).collect(),
            program: comp_ty.path.program().map(|s| s.to_string()),
        }),
        ast::Type::Optional(opt_ty) => {
            abi::Plaintext::Optional(abi::Optional(Box::new(convert_plaintext(&opt_ty.inner))))
        }
        // These types cannot appear in plaintext contexts:
        // - Tuple: not allowed in storage or transition inputs/outputs
        // - Vector: only allowed in storage variables (handled by convert_storage_type)
        // - Others: resolved or invalid after type checking
        ast::Type::Future(_)
        | ast::Type::Mapping(_)
        | ast::Type::Tuple(_)
        | ast::Type::Vector(_)
        | ast::Type::String
        | ast::Type::Unit
        | ast::Type::Identifier(_)
        | ast::Type::Numeric
        | ast::Type::Err => {
            unreachable!("unexpected type in plaintext context: {ty}")
        }
    }
}

fn convert_transition_input(ty: &ast::Type, ctx: &Ctx) -> abi::TransitionInput {
    if let ast::Type::Composite(comp_ty) = ty
        && is_record(comp_ty, ctx)
    {
        return abi::TransitionInput::Record(abi::RecordRef {
            path: comp_ty.path.segments_iter().map(|s| s.to_string()).collect(),
            program: comp_ty.path.program().map(|s| s.to_string()),
        });
    }
    abi::TransitionInput::Plaintext(convert_plaintext(ty))
}

fn convert_transition_output(ty: &ast::Type, ctx: &Ctx) -> abi::TransitionOutput {
    match ty {
        ast::Type::Future(_) => abi::TransitionOutput::Future,
        ast::Type::Composite(comp_ty) if is_record(comp_ty, ctx) => abi::TransitionOutput::Record(abi::RecordRef {
            path: comp_ty.path.segments_iter().map(|s| s.to_string()).collect(),
            program: comp_ty.path.program().map(|s| s.to_string()),
        }),
        _ => abi::TransitionOutput::Plaintext(convert_plaintext(ty)),
    }
}

/// Checks if a composite type refers to a record.
fn is_record(comp_ty: &ast::CompositeType, ctx: &Ctx) -> bool {
    let name = comp_ty.path.identifier().name;

    // Check if it's defined in the current program scope
    if let Some((_, composite)) = ctx.scope.composites.iter().find(|(sym, _)| *sym == name) {
        return composite.is_record;
    }

    // Check if it's defined in a module
    for module in ctx.modules.values() {
        if let Some((_, composite)) = module.composites.iter().find(|(sym, _)| *sym == name) {
            return composite.is_record;
        }
    }

    // Check if it's defined in an imported stub
    if let Some(program) = comp_ty.path.program()
        && let Some(stub) = ctx.stubs.get(&program)
    {
        let found = match stub {
            ast::Stub::FromAleo { program, .. } => {
                program.composites.iter().find(|(sym, _)| *sym == name).map(|(_, c)| c.is_record)
            }
            ast::Stub::FromLeo { program, .. } => program
                .program_scopes
                .values()
                .flat_map(|scope| scope.composites.iter())
                .find(|(sym, _)| *sym == name)
                .map(|(_, c)| c.is_record),
        };
        if let Some(is_record) = found {
            return is_record;
        }
    }

    // Default to struct if not found (shouldn't happen after type checking)
    false
}

fn extract_array_length(expr: &Expression) -> u32 {
    match expr {
        Expression::Literal(Literal { variant: LiteralVariant::Integer(_, s), .. })
        | Expression::Literal(Literal { variant: LiteralVariant::Unsuffixed(s), .. }) => {
            s.parse().expect("array length should be a valid u32 after type checking and const eval")
        }
        _ => unreachable!("array length should be a literal after type checking and const eval"),
    }
}

fn convert_integer(int_ty: ast::IntegerType) -> abi::Primitive {
    match int_ty {
        ast::IntegerType::I8 => abi::Primitive::Int(abi::Int::I8),
        ast::IntegerType::I16 => abi::Primitive::Int(abi::Int::I16),
        ast::IntegerType::I32 => abi::Primitive::Int(abi::Int::I32),
        ast::IntegerType::I64 => abi::Primitive::Int(abi::Int::I64),
        ast::IntegerType::I128 => abi::Primitive::Int(abi::Int::I128),
        ast::IntegerType::U8 => abi::Primitive::UInt(abi::UInt::U8),
        ast::IntegerType::U16 => abi::Primitive::UInt(abi::UInt::U16),
        ast::IntegerType::U32 => abi::Primitive::UInt(abi::UInt::U32),
        ast::IntegerType::U64 => abi::Primitive::UInt(abi::UInt::U64),
        ast::IntegerType::U128 => abi::Primitive::UInt(abi::UInt::U128),
    }
}

/// Prunes types not referenced in the public interface (transitions, mappings, storage).
pub fn prune_non_interface_types(program: &mut abi::Program) {
    let mut used_types: HashSet<abi::Path> = HashSet::new();

    // Extract program name without .aleo suffix for comparison
    let program_name = program.program.strip_suffix(".aleo").unwrap_or(&program.program);

    // Phase 1: Collect from interface items
    for transition in &program.transitions {
        for input in &transition.inputs {
            collect_from_transition_input(&input.ty, program_name, &mut used_types);
        }
        for output in &transition.outputs {
            collect_from_transition_output(&output.ty, program_name, &mut used_types);
        }
    }

    for mapping in &program.mappings {
        collect_from_plaintext(&mapping.key, program_name, &mut used_types);
        collect_from_plaintext(&mapping.value, program_name, &mut used_types);
    }

    for storage_var in &program.storage_variables {
        collect_from_abi_storage_type(&storage_var.ty, program_name, &mut used_types);
    }

    // Phase 2: Collect transitive dependencies from structs and records
    collect_transitive_from_structs(&program.structs, program_name, &mut used_types);
    collect_transitive_from_records(&program.records, program_name, &mut used_types);

    // Phase 3: Prune
    program.structs.retain(|s| used_types.contains(&s.path));
    program.records.retain(|r| used_types.contains(&r.path));
}

/// Checks if a type reference is local to the current program.
fn is_local_type(type_program: Option<&str>, current_program: &str) -> bool {
    match type_program {
        None => true,
        Some(p) => p == current_program,
    }
}

fn collect_from_plaintext(ty: &abi::Plaintext, program_name: &str, used: &mut HashSet<abi::Path>) {
    match ty {
        abi::Plaintext::Struct(struct_ref) => {
            if is_local_type(struct_ref.program.as_deref(), program_name) {
                used.insert(struct_ref.path.clone());
            }
        }
        abi::Plaintext::Array(arr) => {
            collect_from_plaintext(&arr.element, program_name, used);
        }
        abi::Plaintext::Optional(opt) => {
            collect_from_plaintext(&opt.0, program_name, used);
        }
        abi::Plaintext::Primitive(_) => {}
    }
}

fn collect_from_abi_storage_type(ty: &abi::StorageType, program_name: &str, used: &mut HashSet<abi::Path>) {
    match ty {
        abi::StorageType::Plaintext(p) => collect_from_plaintext(p, program_name, used),
        abi::StorageType::Vector(inner) => collect_from_abi_storage_type(inner, program_name, used),
    }
}

fn collect_from_transition_input(ty: &abi::TransitionInput, program_name: &str, used: &mut HashSet<abi::Path>) {
    match ty {
        abi::TransitionInput::Plaintext(p) => collect_from_plaintext(p, program_name, used),
        abi::TransitionInput::Record(rec_ref) => {
            if is_local_type(rec_ref.program.as_deref(), program_name) {
                used.insert(rec_ref.path.clone());
            }
        }
    }
}

fn collect_from_transition_output(ty: &abi::TransitionOutput, program_name: &str, used: &mut HashSet<abi::Path>) {
    match ty {
        abi::TransitionOutput::Plaintext(p) => collect_from_plaintext(p, program_name, used),
        abi::TransitionOutput::Record(rec_ref) => {
            if is_local_type(rec_ref.program.as_deref(), program_name) {
                used.insert(rec_ref.path.clone());
            }
        }
        abi::TransitionOutput::Future => {}
    }
}

fn collect_transitive_from_structs(structs: &[abi::Struct], program_name: &str, used: &mut HashSet<abi::Path>) {
    let mut changed = true;
    while changed {
        changed = false;
        for s in structs {
            if used.contains(&s.path) {
                for field in &s.fields {
                    let before = used.len();
                    collect_from_plaintext(&field.ty, program_name, used);
                    if used.len() > before {
                        changed = true;
                    }
                }
            }
        }
    }
}

fn collect_transitive_from_records(records: &[abi::Record], program_name: &str, used: &mut HashSet<abi::Path>) {
    let mut changed = true;
    while changed {
        changed = false;
        for r in records {
            if used.contains(&r.path) {
                for field in &r.fields {
                    let before = used.len();
                    collect_from_plaintext(&field.ty, program_name, used);
                    if used.len() > before {
                        changed = true;
                    }
                }
            }
        }
    }
}
