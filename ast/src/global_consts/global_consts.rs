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

use crate::{
    statements::{Declare, VariableName},
    Expression,
    GlobalConst,
    Node,
    Span,
    Type,
};
use leo_grammar::global_consts::GlobalConst as GrammarGlobalConst;

use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct GlobalConsts {
    pub declaration_type: Declare,
    pub variable_names: Vec<VariableName>,
    pub type_: Option<Type>,
    pub value: Expression,
    pub span: Span,
}

impl GlobalConsts {
    pub fn into_global_const(self) -> Vec<GlobalConst> {
        let mut global_consts = vec![];
        let mut types: Vec<Option<Type>> = vec![];

        if self.type_.is_some() {
            match self.type_.clone().unwrap() {
                Type::Tuple(types_old) => {
                    for type_ in &types_old {
                        types.push(Some(type_.clone()));
                    }
                }
                _ => types.push(self.type_.clone()),
            }
        }

        for (i, variable_name) in self.variable_names.iter().enumerate() {
            global_consts.push(GlobalConst {
                declaration_type: self.declaration_type.clone(),
                variable_name: variable_name.clone(),
                type_: types.get(i).unwrap_or(&None).clone(),
                value: self.value.clone(),
                span: self.span.clone(),
            });
        }

        global_consts
    }
}

impl fmt::Display for GlobalConsts {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} ", self.declaration_type)?;
        if self.variable_names.len() == 1 {
            // mut a
            write!(f, "{}", self.variable_names[0])?;
        } else {
            // (a, mut b)
            let names = self
                .variable_names
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<_>>()
                .join(",");

            write!(f, "({})", names)?;
        }

        if self.type_.is_some() {
            write!(f, ": {}", self.type_.as_ref().unwrap())?;
        }
        write!(f, " = {};", self.value)
    }
}

impl<'ast> From<GrammarGlobalConst<'ast>> for GlobalConsts {
    fn from(global_const: GrammarGlobalConst<'ast>) -> Self {
        let variable_names = global_const
            .variables
            .names
            .into_iter()
            .map(VariableName::from)
            .collect::<Vec<_>>();

        let type_ = global_const.variables.type_.map(Type::from);

        GlobalConsts {
            declaration_type: Declare::Const,
            variable_names,
            type_,
            value: Expression::from(global_const.expression),
            span: Span::from(global_const.span),
        }
    }
}

impl Node for GlobalConsts {
    fn span(&self) -> &Span {
        &self.span
    }

    fn set_span(&mut self, span: Span) {
        self.span = span;
    }
}
