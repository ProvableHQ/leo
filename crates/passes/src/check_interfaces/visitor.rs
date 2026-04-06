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

use crate::CompilerState;

use leo_ast::{
    AstVisitor,
    Composite,
    CompositeType,
    Function,
    FunctionPrototype,
    Location,
    Mapping,
    Member,
    ProgramScope,
    ProgramVisitor,
    RecordPrototype,
    StorageVariable,
    Type,
};
use leo_errors::{CheckInterfacesError, Color, Label};
use leo_span::{Span, Symbol, sym};

use indexmap::{IndexMap, IndexSet};
use leo_ast::common::{DiGraph, DiGraphError};

/// A flattened interface with all inherited members collected.
#[derive(Clone, Debug)]
struct FlattenedInterface {
    functions: Vec<(Symbol, FunctionPrototype)>,
    records: Vec<(Symbol, RecordPrototype)>,
    mappings: Vec<Mapping>,
    storages: Vec<StorageVariable>,
}

pub struct CheckInterfacesVisitor<'a> {
    pub state: &'a mut CompilerState,
    /// Current program name being processed.
    current_program: Symbol,
    /// Cache of flattened interfaces (with all inherited members).
    flattened_interfaces: IndexMap<Location, FlattenedInterface>,
    /// Interface inheritance graph
    inheritance_graph: DiGraph<Location>,
}

impl<'a> CheckInterfacesVisitor<'a> {
    pub fn new(state: &'a mut CompilerState) -> Self {
        Self {
            state,
            current_program: Symbol::intern(""),
            flattened_interfaces: IndexMap::new(),
            inheritance_graph: DiGraph::default(),
        }
    }

