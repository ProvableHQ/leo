// Copyright (C) 2019-2021 Aleo Systems Inc.
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

//! This module defines AVM opcodes as associated method calls on Leo types.
use indexmap::IndexSet;
use leo_span::Symbol;

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub struct Method {
    // todo @collinc97: this struct contains type checking info and does not consider opcodes.
    /// The name of the method as an interned string.
    pub name: Symbol,
    /// This count excludes the receiver e.g, `1u8.add(2u8)` contains 1 argument.
    pub num_arguments: usize,
    /// `true` if the receiver type == arguments type == return type
    pub types_are_equal: bool,
}

impl Method {
    pub(crate) fn new(name: &str, num_arguments: usize, types_are_equal: bool) -> Self {
        Self {
            name: Symbol::intern(name),
            num_arguments,
            types_are_equal,
        }
    }
}

use indexmap::{indexmap, indexset, IndexMap};
use leo_ast::Type;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct MethodTable {
    /// Maps method name => method definition.
    method_definitions: IndexMap<Symbol, Method>,
    /// Supported methods for the field type.
    field_methods: IndexSet<Symbol>,
}

impl MethodTable {
    pub fn load() -> Self {
        // Initialize method definitions.
        let add = Method::new("add", 1, true);

        // Initialize associated methods for each type.
        Self {
            field_methods: indexset! {
                add.name
            },
            method_definitions: indexmap! {
                add.name => add
            },
        }
    }

    /// Returns the method corresponding to the given symbol.
    /// Used during type checking.
    pub fn lookup_method(&self, symbol: &Symbol) -> Option<&Method> {
        self.method_definitions.get(symbol)
    }

    /// Returns `true` if the associated method exists for the given type.
    /// Sometimes used during type checking if the receiver type is known.
    pub fn type_method_is_supported(&self, type_: &Type, method: &Symbol) -> bool {
        match type_ {
            Type::Field => self.field_methods.contains(method),
            _ => false,
        }
    }
}
