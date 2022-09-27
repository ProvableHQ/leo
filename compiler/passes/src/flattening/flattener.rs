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

use crate::{Assigner, SymbolTable};

use leo_ast::{
    AccessExpression, CircuitMember, Expression, ExpressionReconstructor, Identifier, Statement, TernaryExpression,
    Type,
};
use leo_span::Symbol;

use indexmap::IndexMap;

pub struct Flattener<'a> {
    /// The symbol table associated with the program.
    /// This table is used to lookup circuit definitions, when they are folded.
    pub(crate) symbol_table: &'a SymbolTable,
    /// An struct used to construct (unique) assignment statements.
    pub(crate) assigner: Assigner,
    /// The set of variables that are circuits.
    pub(crate) circuits: IndexMap<Symbol, Symbol>,
    /// A stack of condition `Expression`s visited up to the current point in the AST.
    pub(crate) condition_stack: Vec<Expression>,
    /// A list containing tuples of guards and expressions associated `ReturnStatement`s.
    /// A guard is an expression that evaluates to true on the execution path of the `ReturnStatement`.
    /// Note that returns are inserted in the order they are encountered during a pre-order traversal of the AST.
    /// Note that type checking guarantees that there is at most one return in a basic block.
    pub(crate) returns: Vec<(Option<Expression>, Expression)>,
    /// A list containing tuples of guards and expressions associated with `FinalizeStatement`s.
    /// A guard is an expression that evaluates to true on the execution path of the `FinalizeStatement`.
    /// Note that finalizes are inserted in the order they are encountered during a pre-order traversal of the AST.
    /// Note that type checking guarantees that there is at most one finalize in a basic block.
    pub(crate) finalizes: Vec<Vec<(Option<Expression>, Expression)>>,
}

impl<'a> Flattener<'a> {
    pub(crate) fn new(symbol_table: &'a SymbolTable, assigner: Assigner) -> Self {
        Self {
            symbol_table,
            assigner,
            circuits: IndexMap::new(),
            condition_stack: Vec::new(),
            returns: Vec::new(),
            finalizes: Vec::new(),
        }
    }

    /// Clears the state associated with `ReturnStatements`, returning the ones that were previously stored.
    pub(crate) fn clear_early_returns(&mut self) -> Vec<(Option<Expression>, Expression)> {
        core::mem::take(&mut self.returns)
    }

    /// Clears the state associated with `FinalizeStatements`, returning the ones that were previously stored.
    pub(crate) fn clear_early_finalizes(&mut self) -> Vec<Vec<(Option<Expression>, Expression)>> {
        core::mem::take(&mut self.finalizes)
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

        // Produce a chain of ternary expressions and assignments for the guards.
        let mut statements = Vec::with_capacity(guards.len());

        // Helper to construct and store ternary assignments. e.g `$ret$0 = $var$0 ? $var$1 : $var$2`
        let mut construct_ternary_assignment = |guard: Expression, if_true: Expression, if_false: Expression| {
            let place = Identifier {
                name: self.assigner.unique_symbol(prefix),
                span: Default::default(),
            };
            let (value, stmts) = self.reconstruct_ternary(TernaryExpression {
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

        let expression = guards
            .into_iter()
            .rev()
            .fold(last_expression, |acc, (guard, expr)| match guard {
                None => unreachable!("All expressions except for the last one must have a guard."),
                // Note that type checking guarantees that all expressions have the same type.
                Some(guard) => construct_ternary_assignment(guard, expr, acc),
            });

        (expression, statements)
    }

    /// Looks up the name of the circuit associated with an identifier or access expression, if it exists.
    pub(crate) fn lookup_circuit_symbol(&self, expression: &Expression) -> Option<Symbol> {
        match expression {
            Expression::Identifier(identifier) => self.circuits.get(&identifier.name).copied(),
            Expression::Access(AccessExpression::Member(access)) => {
                // The inner expression of an access expression is either an identifier or another access expression.
                let name = self.lookup_circuit_symbol(&access.inner).unwrap();
                let circuit = self.symbol_table.lookup_circuit(name).unwrap();
                let CircuitMember::CircuitVariable(_, member_type) = circuit
                    .members
                    .iter()
                    .find(|member| member.name() == access.name.name)
                    .unwrap();
                match member_type {
                    Type::Identifier(identifier) => Some(identifier.name),
                    _ => None,
                }
            }
            _ => None,
        }
    }

    /// Updates `self.circuits` for new assignment statements.
    /// Expects the left hand side of the assignment to be an identifier.
    pub(crate) fn update_circuits(&mut self, lhs: &Identifier, rhs: &Expression) {
        match rhs {
            Expression::Circuit(rhs) => {
                self.circuits.insert(lhs.name, rhs.name.name);
            }
            // If the rhs of the assignment is an identifier that is a circuit, add it to `self.circuits`.
            Expression::Identifier(rhs) if self.circuits.contains_key(&rhs.name) => {
                // Note that this unwrap is safe because we just checked that the key exists.
                self.circuits.insert(lhs.name, *self.circuits.get(&rhs.name).unwrap());
            }
            // Otherwise, do nothing.
            _ => (),
        }
    }

    /// A wrapper around `assigner.unique_simple_assign_statement` that updates `self.circuits`.
    pub(crate) fn unique_simple_assign_statement(&mut self, expr: Expression) -> (Identifier, Statement) {
        let (place, statement) = self.assigner.unique_simple_assign_statement(expr);
        match &statement {
            Statement::Assign(assign) => {
                self.update_circuits(&place, &assign.value);
            }
            _ => unreachable!("`assigner.unique_simple_assign_statement` always returns an assignment statement."),
        }
        (place, statement)
    }

    /// A wrapper around `assigner.simple_assign_statement` that updates `self.circuits`.
    pub(crate) fn simple_assign_statement(&mut self, lhs: Identifier, rhs: Expression) -> Statement {
        self.update_circuits(&lhs, &rhs);
        self.assigner.simple_assign_statement(lhs, rhs)
    }
}
