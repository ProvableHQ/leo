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

use leo_ast::{Expression::Literal, interpreter_value::literal_to_value, *};

use leo_errors::LoopUnrollerError;

use super::UnrollingVisitor;

impl AstReconstructor for UnrollingVisitor<'_> {
    type AdditionalInput = ();
    type AdditionalOutput = ();

    /* Expressions */
    fn reconstruct_repeat(
        &mut self,
        input: RepeatExpression,
        _additional: &(),
    ) -> (Expression, Self::AdditionalOutput) {
        // Because the value of `count` affects the type of a repeat expression, we need to assign a new ID to the
        // reconstructed `RepeatExpression` and update the type table accordingly.
        let new_id = self.state.node_builder.next_id();
        let new_count = self.reconstruct_expression(input.count, &()).0;
        let el_ty = self.state.type_table.get(&input.expr.id()).expect("guaranteed by type checking");
        self.state.type_table.insert(new_id, Type::Array(ArrayType::new(el_ty, new_count.clone())));
        (
            RepeatExpression {
                expr: self.reconstruct_expression(input.expr, &()).0,
                count: new_count,
                id: new_id,
                ..input
            }
            .into(),
            Default::default(),
        )
    }

    fn reconstruct_block(&mut self, mut input: Block) -> (Block, Self::AdditionalOutput) {
        self.in_scope(input.id(), |slf| {
            input.statements = input.statements.into_iter().map(|stmt| slf.reconstruct_statement(stmt).0).collect();

            (input, Default::default())
        })
    }

    fn reconstruct_const(&mut self, input: ConstDeclaration) -> (Statement, Self::AdditionalOutput) {
        (input.into(), Default::default())
    }

    fn reconstruct_definition(&mut self, input: DefinitionStatement) -> (Statement, Self::AdditionalOutput) {
        (
            DefinitionStatement {
                type_: input.type_.map(|ty| self.reconstruct_type(ty).0),
                value: self.reconstruct_expression(input.value, &()).0,
                ..input
            }
            .into(),
            Default::default(),
        )
    }

    fn reconstruct_iteration(&mut self, input: IterationStatement) -> (Statement, Self::AdditionalOutput) {
        // There's no need to reconstruct the bound expressions - they must be constants
        // which can be evaluated through constant propagation.

        let Literal(start_lit_ref) = &input.start else {
            self.loop_not_unrolled = Some(input.start.span());
            return (Statement::Iteration(Box::new(input)), Default::default());
        };

        let Literal(stop_lit_ref) = &input.stop else {
            self.loop_not_unrolled = Some(input.stop.span());
            return (Statement::Iteration(Box::new(input)), Default::default());
        };

        // Helper to clone and resolve Unsuffixed -> Integer literal based on type table
        let resolve_unsuffixed = |lit: &leo_ast::Literal, expr_id| {
            let mut resolved = lit.clone();
            if let LiteralVariant::Unsuffixed(s) = &resolved.variant
                && let Some(Type::Integer(integer_type)) = self.state.type_table.get(&expr_id)
            {
                resolved.variant = LiteralVariant::Integer(integer_type, s.clone());
            }
            resolved
        };

        // Clone and resolve both literals
        let resolved_start_lit = resolve_unsuffixed(start_lit_ref, input.start.id());
        let resolved_stop_lit = resolve_unsuffixed(stop_lit_ref, input.stop.id());

        // Convert resolved literals into constant values
        let start_value =
            literal_to_value(&resolved_start_lit, &None).expect("Parsing and type checking guarantee this works.");
        let stop_value =
            literal_to_value(&resolved_stop_lit, &None).expect("Parsing and type checking guarantee this works.");

        // Ensure loop bounds are strictly increasing
        if start_value.gte(&stop_value).expect("Type checking guarantees these are the same type") {
            self.emit_err(LoopUnrollerError::loop_range_decreasing(input.stop.span()));
        }

        self.loop_unrolled = true;

        // Actually unroll.
        (self.unroll_iteration_statement(input, start_value, stop_value), Default::default())
    }
}
