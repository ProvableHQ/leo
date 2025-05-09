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

use super::FlatteningVisitor;

use leo_ast::{
    Expression,
    ExpressionReconstructor,
    Identifier,
    Location,
    Node,
    Statement,
    StructExpression,
    StructVariableInitializer,
    TernaryExpression,
    Type,
};

impl ExpressionReconstructor for FlatteningVisitor<'_> {
    type AdditionalOutput = Vec<Statement>;

    /// Reconstructs a struct init expression, flattening any tuples in the expression.
    fn reconstruct_struct_init(&mut self, input: StructExpression) -> (Expression, Self::AdditionalOutput) {
        let mut statements = Vec::new();
        let mut members = Vec::with_capacity(input.members.len());

        // Reconstruct and flatten the argument expressions.
        for member in input.members.into_iter() {
            // Note that this unwrap is safe since SSA guarantees that all struct variable initializers are of the form `<name>: <expr>`.
            let (expr, stmts) = self.reconstruct_expression(member.expression.unwrap());
            // Accumulate any statements produced.
            statements.extend(stmts);
            // Accumulate the struct members.
            members.push(StructVariableInitializer {
                identifier: member.identifier,
                expression: Some(expr),
                span: member.span,
                id: member.id,
            });
        }

        (StructExpression { members, ..input }.into(), statements)
    }

    /// Reconstructs ternary expressions over arrays, structs, and tuples, accumulating any statements that are generated.
    /// This is necessary because Aleo instructions does not support ternary expressions over composite data types.
    /// For example, the ternary expression `cond ? (a, b) : (c, d)` is flattened into the following:
    /// ```leo
    /// let var$0 = cond ? a : c;
    /// let var$1 = cond ? b : d;
    /// (var$0, var$1)
    /// ```
    /// For structs, the ternary expression `cond ? a : b`, where `a` and `b` are both structs `Foo { bar: u8, baz: u8 }`, is flattened into the following:
    /// ```leo
    /// let var$0 = cond ? a.bar : b.bar;
    /// let var$1 = cond ? a.baz : b.baz;
    /// let var$2 = Foo { bar: var$0, baz: var$1 };
    /// var$2
    /// ```
    fn reconstruct_ternary(&mut self, input: TernaryExpression) -> (Expression, Self::AdditionalOutput) {
        let if_true_type = self
            .state
            .type_table
            .get(&input.if_true.id())
            .expect("Type checking guarantees that all expressions are typed.");
        let if_false_type = self
            .state
            .type_table
            .get(&input.if_false.id())
            .expect("Type checking guarantees that all expressions are typed.");

        // Note that type checking guarantees that both expressions have the same same type. This is a sanity check.
        assert!(if_true_type.eq_flat_relaxed(&if_false_type));

        fn as_identifier(ident_expr: Expression) -> Identifier {
            let Expression::Identifier(identifier) = ident_expr else {
                panic!("SSA form should have guaranteed this is an identifier: {}.", ident_expr);
            };
            identifier
        }

        match &if_true_type {
            Type::Array(if_true_type) => self.ternary_array(
                if_true_type,
                &input.condition,
                &as_identifier(input.if_true),
                &as_identifier(input.if_false),
            ),
            Type::Composite(if_true_type) => {
                // Get the struct definitions.
                let program = if_true_type.program.unwrap_or(self.program);
                let if_true_type = self
                    .state
                    .symbol_table
                    .lookup_struct(if_true_type.id.name)
                    .or_else(|| self.state.symbol_table.lookup_record(Location::new(program, if_true_type.id.name)))
                    .expect("This definition should exist")
                    .clone();

                self.ternary_struct(
                    &if_true_type,
                    &input.condition,
                    &as_identifier(input.if_true),
                    &as_identifier(input.if_false),
                )
            }
            Type::Tuple(if_true_type) => {
                self.ternary_tuple(if_true_type, &input.condition, &input.if_true, &input.if_false)
            }
            _ => {
                // There's nothing to be done - SSA has guaranteed that `if_true` and `if_false` are identifiers,
                // so there's not even any point in reconstructing them.

                assert!(matches!(&input.if_true, Expression::Identifier(..)));
                assert!(matches!(&input.if_false, Expression::Identifier(..)));

                (input.into(), Default::default())
            }
        }
    }
}
