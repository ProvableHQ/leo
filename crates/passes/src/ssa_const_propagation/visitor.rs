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

use crate::CompilerState;

use leo_ast::{Expression, FromStrRadix, Literal, LiteralVariant, Location, Node, NodeID, const_eval::Value};
use leo_errors::Formatted;
use leo_span::{Span, Symbol};

use indexmap::IndexMap;

/// Visitor that propagates constant values through the program.
pub struct SsaConstPropagationVisitor<'a> {
    pub state: &'a mut CompilerState,
    /// Current program being analyzed.
    pub program: Symbol,
    /// Maps variable names to their constant values.
    /// Only variables assigned constant values are tracked here.
    pub constants: IndexMap<Symbol, Value>,
    /// Maps variable names bound to a composite RHS — where every field
    /// initializer is an atom — to the atom each field was initialized with.
    /// Used to forward `x.field` to the stored atom without rematerializing
    /// the enclosing struct — effectively scalarizing short-lived aggregates.
    pub atom_fielded_composites: IndexMap<Symbol, IndexMap<Symbol, Expression>>,
    /// Maps local variables that are simple aliases of atom-fielded composites.
    pub aliases: IndexMap<Symbol, Symbol>,
    /// Whether direct `x.field` accesses can be forwarded, or only accesses through aliases.
    pub forward_direct_composites: bool,
    /// Maps local variables bound to atom-only ternaries so later redundant
    /// ternaries over the same condition can be absorbed.
    pub ternaries: IndexMap<Symbol, TrackedTernary>,
    /// Have we actually modified the program at all?
    pub changed: bool,
}

#[derive(Clone)]
pub struct TrackedTernary {
    pub condition: Expression,
    pub if_true: Expression,
    pub if_false: Expression,
}

/// An "atom" is an expression simple enough to substitute for another use-site
/// without re-running arbitrary effects or duplicating work. Post-SSA, paths
/// and literals are the only expression shapes that round-trip freely.
pub(super) fn is_atom(expr: &Expression) -> bool {
    matches!(expr, Expression::Path(_) | Expression::Literal(_))
}

/// Parse a numeric literal string, handling underscores and radix prefixes (0x, 0o, 0b).
fn parse_literal_value(s: &str) -> Option<i128> {
    let clean = s.replace('_', "");
    i128::from_str_by_radix(&clean).ok()
}

/// Check if a literal represents the zero/identity value for addition.
pub(super) fn is_zero_literal(lit: &Literal) -> bool {
    match &lit.variant {
        LiteralVariant::Integer(_, s)
        | LiteralVariant::Field(s)
        | LiteralVariant::Group(s)
        | LiteralVariant::Scalar(s)
        | LiteralVariant::Unsuffixed(s) => parse_literal_value(s) == Some(0),
        LiteralVariant::Boolean(b) => !b,
        _ => false,
    }
}

/// Check if a literal represents the one/identity value for multiplication.
pub(super) fn is_one_literal(lit: &Literal) -> bool {
    match &lit.variant {
        LiteralVariant::Integer(_, s)
        | LiteralVariant::Field(s)
        | LiteralVariant::Scalar(s)
        | LiteralVariant::Unsuffixed(s) => parse_literal_value(s) == Some(1),
        LiteralVariant::Boolean(b) => *b,
        _ => false,
    }
}

/// Check if two expressions refer to the same SSA variable.
/// In SSA form, each variable name is unique, so name equality implies value equality.
pub(super) fn same_ssa_atom(a: &Expression, b: &Expression) -> bool {
    match (a, b) {
        (Expression::Path(pa), Expression::Path(pb)) => {
            let sa = pa.try_local_symbol();
            let sb = pb.try_local_symbol();
            sa == sb && sa.is_some()
        }
        (Expression::Literal(a), Expression::Literal(b)) => a.variant == b.variant,
        _ => false,
    }
}

impl SsaConstPropagationVisitor<'_> {
    /// Clear analysis state that is only valid within one SSA function body.
    pub(super) fn clear_tracked_values(&mut self) {
        self.constants.clear();
        self.atom_fielded_composites.clear();
        self.aliases.clear();
        self.ternaries.clear();
    }

    pub(super) fn resolve_composite_alias(&self, mut name: Symbol) -> Symbol {
        while let Some(alias) = self.aliases.get(&name).copied() {
            if alias == name {
                break;
            }
            name = alias;
        }
        name
    }

    /// Emit a `StaticAnalyzerError`.
    pub fn emit_err(&self, err: Formatted) {
        self.state.handler.emit_err(err);
    }

    /// Convert a Value to an Expression.
    /// Returns the new expression and its node ID.
    /// If the original node doesn't have a type, this will return None.
    pub fn value_to_expression(&mut self, value: &Value, span: Span, id: NodeID) -> Option<(Expression, NodeID)> {
        let ty = self.state.type_table.get(&id)?.clone();
        let symbol_table = &self.state.symbol_table;
        let struct_lookup = |loc: &Location| {
            symbol_table
                .lookup_struct(self.program, loc)
                .unwrap()
                .members
                .iter()
                .map(|mem| (mem.identifier.name, mem.type_.clone()))
                .collect()
        };
        let new_expr = value.to_expression(span, &self.state.node_builder, &ty, &struct_lookup)?;
        let new_id = new_expr.id();

        // Copy the type information to the new node ID.
        self.copy_types_recursively(&new_expr, &ty);

        Some((new_expr, new_id))
    }

    /// Copy types for nested expressions based on the type structure.
    /// This ensures that ALL expressions in the tree have proper type information.
    fn copy_types_recursively(&mut self, expr: &Expression, ty: &leo_ast::Type) {
        use leo_ast::Type;

        self.state.type_table.insert(expr.id(), ty.clone());

        // Then recursively handle nested expressions
        match (expr, ty) {
            (Expression::Array(array_expr), Type::Array(array_ty)) => {
                for element in &array_expr.elements {
                    self.copy_types_recursively(element, array_ty.element_type());
                }
            }
            (Expression::Tuple(tuple_expr), Type::Tuple(tuple_ty)) => {
                for (element, elem_ty) in tuple_expr.elements.iter().zip(tuple_ty.elements()) {
                    self.copy_types_recursively(element, elem_ty);
                }
            }
            (Expression::Composite(composite_expr), Type::Composite(composite_ty)) => {
                // We only look for structs here (not records) because `copy_types_recursively` is
                // only called from `value_to_expression`, which never produces record expressions.
                let member_types: Vec<leo_ast::Type> = self
                    .state
                    .symbol_table
                    .lookup_struct(self.program, composite_ty.path.expect_global_location())
                    .map(|struct_def| struct_def.members.iter().map(|m| m.type_.clone()).collect())
                    .unwrap_or_default();
                for (member, member_ty) in composite_expr.members.iter().zip(member_types.iter()) {
                    if let Some(expr) = &member.expression {
                        self.copy_types_recursively(expr, member_ty);
                    }
                }
            }
            _ => {
                // For leaf expressions (literals, etc.), we've already set the type above
            }
        }
    }
}
