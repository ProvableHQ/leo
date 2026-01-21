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

use leo_ast::{Expression, Location, Node, NodeID, interpreter_value::Value};
use leo_errors::StaticAnalyzerError;
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
    /// Have we actually modified the program at all?
    pub changed: bool,
}

impl SsaConstPropagationVisitor<'_> {
    /// Emit a `StaticAnalyzerError`.
    pub fn emit_err(&self, err: StaticAnalyzerError) {
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
