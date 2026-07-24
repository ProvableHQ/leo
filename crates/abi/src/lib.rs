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
pub mod compatibility;
pub mod interfaces;

#[cfg(test)]
mod tests;

pub use leo_abi_types::*;

use leo_abi_types as abi;
use leo_ast::{self as ast, Expression, Literal, LiteralVariant};
use leo_span::Symbol;

use indexmap::IndexMap;
use std::collections::HashSet;

pub(crate) fn interface_ref_from_type(ty: &ast::TypeKind, current_program: &str) -> Option<abi::InterfaceRef> {
    let ast::TypeKind::Composite(ct) = ty else { return None };
    let loc = ct.path.try_global_location()?;
    let prog_str = loc.program.to_string();
    let program = if prog_str == current_program { None } else { Some(prog_str) };
    Some(abi::InterfaceRef { program, path: loc.path.iter().map(|s| s.to_string()).collect() })
}

struct Ctx<'a> {
    scope: &'a ast::ProgramScope,
    stubs: &'a IndexMap<Symbol, ast::Stub>,
    modules: &'a IndexMap<Vec<Symbol>, ast::Module>,
}

/// Generates the ABI for a Leo program.
///
/// The returned ABI is pruned to only include types that appear in the public
/// interface (functions, mappings, storage variables).
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

    let functions =
        scope.functions.iter().filter(|(_, f)| f.variant.is_entry()).map(|(_, f)| convert_function(f, &ctx)).collect();

    let views =
        scope.functions.iter().filter(|(_, f)| f.variant.is_view()).map(|(_, f)| convert_function(f, &ctx)).collect();

    let mut program = abi::Program { program, structs, records, mappings, storage_variables, functions, views };

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
    abi::StructField { name: member.identifier.name.to_string(), ty: convert_plaintext(member.type_.kind()) }
}

