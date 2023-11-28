// Copyright (C) 2019-2023 Aleo Systems Inc.
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

use crate::StaticSingleAssigner;

use leo_ast::{
    AccessExpression,
    ArrayAccess,
    ArrayExpression,
    AssociatedFunction,
    BinaryExpression,
    CallExpression,
    CastExpression,
    Expression,
    ExpressionConsumer,
    Identifier,
    Literal,
    MemberAccess,
    Statement,
    Struct,
    StructExpression,
    StructVariableInitializer,
    TernaryExpression,
    TupleAccess,
    TupleExpression,
    UnaryExpression,
    UnitExpression,
};
use leo_span::{sym, Symbol};

use indexmap::IndexMap;

impl ExpressionConsumer for StaticSingleAssigner<'_> {
    type Output = (Expression, Vec<Statement>);

    /// Consumes an access expression, accumulating any statements that are generated.
    fn consume_access(&mut self, input: AccessExpression) -> Self::Output {
        let (expr, mut statements) = match input {
            AccessExpression::AssociatedFunction(function) => {
                let mut statements = Vec::new();
                (
                    AccessExpression::AssociatedFunction(AssociatedFunction {
                        ty: function.ty,
                        name: function.name,
                        arguments: function
                            .arguments
                            .into_iter()
                            .map(|arg| {
                                let (arg, mut stmts) = self.consume_expression(arg);
                                statements.append(&mut stmts);
                                arg
                            })
                            .collect(),
                        span: function.span,
                        id: function.id,
                    }),
                    statements,
                )
            }
            AccessExpression::Member(member) => {
                // TODO: Create AST node for native access expressions?
                // If the access expression is of the form `self.<name>`, then don't rename it.
                if let Expression::Identifier(Identifier { name, .. }) = *member.inner {
                    if name == sym::SelfLower {
                        return (Expression::Access(AccessExpression::Member(member)), Vec::new());
                    }
                }

                let (expr, statements) = self.consume_expression(*member.inner);
                (
                    AccessExpression::Member(MemberAccess {
                        inner: Box::new(expr),
                        name: member.name,
                        span: member.span,
                        id: member.id,
                    }),
                    statements,
                )
            }
            AccessExpression::Tuple(tuple) => {
                let (expr, statements) = self.consume_expression(*tuple.tuple);
                (
                    AccessExpression::Tuple(TupleAccess {
                        tuple: Box::new(expr),
                        index: tuple.index,
                        span: tuple.span,
                        id: tuple.id,
                    }),
                    statements,
                )
            }
            AccessExpression::Array(input) => {
                let (array, statements) = self.consume_expression(*input.array);

                (
                    AccessExpression::Array(ArrayAccess {
                        array: Box::new(array),
                        index: input.index,
                        span: input.span,
                        id: input.id,
                    }),
                    statements,
                )
            }
            expr => (expr, Vec::new()),
        };
        let (place, statement) = self.unique_simple_assign_statement(Expression::Access(expr));
        statements.push(statement);

        (Expression::Identifier(place), statements)
    }

    /// Consumes an array expression, accumulating any statements that are generated.
    fn consume_array(&mut self, input: ArrayExpression) -> Self::Output {
        let mut statements = Vec::new();

        // Process the elements, accumulating any statements produced.
        let elements = input
            .elements
            .into_iter()
            .map(|element| {
                let (element, mut stmts) = self.consume_expression(element);
                statements.append(&mut stmts);
                element
            })
            .collect();

        // Construct and accumulate a new assignment statement for the array expression.
        let (place, statement) = self.unique_simple_assign_statement(Expression::Array(ArrayExpression {
            elements,
            span: input.span,
            id: input.id,
        }));
        statements.push(statement);

        (Expression::Identifier(place), statements)
    }

    /// Consumes a binary expression, accumulating any statements that are generated.
    fn consume_binary(&mut self, input: BinaryExpression) -> Self::Output {
        // Reconstruct the lhs of the binary expression.
        let (left_expression, mut statements) = self.consume_expression(*input.left);
        // Reconstruct the rhs of the binary expression.
        let (right_expression, mut right_statements) = self.consume_expression(*input.right);
        // Accumulate any statements produced.
        statements.append(&mut right_statements);

        // Construct and accumulate a unique assignment statement storing the result of the binary expression.
        let (place, statement) = self.unique_simple_assign_statement(Expression::Binary(BinaryExpression {
            left: Box::new(left_expression),
            right: Box::new(right_expression),
            op: input.op,
            span: input.span,
            id: input.id,
        }));
        statements.push(statement);

        (Expression::Identifier(place), statements)
    }

    /// Consumes a call expression without visiting the function name, accumulating any statements that are generated.
    fn consume_call(&mut self, input: CallExpression) -> Self::Output {
        let mut statements = Vec::new();

        // Process the arguments, accumulating any statements produced.
        let arguments = input
            .arguments
            .into_iter()
            .map(|argument| {
                let (argument, mut stmts) = self.consume_expression(argument);
                statements.append(&mut stmts);
                argument
            })
            .collect();

        // Construct and accumulate a new assignment statement for the call expression.
        let (place, statement) = self.unique_simple_assign_statement(Expression::Call(CallExpression {
            // Note that we do not rename the function name.
            function: input.function,
            // Consume the arguments.
            arguments,
            external: input.external,
            span: input.span,
            id: input.id,
        }));
        statements.push(statement);

        (Expression::Identifier(place), statements)
    }

    /// Consumes a cast expression, accumulating any statements that are generated.
    fn consume_cast(&mut self, input: CastExpression) -> Self::Output {
        // Reconstruct the expression being casted.
        let (expression, mut statements) = self.consume_expression(*input.expression);

        // Construct and accumulate a unique assignment statement storing the result of the cast expression.
        let (place, statement) = self.unique_simple_assign_statement(Expression::Cast(CastExpression {
            expression: Box::new(expression),
            type_: input.type_,
            span: input.span,
            id: input.id,
        }));
        statements.push(statement);

        (Expression::Identifier(place), statements)
    }

    /// Consumes a struct initialization expression with renamed variables, accumulating any statements that are generated.
    fn consume_struct_init(&mut self, input: StructExpression) -> Self::Output {
        let mut statements = Vec::new();

        // Process the members, accumulating any statements produced.
        let members: Vec<StructVariableInitializer> = input
            .members
            .into_iter()
            .map(|arg| {
                let (expression, mut stmts) = match &arg.expression.is_some() {
                    // If the expression is None, then `arg` is a `StructVariableInitializer` of the form `<id>,`.
                    // In this case, we must consume the identifier and produce an initializer of the form `<id>: <renamed_id>`.
                    false => self.consume_identifier(arg.identifier),
                    // If expression is `Some(..)`, then `arg is a `StructVariableInitializer` of the form `<id>: <expr>,`.
                    // In this case, we must consume the expression.
                    true => self.consume_expression(arg.expression.unwrap()),
                };
                // Accumulate any statements produced.
                statements.append(&mut stmts);

                // Return the new member.
                StructVariableInitializer {
                    identifier: arg.identifier,
                    expression: Some(expression),
                    span: arg.span,
                    id: arg.id,
                }
            })
            .collect();

        // Reorder the members to match that of the struct definition.

        // Lookup the struct definition.
        // Note that type checking guarantees that the correct struct definition exists.
        let struct_definition: &Struct = self.symbol_table.lookup_struct(input.name.name).unwrap();

        // Initialize the list of reordered members.
        let mut reordered_members = Vec::with_capacity(members.len());

        // Collect the members of the init expression into a map.
        let mut member_map: IndexMap<Symbol, StructVariableInitializer> =
            members.into_iter().map(|member| (member.identifier.name, member)).collect();

        // If we are initializing a record, add the `owner` first.
        // Note that type checking guarantees that the above fields exist.
        if struct_definition.is_record {
            // Add the `owner` field.
            // Note that the `unwrap` is safe, since type checking guarantees that the member exists.
            reordered_members.push(member_map.remove(&sym::owner).unwrap());
        }

        // For each member of the struct definition, push the corresponding member of the init expression.
        for member in &struct_definition.members {
            // If the member is part of a record and it is `owner` then we have already added it.
            if !(struct_definition.is_record && matches!(member.identifier.name, sym::owner)) {
                // Lookup and push the member of the init expression.
                // Note that the `unwrap` is safe, since type checking guarantees that the member exists.
                reordered_members.push(member_map.remove(&member.identifier.name).unwrap());
            }
        }

        // Construct and accumulate a new assignment statement for the struct expression.
        let (place, statement) = self.unique_simple_assign_statement(Expression::Struct(StructExpression {
            name: input.name,
            span: input.span,
            members: reordered_members,
            id: input.id,
        }));
        statements.push(statement);

        (Expression::Identifier(place), statements)
    }

    /// Produces a new `Identifier` with a unique name.
    fn consume_identifier(&mut self, identifier: Identifier) -> Self::Output {
        let name = match self.is_lhs {
            // If consuming the left-hand side of a definition or assignment, a new unique name is introduced.
            true => {
                let new_name = self.assigner.unique_symbol(identifier.name, "$");
                self.rename_table.update(identifier.name, new_name, identifier.id);
                new_name
            }
            // Otherwise, we look up the previous name in the `RenameTable`.
            // Note that we do not panic if the identifier is not found in the rename table.
            // Variables that do not exist in the rename table are ones that have been introduced during the SSA pass.
            // These variables are never re-assigned, and will never have an entry in the rename-table.
            false => *self.rename_table.lookup(identifier.name).unwrap_or(&identifier.name),
        };

        (Expression::Identifier(Identifier { name, span: identifier.span, id: identifier.id }), Default::default())
    }

    /// Consumes and returns the literal without making any modifications.
    fn consume_literal(&mut self, input: Literal) -> Self::Output {
        // Construct and accumulate a new assignment statement for the literal.
        let (place, statement) = self.unique_simple_assign_statement(Expression::Literal(input));
        (Expression::Identifier(place), vec![statement])
    }

    /// Consumes a ternary expression, accumulating any statements that are generated.
    fn consume_ternary(&mut self, input: TernaryExpression) -> Self::Output {
        // Reconstruct the condition of the ternary expression.
        let (cond_expr, mut statements) = self.consume_expression(*input.condition);
        // Reconstruct the if-true case of the ternary expression.
        let (if_true_expr, mut if_true_statements) = self.consume_expression(*input.if_true);
        // Reconstruct the if-false case of the ternary expression.
        let (if_false_expr, mut if_false_statements) = self.consume_expression(*input.if_false);

        // Accumulate any statements produced.
        statements.append(&mut if_true_statements);
        statements.append(&mut if_false_statements);

        // Construct and accumulate a unique assignment statement storing the result of the ternary expression.
        let (place, statement) = self.unique_simple_assign_statement(Expression::Ternary(TernaryExpression {
            condition: Box::new(cond_expr),
            if_true: Box::new(if_true_expr),
            if_false: Box::new(if_false_expr),
            span: input.span,
            id: input.id,
        }));
        statements.push(statement);

        (Expression::Identifier(place), statements)
    }

    /// Consumes a tuple expression, accumulating any statements that are generated
    fn consume_tuple(&mut self, input: TupleExpression) -> Self::Output {
        let mut statements = Vec::new();

        // Process the elements, accumulating any statements produced.
        let elements = input
            .elements
            .into_iter()
            .map(|element| {
                let (element, mut stmts) = self.consume_expression(element);
                statements.append(&mut stmts);
                element
            })
            .collect();

        // Construct and accumulate a new assignment statement for the tuple expression.
        let (place, statement) = self.unique_simple_assign_statement(Expression::Tuple(TupleExpression {
            elements,
            span: input.span,
            id: input.id,
        }));
        statements.push(statement);

        (Expression::Identifier(place), statements)
    }

    /// Consumes a unary expression, accumulating any statements that are generated.
    fn consume_unary(&mut self, input: UnaryExpression) -> Self::Output {
        // Reconstruct the operand of the unary expression.
        let (receiver, mut statements) = self.consume_expression(*input.receiver);

        // Construct and accumulate a new assignment statement for the unary expression.
        let (place, statement) = self.unique_simple_assign_statement(Expression::Unary(UnaryExpression {
            op: input.op,
            receiver: Box::new(receiver),
            span: input.span,
            id: input.id,
        }));
        statements.push(statement);

        (Expression::Identifier(place), statements)
    }

    fn consume_unit(&mut self, input: UnitExpression) -> Self::Output {
        (Expression::Unit(input), Default::default())
    }
}
