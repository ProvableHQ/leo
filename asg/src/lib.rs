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

//! The abstract semantic graph (ASG) for a Leo program.
//!
//! This module contains the [`Asg`] type, an abstract data type that represents a Leo program
//! as a series of graph nodes. The [`Asg`] type is at a greater level of abstraction than an [`Ast`].
//!
//! A new [`Asg`] type can be created from an [`Ast`].
//! Converting to an [`Asg`] provides greater type safety by canonicalizing and checking program types.

#![allow(clippy::from_over_into)]
#![allow(clippy::result_unit_err)]

pub mod checks;
pub use checks::*;

pub mod const_value;
pub use const_value::*;

pub mod expression;
pub use expression::*;

pub mod import;
pub use import::*;

mod input;
pub use input::*;

pub mod node;
pub use node::*;

pub mod prelude;
pub use prelude::*;

pub mod program;
pub use program::*;

pub mod reducer;
pub use reducer::*;

pub mod scope;
pub use scope::*;

pub mod statement;
pub use statement::*;

pub mod type_;
pub use type_::*;

pub mod variable;
use typed_arena::Arena;
pub use variable::*;

pub mod pass;
pub use pass::*;

pub mod context;
pub use context::*;

pub use leo_ast::{Ast, Identifier};
use leo_errors::Result;

/// The abstract semantic graph (ASG) for a Leo program.
///
/// The [`Asg`] type represents a Leo program as a series of recursive data types.
/// These data types form a graph that begins from a [`Program`] type node.
///
/// A new [`Asg`] can be created from an [`Ast`] generated in the `ast` module.
#[derive(Clone)]
pub struct Asg<'a> {
    context: AsgContext<'a>,
    asg: Program<'a>,
}

impl<'a> Asg<'a> {
    /// Creates a new ASG from a given AST and import resolver.
    pub fn new<T: ImportResolver<'a>, Y: AsRef<leo_ast::Program>>(
        context: AsgContext<'a>,
        ast: Y,
        resolver: &mut T,
    ) -> Result<Self> {
        Ok(Self {
            context,
            asg: Program::new(context, ast.as_ref(), resolver)?,
        })
    }

    /// Returns the internal program ASG representation.
    pub fn as_repr(&self) -> &Program<'a> {
        &self.asg
    }

    pub fn into_repr(self) -> Program<'a> {
        self.asg
    }

    // /// Serializes the ast into a JSON string.
    // pub fn to_json_string(&self) -> Result<String, serde_json::Error> {
    //     serde_json::to_string_pretty(&self.asg)
    // }
    //
    // /// Deserializes the JSON string into a ast.
    // pub fn from_json_string(json: &str) -> Result<Self, serde_json::Error> {
    //     let ast: Program = serde_json::from_str(json)?;
    //     Ok(Self { ast })
    // }
}

// TODO (howardwu): Remove this.
pub fn load_asg<'a, T: ImportResolver<'a>>(
    context: AsgContext<'a>,
    content: &str,
    resolver: &mut T,
) -> Result<Program<'a>> {
    // Parses the Leo file and constructs a grammar ast.
    let ast = leo_parser::parse_ast("input.leo", content)?;

    Program::new(context, ast.as_repr(), resolver)
}

pub fn new_alloc_context<'a>() -> Arena<ArenaNode<'a>> {
    Arena::new()
}

pub fn new_context<'a>(arena: &'a Arena<ArenaNode<'a>>) -> AsgContext<'a> {
    AsgContextInner::new(arena)
}
