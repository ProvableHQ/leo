// Copyright (C) 2019-2026 Provable Inc.
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

use leo_ast::{CompositeExpression, Expression};
use leo_span::Symbol;

use indexmap::IndexMap;

pub(super) type AtomFields = IndexMap<Symbol, Expression>;

/// An atom can be substituted at multiple member-use sites without repeating
/// evaluation. Post-SSA, only local paths and literals have that property.
pub(super) fn is_atom(expression: &Expression) -> bool {
    matches!(expression, Expression::Path(_) | Expression::Literal(_))
}

/// Capture a composite's explicit fields only when every initializer is an atom.
pub(super) fn atom_fields_from_composite(composite: &CompositeExpression) -> Option<AtomFields> {
    let mut fields = IndexMap::with_capacity(composite.members.len());
    for member in &composite.members {
        let expression = member.expression.as_ref()?;
        if !is_atom(expression) {
            return None;
        }
        fields.insert(member.identifier.name, expression.clone());
    }
    Some(fields)
}

/// Copy field facts through an SSA path alias.
pub(super) fn atom_fields_from_alias(
    expression: &Expression,
    tracked: &IndexMap<Symbol, AtomFields>,
) -> Option<AtomFields> {
    let Expression::Path(path) = expression else {
        return None;
    };
    tracked.get(&path.try_local_symbol()?).cloned()
}

/// Forward a member access from a tracked local composite to its original atom.
pub(super) fn atom_for_member(
    tracked: &IndexMap<Symbol, AtomFields>,
    expression: &Expression,
    field: Symbol,
) -> Option<Expression> {
    let Expression::Path(path) = expression else {
        return None;
    };
    tracked.get(&path.try_local_symbol()?)?.get(&field).cloned()
}
