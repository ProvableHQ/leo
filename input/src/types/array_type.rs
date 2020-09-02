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

use crate::{
    ast::Rule,
    types::{ArrayDimensions, ArrayElement},
};

use pest::Span;
use pest_ast::FromPest;

#[derive(Clone, Debug, FromPest, PartialEq, Eq)]
#[pest_ast(rule(Rule::type_array))]
pub struct ArrayType<'ast> {
    pub type_: ArrayElement<'ast>,
    pub dimensions: ArrayDimensions<'ast>,
    #[pest_ast(outer())]
    pub span: Span<'ast>,
}

impl<'ast> std::fmt::Display for ArrayType<'ast> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[{}; {}]", self.type_, self.dimensions)
    }
}