    /// Flatten an interface by collecting all inherited members.
    /// Detects conflicts during flattening. Assumes cycles have already been checked.
    fn flatten_interface(&mut self, location: &Location, location_span: Span) -> Option<FlattenedInterface> {
        // Check cache first.
        if let Some(flattened) = self.flattened_interfaces.get(location) {
            return Some(flattened.clone());
        }

        let Some(interface) = self.state.symbol_table.lookup_interface(self.current_program, location) else {
            self.state.handler.emit_err(CheckInterfacesError::interface_not_found(location, location_span));
            return None;
        };

        let interface_name = interface.identifier.name;
        let interface_span = interface.span;

        // Start with the interface's own members.
        let mut flattened = FlattenedInterface {
            functions: interface.functions.clone(),
            records: interface.records.clone(),
            mappings: interface.mappings.clone(),
            storages: interface.storages.clone(),
        };

        // Merge members from all parent interfaces (supports multiple inheritance).

        let all_parents = self.inheritance_graph.transitive_closure(location);

        for parent_location in &all_parents {
            let Some(parent_interface) =
                self.state.symbol_table.lookup_interface(self.current_program, parent_location)
            else {
                self.state.handler.emit_err(CheckInterfacesError::interface_not_found(parent_location, interface_span));
                return None;
            };

            let parent_interface_name = parent_interface.identifier.name;
            let parent_interface_span = parent_interface.identifier.span;

            let parent_flattened = self.flatten_interface(parent_location, parent_interface_span)?;

            // Merge parent functions, checking for conflicts.
            for (name, parent_func) in &parent_flattened.functions {
                if let Some((_, existing_func)) = flattened.functions.iter().find(|(n, _)| n == name) {
                    // Same name exists - check if compatible.
                    if !Self::prototypes_match(existing_func, parent_func) {
                        self.state.handler.emit_err(CheckInterfacesError::conflicting_interface_member(
                            name,
                            interface_name,
                            parent_interface_name,
                            interface_span,
                        ));
                        return None;
                    }
                    // Compatible - no action needed, child's version takes precedence.
                } else {
                    // Add parent's function.
                    flattened.functions.push((*name, parent_func.clone()));
                }
            }

            // Merge parent records, checking for field conflicts.
            for (name, parent_record) in &parent_flattened.records {
                if let Some((_, existing_record)) = flattened.records.iter().find(|(n, _)| n == name) {
                    // Same record name exists - check if fields are compatible.
                    if !Self::record_fields_compatible(existing_record, parent_record) {
                        // Find the specific field that conflicts for the error message.
                        for parent_member in &parent_record.members {
                            let child_member = existing_record
                                .members
                                .iter()
                                .find(|m| m.identifier.name == parent_member.identifier.name);
                            match child_member {
                                None => {
                                    // Parent has a field that child doesn't have.
                                    self.state.handler.emit_err(
                                        CheckInterfacesError::conflicting_record_field(
                                            parent_member.identifier.name,
                                            name,
                                            interface_name,
                                            parent_interface_name,
                                            interface_span,
                                        )
                                        .with_labels(vec![
                                            Label::new(
                                                format!("defined in `{parent_interface_name}` here"),
                                                parent_member.span,
                                            )
                                            .with_color(Color::Blue),
                                            Label::new("conflict detected here", interface_span),
                                        ]),
                                    );
                                    return None;
                                }
                                Some(cm)
                                    if !cm.type_.eq_user(&parent_member.type_)
                                        || !cm.mode.eq_user(&parent_member.mode) =>
                                {
                                    self.state.handler.emit_err(
                                        CheckInterfacesError::conflicting_record_field(
                                            parent_member.identifier.name,
                                            name,
                                            interface_name,
                                            parent_interface_name,
                                            interface_span,
                                        )
                                        .with_labels(vec![
                                            Label::new(
                                                format!("defined in `{parent_interface_name}` here"),
                                                parent_member.span,
                                            )
                                            .with_color(Color::Blue),
                                            Label::new("conflicts with definition here", cm.span)
                                                .with_color(Color::Blue),
                                            Label::new("conflict detected here", interface_span),
                                        ]),
                                    );
                                    return None;
                                }
                                _ => {} // Field matches, continue checking.
                            }
                        }
                    }
                    // Compatible or child is superset - child's version takes precedence.
                } else {
                    // Add parent's record.
                    flattened.records.push((*name, parent_record.clone()));
                }
            }

            // Merge parent mappings, checking for conflicts.
            for parent_mapping in &parent_flattened.mappings {
                if let Some(existing_mapping) =
                    flattened.mappings.iter().find(|m| m.identifier.name == parent_mapping.identifier.name)
                {
                    // Same name exists - check if types are compatible.
                    if !existing_mapping.key_type.eq_user(&parent_mapping.key_type)
                        || !existing_mapping.value_type.eq_user(&parent_mapping.value_type)
                    {
                        self.state.handler.emit_err(CheckInterfacesError::conflicting_interface_member(
                            parent_mapping.identifier.name,
                            interface_name,
                            parent_interface_name,
                            interface_span,
                        ));
                        return None;
                    }
                    // Compatible - no action needed, child's version takes precedence.
                } else {
                    // Add parent's mapping.
                    flattened.mappings.push(parent_mapping.clone());
                }
            }

            // Merge parent storages, checking for conflicts.
            for parent_storage in &parent_flattened.storages {
                if let Some(existing_storage) =
                    flattened.storages.iter().find(|s| s.identifier.name == parent_storage.identifier.name)
                {
                    // Same name exists - check if types are compatible.
                    if !existing_storage.type_.eq_user(&parent_storage.type_) {
                        self.state.handler.emit_err(CheckInterfacesError::conflicting_interface_member(
                            parent_storage.identifier.name,
                            interface_name,
                            parent_interface_name,
                            interface_span,
                        ));
                        return None;
                    }
                    // Compatible - no action needed, child's version takes precedence.
                } else {
                    // Add parent's storage.
                    flattened.storages.push(parent_storage.clone());
                }
            }
        }

        // Cache the result.
        self.flattened_interfaces.insert(location.clone(), flattened.clone());
        Some(flattened)
    }

