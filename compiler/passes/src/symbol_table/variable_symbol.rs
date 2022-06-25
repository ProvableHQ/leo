// Copyright (C) 2019-2022 Aleo Systems Inc.
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

use leo_ast::{Identifier, ParamMode, Type};
use leo_errors::Result;
use leo_span::Span;

use crate::Value;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Declaration {
    Const(Option<Value>),
    Input(Type, ParamMode),
    Mut(Option<Value>),
}

impl Declaration {
    pub fn get_as_usize(&self) -> Result<Option<usize>> {
        use Declaration::*;

        match self {
            Const(Some(value)) => Ok(Some(value.try_into()?)),
            Input(_, _) => Ok(None),
            _ => Ok(None),
        }
    }

    pub fn get_type(&self) -> Option<Type> {
        use Declaration::*;

        match self {
            Const(Some(value)) => Some(value.into()),
            Input(type_, _) => Some(*type_),
            _ => None,
        }
    }
}

impl AsRef<Self> for Declaration {
    fn as_ref(&self) -> &Self {
        self
    }
}

#[derive(Clone, Debug)]
pub struct VariableSymbol {
    pub type_: Type,
    pub span: Span,
    pub declaration: Declaration,
}

impl VariableSymbol {
    pub fn get_const_value(&self, ident: Identifier) -> Option<Value> {
        use Declaration::*;
        match &self.declaration {
            Const(Some(v)) | Mut(Some(v)) => Some(v.clone()),
            Input(type_, ParamMode::Const) => Some(Value::Input(*type_, ident)),
            _ => None,
        }
    }
}
