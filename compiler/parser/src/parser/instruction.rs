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

// TODO: Consider designing a separate Parser for instructions. This would improve modularity at the
//   cost of building/managing the new parser.
// TODO: If snarkVM instructions are used directly, then we should directly use the associated instruction parsers.

use crate::{ParserContext, Token};

use leo_ast::{
    Add, And, Div, Expression, GreaterThan, GreaterThanOrEqual, Instruction, IsEqual, IsNotEqual, LessThan,
    LessThanOrEqual, Mul, Node, Nop, Not, Operand, Or, Sub, Ternary,
};
use leo_errors::{ParserError, Result};
use leo_span::Span;

// TODO: Note that this design is a prototype.
impl ParserContext<'_> {
    pub fn parse_instruction(&mut self) -> Result<Instruction> {
        // Parse the opcode. Since we are using the Leo tokenizer, the opcode will be tokenized as an identifier.
        let identifier = self.expect_ident()?;
        match &*identifier.name.as_str() {
            "add" => self.parse_add_instruction(identifier.span),
            "and" => self.parse_and_instruction(identifier.span),
            "div" => self.parse_div_instruction(identifier.span),
            "gt" => self.parse_greater_than_instruction(identifier.span),
            "gte" => self.parse_greater_than_or_equal_instruction(identifier.span),
            "eq" => self.parse_equal_instruction(identifier.span),
            "neq" => self.parse_not_equal_instruction(identifier.span),
            "lt" => self.parse_less_than_instruction(identifier.span),
            "lte" => self.parse_less_than_or_equal_instruction(identifier.span),
            "mul" => self.parse_mul_instruction(identifier.span),
            "not" => self.parse_not_instruction(identifier.span),
            "or" => self.parse_or_instruction(identifier.span),
            "sub" => self.parse_sub_instruction(identifier.span),
            "ter" => self.parse_ternary_instruction(identifier.span),
            _ => {
                self.emit_err(ParserError::invalid_opcode_in_assembly_instruction(identifier.span));
                // TODO: Do we need to eat tokens here?
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

    // TODO: Better error handling.
    //   Separate tokens and symbols for assembly block.
    pub fn expect_into(&mut self) -> Result<()> {
        let identifier = self.expect_ident()?;
        match &*identifier.name.as_str() {
            "into" => Ok(()),
            string => Err(ParserError::unexpected(string, "into", identifier.span).into()),
        }
    }

    pub fn parse_operand(&mut self) -> Result<Operand> {
        let expression = self.parse_expression()?;
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
