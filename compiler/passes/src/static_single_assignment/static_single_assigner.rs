// Copyright (C) 2019-2022 Aleo Systems Inc.
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

use crate::{RenameTable, SymbolTable};

use leo_ast::{AssignStatement, Circuit, Expression, ExpressionConsumer, Identifier, Statement, TernaryExpression};
use leo_errors::emitter::Handler;
use leo_span::Symbol;

use indexmap::{IndexMap, IndexSet};
use std::fmt::Display;

pub struct StaticSingleAssigner<'a> {
    /// The symbol table associated with the program.
    pub(crate) symbol_table: &'a SymbolTable,
    /// The `RenameTable` for the current basic block in the AST
    pub(crate) rename_table: RenameTable,
    /// An error handler used for any errors found during unrolling.
    pub(crate) _handler: &'a Handler,
    /// A strictly increasing counter, used to ensure that new variable names are unique.
    pub(crate) counter: usize,
    /// A flag to determine whether or not the traversal is on the left-hand side of a definition or an assignment.
    pub(crate) is_lhs: bool,
    /// The set of variables that are circuits.
    pub(crate) circuits: IndexMap<Symbol, Symbol>,
    /// A stack of condition `Expression`s visited up to the current point in the AST.
    pub(crate) condition_stack: Vec<Expression>,
    /// A list containing tuples of guards and expressions associated with early `ReturnStatement`s.
    /// Note that early returns are inserted in the order they are encountered during a pre-order traversal of the AST.
    pub(crate) early_returns: Vec<(Option<Expression>, Expression)>,
    /// A list containing tuples of guards and expressions associated with early `FinalizeStatement`s.
    /// Note that early finalizes are inserted in the order they are encountered during a pre-order traversal of the AST.
    pub(crate) early_finalizes: Vec<(Option<Expression>, Expression)>,
}

impl<'a> StaticSingleAssigner<'a> {
    pub(crate) fn new(handler: &'a Handler, symbol_table: &'a SymbolTable) -> Self {
        Self {
            symbol_table,
            rename_table: RenameTable::new(None),
            _handler: handler,
            counter: 0,
            is_lhs: false,
            circuits: IndexMap::new(),
            condition_stack: Vec::new(),
            early_returns: Vec::new(),
            early_finalizes: Vec::new(),
        }
    }

    /// Return a new unique `Symbol` from a `&str`.
    pub(crate) fn unique_symbol(&mut self, arg: impl Display) -> Symbol {
        self.counter += 1;
        Symbol::intern(&format!("{}${}", arg, self.counter - 1))
    }

    /// Constructs the assignment statement `place = expr;`.
    /// This function should be the only place where `AssignStatement`s are constructed.
    pub(crate) fn simple_assign_statement(&mut self, identifier: Identifier, value: Expression) -> Statement {
        if let Expression::Circuit(expr) = &value {
            self.circuits.insert(identifier.name, expr.name.name);
        }

        // Update the rename table.
        self.rename_table.update(identifier.name, identifier.name);

        Statement::Assign(Box::new(AssignStatement {
            place: Expression::Identifier(identifier),
            value,
            span: Default::default(),
        }))
    }

    /// Constructs a simple assign statement for `expr` with a unique name.
    /// For example, `expr` is transformed into `$var$0 = expr;`.
    pub(crate) fn unique_simple_assign_statement(&mut self, expr: Expression) -> (Expression, Statement) {
        // Create a new variable for the expression.
        let name = self.unique_symbol("$var");

        let place = Identifier {
            name,
            span: Default::default(),
        };

        (Expression::Identifier(place), self.simple_assign_statement(place, expr))
    }

    /// Clears the state associated with `ReturnStatements`, returning the ones that were previously produced.
    pub(crate) fn clear_early_returns(&mut self) -> Vec<(Option<Expression>, Expression)> {
        core::mem::take(&mut self.early_returns)
    }

    // Clears the state associated with `FinalizeStatements`, returning the ones that were previously produced.
    pub(crate) fn clear_early_finalizes(&mut self) -> Vec<(Option<Expression>, Expression)> {
        core::mem::take(&mut self.early_finalizes)
    }

    /// Pushes a new scope, setting the current scope as the new scope's parent.
    pub(crate) fn push(&mut self) {
        let parent_table = core::mem::take(&mut self.rename_table);
        self.rename_table = RenameTable::new(Some(Box::from(parent_table)));
    }

    /// If the RenameTable has a parent, then `self.rename_table` is set to the parent, otherwise it is set to a default `RenameTable`.
    pub(crate) fn pop(&mut self) -> RenameTable {
        let parent = self.rename_table.parent.clone().unwrap_or_default();
        core::mem::replace(&mut self.rename_table, *parent)
    }

    /// Fold guards and expressions into a single expression.
    /// Note that this function assumes that at least one guard is present.
    pub(crate) fn fold_guards(
        &mut self,
        prefix: &str,
        mut guards: Vec<(Option<Expression>, Expression)>,
    ) -> (Vec<Statement>, Expression) {
        // Type checking guarantees that there exists at least one return statement in the function body.
        let (_, last_expression) = guards.pop().unwrap();

        // Produce a chain of ternary expressions and assignments for the guards.
        let mut statements = Vec::with_capacity(guards.len());

        // Helper to construct and store ternary assignments. e.g `$ret$0 = $var$0 ? $var$1 : $var$2`
        let mut construct_ternary_assignment = |guard: Expression, if_true: Expression, if_false: Expression| {
            let place = Identifier {
                name: self.unique_symbol(prefix),
                span: Default::default(),
            };
            let (value, stmts) = self.consume_ternary(TernaryExpression {
                condition: Box::new(guard),
                if_true: Box::new(if_true),
                if_false: Box::new(if_false),
                span: Default::default(),
            });
            statements.extend(stmts);

            statements.push(self.simple_assign_statement(place, value));
            Expression::Identifier(place)
        };

        let expression = guards
            .into_iter()
            .rev()
            .fold(last_expression, |acc, (guard, expr)| match guard {
                None => unreachable!("All expression except for the last one must have a guard."),
                // Note that type checking guarantees that all expressions have the same type.
                Some(guard) => construct_ternary_assignment(guard, expr, acc),
            });

        (statements, expression)
    }
}
