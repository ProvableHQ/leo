// Copyright (C) 2019-2020 Aleo Systems Inc.
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

#[macro_use]
extern crate thiserror;

pub mod errors;
pub use self::errors::*;

pub mod types;
pub use self::types::*;

/// A resolved node in an abstract syntax tree (AST).
///
/// Resolved nodes can be any function, statement, expression, type, etc. in an AST.
/// Resolved nodes should not contain any illegal types.
/// Resolved nodes should not contain any implicit types.
pub trait ResolvedNode {
    /// The expected error type if the type resolution fails.
    type Error;

    /// The unresolved AST node that is being resolved.
    type UnresolvedNode;

    ///
    /// Returns a resolved AST representation given an unresolved AST representation.
    ///
    /// User-defined types are looked up using the given symbol table.
    ///
    fn resolve(table: &mut SymbolTable, unresolved: Self::UnresolvedNode) -> Result<Self, Self::Error>
    where
        Self: std::marker::Sized;
}
