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

use crate::{Assigner, SymbolTable, TypeTable};

use leo_ast::{
    AccessExpression,
    ArrayAccess,
    ArrayExpression,
    ArrayType,
    BinaryExpression,
    BinaryOperation,
    Block,
    Expression,
    ExpressionReconstructor,
    Identifier,
    IntegerType,
    Literal,
    Member,
    MemberAccess,
    Node,
    NodeBuilder,
    NonNegativeNumber,
    ReturnStatement,
    Statement,
    Struct,
    StructExpression,
    StructVariableInitializer,
    TernaryExpression,
    TupleAccess,
    TupleExpression,
    TupleType,
    Type,
    UnitExpression,
};

pub struct Flattener<'a> {
    /// The symbol table associated with the program.
    pub(crate) symbol_table: &'a SymbolTable,
    /// A mapping between node IDs and their types.
    pub(crate) type_table: &'a TypeTable,
    /// A counter used to generate unique node IDs.
    pub(crate) node_builder: &'a NodeBuilder,
    /// A struct used to construct (unique) assignment statements.
    pub(crate) assigner: &'a Assigner,
    /// A stack of condition `Expression`s visited up to the current point in the AST.
    pub(crate) condition_stack: Vec<Expression>,
    /// A list containing tuples of guards and expressions associated `ReturnStatement`s.
    /// A guard is an expression that evaluates to true on the execution path of the `ReturnStatement`.
    /// Note that returns are inserted in the order they are encountered during a pre-order traversal of the AST.
    /// Note that type checking guarantees that there is at most one return in a basic block.
    pub(crate) returns: Vec<(Option<Expression>, ReturnStatement)>,
}

impl<'a> Flattener<'a> {
    pub(crate) fn new(
        symbol_table: &'a SymbolTable,
        type_table: &'a TypeTable,
        node_builder: &'a NodeBuilder,
        assigner: &'a Assigner,
    ) -> Self {
        Self { symbol_table, type_table, node_builder, assigner, condition_stack: Vec::new(), returns: Vec::new() }
    }

    /// Clears the state associated with `ReturnStatements`, returning the ones that were previously stored.
    pub(crate) fn clear_early_returns(&mut self) -> Vec<(Option<Expression>, ReturnStatement)> {
        core::mem::take(&mut self.returns)
    }

    /// Constructs a guard from the current state of the condition stack.
    pub(crate) fn construct_guard(&mut self) -> Option<Expression> {
        match self.condition_stack.is_empty() {
            true => None,
            false => {
                let (first, rest) = self.condition_stack.split_first().unwrap();
                Some(rest.iter().cloned().fold(first.clone(), |acc, condition| {
                    // Construct the binary expression.
                    Expression::Binary(BinaryExpression {
                        op: BinaryOperation::And,
                        left: Box::new(acc),
                        right: Box::new(condition),
                        span: Default::default(),
                        id: {
                            // Create a new node ID for the binary expression.
                            let id = self.node_builder.next_id();
                            // Set the type of the node ID.
                            self.type_table.insert(id, Type::Boolean);
                            id
                        },
                    })
                }))
            }
        }
    }

    /// Fold guards and expressions into a single expression.
    /// Note that this function assumes that at least one guard is present.
    pub(crate) fn fold_guards(
        &mut self,
        prefix: &str,
        mut guards: Vec<(Option<Expression>, Expression)>,
    ) -> (Expression, Vec<Statement>) {
        // Type checking guarantees that there exists at least one return statement in the function body.
        let (_, last_expression) = guards.pop().unwrap();

        match last_expression {
            // If the expression is a unit expression, then return it directly.
            Expression::Unit(_) => (last_expression, Vec::new()),
            // Otherwise, fold the guards and expressions into a single expression.
            _ => {
                // Produce a chain of ternary expressions and assignments for the guards.
                let mut statements = Vec::with_capacity(guards.len());

                // Helper to construct and store ternary assignments. e.g `$ret$0 = $var$0 ? $var$1 : $var$2`
                let mut construct_ternary_assignment =
                    |guard: Expression, if_true: Expression, if_false: Expression| {
                        let place = Identifier {
                            name: self.assigner.unique_symbol(prefix, "$"),
                            span: Default::default(),
                            id: self.node_builder.next_id(),
                        };
                        let (value, stmts) = self.reconstruct_ternary(TernaryExpression {
                            id: {
                                // Create a new node ID for the ternary expression.
                                let id = self.node_builder.next_id();
                                // Get the type of the node ID.
                                let type_ = match self.type_table.get(&if_true.id()) {
                                    Some(type_) => type_,
                                    None => unreachable!("Type checking guarantees that all expressions have a type."),
                                };
                                // Set the type of the node ID.
                                self.type_table.insert(id, type_);
                                id
                            },
                            condition: Box::new(guard),
                            if_true: Box::new(if_true),
                            if_false: Box::new(if_false),
                            span: Default::default(),
                        });
                        statements.extend(stmts);

                        match &value {
                            // If the expression is a tuple, then use it directly.
                            // This must be done to ensure that intermediate tuple assignments are not created.
                            Expression::Tuple(_) => value,
                            // Otherwise, assign the expression to a variable and return the variable.
                            _ => {
                                statements.push(self.simple_assign_statement(place, value));
                                Expression::Identifier(place)
                            }
                        }
                    };

                let expression = guards.into_iter().rev().fold(last_expression, |acc, (guard, expr)| match guard {
                    None => unreachable!("All expressions except for the last one must have a guard."),
                    // Note that type checking guarantees that all expressions have the same type.
                    Some(guard) => construct_ternary_assignment(guard, expr, acc),
                });

                (expression, statements)
            }
        }
    }

