// Copyright (C) 2019-2023 Aleo Systems Inc.
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

use crate::SymbolTable;
use crate::{CallGraph, StructGraph};

use leo_ast::{Mode, Type};
use leo_span::Symbol;

use indexmap::IndexMap;

pub struct CodeGenerator<'a> {
    /// The symbol table for the program.
    pub(crate) symbol_table: &'a SymbolTable,
    /// The struct dependency graph for the program.
    pub(crate) struct_graph: &'a StructGraph,
    /// The call graph for the program.
    pub(crate) _call_graph: &'a CallGraph,
    /// A counter to track the next available register.
    pub(crate) next_register: u64,
    /// The name of the current function.
    pub(crate) current_function: Option<Symbol>,
    /// Mapping of variables to registers.
    pub(crate) variable_mapping: IndexMap<Symbol, String>,
    /// Mapping of composite names to a tuple containing metadata associated with the name.
    /// The first element of the tuple indicate whether the composite is a record or not.
    /// The second element of the tuple is a string modifier used for code generation.
    pub(crate) composite_mapping: IndexMap<Symbol, (bool, String)>,
    /// Are we traversing a transition function?
    pub(crate) is_transition_function: bool,
    /// Are we traversing a finalize block?
    pub(crate) in_finalize: bool,
}

impl<'a> CodeGenerator<'a> {
    /// Initializes a new `CodeGenerator`.
    pub fn new(symbol_table: &'a SymbolTable, struct_graph: &'a StructGraph, _call_graph: &'a CallGraph) -> Self {
        // Initialize variable mapping.
        Self {
            symbol_table,
            struct_graph,
            _call_graph,
            next_register: 0,
            current_function: None,
            variable_mapping: IndexMap::new(),
            composite_mapping: IndexMap::new(),
            is_transition_function: false,
            in_finalize: false,
        }
    }
}

impl CodeGenerator<'_> {
    fn consume_type(&mut self, input: &Type) -> String {
        match input {
            Type::Address
            | Type::Boolean
            | Type::Field
            | Type::Group
            | Type::Scalar
            | Type::String
            | Type::Integer(..) => format!("{input}"),
            Type::Identifier(ident) => format!("{ident}"),
            Type::Mapping(_) => {
                unreachable!("Mapping types are not supported at this phase of compilation")
            }
            Type::Tuple(_) => {
                unreachable!("Tuple types should not be visited at this phase of compilation")
            }
            Type::Err => unreachable!("Error types should not exist at this phase of compilation"),
            Type::Unit => unreachable!("Unit types are not supported at this phase of compilation"),
        }
    }

    pub(crate) fn consume_type_with_visibility(&mut self, type_: &Type, visibility: Mode) -> String {
        match type_ {
            // When the type is a record.
            // Note that this unwrap is safe because all composite types have been added to the mapping.
            Type::Identifier(identifier) if self.composite_mapping.get(&identifier.name).unwrap().0 => {
                format!("{identifier}.record")
            }
            _ => match visibility {
                Mode::None => self.consume_type(type_),
                _ => format!("{}.{visibility}", self.consume_type(type_)),
            },
        }
    }
}
