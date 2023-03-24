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

use crate::*;

/// A Consumer trait for statements in the AST.
pub trait StatementConsumer {
    type Output;

    fn consume_statement(&mut self, input: Statement) -> Self::Output {
        match input {
            Statement::AssemblyBlock(stmt) => self.consume_assembly_block(stmt),
            Statement::Assert(assert) => self.consume_assert(assert),
            Statement::Assign(stmt) => self.consume_assign(*stmt),
            Statement::Block(stmt) => self.consume_block(stmt),
            Statement::Conditional(stmt) => self.consume_conditional(stmt),
            Statement::Console(stmt) => self.consume_console(stmt),
            Statement::Decrement(stmt) => self.consume_decrement(stmt),
            Statement::Definition(stmt) => self.consume_definition(stmt),
            Statement::Expression(stmt) => self.consume_expression_statement(stmt),
            Statement::Increment(stmt) => self.consume_increment(stmt),
            Statement::Iteration(stmt) => self.consume_iteration(*stmt),
            Statement::Return(stmt) => self.consume_return(stmt),
        }
    }

    fn consume_assembly_block(&mut self, input: AssemblyBlock) -> Self::Output;

    fn consume_assert(&mut self, input: AssertStatement) -> Self::Output;

    fn consume_assign(&mut self, input: AssignStatement) -> Self::Output;

    fn consume_block(&mut self, input: Block) -> Self::Output;

    fn consume_conditional(&mut self, input: ConditionalStatement) -> Self::Output;

    fn consume_console(&mut self, input: ConsoleStatement) -> Self::Output;

    fn consume_decrement(&mut self, input: DecrementStatement) -> Self::Output;

    fn consume_definition(&mut self, input: DefinitionStatement) -> Self::Output;

    fn consume_expression_statement(&mut self, input: ExpressionStatement) -> Self::Output;

    fn consume_increment(&mut self, input: IncrementStatement) -> Self::Output;

    fn consume_iteration(&mut self, input: IterationStatement) -> Self::Output;

    fn consume_return(&mut self, input: ReturnStatement) -> Self::Output;
}
