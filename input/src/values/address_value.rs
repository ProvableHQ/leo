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

use crate::{
    ast::Rule,
    values::{Address, AddressTyped},
};

use pest::Span;
use pest_ast::FromPest;
use std::fmt;

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::value_address))]
pub enum AddressValue<'ast> {
    Implicit(Address<'ast>),
    Explicit(AddressTyped<'ast>),
}

impl<'ast> AddressValue<'ast> {
    pub(crate) fn span(&self) -> &Span<'ast> {
        match self {
            AddressValue::Implicit(address) => &address.span,
            AddressValue::Explicit(address) => &address.span,
        }
    }
}

impl<'ast> fmt::Display for AddressValue<'ast> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AddressValue::Explicit(address) => write!(f, "{}", address),
            AddressValue::Implicit(address) => write!(f, "{}", address),
        }
    }
}
