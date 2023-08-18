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

use crate::{Assigner, SymbolTable};

use leo_ast::{
    AccessExpression,
    BinaryExpression,
    BinaryOperation,
    Block,
    Expression,
    ExpressionReconstructor,
    Identifier,
    Member,
    NodeBuilder,
    ReturnStatement,
    Statement,
    TernaryExpression,
    TupleExpression,
    Type,
};
use leo_span::Symbol;

use indexmap::IndexMap;

pub struct Flattener<'a> {
    /// The symbol table associated with the program.
    pub(crate) symbol_table: &'a SymbolTable,
    /// A counter used to generate unique node IDs.
    pub(crate) node_builder: &'a NodeBuilder,
    /// A struct used to construct (unique) assignment statements.
    pub(crate) assigner: &'a Assigner,
    /// The set of variables that are structs.
    pub(crate) structs: IndexMap<Symbol, Symbol>,
    /// A stack of condition `Expression`s visited up to the current point in the AST.
    pub(crate) condition_stack: Vec<Expression>,
    /// A list containing tuples of guards and expressions associated `ReturnStatement`s.
    /// A guard is an expression that evaluates to true on the execution path of the `ReturnStatement`.
    /// Note that returns are inserted in the order they are encountered during a pre-order traversal of the AST.
    /// Note that type checking guarantees that there is at most one return in a basic block.
    pub(crate) returns: Vec<(Option<Expression>, ReturnStatement)>,
    /// A mapping between variables and flattened tuple expressions.
    pub(crate) tuples: IndexMap<Symbol, TupleExpression>,
}

impl<'a> Flattener<'a> {
    pub(crate) fn new(symbol_table: &'a SymbolTable, node_builder: &'a NodeBuilder, assigner: &'a Assigner) -> Self {
        Self {
            symbol_table,
            node_builder,
            assigner,
            structs: IndexMap::new(),
            condition_stack: Vec::new(),
            returns: Vec::new(),
            tuples: IndexMap::new(),
        }
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
                    Expression::Binary(BinaryExpression {
                        op: BinaryOperation::And,
                        left: Box::new(acc),
                        right: Box::new(condition),
                        span: Default::default(),
                        id: self.node_builder.next_id(),
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
                            condition: Box::new(guard),
                            if_true: Box::new(if_true),
                            if_false: Box::new(if_false),
                            span: Default::default(),
                            id: self.node_builder.next_id(),
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

    /// Looks up the name of the struct associated with an identifier or access expression, if it exists.
    pub(crate) fn lookup_struct_symbol(&self, expression: &Expression) -> Option<Symbol> {
        match expression {
            Expression::Identifier(identifier) => self.structs.get(&identifier.name).copied(),
            Expression::Access(AccessExpression::Member(access)) => {
                // The inner expression of an access expression is either an identifier or another access expression.
                let name = self.lookup_struct_symbol(&access.inner).unwrap();
                let struct_ = self.symbol_table.lookup_struct(name).unwrap();
                let Member { type_, .. } =
                    struct_.members.iter().find(|member| member.name() == access.name.name).unwrap();
                match type_ {
                    Type::Identifier(identifier) => Some(identifier.name),
                    _ => None,
                }
            }
            _ => None,
        }
    }

    /// Updates `self.structs` for new assignment statements.
    /// Expects the left hand side of the assignment to be an identifier.
    pub(crate) fn update_structs(&mut self, lhs: &Identifier, rhs: &Expression) {
        match rhs {
            Expression::Struct(rhs) => {
                self.structs.insert(lhs.name, rhs.name.name);
            }
            // If the rhs of the assignment is an identifier that is a struct, add it to `self.structs`.
            Expression::Identifier(rhs) if self.structs.contains_key(&rhs.name) => {
                // Note that this unwrap is safe because we just checked that the key exists.
                self.structs.insert(lhs.name, *self.structs.get(&rhs.name).unwrap());
            }
            // Otherwise, do nothing.
            _ => (),
        }
    }

    /// A wrapper around `assigner.unique_simple_assign_statement` that updates `self.structs`.
    pub(crate) fn unique_simple_assign_statement(&mut self, expr: Expression) -> (Identifier, Statement) {
        // Create a new variable for the expression.
        let name = self.assigner.unique_symbol("$var", "$");
        // Construct the lhs of the assignment.
        let place = Identifier { name, span: Default::default(), id: self.node_builder.next_id() };
        // Construct the assignment statement.
        let statement = self.assigner.simple_assign_statement(place, expr, self.node_builder.next_id());

        match &statement {
            Statement::Assign(assign) => {
                self.update_structs(&place, &assign.value);
            }
            _ => unreachable!("`assigner.unique_simple_assign_statement` always returns an assignment statement."),
        }
        (place, statement)
    }

    /// A wrapper around `assigner.simple_assign_statement` that updates `self.structs`.
    pub(crate) fn simple_assign_statement(&mut self, lhs: Identifier, rhs: Expression) -> Statement {
        self.update_structs(&lhs, &rhs);
        self.assigner.simple_assign_statement(lhs, rhs, self.node_builder.next_id())
    }

    /// Folds a list of return statements into a single return statement and adds the produced statements to the block.
    pub(crate) fn fold_returns(&mut self, block: &mut Block, returns: Vec<(Option<Expression>, ReturnStatement)>) {
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
    }
}