    /// A wrapper around `assigner.unique_simple_assign_statement` that updates `self.structs`.
    pub(crate) fn unique_simple_assign_statement(&mut self, expr: Expression) -> (Identifier, Statement) {
        // Create a new variable for the expression.
        let name = self.assigner.unique_symbol("$var", "$");
        // Construct the lhs of the assignment.
        let place = Identifier { name, span: Default::default(), id: self.node_builder.next_id() };
        // Construct the assignment statement.
        let statement = self.simple_assign_statement(place, expr);

        (place, statement)
    }

    /// A wrapper around `assigner.simple_assign_statement` that tracks the type of the lhs.
    pub(crate) fn simple_assign_statement(&mut self, lhs: Identifier, rhs: Expression) -> Statement {
        // Update the type table.
        let type_ = match self.type_table.get(&rhs.id()) {
            Some(type_) => type_,
            None => unreachable!("Type checking guarantees that all expressions have a type."),
        };
        self.type_table.insert(lhs.id(), type_);
        // Construct the statement.
        self.assigner.simple_assign_statement(lhs, rhs, self.node_builder.next_id())
    }

    /// Folds a list of return statements into a single return statement and adds the produced statements to the block.
    pub(crate) fn fold_returns(&mut self, block: &mut Block, returns: Vec<(Option<Expression>, ReturnStatement)>) {
        // If the list of returns is not empty, then fold them into a single return statement.
        if !returns.is_empty() {
            let mut return_expressions = Vec::with_capacity(returns.len());

            // Construct a vector for each argument position.
            // Note that the indexing is safe since we check that `returns` is not empty.
            let (has_finalize, number_of_finalize_arguments) = match &returns[0].1.finalize_arguments {
                None => (false, 0),
                Some(args) => (true, args.len()),
            };
            let mut finalize_arguments: Vec<Vec<(Option<Expression>, Expression)>> =
                vec![Vec::with_capacity(returns.len()); number_of_finalize_arguments];

            // Aggregate the return expressions and finalize arguments and their respective guards.
            for (guard, return_statement) in returns {
                return_expressions.push((guard.clone(), return_statement.expression));
                if let Some(arguments) = return_statement.finalize_arguments {
                    for (i, argument) in arguments.into_iter().enumerate() {
                        // Note that the indexing is safe since we initialize `finalize_arguments` with the correct length.
                        finalize_arguments[i].push((guard.clone(), argument));
                    }
                }
            }

            // Fold the return expressions into a single expression.
            let (expression, stmts) = self.fold_guards("$ret", return_expressions);

            // Add all of the accumulated statements to the end of the block.
            block.statements.extend(stmts);

            // For each position in the finalize call, fold the corresponding arguments into a single expression.
            let finalize_arguments = match has_finalize {
                false => None,
                true => Some(
                    finalize_arguments
                        .into_iter()
                        .enumerate()
                        .map(|(i, arguments)| {
                            let (expression, stmts) = self.fold_guards(&format!("finalize${i}$"), arguments);
                            block.statements.extend(stmts);
                            expression
                        })
                        .collect(),
                ),
            };

            // Add the `ReturnStatement` to the end of the block.
            block.statements.push(Statement::Return(ReturnStatement {
                expression,
                finalize_arguments,
                span: Default::default(),
                id: self.node_builder.next_id(),
            }));
        }
        // Otherwise, push a dummy return statement to the end of the block.
        else {
            block.statements.push(Statement::Return(ReturnStatement {
                expression: {
                    let id = self.node_builder.next_id();
                    Expression::Unit(UnitExpression { span: Default::default(), id })
                },
                finalize_arguments: None,
                span: Default::default(),
                id: self.node_builder.next_id(),
            }));
        }
    }

