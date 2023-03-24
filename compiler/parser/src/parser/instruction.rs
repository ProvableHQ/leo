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

use leo_ast::{Binary, Unary};
use leo_errors::{ParserError, Result};
use leo_span::{sym, Symbol};

macro_rules! parse_standard_instruction {
    ($parser:ident, $function:ident, $name:ident, $base: expr) => {
        $parser.$function::<$name>(|ctx| {
            Ok(ctx.expect(&Token::Identifier($base))?)
        }).map(|inst| Instruction::$name(inst))
    };
}

macro_rules! parse_varied_instruction {
    ($parser:ident, $function: ident, $name:ident, $base:expr, $variant:expr) => {
        $parser.$function::<$name>(|ctx| {
            let start = ctx.expect(&Token::Identifier($base))?;
            ctx.expect(&Token::Dot)?;
            let end = ctx.expect(&Token::Identifier($variant))?;
            Ok(start + end)
        }).map(|inst| Instruction::$name(inst))
    };
}


impl ParserContext<'_> {
    #[rustfmt::skip]
    /// Return an [`Instruction`] AST node if the next tokens represent an instruction.
    pub fn parse_instruction(&mut self) -> Result<Instruction> {
        let next_token = self.look_ahead(1, |t| &t.token);
        let following_token = self.look_ahead(2, |t| &t.token);

        match &self.token.token {
            Token::Identifier(sym::abs) => match next_token {
                Token::Dot => parse_varied_instruction!(self, parse_unary_instruction, AbsWrapped, sym::abs, sym::w),
                _ => parse_standard_instruction!(self, parse_unary_instruction, Abs, sym::abs),
            },
            Token::Identifier(sym::add) => match next_token {
                Token::Dot => parse_varied_instruction!(self, parse_binary_instruction, AddWrapped, sym::add, sym::w),
                _ => parse_standard_instruction!(self, parse_binary_instruction, Add, sym::add),
            },
            Token::Identifier(sym::and) => parse_standard_instruction!(self, parse_binary_instruction, And, sym::and),
            Token::Assert => match (next_token, following_token) {
                (Token::Dot, Token::Identifier(sym::eq)) => {
                    self.parse_binary_instruction::<AssertEq>(|ctx| {
                        let start = ctx.expect(&Token::Assert)?;
                        ctx.expect(&Token::Dot)?;
                        let end = ctx.expect(&Token::Identifier(sym::eq))?;
                        Ok(start + end)
                    }).map(Instruction::AssertEq)
                }
                _ => self.parse_binary_instruction::<AssertNeq>(|ctx| {
                    let start = ctx.expect(&Token::Assert)?;
                    ctx.expect(&Token::Dot)?;
                    let end = ctx.expect(&Token::Identifier(sym::neq))?;
                    Ok(start + end)
                }).map(Instruction::AssertNeq),
            },
            Token::Identifier(sym::call) => self.parse_call_instruction().map(Instruction::Call),
            Token::Identifier(sym::cast) => self.parse_cast_instruction().map(Instruction::Cast),
            Token::Identifier(sym::commit) => match (next_token, following_token) {
                (Token::Dot, Token::Identifier(sym::bhp256)) => parse_varied_instruction!(self, parse_binary_instruction, CommitBHP256, sym::commit, sym::bhp256),
                (Token::Dot, Token::Identifier(sym::bhp512)) => parse_varied_instruction!(self, parse_binary_instruction, CommitBHP512, sym::commit, sym::bhp512),
                (Token::Dot, Token::Identifier(sym::bhp768)) => parse_varied_instruction!(self, parse_binary_instruction, CommitBHP768, sym::commit, sym::bhp768),
                (Token::Dot, Token::Identifier(sym::bhp1024)) => parse_varied_instruction!(self, parse_binary_instruction, CommitBHP1024, sym::commit, sym::bhp1024),
                (Token::Dot, Token::Identifier(sym::ped64)) => parse_varied_instruction!(self, parse_binary_instruction, CommitPED64, sym::commit, sym::ped64),
                _ => parse_varied_instruction!(self, parse_binary_instruction, CommitPED128, sym::commit, sym::ped128),
            }
            Token::Decrement => self.parse_decrement_instruction().map(Instruction::Decrement),
            Token::Identifier(sym::div) => match next_token {
                Token::Dot => parse_varied_instruction!(self, parse_binary_instruction, DivWrapped, sym::div, sym::w),
                _ => parse_standard_instruction!(self, parse_binary_instruction, Div, sym::div),
            },
            Token::Identifier(sym::double) => parse_standard_instruction!(self, parse_unary_instruction, Double, sym::double),
            Token::Identifier(sym::gt) => parse_standard_instruction!(self, parse_binary_instruction, GreaterThan, sym::gt),
            Token::Identifier(sym::gte) => parse_standard_instruction!(self, parse_binary_instruction, GreaterThanOrEqual, sym::gte),
            Token::Identifier(sym::hash) => match (next_token, following_token) {
                (Token::Dot, Token::Identifier(sym::bhp256)) => parse_varied_instruction!(self, parse_unary_instruction, HashBHP256, sym::hash, sym::bhp256),
                (Token::Dot, Token::Identifier(sym::bhp512)) => parse_varied_instruction!(self, parse_unary_instruction, HashBHP512, sym::hash, sym::bhp512),
                (Token::Dot, Token::Identifier(sym::bhp768)) => parse_varied_instruction!(self, parse_unary_instruction, HashBHP768, sym::hash, sym::bhp768),
                (Token::Dot, Token::Identifier(sym::bhp1024)) => parse_varied_instruction!(self, parse_unary_instruction, HashBHP1024, sym::hash, sym::bhp1024),
                (Token::Dot, Token::Identifier(sym::ped64)) => parse_varied_instruction!(self, parse_unary_instruction, HashPED64, sym::hash, sym::ped64),
                (Token::Dot, Token::Identifier(sym::ped128)) => parse_varied_instruction!(self, parse_unary_instruction, HashPED128, sym::hash, sym::ped128),
                (Token::Dot, Token::Identifier(sym::psd2)) => parse_varied_instruction!(self, parse_unary_instruction, HashPSD2, sym::hash, sym::psd2),
                (Token::Dot, Token::Identifier(sym::psd4)) => parse_varied_instruction!(self, parse_unary_instruction, HashPSD4, sym::hash, sym::psd4),
                _ => parse_varied_instruction!(self, parse_unary_instruction, HashPSD8, sym::hash, sym::psd8),
            }
            Token::Increment => self.parse_increment_instruction().map(Instruction::Increment),
            Token::Identifier(sym::inv) => parse_standard_instruction!(self, parse_unary_instruction, Inv, sym::inv),
            Token::Identifier(sym::is) => match (next_token, following_token) {
                (Token::Dot, Token::Identifier(sym::eq)) => parse_varied_instruction!(self, parse_binary_instruction, IsEq, sym::is, sym::eq),
                _ => parse_varied_instruction!(self, parse_binary_instruction, IsNeq, sym::is, sym::neq),
            },
            Token::Identifier(sym::lt) => parse_standard_instruction!(self, parse_binary_instruction, LessThan, sym::lt),
            Token::Identifier(sym::lte) => parse_standard_instruction!(self, parse_binary_instruction, LessThanOrEqual, sym::lte),
            Token::Identifier(sym::Mod) => parse_standard_instruction!(self, parse_binary_instruction, Modulo, sym::Mod),
            Token::Identifier(sym::mul) => match next_token {
                Token::Dot => parse_varied_instruction!(self, parse_binary_instruction, MulWrapped, sym::mul, sym::w),
                _ => parse_standard_instruction!(self, parse_binary_instruction, Mul, sym::mul),
            },
            Token::Identifier(sym::nand) => parse_standard_instruction!(self, parse_binary_instruction, Nand, sym::nand),
            Token::Identifier(sym::neg) => parse_standard_instruction!(self, parse_unary_instruction, Neg, sym::neg),
            Token::Identifier(sym::not) => parse_standard_instruction!(self, parse_unary_instruction, Not, sym::not),
            Token::Identifier(sym::or) => parse_standard_instruction!(self, parse_binary_instruction, Or, sym::or),
            Token::Identifier(sym::pow) => match next_token {
                Token::Dot => parse_varied_instruction!(self, parse_binary_instruction, PowWrapped, sym::pow, sym::w),
                _ => parse_standard_instruction!(self, parse_binary_instruction, Pow, sym::pow),
            },
            Token::Identifier(sym::rem) => match next_token {
                Token::Dot => parse_varied_instruction!(self, parse_binary_instruction, RemWrapped, sym::rem, sym::w),
                _ => parse_standard_instruction!(self, parse_binary_instruction, Rem, sym::rem),
            },
            Token::Identifier(sym::shl) => match next_token {
                Token::Dot => parse_varied_instruction!(self, parse_binary_instruction, ShlWrapped, sym::shl, sym::w),
                _ => parse_standard_instruction!(self, parse_binary_instruction, Shl, sym::shl),
            },
            Token::Identifier(sym::shr) => match next_token {
                Token::Dot => parse_varied_instruction!(self, parse_binary_instruction, ShrWrapped, sym::shr, sym::w),
                _ => parse_standard_instruction!(self, parse_binary_instruction, Shr, sym::shr),
            },
            Token::Identifier(sym::square) => parse_standard_instruction!(self, parse_unary_instruction, Square, sym::square),
            Token::Identifier(sym::square_root) => parse_standard_instruction!(self, parse_unary_instruction, SquareRoot, sym::square_root),
            Token::Identifier(sym::sub) => match next_token {
                Token::Dot => parse_varied_instruction!(self, parse_binary_instruction, SubWrapped, sym::sub, sym::w),
                _ => parse_standard_instruction!(self, parse_binary_instruction, Sub, sym::sub),
            },
            Token::Identifier(sym::ternary) => self.parse_ternary_instruction().map(Instruction::Ternary),
            _ => parse_standard_instruction!(self, parse_binary_instruction, Xor, sym::xor),
        }
    }

    /// Returns a [`<Unary>`] AST node if the next tokens represent a unary instruction.
    /// Note that the exact instruction is parameterized by `N`.
    fn parse_unary_instruction<N: Unary>(&mut self, parse_opcode: impl Fn(&mut Self) -> Result<Span>) -> Result<N> {
        let start = parse_opcode(self)?;
        let source = self.parse_operand()?;
        self.expect_identifier_with_name(sym::into)?;
        let destination = self.expect_identifier()?;
        let end = self.expect(&Token::Semicolon)?;
        Ok(N::new(source, destination, start + end))
    }

    /// Returns a [`<Binary>`] AST node if the next tokens represent a binary instruction.
    /// Note that the exact instruction is parameterized by `N`.
    fn parse_binary_instruction<N: Binary>(&mut self, parse_opcode: impl Fn(&mut Self) -> Result<Span>) -> Result<N> {
        let start = parse_opcode(self)?;
        let first = self.parse_operand()?;
        let second = self.parse_operand()?;
        self.expect_identifier_with_name(sym::into)?;
        let destination = self.expect_identifier()?;
        let end = self.expect(&Token::Semicolon)?;
        Ok(N::new(first, second, destination, start + end))
    }

    /// Returns a [`Ternary`] AST node if the next tokens represent a ternary instruction.
    fn parse_ternary_instruction(&mut self) -> Result<Ternary> {
        let start = self.expect_identifier_with_name(sym::ternary)?.span;
        let condition = self.parse_operand()?;
        let first = self.parse_operand()?;
        let second = self.parse_operand()?;
        self.expect_identifier_with_name(sym::into)?;
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
        let start = self.expect_identifier_with_name(sym::call)?.span;
        let function = self.expect_identifier()?;
        let mut arguments = Vec::new();
        while !self.check_identifier_with_name(sym::into) {
            arguments.push(self.parse_operand()?);
        }
        self.expect_identifier_with_name(sym::into)?;
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
        let start = self.expect_identifier_with_name(sym::cast)?.span;
        let mut arguments = Vec::new();
        while !self.check_identifier_with_name(sym::into) {
            arguments.push(self.parse_operand()?);
        }
        self.expect_identifier_with_name(sym::into)?;
        let destination = self.expect_identifier()?;
        self.expect_identifier_with_name(sym::As)?;
        let register_type = self.parse_register_type()?;
        let end = self.expect(&Token::Semicolon)?;
        Ok(Cast {
            arguments,
            destination,
            register_type,
            span: start + end,
        })
    }

    /// Returns a [`Decrement`] AST node if the next tokens represent a decrement instruction.
    fn parse_decrement_instruction(&mut self) -> Result<Decrement> {
        let start = self.expect(&Token::Decrement)?;
        let mapping = self.expect_identifier()?;
        self.expect(&Token::LeftSquare)?;
        let index = self.parse_operand()?;
        self.expect(&Token::RightSquare)?;
        self.expect_identifier_with_name(sym::by)?;
        let amount = self.parse_operand()?;
        let end = self.expect(&Token::Semicolon)?;
        Ok(Decrement {
            mapping,
            index,
            amount,
            span: start + end,
        })
    }

    /// Returns an [`Increment`] AST node if the next tokens represent an increment instruction.
    fn parse_increment_instruction(&mut self) -> Result<Increment> {
        let start = self.expect(&Token::Increment)?;
        let mapping = self.expect_identifier()?;
        self.expect(&Token::LeftSquare)?;
        let index = self.parse_operand()?;
        self.expect(&Token::RightSquare)?;
        self.expect_identifier_with_name(sym::by)?;
        let amount = self.parse_operand()?;
        let end = self.expect(&Token::Semicolon)?;
        Ok(Increment {
            mapping,
            index,
            amount,
            span: start + end,
        })
    }

    /// Returns an [`Operand`] AST node if the next tokens represent an instruction operand.
    fn parse_operand(&mut self) -> Result<Operand> {
        match &self.token.token {
            // Parse `self.caller`.
            Token::SelfLower => {
                let start = self.expect(&Token::SelfLower)?;
                self.expect(&Token::Dot)?;
                let name = self.expect_identifier_with_name(sym::caller)?;
                Ok(Operand::Caller(MemberAccess {
                    inner: Box::new(Expression::Identifier(Identifier {
                        name: sym::SelfLower,
                        span: start,
                    })),
                    name,
                    span: start + name.span
                }))
            }
            _ => {
                match self.parse_expression()? {
                    Expression::Literal(literal) => Ok(Operand::Literal(literal)),
                    Expression::Identifier(identifier) => Ok(Operand::Register(Expression::Identifier(identifier))),
                    Expression::Access(AccessExpression::Member(member)) => {
                        match (member.inner.as_ref(), member.name) {
                            (Expression::Identifier(Identifier { name: sym::SelfLower, .. }), Identifier { name: sym::caller, .. }) => {
                                Ok(Operand::Caller(member))
                            }
                            (Expression::Identifier(identifier), Identifier { name: sym::aleo, .. }) => Ok(Operand::ProgramID(ProgramId {
                                name: identifier.clone(),
                                network: member.name,
                                span: member.span,
                            })),
                            _ => Ok(Operand::Register(Expression::Access(AccessExpression::Member(member))))
                        }
                    }
                    expression => Err(ParserError::unexpected(&expression, "valid operand", expression.span()).into())
                }
            }
        }
    }

    /// Returns a [`RegisterType`] AST node if the next tokens represent a register type.
    fn parse_register_type(&mut self) -> Result<RegisterType> {
        let first = &self.token.token;
        let second = self.look_ahead(1, |t| &t.token);
        let third = self.look_ahead(2, |t| &t.token);
        match (first, second, third) {
            (Token::Identifier(_), Token::Dot, Token::Identifier(sym::record)) => {
                let record_name = self.expect_identifier()?;
                self.expect(&Token::Dot)?;
                let end = self.expect_identifier_with_name(sym::record)?.span;
                Ok(RegisterType::Record(RecordType {
                    name: record_name,
                    span: record_name.span + end,
                }))
            }
            (Token::Identifier(_), Token::Dot, Token::Identifier(sym::aleo)) => {
                let program_id = self.parse_program_id()?;
                self.expect(&Token::Div)?;
                let record_name = self.expect_identifier()?;
                self.expect(&Token::Dot)?;
                let end = self.expect_identifier_with_name(sym::record)?.span;
                Ok(RegisterType::ExternalRecord(ExternalRecordType {
                    program_id,
                    record_type: RecordType {
                        name: record_name,
                        span: record_name.span + end,
                    },
                    span: program_id.span + end,
                }))
            }
            _ => {
                let (type_, span) = self.parse_type()?;
                Ok(RegisterType::PlaintextType(PlaintextType { type_, span }))
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
            SpannedToken { token: Token::Identifier(symbol), .. } => *symbol == name,
            _ => false,
        }
    }
}
