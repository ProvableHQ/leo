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

//! Interface ABI generation.
//!
//! Generates per-interface JSON ABIs from the Leo AST. Supports both programs
//! (emitting ABIs for locally defined + directly implemented interfaces) and
//! libraries (emitting ABIs for locally defined interfaces).

use crate::{
    collect_from_abi_storage_type,
    collect_from_function_input,
    collect_from_function_output,
    collect_from_plaintext,
    collect_transitive_from_records,
    collect_transitive_from_structs,
    convert_mode,
    convert_plaintext,
    convert_record,
    convert_record_field,
    convert_storage_type,
    convert_struct,
    interface_ref_from_type,
};

use leo_abi_types as abi;
use leo_ast::{self as ast};
use leo_span::Symbol;

use indexmap::IndexMap;
use std::collections::HashSet;

// ------------------------------------------------------------------------- //
// Public types
// ------------------------------------------------------------------------- //

/// A generated interface ABI together with its ownership information.
pub struct CompiledInterface {
    pub owner: InterfaceOwner,
    pub abi: abi::Interface,
}

/// Where the interface is defined relative to the unit being built.
pub enum InterfaceOwner {
    /// Defined in the primary program/library being built.
    /// Written to `build/interfaces/<module_path>/<Name>.json`.
    Local,
    /// Defined in an imported program or library.
    /// Written to `build/interfaces/<owner_program>/<module_path>/<Name>.json`.
    External { owner_program: String },
}

// ------------------------------------------------------------------------- //
// Entry points
// ------------------------------------------------------------------------- //

/// Emits interface ABIs relevant to the given program:
///
/// - All interfaces locally defined in the program's scope and modules.
/// - All interfaces directly implemented (via `scope.parents`), looked up in stubs.
/// - All external parent interfaces referenced transitively by any of the above.
pub fn generate_program_interfaces(ast: &ast::Program) -> Vec<CompiledInterface> {
    let scope = ast.program_scopes.values().next().unwrap();
    let program_str = scope.program_id.to_string();
    let program_sym = scope.program_id.as_symbol();
    let cs = CompositeSource::Program { scope, modules: &ast.modules, stubs: &ast.stubs };

    let mut result = Vec::new();
    let mut seen: HashSet<(Option<String>, Vec<String>)> = HashSet::new();

    // 1. Locally defined interfaces (top-level).
    for (_, iface) in &scope.interfaces {
        let abi = build_interface(iface, program_sym, &[], &cs);
        let key = (None, abi.path.clone());
        if seen.insert(key) {
            result.push(CompiledInterface { owner: InterfaceOwner::Local, abi });
        }
    }

    // 2. Locally defined interfaces (in modules).
    for (module_path, module) in &ast.modules {
        for (_, iface) in &module.interfaces {
            let abi = build_interface(iface, program_sym, module_path, &cs);
            let key = (None, abi.path.clone());
            if seen.insert(key) {
                result.push(CompiledInterface { owner: InterfaceOwner::Local, abi });
            }
        }
    }

    // 3. External interfaces: directly implemented + transitive parents.
    //
    // Use a worklist so that every external interface referenced in a `parents`
    // field (whether on the program itself or on a local/external interface) is
    // emitted together with its own transitive parents.
    let mut worklist: Vec<(Symbol, Vec<Symbol>)> = Vec::new();

    // Seed from program-level parent implementations (scope.parents).
    for (_, ty) in &scope.parents {
        if let ast::Type::Composite(ct) = ty
            && let Some(loc) = ct.path.try_global_location()
        {
            worklist.push((loc.program, loc.path.clone()));
        }
    }

    // Seed from parents of locally-defined interfaces.
    let local_ifaces = scope
        .interfaces
        .iter()
        .map(|(_, i)| i)
        .chain(ast.modules.values().flat_map(|m| m.interfaces.iter().map(|(_, i)| i)));
    for iface in local_ifaces {
        for (_, parent_ty) in &iface.parents {
            if let ast::Type::Composite(ct) = parent_ty
                && let Some(loc) = ct.path.try_global_location()
            {
                worklist.push((loc.program, loc.path.clone()));
            }
        }
    }

    while let Some((ext_program, iface_path)) = worklist.pop() {
        let owner_str = ext_program.to_string();

        // Skip if local (already covered in steps 1 and 2).
        if owner_str == program_str {
            continue;
        }

        let Some(stub) = ast.stubs.get(&ext_program) else { continue };
        let Some(iface) = find_interface_in_stub(stub, &iface_path) else { continue };

        let ext_cs = composite_source_for_stub(stub);
        let module_path: Vec<Symbol> = iface_path[..iface_path.len().saturating_sub(1)].to_vec();
        let abi = build_interface(iface, ext_program, &module_path, &ext_cs);
        let key = (Some(owner_str.clone()), abi.path.clone());
        if seen.insert(key) {
            result.push(CompiledInterface { owner: InterfaceOwner::External { owner_program: owner_str }, abi });
            // Enqueue this interface's parents for processing.
            for (_, parent_ty) in &iface.parents {
                let ast::Type::Composite(ct) = parent_ty else { continue };
                let Some(parent_loc) = ct.path.try_global_location() else { continue };
                worklist.push((parent_loc.program, parent_loc.path.clone()));
            }
        }
    }

    result
}

