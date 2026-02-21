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

use super::SsaFormingVisitor;

use leo_ast::{
    ArrayAccess,
    ArrayExpression,
    BinaryExpression,
    CallExpression,
    CastExpression,
    Composite,
    CompositeExpression,
    CompositeFieldInitializer,
    Expression,
    ExpressionConsumer,
    IntrinsicExpression,
    Literal,
    MemberAccess,
    Path,
    RepeatExpression,
    Statement,
    TernaryExpression,
    TupleAccess,
    TupleExpression,
    UnaryExpression,
    UnitExpression,
};
use leo_span::{Symbol, sym};

use indexmap::IndexMap;

impl SsaFormingVisitor<'_> {
    /// Consume this expression and assign it to a variable (unless it's already an Identifier).
    pub fn consume_expression_and_define(&mut self, input: Expression) -> (Expression, Vec<Statement>) {
        let (expr, mut statements) = self.consume_expression(input);
        if matches!(expr, Expression::Path(..) | Expression::Unit(..) | Expression::Err(..)) {
            (expr, statements)
        } else {
            let (place, statement) = self.unique_simple_definition(expr);
            statements.push(statement);
            (Path::from(place).to_local().into(), statements)
        }
    }
}

impl ExpressionConsumer for SsaFormingVisitor<'_> {
    type Output = (Expression, Vec<Statement>);

    fn consume_array_access(&mut self, input: ArrayAccess) -> Self::Output {
        let (array, statements) = self.consume_expression_and_define(input.array);
        (ArrayAccess { array, ..input }.into(), statements)
    }

    fn consume_member_access(&mut self, input: MemberAccess) -> Self::Output {
        // If the access expression is of the form `self.<name>`, then don't rename it.
        if let Expression::Path(path) = &input.inner
            && path.identifier().name == sym::SelfLower
        {
            return (input.into(), Vec::new());
        }

        let (inner, statements) = self.consume_expression_and_define(input.inner);
        (MemberAccess { inner, ..input }.into(), statements)
    }

    fn consume_tuple_access(&mut self, input: TupleAccess) -> Self::Output {
        let (tuple, statements) = self.consume_expression_and_define(input.tuple);
        (TupleAccess { tuple, ..input }.into(), statements)
    }

    /// Consumes an array expression, accumulating any statements that are generated.
    fn consume_array(&mut self, input: ArrayExpression) -> Self::Output {
        let mut statements = Vec::new();

        // Process the elements, accumulating any statements produced.
        let elements = input
            .elements
            .into_iter()
            .map(|element| {
                let (element, mut stmts) = self.consume_expression_and_define(element);
                statements.append(&mut stmts);
                element
            })
            .collect();

        (ArrayExpression { elements, ..input }.into(), statements)
    }

    /// Consumes a binary expression, accumulating any statements that are generated.
    fn consume_binary(&mut self, input: BinaryExpression) -> Self::Output {
        // Reconstruct the lhs of the binary expression.
        let (left, mut statements) = self.consume_expression_and_define(input.left);
        // Reconstruct the rhs of the binary expression.
        let (right, mut right_statements) = self.consume_expression_and_define(input.right);
        // Accumulate any statements produced.
        statements.append(&mut right_statements);

        (BinaryExpression { left, right, ..input }.into(), statements)
    }

    /// Consumes a call expression without visiting the function name, accumulating any statements that are generated.
    fn consume_call(&mut self, input: CallExpression) -> Self::Output {
        let mut statements = Vec::new();

        // Process the arguments, accumulating any statements produced.
        let arguments = input
            .arguments
            .into_iter()
            .map(|argument| {
                let (argument, mut stmts) = self.consume_expression_and_define(argument);
                statements.append(&mut stmts);
                argument
            })
            .collect();

        (
            CallExpression {
                // Note that we do not rename the function name.
                arguments,
                ..input
            }
            .into(),
            statements,
        )
    }

    /// Consumes a cast expression, accumulating any statements that are generated.
    fn consume_cast(&mut self, input: CastExpression) -> Self::Output {
        // Reconstruct the expression being casted.
        let (expression, statements) = self.consume_expression_and_define(input.expression);
        (CastExpression { expression, ..input }.into(), statements)
    }

    /// Consumes a composite initialization expression with renamed variables, accumulating any statements that are generated.
    fn consume_composite_init(&mut self, input: CompositeExpression) -> Self::Output {
        let mut statements = Vec::new();

        // Process the members, accumulating any statements produced.
        let members: Vec<CompositeFieldInitializer> = input
            .members
            .into_iter()
            .map(|arg| {
                let (expression, mut stmts) = if let Some(expr) = arg.expression {
                    self.consume_expression_and_define(expr)
                } else {
                    self.consume_path(Path::from(arg.identifier).to_local())
                };
                // Accumulate any statements produced.
                statements.append(&mut stmts);

                // Return the new member.
                CompositeFieldInitializer { expression: Some(expression), ..arg }
            })
            .collect();

        // Reorder the members to match that of the composite definition.

        // Lookup the composite definition.
        let composite_location = input.path.expect_global_location();
        let composite_definition: &Composite = self
            .state
            .symbol_table
            .lookup_record(self.program, composite_location)
            .or_else(|| self.state.symbol_table.lookup_struct(self.program, composite_location))
            .expect("Type checking guarantees this definition exists.");

        // Initialize the list of reordered members.
        let mut reordered_members = Vec::with_capacity(members.len());

        // Collect the members of the init expression into a map.
        let mut member_map: IndexMap<Symbol, CompositeFieldInitializer> =
            members.into_iter().map(|member| (member.identifier.name, member)).collect();

        // If we are initializing a record, add the `owner` first.
        // Note that type checking guarantees that the above fields exist.
        if composite_definition.is_record {
            // Add the `owner` field.
            // Note that the `unwrap` is safe, since type checking guarantees that the member exists.
            reordered_members.push(member_map.shift_remove(&sym::owner).unwrap());
        }

        // For each member of the composite definition, push the corresponding member of the init expression.
        for member in &composite_definition.members {
            // If the member is part of a record and it is `owner` then we have already added it.
            if !(composite_definition.is_record && matches!(member.identifier.name, sym::owner)) {
                // Lookup and push the member of the init expression.
                // Note that the `unwrap` is safe, since type checking guarantees that the member exists.
                reordered_members.push(member_map.shift_remove(&member.identifier.name).unwrap());
            }
        }

        (CompositeExpression { members: reordered_members, ..input }.into(), statements)
    }

    /// Retrieve the new name for this `Identifier`.
    ///
    /// Note that this shouldn't be used for `Identifier`s on the lhs of definitions or
    /// assignments.
    fn consume_path(&mut self, path: Path) -> Self::Output {
        if let Some(name) = path.try_local_symbol() {
            // If lookup fails, then we didn't rename it.
            let name = *self.rename_table.lookup(name).unwrap_or(&name);
            (path.with_updated_last_symbol(name).into(), Default::default())
        } else {
            (path.into(), Default::default())
        }
    }

    /// Consumes and returns the literal without making any modifications.
    fn consume_literal(&mut self, input: Literal) -> Self::Output {
        (input.into(), Default::default())
    }

    fn consume_repeat(&mut self, input: RepeatExpression) -> Self::Output {
        let (expr, statements) = self.consume_expression_and_define(input.expr);

        // By now, the repeat count should be a literal. So we just ignore it. There is no need to SSA it.
        (RepeatExpression { expr, ..input }.into(), statements)
    }

    /// Consumes a ternary expression, accumulating any statements that are generated.
    fn consume_ternary(&mut self, input: TernaryExpression) -> Self::Output {
        // Reconstruct the condition of the ternary expression.
        let (cond_expr, mut statements) = self.consume_expression_and_define(input.condition);
        // Reconstruct the if-true case of the ternary expression.
        let (if_true_expr, if_true_statements) = self.consume_expression_and_define(input.if_true);
        // Reconstruct the if-false case of the ternary expression.
        let (if_false_expr, if_false_statements) = self.consume_expression_and_define(input.if_false);

        // Accumulate any statements produced.
        statements.extend(if_true_statements);
        statements.extend(if_false_statements);

        (
            TernaryExpression { condition: cond_expr, if_true: if_true_expr, if_false: if_false_expr, ..input }.into(),
            statements,
        )
    }

    /// Consumes a tuple expression, accumulating any statements that are generated
    fn consume_tuple(&mut self, input: TupleExpression) -> Self::Output {
        let mut statements = Vec::new();

        // Process the elements, accumulating any statements produced.
        let elements = input
            .elements
            .into_iter()
            .map(|element| {
                let (element, mut stmts) = self.consume_expression_and_define(element);
                statements.append(&mut stmts);
                element
            })
            .collect();

        (TupleExpression { elements, ..input }.into(), statements)
    }

    /// Consumes a unary expression, accumulating any statements that are generated.
    fn consume_unary(&mut self, input: UnaryExpression) -> Self::Output {
        // Reconstruct the operand of the unary expression.
        let (receiver, statements) = self.consume_expression_and_define(input.receiver);
        (UnaryExpression { receiver, ..input }.into(), statements)
    }

    fn consume_unit(&mut self, input: UnitExpression) -> Self::Output {
        (input.into(), Default::default())
    }

    fn consume_intrinsic(&mut self, input: leo_ast::IntrinsicExpression) -> Self::Output {
        let mut statements = Vec::new();
        let expr = IntrinsicExpression {
            arguments: input
                .arguments
                .into_iter()
                .map(|arg| {
                    let (arg, mut stmts) = self.consume_expression_and_define(arg);
                    statements.append(&mut stmts);
                    arg
                })
                .collect(),
            ..input
        }
        .into();

        (expr, statements)
    }

    fn consume_async(&mut self, input: leo_ast::AsyncExpression) -> Self::Output {
        (input.into(), Default::default())
    }
}
