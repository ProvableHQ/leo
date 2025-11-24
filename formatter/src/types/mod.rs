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

mod array;
mod composite;
mod future;
mod option;
mod tuple;
mod vector;

use leo_parser_lossless::{SyntaxKind, SyntaxNode};

use crate::{Formatter, Output};

impl Formatter<'_, '_> {
    pub(super) fn format_type(&mut self, node: &SyntaxNode<'_>) -> Output {
        let SyntaxKind::Type(type_kind) = node.kind else { panic!("Can't happen") };

        match type_kind {
            leo_parser_lossless::TypeKind::Address
            | leo_parser_lossless::TypeKind::Boolean
            | leo_parser_lossless::TypeKind::Field
            | leo_parser_lossless::TypeKind::Group
            | leo_parser_lossless::TypeKind::Integer(_)
            | leo_parser_lossless::TypeKind::Scalar
            | leo_parser_lossless::TypeKind::Signature
            | leo_parser_lossless::TypeKind::Numeric
            | leo_parser_lossless::TypeKind::Unit => self.node_with_trivia(node, 0)?,
            leo_parser_lossless::TypeKind::Tuple => self.format_tuple_type(node)?,
            leo_parser_lossless::TypeKind::Array => self.format_array_type(node)?,
            leo_parser_lossless::TypeKind::Composite => self.format_composite_type(node)?,
            leo_parser_lossless::TypeKind::Future => self.format_future_type(node)?,
            leo_parser_lossless::TypeKind::Mapping
            | leo_parser_lossless::TypeKind::Identifier
            | leo_parser_lossless::TypeKind::String => unimplemented!(),
            leo_parser_lossless::TypeKind::Optional => self.format_option_type(node)?,
            leo_parser_lossless::TypeKind::Vector => self.format_vector_type(node)?,
        }

        Ok(())
    }
}
