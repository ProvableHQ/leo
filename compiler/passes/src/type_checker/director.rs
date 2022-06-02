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

use leo_ast::*;
use leo_errors::emitter::Handler;

use crate::{SymbolTable, TypeChecker};

pub(crate) struct Director<'a> {
    pub(crate) visitor: TypeChecker<'a>,
}

impl<'a> Director<'a> {
    pub(crate) fn new(symbol_table: &'a mut SymbolTable<'a>, handler: &'a Handler) -> Self {
        Self {
            visitor: TypeChecker::new(symbol_table, handler),
        }
    }
}

impl<'a> VisitorDirector<'a> for Director<'a> {
    type Visitor = TypeChecker<'a>;

    fn visitor(self) -> Self::Visitor {
        self.visitor
    }

    fn visitor_ref(&mut self) -> &mut Self::Visitor {
        &mut self.visitor
    }
}
