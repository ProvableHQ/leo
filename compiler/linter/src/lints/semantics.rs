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

use indexmap::{IndexMap, IndexSet};
use leo_ast::{AstVisitor, Block, Constructor, DefinitionStatement, Expression, Function, Identifier, Statement};
use leo_errors::Lint;
use leo_span::{Span, Symbol};

use crate::{context::LateContext, passes::LateLintPass};

/// A linter to check for various coding habits
/// that come under the `semantics` category.
pub(super) struct SemanticLinter<'ctx> {
    _context: LateContext<'ctx>,
    unused_variables: UnusedVariables<'ctx>,
    unused_assignments: UnusedAssignments<'ctx>,
}

impl<'ctx> LateLintPass<'ctx> for SemanticLinter<'ctx> {
    fn new(_context: LateContext<'ctx>) -> Box<dyn LateLintPass<'ctx> + 'ctx> {
        Box::new(Self {
            _context,
            unused_variables: UnusedVariables { context: _context, block_stack: vec![] },
            unused_assignments: UnusedAssignments {
                context: _context,
                alive: Default::default(),
                dead: Default::default(),
            },
        })
    }

    fn get_name(&self) -> &str {
        "semantics"
    }

    fn check_expression(&mut self, expr: &Expression) {
        self.unused_variables.check_expression(expr);
    }

    fn check_statement(&mut self, statement: &Statement) {
        self.unused_variables.check_statement(statement);
    }

    fn check_block(&mut self, block: &Block) {
        self.unused_variables.check_block(block);
    }

    fn check_block_post(&mut self, block: &Block) {
        self.unused_variables.check_block_post(block);
    }

    fn check_constructor(&mut self, constructor: &Constructor) {
        self.unused_assignments.check_constructor(constructor);
    }

    fn check_function(&mut self, function: &Function) {
        self.unused_assignments.check_function(function);
    }
}

/// A lint to check for unused variables in the leo programs.
struct UnusedVariables<'ctx> {
    context: LateContext<'ctx>,
    block_stack: Vec<IndexMap<Symbol, Span>>,
}

impl<'ctx> LateLintPass<'ctx> for UnusedVariables<'ctx> {
    fn new(context: LateContext<'ctx>) -> Box<dyn LateLintPass<'ctx> + 'ctx> {
        Box::new(Self { context, block_stack: vec![] })
    }

    fn get_name(&self) -> &str {
        "unused variables"
    }

    fn check_block(&mut self, _block: &Block) {
        self.block_stack.push(Default::default());
    }

    fn check_block_post(&mut self, _block: &Block) {
        for unused_var in self.block_stack.pop().unwrap() {
            self.context.emit_lint(Lint::unused_variable(unused_var.0, unused_var.1));
        }
    }

    fn check_expression(&mut self, expr: &Expression) {
        if let Expression::Path(path) = expr {
            self.block_stack.last_mut().unwrap().swap_remove(&path.identifier().name);
        }
    }

    fn check_statement(&mut self, statement: &Statement) {
        if let Statement::Definition(def) = statement {
            match &def.place {
                leo_ast::DefinitionPlace::Single(identifier) => {
                    self.block_stack.last_mut().unwrap().insert(identifier.name, identifier.span);
                }
                leo_ast::DefinitionPlace::Multiple(identifiers) => {
                    identifiers.iter().for_each(|id| _ = self.block_stack.last_mut().unwrap().insert(id.name, id.span));
                }
            }
        }
    }
}

/// A lint to check for the unused assignments or dead variables in the leo programs.
struct UnusedAssignments<'ctx> {
    context: LateContext<'ctx>,
    alive: IndexSet<Symbol>,
    dead: IndexSet<(Symbol, Span)>,
}

impl<'ctx> LateLintPass<'ctx> for UnusedAssignments<'ctx> {
    fn new(context: LateContext<'ctx>) -> Box<dyn LateLintPass<'ctx> + 'ctx> {
        Box::new(Self { context, alive: Default::default(), dead: Default::default() })
    }

    fn get_name(&self) -> &str {
        "unused assignments"
    }

    fn check_function(&mut self, function: &leo_ast::Function) {
        self.alive.clear();
        self.dead.clear();
        self.visit_block(&function.block);
        for dead in self.dead.as_slice() {
            self.context.emit_lint(Lint::unused_assignments(dead.0, dead.1));
        }
        self.alive.clear();
        self.dead.clear();
    }

    fn check_constructor(&mut self, constructor: &leo_ast::Constructor) {
        self.alive.clear();
        self.dead.clear();
        self.visit_block(&constructor.block);
        for dead in self.dead.as_slice() {
            self.context.emit_lint(Lint::unused_assignments(dead.0, dead.1));
        }
        self.alive.clear();
        self.dead.clear();
    }
}

