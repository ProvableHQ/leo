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

use leo_ast::{Expression, Instruction, Opcode, Operand};
use leo_errors::Result;
use leo_span::Symbol;

// TODO: Note that this design is a prototype.
impl ParserContext<'_> {
    pub fn parse_instruction(&mut self) -> Result<Instruction> {
        // Parse the opcode. Since we are using the Leo tokenizer, the opcode will be tokenized as an identifier.
        let opcode_identifer = self.expect_ident()?;
        let opcode = match &*opcode_identifer.name.as_str() {
            "add" => Opcode::Add,
            "and" => Opcode::And,
            "div" => Opcode::Div,
            "gt" => Opcode::GreaterThan,
            "gte" => Opcode::GreaterThanOrEqual,
            "eq" => Opcode::IsEqual,
            "neq" => Opcode::IsNotEqual,
            "lt" => Opcode::LessThan,
            "lte" => Opcode::LessThanOrEqual,
            "mul" => Opcode::Mul,
            "not" => Opcode::Not,
            "or" => Opcode::Or,
            "sub" => Opcode::Sub,
            "ter" => Opcode::Ternary,
            _ => todo!("Error: invalid opcode. Need to implement an official parser error."),
        };

        let into_token = Token::Ident(Symbol::intern("into"));

        // Parse arguments.
        let mut operands = Vec::new();
        while !self.check(&into_token) || !self.check(&Token::Semicolon) {
            let operand = match self.parse_expression()? {
                Expression::Identifier(identifier) => Operand::Identifier(identifier),
                Expression::Literal(literal) => Operand::Literal(literal),
                Expression::Access(..)
                | Expression::Binary(..)
                | Expression::Call(..)
                | Expression::CircuitInit(..)
                | Expression::Err(..)
                | Expression::Ternary(..)
                | Expression::Unary(..) => todo!("Error: invalid operand. Need to implement an official parser error."),
            };
            operands.push(operand);
        }

        // If the next token is the `into` keyword, then we need to parse destinations.
        let mut destinations = Vec::new();
        if self.check(&into_token) {
            while !self.check(&Token::Semicolon) {
                destinations.push(self.expect_ident()?)
            }
        }

        let end_span = self.expect(&Token::Semicolon)?;

        Ok(Instruction {
            opcode,
            operands,
            destinations,
            span: opcode_identifer.span + end_span,
        })
    }
}