fn convert_record_field(member: &ast::Member) -> abi::RecordField {
    abi::RecordField {
        name: member.identifier.name.to_string(),
        ty: convert_plaintext(member.type_.kind()),
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
    abi::StorageVariable { name: sv.identifier.name.to_string(), ty: convert_storage_type(sv.type_.kind()) }
}

fn convert_storage_type(ty: &ast::TypeKind) -> abi::StorageType {
    match ty {
        ast::TypeKind::Vector(v) => abi::StorageType::Vector(Box::new(convert_storage_type(&v.element_type))),
        other => abi::StorageType::Plaintext(convert_plaintext(other)),
    }
}

fn convert_function(function: &ast::Function, ctx: &Ctx) -> abi::Function {
    let name = function.identifier.name.to_string();
    let is_view = function.variant.is_view();
    let inputs = function.input.iter().map(|i| convert_input(i, ctx, is_view)).collect();
    let outputs = function.output.iter().map(|o| convert_output(o, ctx, is_view)).collect();
    abi::Function { name, inputs, outputs }
}

fn convert_input(input: &ast::Input, ctx: &Ctx, is_view: bool) -> abi::FunctionInput {
    convert_function_input(input.type_.kind(), ctx, resolve_io_mode(input.mode, is_view))
}

fn convert_output(output: &ast::Output, ctx: &Ctx, is_view: bool) -> abi::FunctionOutput {
    convert_function_output(output.type_.kind(), ctx, resolve_io_mode(output.mode, is_view))
}

/// Converts a record-field visibility mode. Unmoded record fields lower to private, so they are
/// recorded as [`abi::Mode::Private`].
fn convert_mode(mode: ast::Mode) -> abi::Mode {
    match mode {
        ast::Mode::Constant => abi::Mode::Constant,
        ast::Mode::Public => abi::Mode::Public,
        ast::Mode::None | ast::Mode::Private => abi::Mode::Private,
    }
}

/// Resolves a function input/output visibility mode for the ABI. An unmoded item lowers to public
/// for view functions and private for transitions, matching code generation; the ABI records the
/// resolved visibility rather than a "none" placeholder.
pub(crate) fn resolve_io_mode(mode: ast::Mode, is_view: bool) -> abi::Mode {
    match mode {
        ast::Mode::Constant => abi::Mode::Constant,
        ast::Mode::Private => abi::Mode::Private,
        ast::Mode::Public => abi::Mode::Public,
        ast::Mode::None => {
            if is_view {
                abi::Mode::Public
            } else {
                abi::Mode::Private
            }
        }
    }
}

fn convert_plaintext(ty: &ast::TypeKind) -> abi::Plaintext {
    match ty {
        ast::TypeKind::Address => abi::Plaintext::Primitive(abi::Primitive::Address),
        ast::TypeKind::Boolean => abi::Plaintext::Primitive(abi::Primitive::Boolean),
        ast::TypeKind::Field => abi::Plaintext::Primitive(abi::Primitive::Field),
        ast::TypeKind::Group => abi::Plaintext::Primitive(abi::Primitive::Group),
        ast::TypeKind::Scalar => abi::Plaintext::Primitive(abi::Primitive::Scalar),
        ast::TypeKind::Identifier => abi::Plaintext::Primitive(abi::Primitive::Identifier),
        ast::TypeKind::Signature => abi::Plaintext::Primitive(abi::Primitive::Signature),
        ast::TypeKind::Integer(int_ty) => abi::Plaintext::Primitive(convert_integer(*int_ty)),
        ast::TypeKind::Array(arr_ty) => abi::Plaintext::Array(abi::Array {
            element: Box::new(convert_plaintext(arr_ty.element_type())),
            length: extract_array_length(&arr_ty.length),
        }),
        ast::TypeKind::Composite(comp_ty) => abi::Plaintext::Struct(abi::StructRef {
            path: comp_ty.path.segments_iter().map(|s| s.to_string()).collect(),
            program: comp_ty.path.program().map(|s| s.to_string()),
        }),
        ast::TypeKind::Optional(opt_ty) => {
            abi::Plaintext::Optional(abi::Optional(Box::new(convert_plaintext(&opt_ty.inner))))
        }
        // These types cannot appear in plaintext contexts:
        // - Tuple: not allowed in storage or transition inputs/outputs
        // - Vector: only allowed in storage variables (handled by convert_storage_type)
        // - Others: resolved or invalid after type checking
        ast::TypeKind::Future(_)
        | ast::TypeKind::Mapping(_)
        | ast::TypeKind::Tuple(_)
        | ast::TypeKind::Vector(_)
        | ast::TypeKind::String
        | ast::TypeKind::Unit
        | ast::TypeKind::Ident(_)
        | ast::TypeKind::Numeric
        | ast::TypeKind::DynRecord
        | ast::TypeKind::Err => {
            unreachable!("unexpected type in plaintext context: {ty}")
        }
    }
}

fn convert_function_input(ty: &ast::TypeKind, ctx: &Ctx, mode: abi::Mode) -> abi::FunctionInput {
    if let ast::TypeKind::DynRecord = ty {
        return abi::FunctionInput::DynamicRecord;
    }
    if let ast::TypeKind::Composite(comp_ty) = ty
        && is_record(comp_ty, ctx)
    {
        return abi::FunctionInput::Record(abi::RecordRef {
            path: comp_ty.path.segments_iter().map(|s| s.to_string()).collect(),
            program: comp_ty.path.program().map(|s| s.to_string()),
        });
    }
    abi::FunctionInput::Plaintext { ty: convert_plaintext(ty), mode }
}

fn convert_function_output(ty: &ast::TypeKind, ctx: &Ctx, mode: abi::Mode) -> abi::FunctionOutput {
    match ty {
        ast::TypeKind::Future(_) => abi::FunctionOutput::Final,
        ast::TypeKind::DynRecord => abi::FunctionOutput::DynamicRecord,
        ast::TypeKind::Composite(comp_ty) if is_record(comp_ty, ctx) => abi::FunctionOutput::Record(abi::RecordRef {
            path: comp_ty.path.segments_iter().map(|s| s.to_string()).collect(),
            program: comp_ty.path.program().map(|s| s.to_string()),
        }),
        _ => abi::FunctionOutput::Plaintext { ty: convert_plaintext(ty), mode },
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

            ast::Stub::FromLibrary { .. } => None,
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

/// Prunes types not referenced in the public interface (functions, mappings, storage).
pub fn prune_non_interface_types(program: &mut abi::Program) {
    let mut used_types: HashSet<abi::Path> = HashSet::new();

    // Extract program name without .aleo suffix for comparison
    let program_name = &program.program;

    // Phase 1: Collect from interface items
    for function in program.functions.iter().chain(program.views.iter()) {
        for input in &function.inputs {
            collect_from_function_input(input, program_name, &mut used_types);
        }
        for output in &function.outputs {
            collect_from_function_output(output, program_name, &mut used_types);
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

fn collect_from_function_input(ty: &abi::FunctionInput, program_name: &str, used: &mut HashSet<abi::Path>) {
    match ty {
        abi::FunctionInput::Plaintext { ty, .. } => collect_from_plaintext(ty, program_name, used),
        abi::FunctionInput::Record(rec_ref) => {
            if is_local_type(rec_ref.program.as_deref(), program_name) {
                used.insert(rec_ref.path.clone());
            }
        }
        abi::FunctionInput::DynamicRecord => {}
    }
}

fn collect_from_function_output(ty: &abi::FunctionOutput, program_name: &str, used: &mut HashSet<abi::Path>) {
    match ty {
        abi::FunctionOutput::Plaintext { ty, .. } => collect_from_plaintext(ty, program_name, used),
        abi::FunctionOutput::Record(rec_ref) => {
            if is_local_type(rec_ref.program.as_deref(), program_name) {
                used.insert(rec_ref.path.clone());
            }
        }
        abi::FunctionOutput::Final | abi::FunctionOutput::DynamicRecord => {}
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