/// Emits interface ABIs for every interface locally defined in the library
/// (top-level + modules).
pub fn generate_library_interfaces(library: &ast::Library) -> Vec<CompiledInterface> {
    let cs = CompositeSource::Library { library, stubs: &library.stubs };
    let mut result = Vec::new();

    for (_, iface) in &library.interfaces {
        let abi = build_interface(iface, library.name, &[], &cs);
        result.push(CompiledInterface { owner: InterfaceOwner::Local, abi });
    }

    for (module_path, module) in &library.modules {
        for (_, iface) in &module.interfaces {
            let abi = build_interface(iface, library.name, module_path, &cs);
            result.push(CompiledInterface { owner: InterfaceOwner::Local, abi });
        }
    }

    result
}

// ------------------------------------------------------------------------- //
// CompositeSource - unified record/struct lookup
// ------------------------------------------------------------------------- //

/// Abstracts where to look up composite definitions (structs and records).
enum CompositeSource<'a> {
    Program {
        scope: &'a ast::ProgramScope,
        modules: &'a IndexMap<Vec<Symbol>, ast::Module>,
        stubs: &'a IndexMap<Symbol, ast::Stub>,
    },
    Library {
        library: &'a ast::Library,
        stubs: &'a IndexMap<Symbol, ast::Stub>,
    },
}

impl<'a> CompositeSource<'a> {
    /// Checks if a composite type refers to a record.
    fn is_record(&self, comp_ty: &ast::CompositeType) -> bool {
        let name = comp_ty.path.identifier().name;

        // Check local composites.
        match self {
            CompositeSource::Program { scope, modules, .. } => {
                if let Some((_, c)) = scope.composites.iter().find(|(sym, _)| *sym == name) {
                    return c.is_record;
                }
                for module in modules.values() {
                    if let Some((_, c)) = module.composites.iter().find(|(sym, _)| *sym == name) {
                        return c.is_record;
                    }
                }
            }
            CompositeSource::Library { library, .. } => {
                if let Some((_, c)) = library.structs.iter().find(|(sym, _)| *sym == name) {
                    return c.is_record;
                }
                for module in library.modules.values() {
                    if let Some((_, c)) = module.composites.iter().find(|(sym, _)| *sym == name) {
                        return c.is_record;
                    }
                }
            }
        }

        // Check stubs.
        let stubs = match self {
            CompositeSource::Program { stubs, .. } | CompositeSource::Library { stubs, .. } => stubs,
        };
        if let Some(program) = comp_ty.path.program()
            && let Some(stub) = stubs.get(&program)
            && let Some(is_rec) = find_is_record_in_stub(stub, name)
        {
            return is_rec;
        }

        false
    }

