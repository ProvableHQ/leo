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

use crate::{ast::Rule, types::UnsignedIntegerType, values::PositiveNumber};

use pest::Span;
use pest_ast::FromPest;
use std::fmt;

#[derive(Clone, Debug, FromPest, PartialEq, Eq)]
#[pest_ast(rule(Rule::value_integer_unsigned))]
pub struct UnsignedIntegerValue<'ast> {
    pub number: PositiveNumber<'ast>,
    pub type_: UnsignedIntegerType,
    #[pest_ast(outer())]
    pub span: Span<'ast>,
}

impl<'ast> fmt::Display for UnsignedIntegerValue<'ast> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.number)
    }
}
