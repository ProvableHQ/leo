// Copyright (C) 2019-2025 Aleo Systems Inc.
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

use crate::ConstPropagator;

use leo_ast::{
    AssertStatement,
    AssertVariant,
    AssignStatement,
    Block,
    ConditionalStatement,
    ConsoleStatement,
    ConstDeclaration,
    DeclarationType,
    DefinitionStatement,
    Expression,
    ExpressionReconstructor,
    ExpressionStatement,
    IterationStatement,
    Node,
    ReturnStatement,
    Statement,
    StatementReconstructor,
};

fn empty_statement() -> Statement {
    Statement::Block(Block { statements: Vec::new(), span: Default::default(), id: Default::default() })
}

fn is_empty_statement(stmt: &Statement) -> bool {
    let Statement::Block(block) = stmt else {
        return false;
    };
    block.statements.is_empty()
}

impl StatementReconstructor for ConstPropagator<'_> {
    fn reconstruct_assert(&mut self, mut input: AssertStatement) -> (Statement, Self::AdditionalOutput) {
        // Catching asserts at compile time is not feasible here due to control flow, but could be done in
        // a later pass after loops are unrolled and conditionals are flattened.
        input.variant = match input.variant {
            AssertVariant::Assert(expr) => AssertVariant::Assert(self.reconstruct_expression(expr).0),

            AssertVariant::AssertEq(lhs, rhs) => {
                AssertVariant::AssertEq(self.reconstruct_expression(lhs).0, self.reconstruct_expression(rhs).0)
            }

            AssertVariant::AssertNeq(lhs, rhs) => {
                AssertVariant::AssertNeq(self.reconstruct_expression(lhs).0, self.reconstruct_expression(rhs).0)
            }
        };

        (Statement::Assert(input), None)
    }

    fn reconstruct_assign(&mut self, mut assign: AssignStatement) -> (Statement, Self::AdditionalOutput) {
        assign.value = self.reconstruct_expression(assign.value).0;
        (Statement::Assign(Box::new(assign)), None)
    }

    fn reconstruct_block(&mut self, mut block: Block) -> (Block, Self::AdditionalOutput) {
        self.in_scope(block.id(), |slf| {
            block.statements.retain_mut(|statement| {
                let bogus_statement = empty_statement();
                let this_statement = std::mem::replace(statement, bogus_statement);
                *statement = slf.reconstruct_statement(this_statement).0;
                !is_empty_statement(statement)
            });
            (block, None)
        })
    }

    fn reconstruct_conditional(
        &mut self,
        mut conditional: ConditionalStatement,
    ) -> (Statement, Self::AdditionalOutput) {
        conditional.condition = self.reconstruct_expression(conditional.condition).0;
        conditional.then = self.reconstruct_block(conditional.then).0;
        if let Some(mut otherwise) = conditional.otherwise {
            *otherwise = self.reconstruct_statement(*otherwise).0;
            conditional.otherwise = Some(otherwise);
        }

        (Statement::Conditional(conditional), None)
    }

    fn reconstruct_console(&mut self, _: ConsoleStatement) -> (Statement, Self::AdditionalOutput) {
        unreachable!("`ConsoleStatement`s should not be in the AST at this phase of compilation.")
    }

    fn reconstruct_const(&mut self, mut input: ConstDeclaration) -> (Statement, Self::AdditionalOutput) {
        let span = input.span();

        let (expr, opt_value) = self.reconstruct_expression(input.value);

        if opt_value.is_some() {
            self.symbol_table.insert_const(self.program, input.place.name, expr.clone());
        } else {
            self.const_not_evaluated = Some(span);
        }

        input.value = expr;

        (Statement::Const(input), None)
    }

    fn reconstruct_definition(&mut self, mut definition: DefinitionStatement) -> (Statement, Self::AdditionalOutput) {
        let span = definition.span();

        let (expr, opt_value) = self.reconstruct_expression(definition.value);

        if definition.declaration_type == DeclarationType::Const {
            if opt_value.is_some() {
                let Expression::Identifier(id) = &definition.place else {
                    panic!("Const definitions always have identifiers as the place.");
                };
                self.symbol_table.insert_const(self.program, id.name, expr.clone());
            } else {
                self.const_not_evaluated = Some(span);
            }
        }

        definition.value = expr;

        (Statement::Definition(definition), None)
    }

    fn reconstruct_expression_statement(
        &mut self,
        mut input: ExpressionStatement,
    ) -> (Statement, Self::AdditionalOutput) {
        input.expression = self.reconstruct_expression(input.expression).0;

        if matches!(&input.expression, Expression::Unit(..) | Expression::Literal(..)) {
            // We were able to evaluate this at compile time, but we need to get rid of this statement as
            // we can't have expression statements that aren't calls.
            (empty_statement(), Default::default())
        } else {
            (Statement::Expression(input), Default::default())
        }
    }

    fn reconstruct_iteration(&mut self, mut iteration: IterationStatement) -> (Statement, Self::AdditionalOutput) {
        iteration.start = self.reconstruct_expression(iteration.start).0;
        iteration.stop = self.reconstruct_expression(iteration.stop).0;
        self.in_scope(iteration.id(), |slf| {
            iteration.block = slf.reconstruct_block(iteration.block).0;
            (Statement::Iteration(Box::new(iteration)), None)
        })
    }

    fn reconstruct_return(&mut self, mut input: ReturnStatement) -> (Statement, Self::AdditionalOutput) {
        input.expression = self.reconstruct_expression(input.expression).0;
        (Statement::Return(input), Default::default())
    }
}
