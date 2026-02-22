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
    Function,
    FunctionPrototype,
    Interface,
    Location,
    ProgramScope,
    ProgramVisitor,
    RecordPrototype,
};
use leo_errors::CheckInterfacesError;
use leo_span::Symbol;

use indexmap::IndexMap;

/// A flattened interface with all inherited members collected.
#[derive(Clone, Debug)]
struct FlattenedInterface {
    functions: Vec<(Symbol, FunctionPrototype)>,
    records: Vec<(Symbol, RecordPrototype)>,
}

pub struct CheckInterfacesVisitor<'a> {
    pub state: &'a mut CompilerState,
    /// Current program name being processed.
    current_program: Symbol,
    /// Cache of flattened interfaces (with all inherited members).
    flattened_interfaces: IndexMap<Location, FlattenedInterface>,
}

impl<'a> CheckInterfacesVisitor<'a> {
    pub fn new(state: &'a mut CompilerState) -> Self {
        Self { state, current_program: Symbol::intern(""), flattened_interfaces: IndexMap::new() }
    }

    /// Flatten an interface by collecting all inherited members.
    /// Detects conflicts during flattening.
    fn flatten_interface(&mut self, interface: &Interface, location: &Location) -> Option<FlattenedInterface> {
        // Check cache first.
        if let Some(flattened) = self.flattened_interfaces.get(location) {
            return Some(flattened.clone());
        }

        // Start with the interface's own members.
        let mut flattened =
            FlattenedInterface { functions: interface.functions.clone(), records: interface.records.clone() };

        // Merge members from all parent interfaces (supports multiple inheritance).
        for parent_name in &interface.parents {
            let parent_location = Location::new(location.program, vec![*parent_name]);

            // Lookup parent interface.
            let parent_interface =
                match self.state.symbol_table.lookup_interface(self.current_program, &parent_location) {
                    Some(p) => p.clone(),
                    None => {
                        self.state
                            .handler
                            .emit_err(CheckInterfacesError::interface_not_found(*parent_name, interface.span));
                        return None;
                    }
                };

            // Recursively flatten parent.
            // FIXME: handle cycles
            let parent_flattened = self.flatten_interface(&parent_interface, &parent_location)?;

            // Merge parent functions, checking for conflicts.
            for (name, parent_func) in &parent_flattened.functions {
                if let Some((_, existing_func)) = flattened.functions.iter().find(|(n, _)| n == name) {
                    // Same name exists - check if compatible.
                    if !Self::prototypes_match(existing_func, parent_func) {
                        dbg!(existing_func, parent_func);
                        self.state.handler.emit_err(CheckInterfacesError::conflicting_interface_member(
                            name,
                            interface.identifier.name,
                            parent_interface.identifier.name,
                            interface.span,
                        ));
                        return None;
                    }
                    // Compatible - no action needed, child's version takes precedence.
                } else {
                    // Add parent's function.
                    flattened.functions.push((*name, parent_func.clone()));
                }
            }

            // Merge parent records.
            for (name, parent_record) in &parent_flattened.records {
                if !flattened.records.iter().any(|(n, _)| n == name) {
                    flattened.records.push((*name, parent_record.clone()));
                }
            }
        }

        // Cache the result.
        self.flattened_interfaces.insert(location.clone(), flattened.clone());
        Some(flattened)
    }

    /// Validate that a program implements all interface requirements.
    fn check_program_implements_interface(&mut self, program_scope: &ProgramScope, interface_name: Symbol) {
        let program_name = program_scope.program_id.name.name;
        let interface_location = Location::new(program_name, vec![interface_name]);

        // Look up the interface.
        let interface = match self.state.symbol_table.lookup_interface(program_name, &interface_location) {
            Some(i) => i.clone(),
            None => {
                self.state
                    .handler
                    .emit_err(CheckInterfacesError::interface_not_found(interface_name, program_scope.span));
                return;
            }
        };

        // Get the flattened interface (with all inherited members).
        let flattened = match self.flatten_interface(&interface, &interface_location) {
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
                            interface_name,
                            Self::format_prototype_signature(required_proto),
                            Self::format_function_signature(&func_symbol.function),
                            func_symbol.function.span,
                        ));
                    }
                }
                None => {
                    self.state.handler.emit_err(CheckInterfacesError::missing_interface_function(
                        func_name,
                        interface_name,
                        program_name,
                        program_scope.span,
                    ));
                }
            }
        }

        // Check all required records are declared.
        for (record_name, _) in &flattened.records {
            let record_location = Location::new(program_name, vec![*record_name]);

            if self.state.symbol_table.lookup_record(program_name, &record_location).is_none() {
                self.state.handler.emit_err(CheckInterfacesError::missing_interface_record(
                    record_name,
                    interface_name,
                    program_name,
                    program_scope.span,
                ));
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
            input_a.mode == input_b.mode
        }) &&

        // Output must match.
        a.output.len() == b.output.len() &&
        a.output.iter().zip(b.output.iter()).all(|(output_a, output_b)| output_a.type_.eq_user(&output_b.type_) && output_a.mode == output_b.mode) &&

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
            func_input.mode == proto_input.mode
        }) &&

        // Output must match.
        func.output.len() == proto.output.len() &&

        func.output.iter().zip(proto.output.iter()).all(
            |(func_output, proto_output)| func_output.type_.eq_user(&proto_output.type_) && func_output.mode == proto_output.mode) &&

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
}

impl AstVisitor for CheckInterfacesVisitor<'_> {
    type AdditionalInput = ();
    type Output = ();
}

impl ProgramVisitor for CheckInterfacesVisitor<'_> {
    fn visit_program_scope(&mut self, input: &ProgramScope) {
        self.current_program = input.program_id.name.name;

        // First, validate all interfaces in this program scope.
        for (_, interface) in &input.interfaces {
            let location = Location::new(self.current_program, vec![interface.identifier.name]);
            // This will validate inheritance and cache the result.
            self.flatten_interface(interface, &location);
        }

        // Then, check if the program implements interfaces (supports multiple inheritance).
        for parent_interface in &input.parents {
            self.check_program_implements_interface(input, *parent_interface);
        }
    }
}