impl AstVisitor for UnusedAssignments<'_> {
    type AdditionalInput = bool;
    type Output = ();

    fn visit_block(&mut self, input: &Block) {
        input.statements.iter().rev().for_each(|stmt| self.visit_statement(stmt));
    }

    fn visit_statement(&mut self, input: &Statement) {
        match input {
            Statement::Assert(assert_statement) => self.visit_assert(assert_statement),
            Statement::Assign(assign_statement) => self.visit_assign(assign_statement),
            Statement::Block(block) => self.visit_block(block),
            Statement::Conditional(conditional_statement) => {
                let alive = self.alive.clone();
                let dead = self.dead.clone();
                self.visit_block(&conditional_statement.then);
                let then_alive = std::mem::replace(&mut self.alive, alive);
                let mut then_dead = std::mem::replace(&mut self.dead, dead);
                if let Some(otherwise) = &conditional_statement.otherwise {
                    self.visit_statement(otherwise);
                }

                self.alive.extend(then_alive);

                if then_dead.len() < self.dead.len() {
                    self.dead.retain(|dead| then_dead.contains(dead));
                } else {
                    then_dead.retain(|dead| self.dead.contains(dead));
                    self.dead = then_dead;
                }
            }
            Statement::Const(const_declaration) => self.visit_const(const_declaration),
            Statement::Definition(definition_statement) => {
                match &definition_statement.place {
                    leo_ast::DefinitionPlace::Single(identifier) => {
                        self.check_and_insert(*identifier);
                    }
                    leo_ast::DefinitionPlace::Multiple(identifiers) => {
                        for identifier in identifiers {
                            self.check_and_insert(*identifier);
                        }
                    }
                }
                if let Some(ty) = definition_statement.type_.as_ref() {
                    self.visit_type(ty)
                }
                self.visit_expression(&definition_statement.value, &Default::default());
            }
            Statement::Expression(expression_statement) => self.visit_expression_statement(expression_statement),
            Statement::Iteration(iteration_statement) => {
                let mut alive_after = self.alive.clone();
                let mut dead_after = self.dead.clone();
                let statement = DefinitionStatement {
                    place: leo_ast::DefinitionPlace::Single(iteration_statement.variable),
                    type_: None,
                    value: Expression::default(),
                    span: iteration_statement.span,
                    id: 0,
                }
                .into();
                let block = Block {
                    statements: vec![statement]
                        .into_iter()
                        .chain(iteration_statement.block.statements.clone())
                        .collect(),
                    ..iteration_statement.block
                };
                loop {
                    self.visit_block(&block);
                    let body_in = self.alive.clone();
                    let mut new_alive_after = alive_after.clone();
                    new_alive_after.extend(body_in);
                    if new_alive_after == alive_after {
                        self.alive = new_alive_after;
                        break;
                    }

                    alive_after = new_alive_after;
                    self.dead = dead_after.clone();
                }

                if dead_after.len() < self.dead.len() {
                    self.dead.retain(|dead| dead_after.contains(dead));
                } else {
                    dead_after.retain(|dead| self.dead.contains(dead));
                    self.dead = dead_after;
                }
            }
            Statement::Return(return_statement) => self.visit_return(return_statement),
        }
    }

    fn visit_assign(&mut self, input: &leo_ast::AssignStatement) {
        self.visit_expression(&input.place, &true);
        self.visit_expression(&input.value, &Default::default());
    }

    fn visit_path(&mut self, input: &leo_ast::Path, additional: &Self::AdditionalInput) -> Self::Output {
        match additional {
            true => self.check_and_insert(input.identifier()),
            false => _ = self.alive.insert(input.identifier().name),
        }
    }
}

impl UnusedAssignments<'_> {
    fn check_and_insert(&mut self, identifier: Identifier) {
        if !self.alive.swap_remove(&identifier.name) {
            self.dead.insert((identifier.name, identifier.span));
        }
    }
}
