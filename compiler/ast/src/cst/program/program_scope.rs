// Copyright (C) 2019-2024 Aleo Systems Inc.
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

//! A Leo program scope consists of struct, function, and mapping definitions.

use crate::ProgramId ;
use crate::cst::{ ConstDeclaration, Composite,  Function, Mapping, Comment };
use leo_span::{Span, Symbol};
use serde::{Deserialize, Serialize};
use std::fmt;

/// Stores the Leo program scope abstract syntax tree.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProgramScope {
    /// The program id of the program scope.
    pub program_id: ProgramId,
    /// A vector of const definitions
    pub consts: Vec<(Symbol, (ConstDeclaration, Vec<Comment>))>,
    /// A vector of struct definitions.
    pub structs: Vec<(Symbol, (Composite, Vec<Comment>))>,
    /// A vector of mapping definitions.
    pub mappings: Vec<(Symbol, (Mapping, Vec<Comment>))>,
    /// A vector of function definitions.
    pub functions: Vec<(Symbol, (Function, Vec<Comment>))>,
    /// The span associated with the program scope.
    pub span: Span,
    ///A comment
    pub comment: Comment,

}

impl fmt::Display for ProgramScope {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.comment == Comment::None {
            writeln!(f, "program {} {{", self.program_id)?;
        }else {
            write!(f, "program {} {{ ", self.program_id)?;
            self.comment.fmt(f)?;
        }
        for (_, const_) in self.consts.iter() {
            for comment in &const_.1 {
                write!(f, "    ")?;
                comment.fmt(f)?;
            }
            writeln!(f, "    {}", const_.0)?;
        }
        for (_, struct_) in self.structs.iter() {
            for comment in &struct_.1 {
                write!(f, "    ")?;
                comment.fmt(f)?;
            }
            writeln!(f, "    {}", struct_.0)?;
        }
        for (_, mapping) in self.mappings.iter() {
            for comment in &mapping.1 {
                write!(f, "    ")?;
                comment.fmt(f)?;
            }
            writeln!(f, "    {}", mapping.0)?;
        }
        for (_, function) in self.functions.iter() {
            for comment in &function.1 {
                write!(f, "    ")?;
                comment.fmt(f)?;
            }
            writeln!(f, "    {}", function.0)?;
        }
        writeln!(f, "}}")?;
        Ok(())
    }
}
