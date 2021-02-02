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

#[macro_use]
extern crate thiserror;

pub mod checks;
pub use checks::*;

pub mod const_value;
pub use const_value::*;

pub mod error;
pub use error::*;

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
pub use variable::*;

pub use leo_ast::{Identifier, Span};

use std::path::Path;

/// The abstract semantic graph (ASG) for a Leo program.
///
/// The [`Asg`] type represents a Leo program as a series of recursive data types.
/// These data types form a graph that begins from a [`Program`] type node.
///
/// A new [`Asg`] can be created from an [`Ast`] generated in the `ast` module.
// #[derive(Debug, Eq, PartialEq)]
// pub struct Asg {
//     asg: InnerProgram,
// }
//
// impl Asg {
//     /// Creates a new asg from a given ast tree and import resolver.
//     pub fn new<T: ImportResolver + 'static>(
//         content: leo_ast::Program,
//         resolver: &mut T,
//     ) -> Result<Program, AsgConvertError> {
//         InnerProgram::new(&content, resolver)
//     }
//
//     /// Returns a reference to the inner program ast representation.
//     pub fn into_repr(self) -> Program {
//         self.asg
//     }
// }

pub fn load_ast<T: AsRef<Path>, Y: AsRef<str>>(path: T, content: Y) -> Result<leo_ast::Program, AsgConvertError> {
    // Parses the Leo file and constructs a grammar ast.
    let ast = leo_grammar::Grammar::new(path.as_ref(), content.as_ref())
        .map_err(|e| AsgConvertError::InternalError(format!("ast: {:?}", e)))?;

    // Parses the pest ast and constructs a Leo ast.
    Ok(leo_ast::Ast::new("load_ast", &ast)?.into_repr())
}

pub fn load_asg_from_ast<T: ImportResolver + 'static>(
    content: leo_ast::Program,
    resolver: &mut T,
) -> Result<Program, AsgConvertError> {
    InnerProgram::new(&content, resolver)
}

pub fn load_asg<T: ImportResolver + 'static>(content: &str, resolver: &mut T) -> Result<Program, AsgConvertError> {
    InnerProgram::new(&load_ast("input.leo", content)?, resolver)
}
