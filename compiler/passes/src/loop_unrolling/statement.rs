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

use leo_ast::{Expression::Literal, *};
use leo_errors::LoopUnrollerError;

use super::UnrollingVisitor;

impl StatementReconstructor for UnrollingVisitor<'_> {
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
        (input.into(), Default::default())
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
            if let LiteralVariant::Unsuffixed(s) = &resolved.variant {
                if let Some(Type::Integer(integer_type)) = self.state.type_table.get(&expr_id) {
                    resolved.variant = LiteralVariant::Integer(integer_type, s.clone());
                }
            }
            resolved
        };

        // Clone and resolve both literals
        let resolved_start_lit = resolve_unsuffixed(start_lit_ref, input.start.id());
        let resolved_stop_lit = resolve_unsuffixed(stop_lit_ref, input.stop.id());

        // Convert resolved literals into constant values
        let start_value = Value::try_from(&resolved_start_lit).unwrap();
        let stop_value = Value::try_from(&resolved_stop_lit).unwrap();

        // Ensure loop bounds are strictly increasing
        let bounds_invalid = match (&start_value, &stop_value) {
            (Value::I8(a, _), Value::I8(b, _)) => a >= b,
            (Value::I16(a, _), Value::I16(b, _)) => a >= b,
            (Value::I32(a, _), Value::I32(b, _)) => a >= b,
            (Value::I64(a, _), Value::I64(b, _)) => a >= b,
            (Value::I128(a, _), Value::I128(b, _)) => a >= b,
            (Value::U8(a, _), Value::U8(b, _)) => a >= b,
            (Value::U16(a, _), Value::U16(b, _)) => a >= b,
            (Value::U32(a, _), Value::U32(b, _)) => a >= b,
            (Value::U64(a, _), Value::U64(b, _)) => a >= b,
            (Value::U128(a, _), Value::U128(b, _)) => a >= b,
            _ => panic!("Type checking guarantees that loop bounds are integers of the same type."),
        };

        if bounds_invalid {
            self.emit_err(LoopUnrollerError::loop_range_decreasing(input.stop.span()));
        }

        self.loop_unrolled = true;

        // Perform loop unrolling using i128 — all numeric bounds are converted to this internally
        (self.unroll_iteration_statement::<i128>(input, start_value, stop_value), Default::default())
    }
}