    /// Collects all struct (non-record) composites as ABI structs.
    fn all_structs(&self) -> Vec<abi::Struct> {
        let mut out = Vec::new();
        match self {
            CompositeSource::Program { scope, modules, .. } => {
                out.extend(scope.composites.iter().filter(|(_, c)| !c.is_record).map(|(_, c)| convert_struct(c, &[])));
                for (mp, module) in *modules {
                    out.extend(
                        module.composites.iter().filter(|(_, c)| !c.is_record).map(|(_, c)| convert_struct(c, mp)),
                    );
                }
            }
            CompositeSource::Library { library, .. } => {
                out.extend(library.structs.iter().filter(|(_, c)| !c.is_record).map(|(_, c)| convert_struct(c, &[])));
                for (mp, module) in &library.modules {
                    out.extend(
                        module.composites.iter().filter(|(_, c)| !c.is_record).map(|(_, c)| convert_struct(c, mp)),
                    );
                }
            }
        }
        out
    }

    /// Collects all record composites as ABI records.
    fn all_records(&self) -> Vec<abi::Record> {
        let mut out = Vec::new();
        match self {
            CompositeSource::Program { scope, modules, .. } => {
                out.extend(scope.composites.iter().filter(|(_, c)| c.is_record).map(|(_, c)| convert_record(c, &[])));
                for (mp, module) in *modules {
                    out.extend(
                        module.composites.iter().filter(|(_, c)| c.is_record).map(|(_, c)| convert_record(c, mp)),
                    );
                }
            }
            CompositeSource::Library { library, .. } => {
                out.extend(library.structs.iter().filter(|(_, c)| c.is_record).map(|(_, c)| convert_record(c, &[])));
                for (mp, module) in &library.modules {
                    out.extend(
                        module.composites.iter().filter(|(_, c)| c.is_record).map(|(_, c)| convert_record(c, mp)),
                    );
                }
            }
        }
        out
    }
}

/// Checks if a name is a record in a stub.
fn find_is_record_in_stub(stub: &ast::Stub, name: Symbol) -> Option<bool> {
    match stub {
        ast::Stub::FromAleo { program, .. } => {
            program.composites.iter().find(|(sym, _)| *sym == name).map(|(_, c)| c.is_record)
        }
        ast::Stub::FromLeo { program, .. } => program
            .program_scopes
            .values()
            .flat_map(|scope| scope.composites.iter())
            .find(|(sym, _)| *sym == name)
            .map(|(_, c)| c.is_record),
        ast::Stub::FromLibrary { library, .. } => {
            library.structs.iter().find(|(sym, _)| *sym == name).map(|(_, c)| c.is_record)
        }
    }
}

/// Builds a `CompositeSource` for a stub (for looking up composites in an external dependency).
fn composite_source_for_stub(stub: &ast::Stub) -> CompositeSource<'_> {
    match stub {
        ast::Stub::FromLeo { program, .. } => {
            let scope = program.program_scopes.values().next().unwrap();
            CompositeSource::Program { scope, modules: &program.modules, stubs: &program.stubs }
        }
        ast::Stub::FromLibrary { library, .. } => CompositeSource::Library { library, stubs: &library.stubs },
        ast::Stub::FromAleo { .. } => {
            // Aleo stubs can't define interfaces, so this shouldn't be reached.
            // Use an empty library as a placeholder.
            unreachable!("Aleo stubs do not contain interfaces")
        }
    }
}

// ------------------------------------------------------------------------- //
// Interface lookup in stubs
// ------------------------------------------------------------------------- //

/// Finds an interface definition in a stub by its path segments.
fn find_interface_in_stub<'a>(stub: &'a ast::Stub, path: &[Symbol]) -> Option<&'a ast::Interface> {
    let (&iface_name, module_path) = path.split_last()?;
    match stub {
        ast::Stub::FromLeo { program, .. } => {
            if module_path.is_empty() {
                program
                    .program_scopes
                    .values()
                    .flat_map(|scope| scope.interfaces.iter())
                    .find(|(name, _)| *name == iface_name)
                    .map(|(_, iface)| iface)
            } else {
                program.modules.iter().find(|(mp, _)| mp.as_slice() == module_path).and_then(|(_, module)| {
                    module.interfaces.iter().find(|(name, _)| *name == iface_name).map(|(_, iface)| iface)
                })
            }
        }
        ast::Stub::FromLibrary { library, .. } => {
            if module_path.is_empty() {
                library.interfaces.iter().find(|(name, _)| *name == iface_name).map(|(_, iface)| iface)
            } else {
                library.modules.iter().find(|(mp, _)| mp.as_slice() == module_path).and_then(|(_, module)| {
                    module.interfaces.iter().find(|(name, _)| *name == iface_name).map(|(_, iface)| iface)
                })
            }
        }
        ast::Stub::FromAleo { .. } => None,
    }
}