    pub(crate) fn ternary_array(
        &mut self,
        array: &ArrayType,
        condition: &Expression,
        first: &Identifier,
        second: &Identifier,
    ) -> (Expression, Vec<Statement>) {
        // Initialize a vector to accumulate any statements generated.
        let mut statements = Vec::new();
        // For each array element, construct a new ternary expression.
        let elements = (0..array.length())
            .map(|i| {
                // Create an assignment statement for the first access expression.
                let (first, stmt) =
                    self.unique_simple_assign_statement(Expression::Access(AccessExpression::Array(ArrayAccess {
                        array: Box::new(Expression::Identifier(*first)),
                        index: Box::new(Expression::Literal(Literal::Integer(
                            IntegerType::U32,
                            i.to_string(),
                            Default::default(),
                            {
                                // Create a new node ID for the literal.
                                let id = self.node_builder.next_id();
                                // Set the type of the node ID.
                                self.type_table.insert(id, Type::Integer(IntegerType::U32));
                                id
                            },
                        ))),
                        span: Default::default(),
                        id: {
                            // Create a new node ID for the access expression.
                            let id = self.node_builder.next_id();
                            // Set the type of the node ID.
                            self.type_table.insert(id, array.element_type().clone());
                            id
                        },
                    })));
                statements.push(stmt);
                // Create an assignment statement for the second access expression.
                let (second, stmt) =
                    self.unique_simple_assign_statement(Expression::Access(AccessExpression::Array(ArrayAccess {
                        array: Box::new(Expression::Identifier(*second)),
                        index: Box::new(Expression::Literal(Literal::Integer(
                            IntegerType::U32,
                            i.to_string(),
                            Default::default(),
                            {
                                // Create a new node ID for the literal.
                                let id = self.node_builder.next_id();
                                // Set the type of the node ID.
                                self.type_table.insert(id, Type::Integer(IntegerType::U32));
                                id
                            },
                        ))),
                        span: Default::default(),
                        id: {
                            // Create a new node ID for the access expression.
                            let id = self.node_builder.next_id();
                            // Set the type of the node ID.
                            self.type_table.insert(id, array.element_type().clone());
                            id
                        },
                    })));
                statements.push(stmt);

                // Recursively reconstruct the ternary expression.
                let (expression, stmts) = self.reconstruct_ternary(TernaryExpression {
                    condition: Box::new(condition.clone()),
                    // Access the member of the first expression.
                    if_true: Box::new(Expression::Identifier(first)),
                    // Access the member of the second expression.
                    if_false: Box::new(Expression::Identifier(second)),
                    span: Default::default(),
                    id: {
                        // Create a new node ID for the ternary expression.
                        let id = self.node_builder.next_id();
                        // Set the type of the node ID.
                        self.type_table.insert(id, array.element_type().clone());
                        id
                    },
                });

                // Accumulate any statements generated.
                statements.extend(stmts);

                expression
            })
            .collect();

        // Construct the array expression.
        let (expr, stmts) = self.reconstruct_array(ArrayExpression {
            elements,
            span: Default::default(),
            id: {
                // Create a node ID for the array expression.
                let id = self.node_builder.next_id();
                // Set the type of the node ID.
                self.type_table.insert(id, Type::Array(array.clone()));
                id
            },
        });

        // Accumulate any statements generated.
        statements.extend(stmts);

        // Create a new assignment statement for the array expression.
        let (identifier, statement) = self.unique_simple_assign_statement(expr);

        statements.push(statement);

        (Expression::Identifier(identifier), statements)
    }

