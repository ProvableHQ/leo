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

use super::SsaConstPropagationMode;

use crate::CompilerState;

use leo_ast::{
    Composite,
    CompositeExpression,
    Expression,
    FromStrRadix,
    Literal,
    LiteralVariant,
    Location,
    Node,
    NodeID,
    Type,
    const_eval::Value,
};
use leo_errors::Formatted;
use leo_span::{Span, Symbol};

use indexmap::IndexMap;

/// Visitor that propagates constant values through the program.
pub struct SsaConstPropagationVisitor<'a> {
    pub state: &'a mut CompilerState,
    /// Which subset of SSA const-prop rewrites this invocation is allowed to perform.
    pub mode: SsaConstPropagationMode,
    /// Current program being analyzed.
    pub program: Symbol,
    /// Maps variable names to their constant values.
    /// Only variables assigned constant values are tracked here.
    pub constants: IndexMap<Symbol, Value>,
    /// Maps variable names bound to a composite RHS - where every field
    /// initializer is an atom - to the atom each field was initialized with.
    /// Used to forward `x.field` to the stored atom without rematerializing
    /// the enclosing struct - effectively scalarizing short-lived aggregates.
    pub atom_fielded_composites: IndexMap<Symbol, IndexMap<Symbol, Expression>>,
    /// Maps local variables that are simple aliases of atom-fielded composites.
    pub aliases: IndexMap<Symbol, Symbol>,
    /// Struct definitions visible in the current program scope, including
    /// compiler-generated Optional wrappers added after the symbol table.
    pub composites: IndexMap<Location, Composite>,
    /// Have we actually modified the program at all?
    pub changed: bool,
}

/// An "atom" is an expression simple enough to substitute for another use-site
/// without re-running arbitrary effects or duplicating work. Post-SSA, paths
/// and literals are the only expression shapes that round-trip freely.
pub fn is_atom(expr: &Expression) -> bool {
    matches!(expr, Expression::Path(_) | Expression::Literal(_))
}

pub(super) fn optional_composite_atoms(composite: &CompositeExpression) -> Option<IndexMap<Symbol, Expression>> {
    if composite.base.is_some() || composite.members.len() != 2 {
        return None;
    }

    let mut fields = IndexMap::with_capacity(composite.members.len());
    for member in &composite.members {
        let expr = member.expression.as_ref()?;
        if !is_atom(expr) {
            return None;
        }
        fields.insert(member.identifier.name, expr.clone());
    }

    fields.get(&Symbol::intern("is_some"))?;
    fields.get(&Symbol::intern("val"))?;
    Some(fields)
}

/// Parse a numeric literal string, handling underscores and radix prefixes (0x, 0o, 0b).
fn parse_literal_value(s: &str) -> Option<i128> {
    let clean = s.replace('_', "");
    i128::from_str_by_radix(&clean).ok()
}

/// Check if a literal represents the zero/identity value for addition.
pub fn is_zero_literal(lit: &Literal) -> bool {
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
pub fn is_one_literal(lit: &Literal) -> bool {
    match &lit.variant {
        LiteralVariant::Integer(_, s)
        | LiteralVariant::Field(s)
        | LiteralVariant::Scalar(s)
        | LiteralVariant::Unsuffixed(s) => parse_literal_value(s) == Some(1),
        LiteralVariant::Boolean(b) => *b,
        _ => false,
    }
}

impl SsaConstPropagationVisitor<'_> {
    pub(super) fn runs_full_const_prop(&self) -> bool {
        self.mode == SsaConstPropagationMode::Full
    }

    pub(super) fn forwards_composite_members(&self) -> bool {
        self.runs_full_const_prop()
    }

    pub(super) fn tracks_optional_unwraps(&self) -> bool {
        self.mode == SsaConstPropagationMode::LateOptionalUnwrap
    }

    /// Clear analysis state that is only valid within one SSA function body.
    pub(super) fn clear_tracked_values(&mut self) {
        self.constants.clear();
        self.atom_fielded_composites.clear();
        self.aliases.clear();
    }

    pub(super) fn is_optional_wrapper_type(&self, ty: &Type) -> bool {
        let Type::Composite(composite_ty) = ty else {
            return false;
        };
        let location = composite_ty.path.expect_global_location();
        let Some(composite) =
            self.state.symbol_table.lookup_struct(self.program, location).or_else(|| self.composites.get(location))
        else {
            return false;
        };
        let [is_some, val] = composite.members.as_slice() else {
            return false;
        };
        is_some.identifier.name == Symbol::intern("is_some")
            && matches!(is_some.type_, Type::Boolean)
            && val.identifier.name == Symbol::intern("val")
    }

    pub(super) fn is_optional_composite_expression(&self, composite: &CompositeExpression) -> bool {
        self.state.type_table.get(&composite.id()).is_some_and(|ty| self.is_optional_wrapper_type(&ty))
    }

    pub(super) fn resolve_composite_alias(&self, mut name: Symbol) -> Symbol {
        for _ in 0..self.aliases.len() {
            let Some(alias) = self.aliases.get(&name).copied() else {
                break;
            };
            if alias == name {
                break;
            }
            name = alias;
        }
        name
    }

    pub(super) fn atom_for_composite_member(
        &self,
        expression: &Expression,
        field: Symbol,
    ) -> Option<(Symbol, Expression)> {
        let Expression::Path(path) = expression else {
            return None;
        };
        let original_name = path.try_local_symbol()?;
        let name = self.resolve_composite_alias(original_name);
        let atom = self.atom_fielded_composites.get(&name)?.get(&field)?.clone();
        Some((name, atom))
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