// ------------------------------------------------------------------------- //
// Building a single interface ABI
// ------------------------------------------------------------------------- //

/// Converts an AST interface to an ABI interface, collecting transitively
/// referenced struct definitions from the composite source.
fn build_interface(
    iface: &ast::Interface,
    owning_program: Symbol,
    module_path: &[Symbol],
    cs: &CompositeSource<'_>,
) -> abi::Interface {
    let name = iface.identifier.name.to_string();
    let program = owning_program.to_string();

    let mut path: Vec<String> = module_path.iter().map(|s| s.to_string()).collect();
    path.push(name.clone());

    let parents: Vec<abi::InterfaceRef> =
        iface.parents.iter().filter_map(|(_, ty)| interface_ref_from_type(ty, &program)).collect();

    let functions: Vec<abi::Function> =
        iface.functions.iter().map(|(_, proto)| convert_function_prototype(proto, iface, cs)).collect();

    let records: Vec<abi::Record> = iface.records.iter().map(|(_, proto)| convert_record_prototype(proto)).collect();

    let mappings: Vec<abi::Mapping> = iface.mappings.iter().map(convert_mapping_prototype).collect();

    let storage_variables: Vec<abi::StorageVariable> =
        iface.storages.iter().map(convert_storage_variable_prototype).collect();

    // Collect transitively referenced structs from the composite source.
    let structs = collect_interface_structs(&program, &functions, &records, &mappings, &storage_variables, cs);

    abi::Interface { name, program, path, parents, functions, records, mappings, storage_variables, structs }
}

// ------------------------------------------------------------------------- //
// Prototype -> ABI converters
// ------------------------------------------------------------------------- //

fn convert_function_prototype(
    proto: &ast::FunctionPrototype,
    iface: &ast::Interface,
    cs: &CompositeSource<'_>,
) -> abi::Function {
    abi::Function {
        name: proto.identifier.name.to_string(),
        is_final: proto.output.iter().any(|o| matches!(o.type_, ast::Type::Future(_))),
        const_parameters: proto.const_parameters.iter().map(convert_const_parameter).collect(),
        inputs: proto.input.iter().map(|i| convert_input(i, iface, cs)).collect(),
        outputs: proto.output.iter().map(|o| convert_output(o, iface, cs)).collect(),
    }
}

fn convert_record_prototype(proto: &ast::RecordPrototype) -> abi::Record {
    abi::Record {
        path: vec![proto.identifier.name.to_string()],
        fields: proto.members.iter().map(convert_record_field).collect(),
    }
}

fn convert_mapping_prototype(proto: &ast::MappingPrototype) -> abi::Mapping {
    abi::Mapping {
        name: proto.identifier.name.to_string(),
        key: convert_plaintext(&proto.key_type),
        value: convert_plaintext(&proto.value_type),
    }
}

fn convert_storage_variable_prototype(proto: &ast::StorageVariablePrototype) -> abi::StorageVariable {
    abi::StorageVariable { name: proto.identifier.name.to_string(), ty: convert_storage_type(&proto.type_) }
}

fn convert_const_parameter(cp: &ast::ConstParameter) -> abi::ConstParameter {
    abi::ConstParameter { name: cp.identifier.name.to_string(), ty: convert_plaintext(&cp.type_) }
}

fn convert_input(input: &ast::Input, iface: &ast::Interface, cs: &CompositeSource<'_>) -> abi::Input {
    abi::Input {
        name: input.identifier.name.to_string(),
        ty: convert_function_input(&input.type_, iface, cs),
        mode: convert_mode(input.mode),
    }
}

