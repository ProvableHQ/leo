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

use crate::{Expression, Statement, Type};
use leo_ast::Identifier;

use std::{
    cell::RefCell,
    sync::{Arc, Weak},
};
use uuid::Uuid;

/// Specifies how a program variable was declared.
#[derive(Debug, PartialEq)]
pub enum VariableDeclaration {
    Definition,
    IterationDefinition,
    Parameter,
    Input,
}

/// Stores information on a program variable.
#[derive(Debug)]
pub struct InnerVariable {
    pub id: Uuid,
    pub name: Identifier,
    pub type_: Type,
    pub mutable: bool,
    pub declaration: VariableDeclaration,
    pub references: Vec<Weak<Expression>>, // all Expression::VariableRef or panic
    pub assignments: Vec<Weak<Statement>>, // all Statement::Assign or panic -- must be 1 if not mutable, or 0 if declaration == input | parameter
}

pub type Variable = Arc<RefCell<InnerVariable>>;
pub type WeakVariable = Weak<RefCell<InnerVariable>>;
