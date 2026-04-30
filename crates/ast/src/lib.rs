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

//! The abstract syntax tree (ast) for a Leo program.
//!
//! This module contains the [`Ast`] type, a wrapper around the [`Program`] type.
//! The [`Ast`] type is intended to be parsed and modified by different passes
//! of the Leo compiler. The Leo compiler can generate a set of R1CS constraints from any [`Ast`].

#![allow(ambiguous_glob_reexports)]

#[cfg(target_arch = "wasm32")]
extern crate self as snarkvm;

// Preserve this crate's existing `snarkvm::...` imports on WASM without pulling
// in the full native-oriented `snarkvm` dependency graph.
#[cfg(target_arch = "wasm32")]
mod snarkvm_wasm;
#[cfg(target_arch = "wasm32")]
#[doc(hidden)]
pub use snarkvm_wasm::{console, prelude, synthesizer};

mod composite;
pub use self::composite::*;

pub mod common;
pub use self::common::*;

pub mod constructor;
pub use self::constructor::*;

mod expressions;
pub use self::expressions::*;

mod functions;
pub use self::functions::*;

mod interface;
pub use self::interface::*;

mod indent_display;
use indent_display::*;

pub mod const_eval;

mod library;
pub use self::library::*;

mod mapping;
pub use self::mapping::*;

mod module;
pub use self::module::*;

mod passes;
pub use self::passes::*;

mod program;
pub use self::program::*;

mod statement;
pub use self::statement::*;

mod storage;
pub use self::storage::*;

mod types;
pub use self::types::*;

mod stub;
pub use self::stub::*;

pub use common::node::*;

/// The abstract syntax tree (AST) for a Leo program.
///
/// The [`Ast`] type represents a Leo program as a series of recursive data types.
/// These data types form a tree that begins from either a [`Program`] or [`Library`] root.
// Ast is a single root value, not stored in collections; boxing would add unnecessary indirection.
#[allow(clippy::large_enum_variant)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Ast {
    Program(Program),
    Library(Library),
}

impl std::fmt::Display for Ast {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Ast::Program(program) => write!(f, "{}", program),
            Ast::Library(library) => write!(f, "{}", library),
        }
    }
}

// The "default" AST is somewhat arbitrary here. Mostly, this is implicitly used when writing
// something like `let ast = std::mem::take(&mut state.ast);`
impl Default for Ast {
    fn default() -> Self {
        Ast::Program(Program::default())
    }
}

impl Ast {
    pub fn map(self, program_fn: impl FnOnce(Program) -> Program, library_fn: impl FnOnce(Library) -> Library) -> Self {
        match self {
            Ast::Program(p) => Ast::Program(program_fn(p)),
            Ast::Library(l) => Ast::Library(library_fn(l)),
        }
    }

    pub fn try_map<E>(
        self,
        program_fn: impl FnOnce(Program) -> Result<Program, E>,
        library_fn: impl FnOnce(Library) -> Result<Library, E>,
    ) -> Result<Self, E> {
        match self {
            Ast::Program(p) => Ok(Ast::Program(program_fn(p)?)),
            Ast::Library(l) => Ok(Ast::Library(library_fn(l)?)),
        }
    }

    pub fn visit(&self, program_fn: impl FnOnce(&Program), library_fn: impl FnOnce(&Library)) {
        match self {
            Ast::Program(p) => program_fn(p),
            Ast::Library(l) => library_fn(l),
        }
    }
}
