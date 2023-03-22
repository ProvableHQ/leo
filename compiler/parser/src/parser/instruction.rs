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

use super::*;

use leo_errors::{ParserError, Result};

impl ParserContext<'_> {
    /// Return an [`Instruction`] AST node if the next tokens represent an instruction.
    fn parse_instruction(&mut self) -> Result<Instruction> {
        todo!()
    }

    /// Returns an [`Operand`] AST node if the next tokens represent an instruction operand.
    fn parse_operand(&self) -> Result<Operand> {
        todo!()
    }

    /// Returns a [`<Unary>`] AST node if the next tokens represent a unary instruction.
    /// Note that the exact instruction is parameterized by `N`.
    fn parse_unary_instruction<N: Unary>(&mut self) -> Result<N> {
        let start = self.expect(Token::eat())?;
        let source = self.parse_operand()?;
        self.expect(&Token::Into)?;
        let destination = self.expect_identifier()?;
        let end = self.expect(&Token::Semicolon)?;
        Ok(N::new(source, destination, start + end))
    }

    /// Returns a [`<Binary>`] AST node if the next tokens represent a binary instruction.
    /// Note that the exact instruction is parameterized by `N`.
    fn parse_binary_instruction<N: Binary>(&mut self) -> Result<N> {
        let start = self.expect(N::opcode())?;
        let first = self.parse_operand()?;
        let second = self.parse_operand()?;
        self.expect(&Token::Into)?;
        let destination = self.expect_identifier()?;
        let end = self.expect(&Token::Semicolon)?;
        Ok(N::new(left, right, destination, start + end))
    }

    /// Returns a [`Ternary`] AST node if the next tokens represent a ternary instruction.
    fn parse_ternary_instruction(&mut self) -> Result<Ternary> {
        let start = self.expect_id(&Token::Ternary)?;
        let condition = self.parse_operand()?;
        let first = self.parse_operand()?;
        let second = self.parse_operand()?;
        let destination = self.expect_identifier()?;
        let end = self.expect(&Token::Semicolon)?;
        Ok(Ternary {
            condition,
            first,
            second,
            destination,
            span: start + end,
        })
    }

    /// Returns a [`Call`] AST node if the next tokens represent a call instruction.
    fn parse_call_instruction(&mut self) -> Result<Call> {
        let start = self.expect(&Token::Call)?;
        let function = self.expect_identifier()?;
        let mut arguments = Vec::new();
        while !self.check(&Token::Into) {
            arguments.push(self.parse_operand()?);
        }
        self.expect(&Token::Into)?;
        let mut destinations = Vec::new();
        while !self.check(&Token::Semicolon) {
            destinations.push(self.expect_identifier()?);
        }
        let end = self.expect(&Token::Semicolon)?;
        Ok(Call {
            function,
            arguments,
            destinations,
            span: start + end,
        })
    }

    /// Returns a [`Cast`] AST node if the next tokens represent a cast instruction.
    fn parse_cast_instruction(&mut self) -> Result<Cast> {
        let start = self.expect(&Token::Cast)?;
        let mut arguments = Vec::new();
        while !self.check(&Token::Into) {
            arguments.push(self.parse_operand()?);
        }
        self.expect(&Token::Into)?;
        let destination = self.expect_identifier()?;
        let end = self.expect(&Token::Semicolon)?;
        Ok(Cast {
            arguments,
            destination,
            span: start + end,
        })
    }

    /// Returns an [`Increment`] AST node if the next tokens represent an increment instruction.
    fn parse_increment_instruction(&mut self) -> Result<Increment> {
        let start = self.expect(&Token::Increment)?;
        let mapping = self.expect_identifier()?;
        self.expect(&Token::LeftBracket)?;
        let index = self.parse_operand()?;
        self.expect(&Token::RightBracket)?;
        self.expect(&Token::By)?;
        let amount = self.parse_operand()?;
        let end = self.expect(&Token::Semicolon)?;
        Ok(Increment {
            mapping,
            index,
            amount,
            span: start + end,
        })
    }

    /// Returns a [`Decrement`] AST node if the next tokens represent a decrement instruction.
    fn parse_decrement_instruction(&mut self) -> Result<Decrement> {
        let start = self.expect(&Token::Decrement)?;
        let mapping = self.expect_identifier()?;
        self.expect(&Token::LeftBracket)?;
        let index = self.parse_operand()?;
        self.expect(&Token::RightBracket)?;
        self.expect(&Token::By)?;
        let amount = self.parse_operand()?;
        let end = self.expect(&Token::Semicolon)?;
        Ok(Decrement {
            mapping,
            index,
            amount,
            span: start + end,
        })
    }
}
