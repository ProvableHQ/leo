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

use super::SsaFormingVisitor;
use crate::RenameTable;

use leo_ast::{
    AssertStatement,
    AssertVariant,
    AssignStatement,
    Block,
    ConditionalStatement,
    ConstDeclaration,
    DefinitionPlace,
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
};
use leo_span::Symbol;

use indexmap::IndexSet;

impl StatementConsumer for SsaFormingVisitor<'_> {
    type Output = Vec<Statement>;

    /// Consumes the expressions in an `AssertStatement`, returning the list of simplified statements.
    fn consume_assert(&mut self, input: AssertStatement) -> Self::Output {
        let (variant, mut statements) = match input.variant {
            AssertVariant::Assert(expr) => {
                let (expr, statements) = self.consume_expression_and_define(expr);
                (AssertVariant::Assert(expr), statements)
            }
            AssertVariant::AssertEq(left, right) => {
                // Reconstruct the lhs of the binary expression.
                let (left, mut statements) = self.consume_expression_and_define(left);
                // Reconstruct the rhs of the binary expression.
                let (right, right_statements) = self.consume_expression_and_define(right);
                // Accumulate any statements produced.
                statements.extend(right_statements);

                (AssertVariant::AssertEq(left, right), statements)
            }
            AssertVariant::AssertNeq(left, right) => {
                // Reconstruct the lhs of the binary expression.
                let (left, mut statements) = self.consume_expression_and_define(left);
                // Reconstruct the rhs of the binary expression.
                let (right, right_statements) = self.consume_expression_and_define(right);
                // Accumulate any statements produced.
                statements.extend(right_statements);

                (AssertVariant::AssertNeq(left, right), statements)
            }
        };

        // Add the assert statement to the list of produced statements.
        statements.push(AssertStatement { variant, ..input }.into());

        statements
    }

    /// Consume all `AssignStatement`s, renaming as necessary.
    fn consume_assign(&mut self, mut assign: AssignStatement) -> Self::Output {
        let (value, mut statements) = self.consume_expression(assign.value);
        if let Expression::Identifier(name) = assign.place {
            // Then assign a new unique name to the left-hand-side of the assignment.
            // Note that this order is necessary to ensure that the right-hand-side uses the correct name when consuming a complex assignment.
            let new_place = self.rename_identifier(name);

            statements.push(self.simple_definition(new_place, value));
            statements
        } else {
            // It must be a sequence of accesses ending in an identifier.
            // This loop will iterate until the identifier is reached.
            // For example, `some_identifier[i].member` -> `some_identifier[i]` -> `some_identifier`.
            // All we need to do is consume that identifier to possibly get a new name.
            let mut place = &mut assign.place;
            loop {
                match place {
                    Expression::ArrayAccess(array_access) => place = &mut array_access.array,
                    Expression::MemberAccess(member_access) => place = &mut member_access.inner,
                    Expression::TupleAccess(tuple_access) => place = &mut tuple_access.tuple,
                    expr @ Expression::Identifier(..) => {
                        let (new_expr, statements2) = self.consume_expression(std::mem::take(expr));
                        *expr = new_expr;
                        statements.extend(statements2);
                        assign.value = value;
                        statements.push(assign.into());
                        return statements;
                    }
                    _ => panic!("Type checking should have ensured this is not possible."),
                }
            }
        }
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
        let (condition, mut statements) = self.consume_expression_and_define(conditional.condition);

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
            _ => panic!("Type checking guarantees that the otherwise-block of a conditional statement is a block or another conditional statement."),
        })));

        // Remove the `RenameTable` for the otherwise-block.
        let else_table = self.pop();

        // Add reconstructed conditional statement to the list of produced statements.
        statements.push(ConditionalStatement { condition: condition.clone(), then, otherwise, ..conditional }.into());

        // Compute the write set for the variables written in the then-block or otherwise-block.
        let if_write_set: IndexSet<&Symbol> = IndexSet::from_iter(if_table.local_names());
        let else_write_set: IndexSet<&Symbol> = IndexSet::from_iter(else_table.local_names());
        let write_set = if_write_set.union(&else_write_set);

        // For each variable in the write set, instantiate and add a phi function to the list of produced statements.
        for symbol in write_set {
            // Note that phi functions only need to be instantiated if the variable exists before the `ConditionalStatement`.
            if self.rename_table.lookup(**symbol).is_some() {
                // Helper to lookup an and create an argument for the phi function.
                let create_phi_argument = |table: &RenameTable, symbol: Symbol| -> Expression {
                    let name =
                        *table.lookup(symbol).unwrap_or_else(|| panic!("Symbol {symbol} should exist in the program."));
                    let id = *table
                        .lookup_id(&name)
                        .unwrap_or_else(|| panic!("Symbol {name} should exist in the rename table."));
                    Identifier { name, span: Default::default(), id }.into()
                };

                // Create a new name for the variable written to in the `ConditionalStatement`.
                let new_name = self.state.assigner.unique_symbol(symbol, "$");

                // Create the arguments for the phi function.
                let if_true = create_phi_argument(&if_table, **symbol);
                let if_false = create_phi_argument(&else_table, **symbol);

                // Create a new node ID for the phi function.
                let id = self.state.node_builder.next_id();
                // Update the type of the node ID.
                let Some(type_) = self.state.type_table.get(&if_true.id()) else {
                    panic!("Type checking guarantees that all expressions have a type.");
                };
                self.state.type_table.insert(id, type_);

                // Construct a ternary expression for the phi function.
                let (value, stmts) = self.consume_ternary(TernaryExpression {
                    condition: condition.clone(),
                    if_true,
                    if_false,
                    span: Default::default(),
                    id,
                });

                statements.extend(stmts);

                // Get the ID for the new name of the variable.
                let id = *self.rename_table.lookup_id(symbol).unwrap_or_else(|| {
                    panic!("The ID for the symbol `{symbol}` should already exist in the rename table.")
                });

                // Update the `RenameTable` with the new name of the variable.
                self.rename_table.update(**symbol, new_name, id);

                // Create a new `DefinitionStatement` for the phi function.
                let identifier = Identifier { name: new_name, span: Default::default(), id };
                let assignment = self.simple_definition(identifier, value);

                // Store the generated phi function.
                statements.push(assignment);
            }
        }

        statements
    }

    fn consume_const(&mut self, _: ConstDeclaration) -> Self::Output {
        // Constants have been propagated everywhere by this point, so we no longer need const declarations.
        Default::default()
    }

    /// Consumes the `DefinitionStatement` into an `AssignStatement`, renaming the left-hand-side as appropriate.
    fn consume_definition(&mut self, definition: DefinitionStatement) -> Self::Output {
        let mut statements = Vec::new();

        match definition.place {
            DefinitionPlace::Single(identifier) => {
                // Consume the right-hand-side of the definition.
                let (value, statements2) = self.consume_expression(definition.value);
                statements = statements2;
                let new_identifier = if self.rename_defs { self.rename_identifier(identifier) } else { identifier };
                // Create a new assignment statement.
                statements.push(self.simple_definition(new_identifier, value));
            }
            DefinitionPlace::Multiple(identifiers) => {
                let new_identifiers: Vec<Identifier> = if self.rename_defs {
                    identifiers
                        .into_iter()
                        .map(
                            |identifier| if self.rename_defs { self.rename_identifier(identifier) } else { identifier },
                        )
                        .collect()
                } else {
                    identifiers
                };

                // We don't need to update the type table, as the new identifiers have
                // the same IDs as the old ones.

                // Construct the lhs of the assignment.
                let place = DefinitionPlace::Multiple(new_identifiers);

                let value = if let Expression::Call(mut call) = definition.value {
                    for argument in call.arguments.iter_mut() {
                        let (new_argument, new_statements) = self.consume_expression(std::mem::take(argument));
                        *argument = new_argument;
                        statements.extend(new_statements);
                    }
                    Expression::Call(call)
                } else {
                    let (value, new_statements) = self.consume_expression(definition.value);
                    statements.extend(new_statements);
                    value
                };

                // Create the definition.
                let definition = DefinitionStatement { place, type_: None, value, ..definition }.into();

                statements.push(definition);
            }
        }

        statements
    }

    /// Consumes the expressions associated with `ExpressionStatement`, returning the simplified `ExpressionStatement`.
    fn consume_expression_statement(&mut self, mut input: ExpressionStatement) -> Self::Output {
        let (expr, mut statements) = self.consume_expression(input.expression);
        input.expression = expr;
        statements.push(input.into());
        statements
    }

    fn consume_iteration(&mut self, _input: IterationStatement) -> Self::Output {
        panic!("`IterationStatement`s should not be in the AST at this phase of compilation.");
    }

    /// Reconstructs the expression associated with the return statement, returning a simplified `ReturnStatement`.
    /// Note that type checking guarantees that there is at most one `ReturnStatement` in a block.
    fn consume_return(&mut self, mut input: ReturnStatement) -> Self::Output {
        if let Expression::Tuple(tuple_expr) = &mut input.expression {
            // Leave tuple expressions alone.
            let mut statements = Vec::new();
            for element in tuple_expr.elements.iter_mut() {
                let (new_element, new_statements) = self.consume_expression_and_define(std::mem::take(element));
                *element = new_element;
                statements.extend(new_statements);
            }
            statements.push(input.into());
            statements
        } else {
            let (expression, mut statements) = self.consume_expression_and_define(input.expression);
            input.expression = expression;
            statements.push(input.into());
            statements
        }
    }
}
