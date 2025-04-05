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

use super::ConstPropagationVisitor;

use leo_ast::{
    AssertStatement,
    AssertVariant,
    AssignStatement,
    Block,
    ConditionalStatement,
    ConstDeclaration,
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

impl StatementReconstructor for ConstPropagationVisitor<'_> {
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

        (input.into(), None)
    }

    fn reconstruct_assign(&mut self, assign: AssignStatement) -> (Statement, Self::AdditionalOutput) {
        let value = self.reconstruct_expression(assign.value).0;
        let place = self.reconstruct_expression(assign.place).0;
        (AssignStatement { value, place, ..assign }.into(), None)
    }

    fn reconstruct_block(&mut self, mut block: Block) -> (Block, Self::AdditionalOutput) {
        self.in_scope(block.id(), |slf| {
            block.statements.retain_mut(|statement| {
                let bogus_statement = Statement::dummy();
                let this_statement = std::mem::replace(statement, bogus_statement);
                *statement = slf.reconstruct_statement(this_statement).0;
                !statement.is_empty()
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

    fn reconstruct_const(&mut self, mut input: ConstDeclaration) -> (Statement, Self::AdditionalOutput) {
        let span = input.span();

        let (expr, opt_value) = self.reconstruct_expression(input.value);

        if opt_value.is_some() {
            self.state.symbol_table.insert_const(self.program, input.place.name, expr.clone());
        } else {
            self.const_not_evaluated = Some(span);
        }

        input.value = expr;

        (Statement::Const(input), None)
    }

    fn reconstruct_definition(&mut self, definition: DefinitionStatement) -> (Statement, Self::AdditionalOutput) {
        (DefinitionStatement { value: self.reconstruct_expression(definition.value).0, ..definition }.into(), None)
    }

    fn reconstruct_expression_statement(
        &mut self,
        mut input: ExpressionStatement,
    ) -> (Statement, Self::AdditionalOutput) {
        input.expression = self.reconstruct_expression(input.expression).0;

        if matches!(&input.expression, Expression::Unit(..) | Expression::Literal(..)) {
            // We were able to evaluate this at compile time, but we need to get rid of this statement as
            // we can't have expression statements that aren't calls.
            (Statement::dummy(), Default::default())
        } else {
            (input.into(), Default::default())
        }
    }

    fn reconstruct_iteration(&mut self, iteration: IterationStatement) -> (Statement, Self::AdditionalOutput) {
        let id = iteration.id();
        let start = self.reconstruct_expression(iteration.start).0;
        let stop = self.reconstruct_expression(iteration.stop).0;
        self.in_scope(id, |slf| {
            (
                IterationStatement { start, stop, block: slf.reconstruct_block(iteration.block).0, ..iteration }.into(),
                None,
            )
        })
    }

    fn reconstruct_return(&mut self, input: ReturnStatement) -> (Statement, Self::AdditionalOutput) {
        (
            ReturnStatement { expression: self.reconstruct_expression(input.expression).0, ..input }.into(),
            Default::default(),
        )
    }
}
