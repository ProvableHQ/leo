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

use crate::ast::Rule;

use pest_ast::FromPest;
use serde::Serialize;
use std::fmt;

#[derive(Clone, Debug, FromPest, PartialEq, Serialize)]
#[pest_ast(rule(Rule::operation_unary))]
pub enum UnaryOperation {
    Negate(Negate),
    Not(Not),
}

impl fmt::Display for UnaryOperation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            UnaryOperation::Negate(_) => write!(f, "-"),
            UnaryOperation::Not(_) => write!(f, "!"),
        }
    }
}

#[derive(Clone, Debug, FromPest, PartialEq, Serialize)]
#[pest_ast(rule(Rule::operation_not))]
pub struct Not {}

#[derive(Clone, Debug, FromPest, PartialEq, Serialize)]
#[pest_ast(rule(Rule::operation_negate))]
pub struct Negate {}