    /// Validate that a program implements all interface requirements.
    fn check_program_implements_interface(
        &mut self,
        program_scope: &ProgramScope,
        interface_location: &Location,
        interface_span: Span,
    ) {
        let program_name = program_scope.program_id.as_symbol();

        // Get the flattened interface (with all inherited members).
        let flattened = match self.flatten_interface(interface_location, interface_span) {
            Some(f) => f,
            None => return, // Error already emitted.
        };

        // Check all required functions are implemented.
        for (func_name, required_proto) in &flattened.functions {
            let func_location = Location::new(program_name, vec![*func_name]);

            match self.state.symbol_table.lookup_function(program_name, &func_location) {
                Some(func_symbol) => {
                    // Function exists - check signature matches exactly.
                    if !Self::function_matches_prototype(&func_symbol.function, required_proto) {
                        self.state.handler.emit_err(CheckInterfacesError::signature_mismatch(
                            func_name,
                            interface_location,
                            Self::format_prototype_signature(required_proto),
                            Self::format_function_signature(&func_symbol.function),
                            func_symbol.function.span,
                        ));
                    }
                }
                None => {
                    self.state.handler.emit_err(CheckInterfacesError::missing_interface_function(
                        func_name,
                        interface_location,
                        program_name,
                        program_scope.span,
                    ));
                }
            }
        }

        // Check all required records are declared with required fields.
        for (record_name, required_record) in &flattened.records {
            let record_location = Location::new(program_name, vec![*record_name]);

            match self.state.symbol_table.lookup_record(program_name, &record_location) {
                Some(program_record) => {
                    // Record exists - check that all required fields are present with correct types.
                    if let Some((field_name, required_member, found_member)) =
                        Self::find_record_field_mismatch(required_record, program_record)
                    {
                        match found_member {
                            None => {
                                // Field is missing.
                                self.state.handler.emit_err(
                                    CheckInterfacesError::record_field_missing(
                                        field_name,
                                        record_name,
                                        interface_location,
                                        program_name,
                                        program_record.span,
                                    )
                                    .with_labels(vec![
                                        Label::new("required by interface here", required_member.span)
                                            .with_color(Color::Blue),
                                        Label::new(
                                            format!("record is missing field `{field_name}`"),
                                            program_record.span,
                                        ),
                                    ]),
                                );
                            }
                            Some(actual) => {
                                // Field exists but type or mode doesn't match.
                                let expected = format!("{} {}", required_member.mode, required_member.type_);
                                let found = format!("{} {}", actual.mode, actual.type_);
                                self.state.handler.emit_err(
                                    CheckInterfacesError::record_field_type_mismatch(
                                        field_name,
                                        record_name,
                                        interface_location,
                                        expected,
                                        found,
                                        actual.span,
                                    )
                                    .with_labels(vec![
                                        Label::new("expected by interface here", required_member.span)
                                            .with_color(Color::Blue),
                                        Label::new("type mismatch here", actual.span),
                                    ]),
                                );
                            }
                        }
                    }
                }
                None => {
                    self.state.handler.emit_err(CheckInterfacesError::missing_interface_record(
                        record_name,
                        interface_location,
                        program_name,
                        program_scope.span,
                    ));
                }
            }
        }

        // Check all required mappings are declared with correct types.
        for required_mapping in &flattened.mappings {
            let mapping_name = required_mapping.identifier.name;
            match program_scope.mappings.iter().find(|(name, _)| *name == mapping_name) {
                Some((_, program_mapping)) => {
                    // Mapping exists - check types match.
                    if !program_mapping.key_type.eq_user(&required_mapping.key_type)
                        || !program_mapping.value_type.eq_user(&required_mapping.value_type)
                    {
                        self.state.handler.emit_err(CheckInterfacesError::mapping_type_mismatch(
                            mapping_name,
                            interface_location,
                            &required_mapping.key_type,
                            &required_mapping.value_type,
                            &program_mapping.key_type,
                            &program_mapping.value_type,
                            program_mapping.span,
                        ));
                    }
                }
                None => {
                    self.state.handler.emit_err(CheckInterfacesError::missing_interface_mapping(
                        mapping_name,
                        interface_location,
                        program_name,
                        program_scope.span,
                    ));
                }
            }
        }

        // Check all required storage variables are declared with correct types.
        for required_storage in &flattened.storages {
            let storage_name = required_storage.identifier.name;
            match program_scope.storage_variables.iter().find(|(name, _)| *name == storage_name) {
                Some((_, program_storage)) => {
                    // Storage exists - check type matches.
                    if !program_storage.type_.eq_user(&required_storage.type_) {
                        self.state.handler.emit_err(CheckInterfacesError::storage_type_mismatch(
                            storage_name,
                            interface_location,
                            &required_storage.type_,
                            &program_storage.type_,
                            program_storage.span,
                        ));
                    }
                }
                None => {
                    self.state.handler.emit_err(CheckInterfacesError::missing_interface_storage(
                        storage_name,
                        interface_location,
                        program_name,
                        program_scope.span,
                    ));
                }
            }
        }
    }

