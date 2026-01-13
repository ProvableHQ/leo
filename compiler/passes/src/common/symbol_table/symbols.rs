// Copyright (C) 2019-2025 Provable Inc.
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

use std::fmt::Display;

use serde::{Deserialize, Serialize};

use leo_ast::{Function, Location, Mode, Type};
use leo_span::Span;

/// An enumeration of the different types of variable type.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum VariableType {
    Const,
    ConstParameter,
    Input(Mode),
    Mut,
    Storage,
}

impl Display for VariableType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use VariableType::*;

        match self {
            Const => write!(f, "const var"),
            ConstParameter => write!(f, "const parameter"),
            Input(m) => write!(f, "{m} input"),
            Mut => write!(f, "mut var"),
            Storage => write!(f, "storage var"),
        }
    }
}

/// An entry for a variable in the symbol table.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct VariableSymbol {
    /// The `Type` of the variable. This is an `Option` because variables are inserted into the
    /// symbol table first without types. The types are only set in `TypeChecking`.
    pub type_: Option<Type>,
    /// The `Span` associated with the variable.
    pub span: Span,
    /// The type of declaration for the variable.
    pub declaration: VariableType,
}

impl Display for VariableSymbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(type_) = &self.type_ {
            write!(f, "{}: {}", self.declaration, type_)?;
        } else {
            write!(f, "{}", self.declaration)?;
        }
        Ok(())
    }
}

#[derive(Clone, Debug)]
pub struct FunctionSymbol {
    pub function: Function,
    pub finalizer: Option<Finalizer>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Finalizer {
    /// The name of the async function this async transition calls.
    pub location: Location,

    /// The locations of the futures passed to the async function called by this async transition.
    pub future_inputs: Vec<Location>,

    /// The types passed to the async function called by this async transition.
    pub inferred_inputs: Vec<Type>,
}
