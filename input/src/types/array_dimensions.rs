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

use crate::{ast::Rule, values::PositiveNumber};

use pest::Span;
use pest_ast::FromPest;

#[derive(Clone, Debug, FromPest, PartialEq, Eq)]
#[pest_ast(rule(Rule::array_dimensions))]
pub enum ArrayDimensions<'ast> {
    Single(Single<'ast>),
    Multiple(Multiple<'ast>),
}

#[derive(Clone, Debug, FromPest, PartialEq, Eq)]
#[pest_ast(rule(Rule::dimension_single))]
pub struct Single<'ast> {
    pub number: PositiveNumber<'ast>,
    #[pest_ast(outer())]
    pub span: Span<'ast>,
}

#[derive(Clone, Debug, FromPest, PartialEq, Eq)]
#[pest_ast(rule(Rule::dimension_multiple))]
pub struct Multiple<'ast> {
    pub numbers: Vec<PositiveNumber<'ast>>,
    #[pest_ast(outer())]
    pub span: Span<'ast>,
}

impl<'ast> ArrayDimensions<'ast> {
    pub fn next_dimension(&self) -> Self {
        match self {
            ArrayDimensions::Single(single) => ArrayDimensions::Multiple(Multiple {
                numbers: vec![],
                span: single.span.clone(),
            }),
            ArrayDimensions::Multiple(multiple) => {
                let old_dimension = multiple.numbers.clone();

                ArrayDimensions::Multiple(Multiple {
                    numbers: old_dimension[1..].to_vec(),
                    span: multiple.span.clone(),
                })
            }
        }
    }
}

impl<'ast> std::fmt::Display for ArrayDimensions<'ast> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            ArrayDimensions::Single(ref single) => write!(f, "{}", single.number),
            ArrayDimensions::Multiple(ref multiple) => {
                let string = multiple
                    .numbers
                    .iter()
                    .map(|x| x.value.clone())
                    .collect::<Vec<_>>()
                    .join(", ");

                write!(f, "{}", string)
            }
        }
    }
}