    /// Check if two FunctionPrototypes have matching signatures.
    fn prototypes_match(a: &FunctionPrototype, b: &FunctionPrototype) -> bool {
        // Input parameters must match exactly.
        a.input.len() == b.input.len() &&
        a.input.iter().zip(b.input.iter()).all(|(input_a, input_b)| {
            // Parameter names must match.
            input_a.identifier.name == input_b.identifier.name &&
            // Parameter types must match.
            input_a.type_.eq_user(&input_b.type_) &&
            // Parameter modes must match.
            input_a.mode.eq_user(&input_b.mode)
        }) &&

        // Output must match.
        a.output.len() == b.output.len() &&
        a.output.iter().zip(b.output.iter()).all(|(output_a, output_b)| output_a.type_.eq_user(&output_b.type_) && output_a.mode.eq_user(&output_b.mode)) &&

        // Const parameters must match.
        a.const_parameters.len() == b.const_parameters.len() &&
        a.const_parameters.iter().zip(b.const_parameters.iter()).all(|(const_a, const_b)| const_a.type_.eq_user(&const_b.type_)) &&


        //TODO: we may want to check certain annotations, but they are not significant yet
        // // Annotations must match.
        // a.annotations.len() == b.annotations.len() &&
        // a.annotations.iter().zip(b.annotations.iter()).all(|(ann_a, ann_b)| ann_a == ann_b) &&

        // Output type must match (including Final).
        a.output_type.eq_user(&b.output_type)
    }

    /// Check if a Function matches a FunctionPrototype exactly.
    fn function_matches_prototype(func: &Function, proto: &FunctionPrototype) -> bool {
        // Input parameters must match exactly.
        func.input.len() == proto.input.len() &&

        func.input.iter().zip(proto.input.iter()).all(|(func_input, proto_input)| {
            // Parameter names must match.
            func_input.identifier.name == proto_input.identifier.name &&
            // Parameter types must match.
            func_input.type_.eq_user(&proto_input.type_) &&
            // Parameter modes must match.
            func_input.mode.eq_user(&proto_input.mode)
        }) &&

        // Output must match.
        func.output.len() == proto.output.len() &&

        func.output.iter().zip(proto.output.iter()).all(
            |(func_output, proto_output)| func_output.type_.eq_user(&proto_output.type_) && func_output.mode.eq_user(&proto_output.mode)) &&

        // Const parameters must match.
        func.const_parameters.len() == proto.const_parameters.len() &&
        func.const_parameters.iter().zip(proto.const_parameters.iter()).all(|(func_const, proto_const)| func_const.type_.eq_user(&proto_const.type_)) &&

        //TODO: we may want to check certain annotations, but they are not significant yet
        // // Annotations must match.
        // func.annotations.len() == proto.annotations.len() &&
        // func.annotations.iter().zip(proto.annotations.iter()).all(|(ann_func, ann_proto)| ann_func == ann_proto) &&

        // Output type must match (including Final).
        func.output_type.eq_user(&proto.output_type)
    }

    fn format_prototype_signature(proto: &FunctionPrototype) -> String {
        let inputs: Vec<String> = proto.input.iter().map(|i| format!("{}: {}", i.identifier.name, i.type_)).collect();
        format!(
            "{}fn {}({}) -> {}",
            proto.annotations.iter().map(|ann| format!("{ann}\n")).collect::<Vec<String>>().join(""),
            proto.identifier.name,
            inputs.join(", "),
            proto.output_type
        )
    }

    fn format_function_signature(func: &Function) -> String {
        let inputs: Vec<String> = func.input.iter().map(|i| format!("{}: {}", i.identifier.name, i.type_)).collect();
        format!(
            "{}fn {}({}) -> {}",
            func.annotations.iter().map(|ann| format!("{ann}\n")).collect::<Vec<String>>().join(""),
            func.identifier.name,
            inputs.join(", "),
            func.output_type
        )
    }

    /// Check if all parent record fields exist in child with matching types and modes.
    fn record_fields_compatible(child: &RecordPrototype, parent: &RecordPrototype) -> bool {
        parent.members.iter().all(|parent_member| {
            child.members.iter().any(|child_member| {
                child_member.identifier.name == parent_member.identifier.name
                    && child_member.type_.eq_user(&parent_member.type_)
                    && child_member.mode.eq_user(&parent_member.mode)
            })
        })
    }

