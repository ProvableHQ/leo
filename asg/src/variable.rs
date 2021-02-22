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

use std::cell::RefCell;

use crate::{Expression, Statement, Type};
use leo_ast::Identifier;

use uuid::Uuid;

/// Specifies how a program variable was declared.
#[derive(Clone, Copy, PartialEq)]
pub enum VariableDeclaration {
    Definition,
    IterationDefinition,
    Parameter,
    Input,
}

/// Stores information on a program variable.
#[derive(Clone)]
pub struct InnerVariable<'a> {
    pub id: Uuid,
    pub name: Identifier,
    pub type_: Type<'a>,
    pub mutable: bool,
    pub const_: bool, // only function arguments, const var definitions NOT included
    pub declaration: VariableDeclaration,
    pub references: Vec<&'a Expression<'a>>, // all Expression::VariableRef or panic
    pub assignments: Vec<&'a Statement<'a>>, // all Statement::Assign or panic -- must be 1 if not mutable, or 0 if declaration == input | parameter
}

pub type Variable<'a> = RefCell<InnerVariable<'a>>;
