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

use super::{FlatteningVisitor, Guard, ReturnGuard};

use leo_ast::*;

use itertools::Itertools;

impl AstReconstructor for FlatteningVisitor<'_> {
    type AdditionalOutput = Vec<Statement>;

    /* Expressions */
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

    /* Statements */
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

        // If we are traversing an async function, then we can return the assert as it.
        if self.is_async {
            return (input.into(), statements);
        }

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

        let mut guards: Vec<Expression> = Vec::new();

        if let Some((guard, guard_statements)) = self.construct_guard() {
            statements.extend(guard_statements);

            // The not_guard is true if we didn't follow the condition chain
            // that led to this assertion.
            let not_guard = UnaryExpression {
                op: UnaryOperation::Not,
                receiver: guard.into(),
                span: Default::default(),
                id: {
                    // Create a new node ID for the unary expression.
                    let id = self.state.node_builder.next_id();
                    // Update the type table with the type of the unary expression.
                    self.state.type_table.insert(id, Type::Boolean);
                    id
                },
            }
            .into();
            let (identifier, statement) = self.unique_simple_definition(not_guard);
            statements.push(statement);
            guards.push(identifier.into());
        }

        // We also need to guard against early returns.
        if let Some((guard, guard_statements)) = self.construct_early_return_guard() {
            guards.push(guard.into());
            statements.extend(guard_statements);
        }

        if guards.is_empty() {
            return (assert.into(), statements);
        }

        let is_eq = matches!(assert.variant, AssertVariant::AssertEq(..));

        // We need to `or` the asserted expression with the guards,
        // so extract an appropriate expression.
        let mut expression = match assert.variant {
            // If the assert statement is an `assert`, use the expression as is.
            AssertVariant::Assert(expression) => expression,

            // For `assert_eq` or `assert_neq`, construct a new expression.
            AssertVariant::AssertEq(left, right) | AssertVariant::AssertNeq(left, right) => {
                let binary = BinaryExpression {
                    left,
                    right,
                    op: if is_eq { BinaryOperation::Eq } else { BinaryOperation::Neq },
                    span: Default::default(),
                    id: self.state.node_builder.next_id(),
                };
                self.state.type_table.insert(binary.id, Type::Boolean);
                let (identifier, statement) = self.unique_simple_definition(binary.into());
                statements.push(statement);
                identifier.into()
            }
        };

        // The assertion will be that the original assert statement is true or one of the guards is true
        // (ie, we either didn't follow the condition chain that led to this assert, or else we took an
        // early return).
        for guard in guards.into_iter() {
            let binary = BinaryExpression {
                left: expression,
                right: guard,
                op: BinaryOperation::Or,
                span: Default::default(),
                id: self.state.node_builder.next_id(),
            };
            self.state.type_table.insert(binary.id(), Type::Boolean);
            let (identifier, statement) = self.unique_simple_definition(binary.into());
            statements.push(statement);
            expression = identifier.into();
        }

        let assert_statement = AssertStatement { variant: AssertVariant::Assert(expression), ..input }.into();

        (assert_statement, statements)
    }

    fn reconstruct_assign(&mut self, _assign: AssignStatement) -> (Statement, Self::AdditionalOutput) {
        panic!("`AssignStatement`s should not be in the AST at this phase of compilation");
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

        (Block { span: block.span, statements, id: self.state.node_builder.next_id() }, Default::default())
    }

    /// Flatten a conditional statement into a list of statements.
    fn reconstruct_conditional(&mut self, conditional: ConditionalStatement) -> (Statement, Self::AdditionalOutput) {
        let mut statements = Vec::with_capacity(conditional.then.statements.len());

        // If we are traversing an async function, reconstruct the if and else blocks, but do not flatten them.
        if self.is_async {
            let then_block = self.reconstruct_block(conditional.then).0;
            let otherwise_block = match conditional.otherwise {
                Some(statement) => match *statement {
                    Statement::Block(block) => self.reconstruct_block(block).0,
                    _ => panic!("SSA guarantees that the `otherwise` is always a `Block`"),
                },
                None => {
                    Block { span: Default::default(), statements: Vec::new(), id: self.state.node_builder.next_id() }
                }
            };

            return (
                ConditionalStatement {
                    then: then_block,
                    otherwise: Some(Box::new(otherwise_block.into())),
                    ..conditional
                }
                .into(),
                statements,
            );
        }

        // Assign the condition to a variable, as it may be used multiple times.
        let place = Identifier {
            name: self.state.assigner.unique_symbol("condition", "$"),
            span: Default::default(),
            id: {
                let id = self.state.node_builder.next_id();
                self.state.type_table.insert(id, Type::Boolean);
                id
            },
        };

        statements.push(self.simple_definition(place, conditional.condition.clone()));

        // Add condition to the condition stack.
        self.condition_stack.push(Guard::Unconstructed(place));

        // Reconstruct the then-block and accumulate it constituent statements.
        statements.extend(self.reconstruct_block(conditional.then).0.statements);

        // Remove condition from the condition stack.
        self.condition_stack.pop();

        // Consume the otherwise-block and flatten its constituent statements into the current block.
        if let Some(statement) = conditional.otherwise {
            // Apply Not to the condition, assign it, and put it on the condition stack.
            let not_condition = UnaryExpression {
                op: UnaryOperation::Not,
                receiver: conditional.condition.clone(),
                span: conditional.condition.span(),
                id: conditional.condition.id(),
            }
            .into();
            let not_place = Identifier {
                name: self.state.assigner.unique_symbol("condition", "$"),
                span: Default::default(),
                id: {
                    let id = self.state.node_builder.next_id();
                    self.state.type_table.insert(id, Type::Boolean);
                    id
                },
            };
            statements.push(self.simple_definition(not_place, not_condition));
            self.condition_stack.push(Guard::Unconstructed(not_place));

            // Reconstruct the otherwise-block and accumulate it constituent statements.
            match *statement {
                Statement::Block(block) => statements.extend(self.reconstruct_block(block).0.statements),
                _ => panic!("SSA guarantees that the `otherwise` is always a `Block`"),
            }

            // Remove the negated condition from the condition stack.
            self.condition_stack.pop();
        };

        (Statement::dummy(), statements)
    }

    /// Flattens a definition, if necessary.
    /// Marks variables as structs as necessary.
    /// Note that new statements are only produced if the right hand side is a ternary expression over structs.
    /// Otherwise, the statement is returned as is.
    fn reconstruct_definition(&mut self, definition: DefinitionStatement) -> (Statement, Self::AdditionalOutput) {
        // Flatten the rhs of the assignment.
        let (value, statements) = self.reconstruct_expression(definition.value);
        match (definition.place, &value) {
            (DefinitionPlace::Single(identifier), _) => (self.simple_definition(identifier, value), statements),
            (DefinitionPlace::Multiple(identifiers), expression) => {
                let output_type = match &self.state.type_table.get(&expression.id()) {
                    Some(Type::Tuple(tuple_type)) => tuple_type.clone(),
                    _ => panic!("Type checking guarantees that the output type is a tuple."),
                };

                for (identifier, type_) in identifiers.iter().zip_eq(output_type.elements().iter()) {
                    // Add the type of each identifier to the type table.
                    self.state.type_table.insert(identifier.id, type_.clone());
                }

                (
                    DefinitionStatement {
                        place: DefinitionPlace::Multiple(identifiers),
                        type_: None,
                        value,
                        span: Default::default(),
                        id: self.state.node_builder.next_id(),
                    }
                    .into(),
                    statements,
                )
            }
        }
    }

    fn reconstruct_iteration(&mut self, _input: IterationStatement) -> (Statement, Self::AdditionalOutput) {
        panic!("`IterationStatement`s should not be in the AST at this phase of compilation.");
    }

    /// Transforms a return statement into an empty block statement.
    /// Stores the arguments to the return statement, which are later folded into a single return statement at the end of the function.
    fn reconstruct_return(&mut self, input: ReturnStatement) -> (Statement, Self::AdditionalOutput) {
        use Expression::*;

        // If we are traversing an async function, return as is.
        if self.is_async {
            return (input.into(), Default::default());
        }
        // Construct the associated guard.
        let (guard_identifier, statements) = self.construct_guard().unzip();

        let return_guard = guard_identifier.map_or(ReturnGuard::None, ReturnGuard::Unconstructed);

        let is_tuple_ids = matches!(&input.expression, Tuple(tuple_expr) if tuple_expr .elements.iter() .all(|expr| matches!(expr, Identifier(_))));
        if !matches!(&input.expression, Unit(_) | Identifier(_) | AssociatedConstant(_)) && !is_tuple_ids {
            panic!("SSA guarantees that the expression is always an identifier, unit expression, or tuple literal.")
        }

        self.returns.push((return_guard, input));

        (Statement::dummy(), statements.unwrap_or_default())
    }
}