    /// Find the first mismatching field between a required record prototype and an actual record.
    /// Returns Some((field_name, expected_member, found_member_or_none)) if mismatch found.
    fn find_record_field_mismatch<'b>(
        required: &'b RecordPrototype,
        actual: &'b Composite,
    ) -> Option<(Symbol, &'b Member, Option<&'b Member>)> {
        for required_member in &required.members {
            let found = actual.members.iter().find(|m| m.identifier.name == required_member.identifier.name);
            match found {
                None => return Some((required_member.identifier.name, required_member, None)),
                Some(actual_member) => {
                    if !actual_member.type_.eq_user(&required_member.type_)
                        || !actual_member.mode.eq_user(&required_member.mode)
                    {
                        return Some((required_member.identifier.name, required_member, Some(actual_member)));
                    }
                }
            }
        }
        None
    }

    /// Validate that record prototypes don't specify `owner` with a type other than `address`.
    fn validate_record_prototypes(&mut self, input: &ProgramScope) {
        for (_, interface) in &input.interfaces {
            for (_, record_proto) in &interface.records {
                for member in &record_proto.members {
                    if member.identifier.name == sym::owner && member.type_ != Type::Address {
                        self.state.handler.emit_err(CheckInterfacesError::record_prototype_owner_wrong_type(
                            record_proto.identifier.name,
                            &member.type_,
                            member.span,
                        ));
                    }
                }
            }
        }
    }

    fn build_inheritance_graph(&mut self, input: &ProgramScope) {
        // Populate graph with current program interfaces
        let mut queue: IndexSet<(Location, Span)> = IndexSet::new();
        let mut processed: IndexSet<Location> = IndexSet::new();
        for (_, interface) in &input.interfaces {
            let location = Location::new(self.current_program, vec![interface.identifier.name]);
            let span = interface.identifier.span;
            queue.insert((location, span));
        }

        while let Some((location, location_span)) = queue.pop() {
            if processed.contains(&location) {
                continue;
            }
            self.inheritance_graph.add_node(location.clone());

            let interface = match self.state.symbol_table.lookup_interface(self.current_program, &location) {
                Some(p) => p.clone(),
                None => {
                    self.state.handler.emit_err(CheckInterfacesError::interface_not_found(location, location_span));
                    return;
                }
            };

            for (parent_span, parent_type) in &interface.parents {
                let Type::Composite(CompositeType { path: parent_path, .. }) = parent_type else {
                    self.state.handler.emit_err(CheckInterfacesError::not_an_interface(parent_type, *parent_span));
                    return;
                };
                let parent_location =
                    parent_path.try_global_location().expect("Locations should have been resolved by now");

                self.inheritance_graph.add_node(parent_location.clone());
                self.inheritance_graph.add_edge(location.clone(), parent_location.clone());

                queue.insert((parent_location.clone(), *parent_span));
            }

            processed.insert(location);
        }
    }
}

impl AstVisitor for CheckInterfacesVisitor<'_> {
    type AdditionalInput = ();
    type Output = ();
}

impl ProgramVisitor for CheckInterfacesVisitor<'_> {
    fn visit_program_scope(&mut self, input: &ProgramScope) {
        self.current_program = input.program_id.as_symbol();

        self.build_inheritance_graph(input);

        // Check for cycles using post_order traversal.
        if let Err(DiGraphError::CycleDetected(path)) = self.inheritance_graph.post_order() {
            self.state.handler.emit_err(CheckInterfacesError::cyclic_interface_inheritance(
                path.iter().map(|loc| loc.to_string()).collect::<Vec<_>>().join(" -> "),
            ));
            return;
        }

        // Validate record prototypes (e.g. owner must be address).
        self.validate_record_prototypes(input);

        // Flatten all interfaces in this program scope.
        for (_, interface) in &input.interfaces {
            let location = Location::new(self.current_program, vec![interface.identifier.name]);
            // This will validate inheritance and cache the result.
            self.flatten_interface(&location, interface.identifier.span);
        }

        // Check if the program implements interfaces (supports multiple inheritance).
        for (parent_span, parent_type) in &input.parents {
            let Type::Composite(CompositeType { path: parent_path, .. }) = parent_type else {
                self.state.handler.emit_err(CheckInterfacesError::not_an_interface(parent_type, *parent_span));
                return;
            };
            let parent_location =
                parent_path.try_global_location().expect("Locations should have been resolved by now");
            self.check_program_implements_interface(input, parent_location, *parent_span);
        }
    }
}