    pub(crate) fn ternary_struct(
        &mut self,
        struct_: &Struct,
        condition: &Expression,
        first: &Identifier,
        second: &Identifier,
    ) -> (Expression, Vec<Statement>) {
        // Initialize a vector to accumulate any statements generated.
        let mut statements = Vec::new();
        // For each struct member, construct a new ternary expression.
        let members = struct_
            .members
            .iter()
            .map(|Member { identifier, type_, .. }| {
                // Create an assignment statement for the first access expression.
                let (first, stmt) =
                    self.unique_simple_assign_statement(Expression::Access(AccessExpression::Member(MemberAccess {
                        inner: Box::new(Expression::Identifier(*first)),
                        name: *identifier,
                        span: Default::default(),
                        id: {
                            // Create a new node ID for the access expression.
                            let id = self.node_builder.next_id();
                            // Set the type of the node ID.
                            self.type_table.insert(id, type_.clone());
                            id
                        },
                    })));
                statements.push(stmt);
                // Create an assignment statement for the second access expression.
                let (second, stmt) =
                    self.unique_simple_assign_statement(Expression::Access(AccessExpression::Member(MemberAccess {
                        inner: Box::new(Expression::Identifier(*second)),
                        name: *identifier,
                        span: Default::default(),
                        id: {
                            // Create a new node ID for the access expression.
                            let id = self.node_builder.next_id();
                            // Set the type of the node ID.
                            self.type_table.insert(id, type_.clone());
                            id
                        },
                    })));
                statements.push(stmt);
                // Recursively reconstruct the ternary expression.
                let (expression, stmts) = self.reconstruct_ternary(TernaryExpression {
                    condition: Box::new(condition.clone()),
                    // Access the member of the first expression.
                    if_true: Box::new(Expression::Identifier(first)),
                    // Access the member of the second expression.
                    if_false: Box::new(Expression::Identifier(second)),
                    span: Default::default(),
                    id: {
                        // Create a new node ID for the ternary expression.
                        let id = self.node_builder.next_id();
                        // Set the type of the node ID.
                        self.type_table.insert(id, type_.clone());
                        id
                    },
                });

                // Accumulate any statements generated.
                statements.extend(stmts);

                StructVariableInitializer {
                    identifier: *identifier,
                    expression: Some(expression),
                    span: Default::default(),
                    id: self.node_builder.next_id(),
                }
            })
            .collect();

        let (expr, stmts) = self.reconstruct_struct_init(StructExpression {
            name: struct_.identifier,
            members,
            span: Default::default(),
            id: {
                // Create a new node ID for the struct expression.
                let id = self.node_builder.next_id();
                // Set the type of the node ID.
                self.type_table.insert(id, Type::Identifier(struct_.identifier));
                id
            },
        });

        // Accumulate any statements generated.
        statements.extend(stmts);

        // Create a new assignment statement for the struct expression.
        let (identifier, statement) = self.unique_simple_assign_statement(expr);

        statements.push(statement);

        (Expression::Identifier(identifier), statements)
    }

    pub(crate) fn ternary_tuple(
        &mut self,
        tuple_type: &TupleType,
        condition: &Expression,
        first: &Identifier,
        second: &Identifier,
    ) -> (Expression, Vec<Statement>) {
        // Initialize a vector to accumulate any statements generated.
        let mut statements = Vec::new();
        // For each tuple element, construct a new ternary expression.
        let elements = tuple_type
            .elements()
            .iter()
            .enumerate()
            .map(|(i, type_)| {
                // Create an assignment statement for the first access expression.
                let (first, stmt) =
                    self.unique_simple_assign_statement(Expression::Access(AccessExpression::Tuple(TupleAccess {
                        tuple: Box::new(Expression::Identifier(*first)),
                        index: NonNegativeNumber::from(i),
                        span: Default::default(),
                        id: {
                            // Create a new node ID for the access expression.
                            let id = self.node_builder.next_id();
                            // Set the type of the node ID.
                            self.type_table.insert(id, type_.clone());
                            id
                        },
                    })));
                statements.push(stmt);
                // Create an assignment statement for the second access expression.
                let (second, stmt) =
                    self.unique_simple_assign_statement(Expression::Access(AccessExpression::Tuple(TupleAccess {
                        tuple: Box::new(Expression::Identifier(*second)),
                        index: NonNegativeNumber::from(i),
                        span: Default::default(),
                        id: {
                            // Create a new node ID for the access expression.
                            let id = self.node_builder.next_id();
                            // Set the type of the node ID.
                            self.type_table.insert(id, type_.clone());
                            id
                        },
                    })));
                statements.push(stmt);

                // Recursively reconstruct the ternary expression.
                let (expression, stmts) = self.reconstruct_ternary(TernaryExpression {
                    condition: Box::new(condition.clone()),
                    // Access the member of the first expression.
                    if_true: Box::new(Expression::Identifier(first)),
                    // Access the member of the second expression.
                    if_false: Box::new(Expression::Identifier(second)),
                    span: Default::default(),
                    id: {
                        // Create a new node ID for the ternary expression.
                        let id = self.node_builder.next_id();
                        // Set the type of the node ID.
                        self.type_table.insert(id, type_.clone());
                        id
                    },
                });

                // Accumulate any statements generated.
                statements.extend(stmts);

                expression
            })
            .collect();

        // Construct the tuple expression.
        let tuple = TupleExpression {
            elements,
            span: Default::default(),
            id: {
                // Create a new node ID for the tuple expression.
                let id = self.node_builder.next_id();
                // Set the type of the node ID.
                self.type_table.insert(id, Type::Tuple(tuple_type.clone()));
                id
            },
        };
        let (expr, stmts) = self.reconstruct_tuple(tuple.clone());

        // Accumulate any statements generated.
        statements.extend(stmts);

        // Create a new assignment statement for the tuple expression.
        let (identifier, statement) = self.unique_simple_assign_statement(expr);

        statements.push(statement);

        (Expression::Identifier(identifier), statements)
    }
}