fn convert_output(output: &ast::Output, iface: &ast::Interface, cs: &CompositeSource<'_>) -> abi::Output {
    abi::Output { ty: convert_function_output(&output.type_, iface, cs), mode: convert_mode(output.mode) }
}

/// Checks if a composite type is a record in the context of an interface.
///
/// Checks the interface's own record prototypes first, then falls back to the
/// composite source for records from the surrounding scope.
fn is_record_for_interface(comp_ty: &ast::CompositeType, iface: &ast::Interface, cs: &CompositeSource<'_>) -> bool {
    // Check the interface's own records.
    let name = comp_ty.path.identifier().name;
    if iface.records.iter().any(|(n, _)| *n == name) {
        return true;
    }
    cs.is_record(comp_ty)
}

fn convert_function_input(ty: &ast::Type, iface: &ast::Interface, cs: &CompositeSource<'_>) -> abi::FunctionInput {
    if let ast::Type::DynRecord = ty {
        return abi::FunctionInput::DynamicRecord;
    }
    if let ast::Type::Composite(comp_ty) = ty
        && is_record_for_interface(comp_ty, iface, cs)
    {
        return abi::FunctionInput::Record(abi::RecordRef {
            path: comp_ty.path.segments_iter().map(|s| s.to_string()).collect(),
            program: comp_ty.path.program().map(|s| s.to_string()),
        });
    }
    abi::FunctionInput::Plaintext(convert_plaintext(ty))
}

fn convert_function_output(ty: &ast::Type, iface: &ast::Interface, cs: &CompositeSource<'_>) -> abi::FunctionOutput {
    match ty {
        ast::Type::Future(_) => abi::FunctionOutput::Final,
        ast::Type::DynRecord => abi::FunctionOutput::DynamicRecord,
        ast::Type::Composite(comp_ty) if is_record_for_interface(comp_ty, iface, cs) => {
            abi::FunctionOutput::Record(abi::RecordRef {
                path: comp_ty.path.segments_iter().map(|s| s.to_string()).collect(),
                program: comp_ty.path.program().map(|s| s.to_string()),
            })
        }
        _ => abi::FunctionOutput::Plaintext(convert_plaintext(ty)),
    }
}

// ------------------------------------------------------------------------- //
// Transitive struct collection
// ------------------------------------------------------------------------- //

/// Collects struct definitions transitively referenced by an interface's items.
///
/// Only includes structs defined in the same program as the interface; external
/// struct references remain as `StructRef` with a program field.
fn collect_interface_structs(
    program_name: &str,
    functions: &[abi::Function],
    records: &[abi::Record],
    mappings: &[abi::Mapping],
    storage_variables: &[abi::StorageVariable],
    cs: &CompositeSource<'_>,
) -> Vec<abi::Struct> {
    let mut used_types: HashSet<abi::Path> = HashSet::new();

    // Seed from functions.
    for function in functions {
        for cp in &function.const_parameters {
            collect_from_plaintext(&cp.ty, program_name, &mut used_types);
        }
        for input in &function.inputs {
            collect_from_function_input(&input.ty, program_name, &mut used_types);
        }
        for output in &function.outputs {
            collect_from_function_output(&output.ty, program_name, &mut used_types);
        }
    }

    // Seed from records.
    for record in records {
        for field in &record.fields {
            collect_from_plaintext(&field.ty, program_name, &mut used_types);
        }
    }

    // Seed from mappings.
    for mapping in mappings {
        collect_from_plaintext(&mapping.key, program_name, &mut used_types);
        collect_from_plaintext(&mapping.value, program_name, &mut used_types);
    }

    // Seed from storage variables.
    for sv in storage_variables {
        collect_from_abi_storage_type(&sv.ty, program_name, &mut used_types);
    }

    // Collect all composites from the source, then close transitively.
    let all_structs = cs.all_structs();
    let all_records = cs.all_records();
    collect_transitive_from_structs(&all_structs, program_name, &mut used_types);
    collect_transitive_from_records(&all_records, program_name, &mut used_types);

    all_structs.into_iter().filter(|s| used_types.contains(&s.path)).collect()
}
