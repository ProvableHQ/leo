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

use crate::Types;

use leo_ast::{Identifier, Type};
use leo_span::{Span, Symbol};

pub struct Account;

impl Types for Account {
    fn types() -> Vec<Type> {
        vec![
            Type::Identifier(Identifier {
                name: Symbol::intern("ComputeKey"),
                span: Span::dummy(),
            }),
            Type::Identifier(Identifier {
                name: Symbol::intern("PrivateKey"),
                span: Span::dummy(),
            }),
            Type::Identifier(Identifier {
                name: Symbol::intern("Record"),
                span: Span::dummy(),
            }),
            Type::Identifier(Identifier {
                name: Symbol::intern("Signature"),
                span: Span::dummy(),
            }),
            Type::Identifier(Identifier {
                name: Symbol::intern("ViewKey"),
                span: Span::dummy(),
            }),
        ]
    }
}
