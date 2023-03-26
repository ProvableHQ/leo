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

use leo_ast::Opcode::*;
use leo_errors::{ParserError, Result};
use leo_span::{sym, Symbol};

macro_rules! parse_opcode {
    ($parser: ident, $opcode: ident, $base:expr) => {{
        let span = $parser.expect($base)?;
        Ok(($opcode, span))
    }};
    ($parser: ident, $opcode: ident, $base:expr, $variant:expr) => {{
        let start = $parser.expect($base)?;
        $parser.expect(&Token::Dot)?;
        let end = $parser.expect($variant)?;
        Ok((Opcode::$opcode, start + end))
    }};
}

impl ParserContext<'_> {
    /// Returns a [`Instruction`] AST node if the next tokens represent an instruction.
    pub fn parse_instruction(&mut self) -> Result<Instruction> {
        // Parse the opcode.
        let (opcode, start) = self.parse_opcode()?;
        // Initialize storage for components of the instruction.
        let mut operands = Vec::new();
        let mut destinations = Vec::new();
        // Parse the operands until the `into` keyword or `;` token.
        while !(self.check_identifier_with_name(sym::into) || self.check(&Token::Semicolon)) {
            operands.push(self.parse_expression()?);
        }
        if self.check_identifier_with_name(sym::into) {
            // Parse the `into` keyword.
            self.expect_identifier_with_name(sym::into)?;
            // Parse at least one destination register.
            destinations.push(self.expect_identifier()?);
            // Parse the destinations until the `;`.
            while !self.check(&Token::Semicolon) {
                destinations.push(self.expect_identifier()?);
            }
        }
        let end = self.expect(&Token::Semicolon)?;
        Ok(Instruction {
            opcode,
            operands,
            destinations,
            span: start + end,
        })
    }

    #[rustfmt::skip]
    /// Return an [`(Opcode, Span)`] if the next tokens represent an opcode.
    pub fn parse_opcode(&mut self) -> Result<(Opcode, Span)> {
        let first = &self.token.token;
        let second = self.look_ahead(1, |t| &t.token);
        let third = self.look_ahead(2, |t| &t.token);

        match first {
            Token::Identifier(sym::abs) => match second {
                Token::Dot => parse_opcode!(self, AbsWrapped, &Token::Identifier(sym::abs), &Token::Identifier(sym::w)),
                _ => parse_opcode!(self, Abs, &Token::Identifier(sym::abs)),
            },
            Token::Identifier(sym::add) => match second {
                Token::Dot => parse_opcode!(self, AddWrapped, &Token::Identifier(sym::add), &Token::Identifier(sym::w)),
                _ => parse_opcode!(self, Add, &Token::Identifier(sym::add)),
            },
            Token::Identifier(sym::and) => parse_opcode!(self, And, &Token::Identifier(sym::and)),
            Token::Assert => match (second, third) {
                (Token::Dot, Token::Identifier(sym::eq)) => parse_opcode!(self, AssertEq, &Token::Assert, &Token::Identifier(sym::eq)),
                _ => parse_opcode!(self, AssertNeq, &Token::Assert, &Token::Identifier(sym::neq)),
            },
            Token::Identifier(sym::commit) => match (second, third) {
                (Token::Dot, Token::Identifier(sym::bhp256)) => parse_opcode!(self, CommitBHP256, &Token::Identifier(sym::commit), &Token::Identifier(sym::bhp256)),
                (Token::Dot, Token::Identifier(sym::bhp512)) => parse_opcode!(self, CommitBHP512, &Token::Identifier(sym::commit), &Token::Identifier(sym::bhp512)),
                (Token::Dot, Token::Identifier(sym::bhp768)) => parse_opcode!(self, CommitBHP768, &Token::Identifier(sym::commit), &Token::Identifier(sym::bhp768)),
                (Token::Dot, Token::Identifier(sym::bhp1024)) => parse_opcode!(self, CommitBHP1024, &Token::Identifier(sym::commit), &Token::Identifier(sym::bhp1024)),
                (Token::Dot, Token::Identifier(sym::ped64)) => parse_opcode!(self, CommitPED64, &Token::Identifier(sym::commit), &Token::Identifier(sym::ped64)),
                _ => parse_opcode!(self, CommitPED128, &Token::Identifier(sym::commit), &Token::Identifier(sym::ped128)),
            }
            Token::Identifier(sym::div) => match second {
                Token::Dot => parse_opcode!(self, Div, &Token::Identifier(sym::div), &Token::Identifier(sym::w)),
                _ => parse_opcode!(self, Div, &Token::Identifier(sym::div)),
            },
            Token::Identifier(sym::double) => parse_opcode!(self, Double, &Token::Identifier(sym::double)),
            Token::Identifier(sym::gt) => parse_opcode!(self, GreaterThan, &Token::Identifier(sym::gt)),
            Token::Identifier(sym::gte) => parse_opcode!(self, GreaterThanOrEqual, &Token::Identifier(sym::gte)),
            Token::Identifier(sym::hash) => match (second, third) {
                (Token::Dot, Token::Identifier(sym::bhp256)) => parse_opcode!(self, HashBHP256, &Token::Identifier(sym::hash), &Token::Identifier(sym::bhp256)),
                (Token::Dot, Token::Identifier(sym::bhp512)) => parse_opcode!(self, HashBHP512, &Token::Identifier(sym::hash), &Token::Identifier(sym::bhp512)),
                (Token::Dot, Token::Identifier(sym::bhp768)) => parse_opcode!(self, HashBHP768, &Token::Identifier(sym::hash), &Token::Identifier(sym::bhp768)),
                (Token::Dot, Token::Identifier(sym::bhp1024)) => parse_opcode!(self, HashBHP1024, &Token::Identifier(sym::hash), &Token::Identifier(sym::bhp1024)),
                (Token::Dot, Token::Identifier(sym::ped64)) => parse_opcode!(self, HashPED64, &Token::Identifier(sym::hash), &Token::Identifier(sym::ped64)),
                (Token::Dot, Token::Identifier(sym::ped128)) => parse_opcode!(self, HashPED128, &Token::Identifier(sym::hash), &Token::Identifier(sym::ped128)),
                (Token::Dot, Token::Identifier(sym::psd2)) => parse_opcode!(self, HashPSD2, &Token::Identifier(sym::hash), &Token::Identifier(sym::psd2)),
                (Token::Dot, Token::Identifier(sym::psd4)) => parse_opcode!(self, HashPSD4, &Token::Identifier(sym::hash), &Token::Identifier(sym::psd4)),
                _ => parse_opcode!(self, HashPSD8, &Token::Identifier(sym::hash), &Token::Identifier(sym::psd8)),
            }
            Token::Identifier(sym::inv) => parse_opcode!(self, Inv, &Token::Identifier(sym::inv)),
            Token::Identifier(sym::is) => match (second, third) {
                (Token::Dot, Token::Identifier(sym::eq)) => parse_opcode!(self, IsEq, &Token::Identifier(sym::is), &Token::Identifier(sym::eq)),
                _ => parse_opcode!(self, IsNeq, &Token::Identifier(sym::is), &Token::Identifier(sym::neq)),
            },
            Token::Identifier(sym::lt) => parse_opcode!(self, LessThan, &Token::Identifier(sym::lt)),
            Token::Identifier(sym::lte) => parse_opcode!(self, LessThanOrEqual, &Token::Identifier(sym::lte)),
            Token::Identifier(sym::Mod) => parse_opcode!(self, Modulo, &Token::Identifier(sym::Mod)),
            Token::Identifier(sym::mul) => match second {
                Token::Dot => parse_opcode!(self, MulWrapped, &Token::Identifier(sym::mul), &Token::Identifier(sym::w)),
                _ => parse_opcode!(self, Mul, &Token::Identifier(sym::mul)),
            },
            Token::Identifier(sym::nand) => parse_opcode!(self, Nand, &Token::Identifier(sym::nand)),
            Token::Identifier(sym::neg) => parse_opcode!(self, Neg, &Token::Identifier(sym::neg)),
            Token::Identifier(sym::not) => parse_opcode!(self, Not, &Token::Identifier(sym::not)),
            Token::Identifier(sym::or) => parse_opcode!(self, Or, &Token::Identifier(sym::or)),
            Token::Identifier(sym::pow) => match second {
                Token::Dot => parse_opcode!(self, PowWrapped, &Token::Identifier(sym::pow), &Token::Identifier(sym::w)),
                _ => parse_opcode!(self, Pow, &Token::Identifier(sym::pow)),
            },
            Token::Identifier(sym::rem) => match second {
                Token::Dot => parse_opcode!(self, RemWrapped, &Token::Identifier(sym::rem), &Token::Identifier(sym::w)),
                _ => parse_opcode!(self, Rem, &Token::Identifier(sym::rem)),
            },
            Token::Identifier(sym::shl) => match second {
                Token::Dot => parse_opcode!(self, ShlWrapped, &Token::Identifier(sym::shl), &Token::Identifier(sym::w)),
                _ => parse_opcode!(self, Shl, &Token::Identifier(sym::shl)),
            },
            Token::Identifier(sym::shr) => match second {
                Token::Dot => parse_opcode!(self, ShrWrapped, &Token::Identifier(sym::shr), &Token::Identifier(sym::w)),
                _ => parse_opcode!(self, Shr, &Token::Identifier(sym::shr)),
            },
            Token::Identifier(sym::square) => parse_opcode!(self, Square, &Token::Identifier(sym::square)),
            Token::Identifier(sym::sqrt) => parse_opcode!(self, SquareRoot, &Token::Identifier(sym::sqrt)),
            Token::Identifier(sym::sub) => match second {
                Token::Dot => parse_opcode!(self, SubWrapped, &Token::Identifier(sym::sub), &Token::Identifier(sym::w)),
                _ => parse_opcode!(self, Sub, &Token::Identifier(sym::sub)),
            },
            Token::Identifier(sym::ternary) => parse_opcode!(self, Ternary, &Token::Identifier(sym::ternary)),
            Token::Identifier(sym::xor) => parse_opcode!(self, Xor, &Token::Identifier(sym::xor)),
            _ => {
                return Err(ParserError::invalid_opcode(self.token.span).into());
            }
        }
    }

    /// Expects an `Identifier` with a given name.
    fn expect_identifier_with_name(&mut self, name: Symbol) -> Result<Identifier> {
        match self.expect_identifier() {
            Ok(identifier) => match identifier.name == name {
                true => Ok(identifier),
                false => Err(ParserError::expected_identifier_with_name(name, identifier.span).into()),
            },
            Err(error) => Err(error),
        }
    }

    /// Checks if the next token is an `Identifier` with a given name.
    fn check_identifier_with_name(&mut self, name: Symbol) -> bool {
        match &self.token {
            SpannedToken {
                token: Token::Identifier(symbol),
                ..
            } => *symbol == name,
            _ => false,
        }
    }
}
