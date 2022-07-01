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

// TODO: We currently rely on the Leo tokenizer and parser to parse the assembly block.
//   Consider designing a separate Parser for instructions.
//   This would improve modularity, reduce the likelihood of errors, improve error handling, at the cost of building, linking, and managing the new parser.
// TODO: If snarkVM instructions are used directly, then we should directly use the associated instruction parsers.

use crate::{ParserContext, Token};

use leo_ast::{
    Add, And, Div, Expression, GreaterThan, GreaterThanOrEqual, Instruction, IsEqual, IsNotEqual, LessThan,
    LessThanOrEqual, Mul, Node, Nop, Not, Operand, Or, Sub, Ternary,
};
use leo_errors::{ParserError, Result};
use leo_span::{sym, Span};

impl ParserContext<'_> {
    pub fn parse_instruction(&mut self) -> Result<Instruction> {
        // Parse the opcode. Since we are using the Leo tokenizer, the opcode will be tokenized as an identifier.
        let identifier = self.expect_ident()?;
        match identifier.name {
            sym::add => self.parse_add_instruction(identifier.span),
            sym::and => self.parse_and_instruction(identifier.span),
            sym::div => self.parse_div_instruction(identifier.span),
            sym::gt => self.parse_greater_than_instruction(identifier.span),
            sym::gte => self.parse_greater_than_or_equal_instruction(identifier.span),
            sym::eq => self.parse_equal_instruction(identifier.span),
            sym::neq => self.parse_not_equal_instruction(identifier.span),
            sym::lt => self.parse_less_than_instruction(identifier.span),
            sym::lte => self.parse_less_than_or_equal_instruction(identifier.span),
            sym::mul => self.parse_mul_instruction(identifier.span),
            sym::not => self.parse_not_instruction(identifier.span),
            sym::or => self.parse_or_instruction(identifier.span),
            sym::sub => self.parse_sub_instruction(identifier.span),
            sym::ter => self.parse_ternary_instruction(identifier.span),
            _ => {
                self.emit_err(ParserError::invalid_opcode_in_assembly_instruction(identifier.span));
                // Attempt to recover the parser by eating tokens until we find a semi-colon or closing bracket.
                while !(self.check(&Token::Eof) || self.check(&Token::RightCurly) || self.check(&Token::Semicolon)) {
                    self.bump();
                }
                if let Ok(span) = self.expect(&Token::Eof) {
                    self.emit_err(ParserError::unexpected_eof(span));
                } else if let Ok(span) = self.expect(&Token::RightCurly) {
                    self.emit_err(ParserError::unexpected_end_of_assembly_block(span));
                } else {
                    self.expect(&Token::Semicolon)?;
                }
                Ok(Instruction::Nop(Nop { span: identifier.span }))
            }
        }
    }

    // TODO: Consider a macro to simplify boilerplate code.

    pub fn parse_add_instruction(&mut self, start: Span) -> Result<Instruction> {
        let first = self.parse_operand()?;
        let second = self.parse_operand()?;
        self.expect_into()?;
        let destination = self.expect_ident()?;
        self.expect(&Token::Semicolon)?;
        Ok(Instruction::Add(Add {
            first,
            second,
            destination,
            span: start + destination.span,
        }))
    }

    pub fn parse_and_instruction(&mut self, start: Span) -> Result<Instruction> {
        let first = self.parse_operand()?;
        let second = self.parse_operand()?;
        self.expect_into()?;
        let destination = self.expect_ident()?;
        self.expect(&Token::Semicolon)?;
        Ok(Instruction::And(And {
            first,
            second,
            destination,
            span: start + destination.span,
        }))
    }

    pub fn parse_div_instruction(&mut self, start: Span) -> Result<Instruction> {
        let first = self.parse_operand()?;
        let second = self.parse_operand()?;
        self.expect_into()?;
        let destination = self.expect_ident()?;
        self.expect(&Token::Semicolon)?;
        Ok(Instruction::Div(Div {
            first,
            second,
            destination,
            span: start + destination.span,
        }))
    }

    pub fn parse_greater_than_instruction(&mut self, start: Span) -> Result<Instruction> {
        let first = self.parse_operand()?;
        let second = self.parse_operand()?;
        self.expect_into()?;
        let destination = self.expect_ident()?;
        self.expect(&Token::Semicolon)?;
        Ok(Instruction::GreaterThan(GreaterThan {
            first,
            second,
            destination,
            span: start + destination.span,
        }))
    }

    pub fn parse_greater_than_or_equal_instruction(&mut self, start: Span) -> Result<Instruction> {
        let first = self.parse_operand()?;
        let second = self.parse_operand()?;
        self.expect_into()?;
        let destination = self.expect_ident()?;
        self.expect(&Token::Semicolon)?;
        Ok(Instruction::GreaterThanOrEqual(GreaterThanOrEqual {
            first,
            second,
            destination,
            span: start + destination.span,
        }))
    }

    pub fn parse_equal_instruction(&mut self, start: Span) -> Result<Instruction> {
        let first = self.parse_operand()?;
        let second = self.parse_operand()?;
        self.expect_into()?;
        let destination = self.expect_ident()?;
        self.expect(&Token::Semicolon)?;
        Ok(Instruction::IsEqual(IsEqual {
            first,
            second,
            destination,
            span: start + destination.span,
        }))
    }

    pub fn parse_not_equal_instruction(&mut self, start: Span) -> Result<Instruction> {
        let first = self.parse_operand()?;
        let second = self.parse_operand()?;
        self.expect_into()?;
        let destination = self.expect_ident()?;
        self.expect(&Token::Semicolon)?;
        Ok(Instruction::IsNotEqual(IsNotEqual {
            first,
            second,
            destination,
            span: start + destination.span,
        }))
    }

    pub fn parse_less_than_instruction(&mut self, start: Span) -> Result<Instruction> {
        let first = self.parse_operand()?;
        let second = self.parse_operand()?;
        self.expect_into()?;
        let destination = self.expect_ident()?;
        self.expect(&Token::Semicolon)?;
        Ok(Instruction::LessThan(LessThan {
            first,
            second,
            destination,
            span: start + destination.span,
        }))
    }

    pub fn parse_less_than_or_equal_instruction(&mut self, start: Span) -> Result<Instruction> {
        let first = self.parse_operand()?;
        let second = self.parse_operand()?;
        self.expect_into()?;
        let destination = self.expect_ident()?;
        self.expect(&Token::Semicolon)?;
        Ok(Instruction::LessThanOrEqual(LessThanOrEqual {
            first,
            second,
            destination,
            span: start + destination.span,
        }))
    }

    pub fn parse_mul_instruction(&mut self, start: Span) -> Result<Instruction> {
        let first = self.parse_operand()?;
        let second = self.parse_operand()?;
        self.expect_into()?;
        let destination = self.expect_ident()?;
        self.expect(&Token::Semicolon)?;
        Ok(Instruction::Mul(Mul {
            first,
            second,
            destination,
            span: start + destination.span,
        }))
    }

    pub fn parse_not_instruction(&mut self, start: Span) -> Result<Instruction> {
        let operand = self.parse_operand()?;
        self.expect_into()?;
        let destination = self.expect_ident()?;
        self.expect(&Token::Semicolon)?;
        Ok(Instruction::Not(Not {
            operand,
            destination,
            span: start + destination.span,
        }))
    }

    pub fn parse_or_instruction(&mut self, start: Span) -> Result<Instruction> {
        let first = self.parse_operand()?;
        let second = self.parse_operand()?;
        self.expect_into()?;
        let destination = self.expect_ident()?;
        self.expect(&Token::Semicolon)?;
        Ok(Instruction::Or(Or {
            first,
            second,
            destination,
            span: start + destination.span,
        }))
    }

    pub fn parse_sub_instruction(&mut self, start: Span) -> Result<Instruction> {
        let first = self.parse_operand()?;
        let second = self.parse_operand()?;
        self.expect_into()?;
        let destination = self.expect_ident()?;
        self.expect(&Token::Semicolon)?;
        Ok(Instruction::Sub(Sub {
            first,
            second,
            destination,
            span: start + destination.span,
        }))
    }

    pub fn parse_ternary_instruction(&mut self, start: Span) -> Result<Instruction> {
        let first = self.parse_operand()?;
        let second = self.parse_operand()?;
        let third = self.parse_operand()?;
        self.expect_into()?;
        let destination = self.expect_ident()?;
        self.expect(&Token::Semicolon)?;
        Ok(Instruction::Ternary(Ternary {
            first,
            second,
            third,
            destination,
            span: start + destination.span,
        }))
    }

    pub fn expect_into(&mut self) -> Result<()> {
        let identifier = self.expect_ident()?;
        match identifier.name {
            sym::into => Ok(()),
            symbol => Err(ParserError::unexpected(symbol.as_str(), "`into`", identifier.span).into()),
        }
    }

    pub fn parse_operand(&mut self) -> Result<Operand> {
        let expression = self.parse_primary_expression()?;
        match expression {
            Expression::Identifier(identifier) => Ok(Operand::Identifier(identifier)),
            Expression::Literal(literal) => Ok(Operand::Literal(literal)),
            Expression::Access(..)
            | Expression::Binary(..)
            | Expression::Call(..)
            | Expression::CircuitInit(..)
            | Expression::Err(..)
            | Expression::Ternary(..)
            | Expression::Unary(..) => {
                self.emit_err(ParserError::invalid_operand_in_assembly_instruction(expression.span()));
                Ok(Operand::Invalid)
            }
        }
    }
}
