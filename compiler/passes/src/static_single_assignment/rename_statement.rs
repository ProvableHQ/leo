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

use crate::{RenameTable, StaticSingleAssigner};

use leo_ast::{
    AccessExpression,
    AssertStatement,
    AssertVariant,
    AssignStatement,
    AssociatedFunction,
    Block,
    CallExpression,
    ConditionalStatement,
    ConsoleStatement,
    ConstDeclaration,
    DefinitionStatement,
    Expression,
    ExpressionConsumer,
    ExpressionStatement,
    Identifier,
    IterationStatement,
    Node,
    ReturnStatement,
    Statement,
    StatementConsumer,
    TernaryExpression,
    TupleExpression,
    Type,
};
use leo_span::Symbol;

use indexmap::IndexSet;
use itertools::Itertools;

impl StatementConsumer for StaticSingleAssigner<'_> {
    type Output = Vec<Statement>;

    /// Consumes the expressions in an `AssertStatement`, returning the list of simplified statements.
    fn consume_assert(&mut self, input: AssertStatement) -> Self::Output {
        let (variant, mut statements) = match input.variant {
            AssertVariant::Assert(expr) => {
                let (expr, statements) = self.consume_expression(expr);
                (AssertVariant::Assert(expr), statements)
            }
            AssertVariant::AssertEq(left, right) => {
                // Reconstruct the lhs of the binary expression.
                let (left, mut statements) = self.consume_expression(left);
                // Reconstruct the rhs of the binary expression.
                let (right, right_statements) = self.consume_expression(right);
                // Accumulate any statements produced.
                statements.extend(right_statements);

                (AssertVariant::AssertEq(left, right), statements)
            }
            AssertVariant::AssertNeq(left, right) => {
                // Reconstruct the lhs of the binary expression.
                let (left, mut statements) = self.consume_expression(left);
                // Reconstruct the rhs of the binary expression.
                let (right, right_statements) = self.consume_expression(right);
                // Accumulate any statements produced.
                statements.extend(right_statements);

                (AssertVariant::AssertNeq(left, right), statements)
            }
        };

        // Add the assert statement to the list of produced statements.
        statements.push(Statement::Assert(AssertStatement { variant, span: input.span, id: input.id }));

        statements
    }

    /// Consume all `AssignStatement`s, renaming as necessary.
    fn consume_assign(&mut self, assign: AssignStatement) -> Self::Output {
        // First consume the right-hand-side of the assignment.
        let (value, mut statements) = self.consume_expression(assign.value);

        // Then assign a new unique name to the left-hand-side of the assignment.
        // Note that this order is necessary to ensure that the right-hand-side uses the correct name when consuming a complex assignment.
        self.is_lhs = true;
        let place = match self.consume_expression(assign.place).0 {
            Expression::Identifier(identifier) => identifier,
            _ => panic!("Type checking guarantees that the left-hand-side of an assignment is an identifier."),
        };
        self.is_lhs = false;

        statements.push(self.simple_assign_statement(place, value));

        statements
    }

    /// Consumes a `Block`, flattening its constituent `ConditionalStatement`s.
    fn consume_block(&mut self, block: Block) -> Self::Output {
        block.statements.into_iter().flat_map(|statement| self.consume_statement(statement)).collect()
    }

    /// Consumes a `ConditionalStatement`, producing phi functions (assign statements) for variables written in the then-block and otherwise-block.
    /// For more information on phi functions, see https://en.wikipedia.org/wiki/Static_single_assignment_form.
    /// Furthermore a new `AssignStatement` is introduced for non-trivial expressions in the condition of `ConditionalStatement`s.
    /// For example,
    ///   - `if x > 0 { x = x + 1 }` becomes `let $cond$0 = x > 0; if $cond$0 { x = x + 1; }`
    ///   - `if true { x = x + 1 }` remains the same.
    ///   - `if b { x = x + 1 }` remains the same.
    fn consume_conditional(&mut self, conditional: ConditionalStatement) -> Self::Output {
        // Simplify the condition and add it into the rename table.
        let (condition, mut statements) = self.consume_expression(conditional.condition);

        // Instantiate a `RenameTable` for the then-block.
        self.push();

        // Consume the then-block.
        let then = Block {
            span: conditional.then.span,
            id: conditional.then.id,
            statements: self.consume_block(conditional.then),
        };

        // Remove the `RenameTable` for the then-block.
        let if_table = self.pop();

        // Instantiate a `RenameTable` for the otherwise-block.
        self.push();

        // Consume the otherwise-block and flatten its constituent statements into the current block.
        let otherwise = conditional.otherwise.map(|otherwise| Box::new(Statement::Block(match *otherwise {
            Statement::Block(block) => Block {
                span: block.span,
                id: block.id,
                statements: self.consume_block(block),
            },
            Statement::Conditional(conditional) => Block {
                span: conditional.span,
                id: conditional.id,
                statements: self.consume_conditional(conditional),
            },
            _ => unreachable!("Type checking guarantees that the otherwise-block of a conditional statement is a block or another conditional statement."),
        })));

        // Remove the `RenameTable` for the otherwise-block.
        let else_table = self.pop();

        // Add reconstructed conditional statement to the list of produced statements.
        statements.push(Statement::Conditional(ConditionalStatement {
            span: conditional.span,
            id: conditional.id,
            condition: condition.clone(),
            then,
            otherwise,
        }));

        // Compute the write set for the variables written in the then-block or otherwise-block.
        let if_write_set: IndexSet<&Symbol> = IndexSet::from_iter(if_table.local_names());
        let else_write_set: IndexSet<&Symbol> = IndexSet::from_iter(else_table.local_names());
        let write_set = if_write_set.union(&else_write_set);

        // For each variable in the write set, instantiate and add a phi function to the list of produced statements.
        for symbol in write_set {
            // Note that phi functions only need to be instantiated if the variable exists before the `ConditionalStatement`.
            if self.rename_table.lookup(**symbol).is_some() {
                // Helper to lookup an and create an argument for the phi function.
                let create_phi_argument = |table: &RenameTable, symbol: Symbol| {
                    let name =
                        *table.lookup(symbol).unwrap_or_else(|| panic!("Symbol {symbol} should exist in the program."));
                    let id = *table
                        .lookup_id(&name)
                        .unwrap_or_else(|| panic!("Symbol {name} should exist in the rename table."));
                    Box::new(Expression::Identifier(Identifier { name, span: Default::default(), id }))
                };

                // Create a new name for the variable written to in the `ConditionalStatement`.
                let new_name = self.assigner.unique_symbol(symbol, "$");

                // Create the arguments for the phi function.
                let if_true = create_phi_argument(&if_table, **symbol);
                let if_false = create_phi_argument(&else_table, **symbol);

                // Create a new node ID for the the phi function.
                let id = self.node_builder.next_id();
                // Update the type of the node ID.
                let type_ = match self.type_table.get(&if_true.id()) {
                    Some(type_) => type_,
                    None => unreachable!("Type checking guarantees that all expressions have a type."),
                };
                self.type_table.insert(id, type_);

                // Construct a ternary expression for the phi function.
                let (value, stmts) = self.consume_ternary(TernaryExpression {
                    condition: Box::new(condition.clone()),
                    if_true,
                    if_false,
                    span: Default::default(),
                    id,
                });

                statements.extend(stmts);

                // Get the ID for the new name of the variable.
                let id = match self.rename_table.lookup_id(symbol) {
                    Some(id) => *id,
                    None => {
                        unreachable!("The ID for the symbol `{}` should already exist in the rename table.", symbol)
                    }
                };

                // Update the `RenameTable` with the new name of the variable.
                self.rename_table.update(*(*symbol), new_name, id);

                // Create a new `AssignStatement` for the phi function.
                let identifier = Identifier { name: new_name, span: Default::default(), id };
                let assignment = self.simple_assign_statement(identifier, value);

                // Store the generated phi function.
                statements.push(assignment);
            }
        }

        statements
    }

    /// Parsing guarantees that console statements are not present in the program.
    fn consume_console(&mut self, _: ConsoleStatement) -> Self::Output {
        unreachable!("Parsing guarantees that console statements are not present in the program.")
    }

    fn consume_const(&mut self, _: ConstDeclaration) -> Self::Output {
        unreachable!("Loop unrolling pass removes all constant declaration statements in the program.")
    }

    /// Consumes the `DefinitionStatement` into an `AssignStatement`, renaming the left-hand-side as appropriate.
    fn consume_definition(&mut self, definition: DefinitionStatement) -> Self::Output {
        // First consume the right-hand-side of the definition.
        let (value, mut statements) = self.consume_expression(definition.value);

        // Then assign a new unique name to the left-hand-side of the definition.
        // Note that this order is necessary to ensure that the right-hand-side uses the correct name when consuming a complex assignment.
        self.is_lhs = true;
        match definition.place {
            Expression::Identifier(identifier) => {
                // Add the identifier to the rename table.
                self.rename_table.update(identifier.name, identifier.name, identifier.id);
                // Rename the identifier.
                let identifier = match self.consume_identifier(identifier).0 {
                    Expression::Identifier(identifier) => identifier,
                    _ => unreachable!("`self.consume_identifier` will always return an `Identifier`."),
                };
                // Create a new assignment statement.
                statements.push(self.simple_assign_statement(identifier, value));
            }
            Expression::Tuple(tuple) => {
                let elements: Vec<Expression> = tuple.elements.into_iter().map(|element| {
                    match element {
                        Expression::Identifier(identifier) => {
                            // Add the identifier to the rename table.
                            self.rename_table.update(identifier.name, identifier.name, identifier.id);
                            // Rename the identifier.
                            let identifier = match self.consume_identifier(identifier).0 {
                                Expression::Identifier(identifier) => identifier,
                                _ => unreachable!("`self.consume_identifier` will always return an `Identifier`."),
                            };
                            // Return the renamed identifier.
                            Expression::Identifier(identifier)
                        }
                        _ => unreachable!("Type checking guarantees that the tuple elements on the lhs of a `DefinitionStatement` are always be identifiers."),
                    }
                }).collect();

                // Get the type of `value`.
                let tuple_type_ = match self.type_table.get(&value.id()) {
                    Some(Type::Tuple(type_)) => type_,
                    _ => unreachable!("Type checking guarantees that this expression is a tuple."),
                };

                // Update the type of each element in the tuple.
                for (element, type_) in elements.iter().zip_eq(tuple_type_.elements()) {
                    self.type_table.insert(element.id(), type_.clone());
                }

                // Construct the lhs of the assignment.
                let place = Expression::Tuple(TupleExpression {
                    elements,
                    span: Default::default(),
                    id: self.node_builder.next_id(),
                });

                // Update the type of the lhs.
                self.type_table.insert(place.id(), Type::Tuple(tuple_type_));

                // Create the assignment statement.
                let assignment = Statement::Assign(Box::new(AssignStatement {
                    place,
                    value,
                    span: definition.span,
                    id: definition.id,
                }));

                statements.push(assignment);
            }
            _ => unreachable!(
                "Type checking guarantees that the left-hand-side of a `DefinitionStatement` is an identifier or tuple."
            ),
        }
        self.is_lhs = false;

        statements
    }

    /// Consumes the expressions associated with `ExpressionStatement`, returning the simplified `ExpressionStatement`.
    fn consume_expression_statement(&mut self, input: ExpressionStatement) -> Self::Output {
        let mut statements = Vec::new();

        // Helper to process the arguments of a function call, accumulating any statements produced.
        let mut process_arguments = |arguments: Vec<Expression>| {
            arguments
                .into_iter()
                .map(|argument| {
                    let (argument, mut stmts) = self.consume_expression(argument);
                    statements.append(&mut stmts);
                    argument
                })
                .collect::<Vec<_>>()
        };

        match input.expression {
            Expression::Call(call) => {
                // Process the arguments.
                let arguments = process_arguments(call.arguments);
                // Create and accumulate the new expression statement.
                // Note that we do not create a new assignment for the call expression; this is necessary for correct code generation.
                statements.push(Statement::Expression(ExpressionStatement {
                    expression: Expression::Call(CallExpression {
                        function: call.function,
                        arguments,
                        external: call.external,
                        span: call.span,
                        id: call.id,
                    }),
                    span: input.span,
                    id: input.id,
                }));
            }
            Expression::Access(AccessExpression::AssociatedFunction(associated_function)) => {
                // Process the arguments.
                let arguments = process_arguments(associated_function.arguments);
                // Create and accumulate the new expression statement.
                // Note that we do not create a new assignment for the associated function; this is necessary for correct code generation.
                statements.push(Statement::Expression(ExpressionStatement {
                    expression: Expression::Access(AccessExpression::AssociatedFunction(AssociatedFunction {
                        ty: associated_function.ty,
                        name: associated_function.name,
                        arguments,
                        span: associated_function.span,
                        id: associated_function.id,
                    })),
                    span: input.span,
                    id: input.id,
                }))
            }

            _ => unreachable!("Type checking guarantees that expression statements are always function calls."),
        }

        statements
    }

    // TODO: Error message
    fn consume_iteration(&mut self, _input: IterationStatement) -> Self::Output {
        unreachable!("`IterationStatement`s should not be in the AST at this phase of compilation.");
    }

    /// Reconstructs the expression associated with the return statement, returning a simplified `ReturnStatement`.
    /// Note that type checking guarantees that there is at most one `ReturnStatement` in a block.
    fn consume_return(&mut self, input: ReturnStatement) -> Self::Output {
        // Consume the return expression.
        let (expression, mut statements) = self.consume_expression(input.expression);

        // Consume the finalize arguments if they exist.
        // Process the arguments, accumulating any statements produced.
        let finalize_args = input.finalize_arguments.map(|arguments| {
            arguments
                .into_iter()
                .map(|argument| {
                    let (argument, stmts) = self.consume_expression(argument);
                    statements.extend(stmts);
                    argument
                })
                .collect()
        });

        // Add the simplified return statement to the list of produced statements.
        statements.push(Statement::Return(ReturnStatement {
            expression,
            finalize_arguments: finalize_args,
            span: input.span,
            id: input.id,
        }));

        statements
    }
}
