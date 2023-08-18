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

use crate::Flattener;
use itertools::Itertools;
use std::borrow::Borrow;

use leo_ast::{
    AccessExpression,
    AssertStatement,
    AssertVariant,
    AssignStatement,
    AssociatedFunction,
    BinaryExpression,
    BinaryOperation,
    Block,
    ConditionalStatement,
    ConsoleStatement,
    DefinitionStatement,
    Expression,
    ExpressionReconstructor,
    Identifier,
    IterationStatement,
    Node,
    ReturnStatement,
    Statement,
    StatementReconstructor,
    TupleExpression,
    Type,
    UnaryExpression,
    UnaryOperation,
};
use leo_span::sym;

impl StatementReconstructor for Flattener<'_> {
    /// Rewrites an assert statement into a flattened form.
    /// Assert statements at the top level only have their arguments flattened.
    /// Assert statements inside a conditional statement are flattened to such that the check is conditional on
    /// the execution path being valid.
    /// For example, the following snippet:
    /// ```leo
    /// if condition1 {
    ///    if condition2 {
    ///        assert(foo);
    ///    }
    /// }
    /// ```
    /// is flattened to:
    /// ```leo
    /// assert(!(condition1 && condition2) || foo);
    /// ```
    /// which is equivalent to the logical formula `(condition1 /\ condition2) ==> foo`.
    fn reconstruct_assert(&mut self, input: AssertStatement) -> (Statement, Self::AdditionalOutput) {
        let mut statements = Vec::new();

        // Flatten the arguments of the assert statement.
        let assert = AssertStatement {
            span: input.span,
            id: input.id,
            variant: match input.variant {
                AssertVariant::Assert(expression) => {
                    let (expression, additional_statements) = self.reconstruct_expression(expression);
                    statements.extend(additional_statements);
                    AssertVariant::Assert(expression)
                }
                AssertVariant::AssertEq(left, right) => {
                    let (left, additional_statements) = self.reconstruct_expression(left);
                    statements.extend(additional_statements);
                    let (right, additional_statements) = self.reconstruct_expression(right);
                    statements.extend(additional_statements);
                    AssertVariant::AssertEq(left, right)
                }
                AssertVariant::AssertNeq(left, right) => {
                    let (left, additional_statements) = self.reconstruct_expression(left);
                    statements.extend(additional_statements);
                    let (right, additional_statements) = self.reconstruct_expression(right);
                    statements.extend(additional_statements);
                    AssertVariant::AssertNeq(left, right)
                }
            },
        };

        // Add the appropriate guards.
        match self.construct_guard() {
            // If the condition stack is empty, we can return the flattened assert statement.
            None => (Statement::Assert(assert), statements),
            // Otherwise, we need to join the guard with the expression in the flattened assert statement.
            // Note given the guard and the expression, we construct the logical formula `guard => expression`,
            // which is equivalent to `!guard || expression`.
            Some(guard) => (
                Statement::Assert(AssertStatement {
                    span: input.span,
                    id: input.id,
                    variant: AssertVariant::Assert(Expression::Binary(BinaryExpression {
                        op: BinaryOperation::Or,
                        span: Default::default(),
                        id: self.node_builder.next_id(),
                        // Take the logical negation of the guard.
                        left: Box::new(Expression::Unary(UnaryExpression {
                            op: UnaryOperation::Not,
                            receiver: Box::new(guard),
                            span: Default::default(),
                            id: self.node_builder.next_id(),
                        })),
                        right: Box::new(match assert.variant {
                            // If the assert statement is an `assert`, use the expression as is.
                            AssertVariant::Assert(expression) => expression,
                            // If the assert statement is an `assert_eq`, construct a new equality expression.
                            AssertVariant::AssertEq(left, right) => Expression::Binary(BinaryExpression {
                                left: Box::new(left),
                                op: BinaryOperation::Eq,
                                right: Box::new(right),
                                span: Default::default(),
                                id: self.node_builder.next_id(),
                            }),
                            // If the assert statement is an `assert_ne`, construct a new inequality expression.
                            AssertVariant::AssertNeq(left, right) => Expression::Binary(BinaryExpression {
                                left: Box::new(left),
                                op: BinaryOperation::Neq,
                                right: Box::new(right),
                                span: Default::default(),
                                id: self.node_builder.next_id(),
                            }),
                        }),
                    })),
                }),
                statements,
            ),
        }
    }

    /// Flattens an assign statement, if necessary.
    /// Marks variables as structs as necessary.
    /// Note that new statements are only produced if the right hand side is a ternary expression over structs.
    /// Otherwise, the statement is returned as is.
    fn reconstruct_assign(&mut self, assign: AssignStatement) -> (Statement, Self::AdditionalOutput) {
        // Flatten the rhs of the assignment.
        let (value, mut statements) = self.reconstruct_expression(assign.value);
        match (assign.place, value.clone()) {
            // If the lhs is an identifier and the rhs is a tuple, then add the tuple to `self.tuples`.
            (Expression::Identifier(identifier), Expression::Tuple(tuple)) => {
                self.tuples.insert(identifier.name, tuple);
                // Note that tuple assignments are removed from the AST.
                (Statement::dummy(Default::default(), self.node_builder.next_id()), statements)
            }
            // If the lhs is an identifier and the rhs is an identifier that is a tuple, then add it to `self.tuples`.
            (Expression::Identifier(lhs_identifier), Expression::Identifier(rhs_identifier))
                if self.tuples.contains_key(&rhs_identifier.name) =>
            {
                // Lookup the entry in `self.tuples` and add it for the lhs of the assignment.
                // Note that the `unwrap` is safe since the match arm checks that the entry exists.
                self.tuples.insert(lhs_identifier.name, self.tuples.get(&rhs_identifier.name).unwrap().clone());
                // Note that tuple assignments are removed from the AST.
                (Statement::dummy(Default::default(), self.node_builder.next_id()), statements)
            }
            // If the lhs is an identifier and the rhs is a function call that produces a tuple, then add it to `self.tuples`.
            (Expression::Identifier(lhs_identifier), Expression::Call(call)) => {
                // Retrieve the entry in the symbol table for the function call.
                // Note that this unwrap is safe since type checking ensures that the function exists.
                let function_name = match call.function.borrow() {
                    Expression::Identifier(rhs_identifier) => rhs_identifier.name,
                    _ => unreachable!("Parsing guarantees that `function` is an identifier."),
                };

                let function = self.symbol_table.lookup_fn_symbol(function_name).unwrap();
                match &function.output_type {
                    // If the function returns a tuple, reconstruct the assignment and add an entry to `self.tuples`.
                    Type::Tuple(tuple) => {
                        // Create a new tuple expression with unique identifiers for each index of the lhs.
                        let tuple_expression = TupleExpression {
                            elements: (0..tuple.len())
                                .zip_eq(tuple.0.iter())
                                .map(|(i, type_)| {
                                    let identifier = Identifier::new(
                                        self.assigner.unique_symbol(lhs_identifier.name, format!("$index${i}$")),
                                        self.node_builder.next_id(),
                                    );

                                    // If the output type is a struct, add it to `self.structs`.
                                    if let Type::Identifier(struct_name) = type_ {
                                        self.structs.insert(identifier.name, struct_name.name);
                                    }

                                    Expression::Identifier(identifier)
                                })
                                .collect(),
                            span: Default::default(),
                            id: self.node_builder.next_id(),
                        };
                        // Add the `tuple_expression` to `self.tuples`.
                        self.tuples.insert(lhs_identifier.name, tuple_expression.clone());
                        // Construct a new assignment statement with a tuple expression on the lhs.
                        (
                            Statement::Assign(Box::new(AssignStatement {
                                place: Expression::Tuple(tuple_expression),
                                value: Expression::Call(call),
                                span: Default::default(),
                                id: self.node_builder.next_id(),
                            })),
                            statements,
                        )
                    }
                    // Otherwise, reconstruct the assignment as is.
                    type_ => {
                        // If the function returns a struct, add it to `self.structs`.
                        if let Type::Identifier(struct_name) = type_ {
                            self.structs.insert(lhs_identifier.name, struct_name.name);
                        };
                        (
                            Statement::Assign(Box::new(AssignStatement {
                                place: Expression::Identifier(lhs_identifier),
                                value: Expression::Call(call),
                                span: Default::default(),
                                id: self.node_builder.next_id(),
                            })),
                            statements,
                        )
                    }
                }
            }
            // If the `rhs` is an invocation of `get` or `get_or_use` on a mapping, then check if the value type is a struct.
            // Note that the parser rewrites `.get` and `.get_or_use` to `Mapping::get` and `Mapping::get_or_use` respectively.
            (
                Expression::Identifier(lhs_identifier),
                Expression::Access(AccessExpression::AssociatedFunction(AssociatedFunction {
                    ty: Type::Identifier(Identifier { name: sym::Mapping, .. }),
                    name: Identifier { name: sym::get, .. },
                    arguments,
                    ..
                })),
            )
            | (
                Expression::Identifier(lhs_identifier),
                Expression::Access(AccessExpression::AssociatedFunction(AssociatedFunction {
                    ty: Type::Identifier(Identifier { name: sym::Mapping, .. }),
                    name: Identifier { name: sym::get_or_use, .. },
                    arguments,
                    ..
                })),
            ) => {
                // Get the value type of the mapping.
                let value_type = match arguments[0] {
                    Expression::Identifier(identifier) => {
                        // Retrieve the entry in the symbol table for the mapping.
                        // Note that this unwrap is safe since type checking ensures that the mapping exists.
                        let variable = self.symbol_table.lookup_variable(identifier.name).unwrap();
                        match &variable.type_ {
                            Type::Mapping(mapping_type) => &*mapping_type.value,
                            _ => unreachable!("Type checking guarantee that `arguments[0]` is a mapping."),
                        }
                    }
                    _ => unreachable!("Type checking guarantee that `arguments[0]` is the name of the mapping."),
                };
                // If the value type is a struct, add it to `self.structs`.
                if let Type::Identifier(struct_name) = value_type {
                    self.structs.insert(lhs_identifier.name, struct_name.name);
                }
                // Reconstruct the assignment.
                (
                    Statement::Assign(Box::new(AssignStatement {
                        place: Expression::Identifier(lhs_identifier),
                        value,
                        span: Default::default(),
                        id: self.node_builder.next_id(),
                    })),
                    statements,
                )
            }
            (Expression::Identifier(identifier), expression) => {
                self.update_structs(&identifier, &expression);
                (self.assigner.simple_assign_statement(identifier, expression, self.node_builder.next_id()), statements)
            }
            // If the lhs is a tuple and the rhs is a function call, then return the reconstructed statement.
            (Expression::Tuple(tuple), Expression::Call(call)) => {
                // Retrieve the entry in the symbol table for the function call.
                // Note that this unwrap is safe since type checking ensures that the function exists.
                let function_name = match call.function.borrow() {
                    Expression::Identifier(rhs_identifier) => rhs_identifier.name,
                    _ => unreachable!("Parsing guarantees that `function` is an identifier."),
                };

                let function = self.symbol_table.lookup_fn_symbol(function_name).unwrap();

                let output_type = match &function.output_type {
                    Type::Tuple(tuple) => tuple.clone(),
                    _ => unreachable!("Type checking guarantees that the output type is a tuple."),
                };

                tuple.elements.iter().zip_eq(output_type.0.iter()).for_each(|(identifier, type_)| {
                    let identifier = match identifier {
                        Expression::Identifier(identifier) => identifier,
                        _ => unreachable!("Type checking guarantees that a tuple element on the lhs is an identifier."),
                    };
                    // If the output type is a struct, add it to `self.structs`.
                    if let Type::Identifier(struct_name) = type_ {
                        self.structs.insert(identifier.name, struct_name.name);
                    }
                });

                (
                    Statement::Assign(Box::new(AssignStatement {
                        place: Expression::Tuple(tuple),
                        value: Expression::Call(call),
                        span: Default::default(),
                        id: self.node_builder.next_id(),
                    })),
                    statements,
                )
            }
            // If the lhs is a tuple and the rhs is a tuple, create a new assign statement for each tuple element.
            (Expression::Tuple(lhs_tuple), Expression::Tuple(rhs_tuple)) => {
                statements.extend(lhs_tuple.elements.into_iter().zip(rhs_tuple.elements).map(|(lhs, rhs)| {
                    let identifier = match &lhs {
                        Expression::Identifier(identifier) => identifier,
                        _ => unreachable!("Type checking guarantees that `lhs` is an identifier."),
                    };
                    self.update_structs(identifier, &rhs);
                    Statement::Assign(Box::new(AssignStatement {
                        place: lhs,
                        value: rhs,
                        span: Default::default(),
                        id: self.node_builder.next_id(),
                    }))
                }));
                (Statement::dummy(Default::default(), self.node_builder.next_id()), statements)
            }
            // If the lhs is a tuple and the rhs is an identifier that is a tuple, create a new assign statement for each tuple element.
            (Expression::Tuple(lhs_tuple), Expression::Identifier(identifier))
                if self.tuples.contains_key(&identifier.name) =>
            {
                // Lookup the entry in `self.tuples`.
                // Note that the `unwrap` is safe since the match arm checks that the entry exists.
                let rhs_tuple = self.tuples.get(&identifier.name).unwrap().clone();
                // Create a new assign statement for each tuple element.
                for (lhs, rhs) in lhs_tuple.elements.into_iter().zip(rhs_tuple.elements) {
                    let identifier = match &lhs {
                        Expression::Identifier(identifier) => identifier,
                        _ => unreachable!("Type checking guarantees that `lhs` is an identifier."),
                    };
                    self.update_structs(identifier, &rhs);

                    statements.push(Statement::Assign(Box::new(AssignStatement {
                        place: lhs,
                        value: rhs,
                        span: Default::default(),
                        id: self.node_builder.next_id(),
                    })));
                }
                (Statement::dummy(Default::default(), self.node_builder.next_id()), statements)
            }
            // If the lhs of an assignment is a tuple, then the rhs can be one of the following:
            //  - A function call that produces a tuple. (handled above)
            //  - A tuple. (handled above)
            //  - An identifier that is a tuple. (handled above)
            //  - A ternary expression that produces a tuple. (handled when the rhs is flattened above)
            (Expression::Tuple(_), _) => {
                unreachable!("`Type checking guarantees that the rhs of an assignment to a tuple is a tuple.`")
            }
            _ => unreachable!("`AssignStatement`s can only have `Identifier`s or `Tuple`s on the left hand side."),
        }
    }

    // TODO: Do we want to flatten nested blocks? They do not affect code generation but it would regularize the AST structure.
    /// Flattens the statements inside a basic block.
    /// The resulting block does not contain any conditional statements.
    fn reconstruct_block(&mut self, block: Block) -> (Block, Self::AdditionalOutput) {
        let mut statements = Vec::with_capacity(block.statements.len());

        // Flatten each statement, accumulating any new statements produced.
        for statement in block.statements {
            let (reconstructed_statement, additional_statements) = self.reconstruct_statement(statement);
            statements.extend(additional_statements);
            statements.push(reconstructed_statement);
        }

        (Block { span: block.span, statements, id: self.node_builder.next_id() }, Default::default())
    }

    /// Flatten a conditional statement into a list of statements.
    fn reconstruct_conditional(&mut self, conditional: ConditionalStatement) -> (Statement, Self::AdditionalOutput) {
        let mut statements = Vec::with_capacity(conditional.then.statements.len());

        // Add condition to the condition stack.
        self.condition_stack.push(conditional.condition.clone());

        // Reconstruct the then-block and accumulate it constituent statements.
        statements.extend(self.reconstruct_block(conditional.then).0.statements);

        // Remove condition from the condition stack.
        self.condition_stack.pop();

        // Consume the otherwise-block and flatten its constituent statements into the current block.
        if let Some(statement) = conditional.otherwise {
            // Add the negated condition to the condition stack.
            self.condition_stack.push(Expression::Unary(UnaryExpression {
                op: UnaryOperation::Not,
                receiver: Box::new(conditional.condition.clone()),
                span: conditional.condition.span(),
                id: conditional.condition.id(),
            }));

            // Reconstruct the otherwise-block and accumulate it constituent statements.
            match *statement {
                Statement::Block(block) => statements.extend(self.reconstruct_block(block).0.statements),
                _ => unreachable!("SSA guarantees that the `otherwise` is always a `Block`"),
            }

            // Remove the negated condition from the condition stack.
            self.condition_stack.pop();
        };

        (Statement::dummy(Default::default(), self.node_builder.next_id()), statements)
    }

    fn reconstruct_console(&mut self, _: ConsoleStatement) -> (Statement, Self::AdditionalOutput) {
        unreachable!("`ConsoleStatement`s should not be in the AST at this phase of compilation.")
    }

    /// Static single assignment converts definition statements into assignment statements.
    fn reconstruct_definition(&mut self, _definition: DefinitionStatement) -> (Statement, Self::AdditionalOutput) {
        unreachable!("`DefinitionStatement`s should not exist in the AST at this phase of compilation.")
    }

    // TODO: Error message requesting the user to enable loop-unrolling.
    fn reconstruct_iteration(&mut self, _input: IterationStatement) -> (Statement, Self::AdditionalOutput) {
        unreachable!("`IterationStatement`s should not be in the AST at this phase of compilation.");
    }

    /// Transforms a return statement into an empty block statement.
    /// Stores the arguments to the return statement, which are later folded into a single return statement at the end of the function.
    fn reconstruct_return(&mut self, input: ReturnStatement) -> (Statement, Self::AdditionalOutput) {
        // Construct the associated guard.
        let guard = self.construct_guard();

        // Note that SSA guarantees that `input.expression` is either a literal, identifier, or unit expression.
        match input.expression {
            // If the input is an identifier that maps to a tuple,
            // construct a `ReturnStatement` with the tuple and add it to `self.returns`
            Expression::Identifier(identifier) if self.tuples.contains_key(&identifier.name) => {
                // Note that the `unwrap` is safe since the match arm checks that the entry exists in `self.tuples`.
                let tuple = self.tuples.get(&identifier.name).unwrap().clone();
                self.returns.push((guard, ReturnStatement {
                    span: input.span,
                    expression: Expression::Tuple(tuple),
                    finalize_arguments: input.finalize_arguments,
                    id: input.id,
                }));
            }
            // Otherwise, add the expression directly.
            _ => self.returns.push((guard, input)),
        };

        (Statement::dummy(Default::default(), self.node_builder.next_id()), Default::default())
    }
}
