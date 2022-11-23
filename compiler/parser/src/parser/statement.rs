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

use super::*;

use leo_errors::{ParserError, Result};
use leo_span::sym;

const ASSIGN_TOKENS: &[Token] = &[
    Token::Assign,
    Token::AddAssign,
    Token::SubAssign,
    Token::MulAssign,
    Token::DivAssign,
    Token::RemAssign,
    Token::PowAssign,
    Token::OrAssign,
    Token::AndAssign,
    Token::BitAndAssign,
    Token::BitOrAssign,
    Token::ShrAssign,
    Token::ShlAssign,
    Token::BitXorAssign,
];

impl ParserContext<'_> {
    /// Returns a [`Statement`] AST node if the next tokens represent a statement.
    pub(crate) fn parse_statement(&mut self) -> Result<Statement> {
        match &self.token.token {
            Token::Return => Ok(Statement::Return(self.parse_return_statement()?)),
            Token::Async => Ok(Statement::Finalize(self.parse_finalize_statement()?)),
            // If a finalize token is found without a preceding async token, return an error.
            Token::Finalize => Err(ParserError::finalize_without_async(self.token.span).into()),
            Token::Increment => Ok(Statement::Increment(self.parse_increment_statement()?)),
            Token::Decrement => Ok(Statement::Decrement(self.parse_decrement_statement()?)),
            Token::If => Ok(Statement::Conditional(self.parse_conditional_statement()?)),
            Token::For => Ok(Statement::Iteration(Box::new(self.parse_loop_statement()?))),
            Token::Console => Ok(Statement::Console(self.parse_console_statement()?)),
            Token::Let | Token::Const => Ok(Statement::Definition(self.parse_definition_statement()?)),
            Token::LeftCurly => Ok(Statement::Block(self.parse_block()?)),
            _ => Ok(self.parse_assign_statement()?),
        }
    }

    /// Returns a [`Block`] AST node if the next tokens represent a assign, or expression statement.
    fn parse_assign_statement(&mut self) -> Result<Statement> {
        let place = self.parse_expression()?;

        if self.eat_any(ASSIGN_TOKENS) {
            // Determine the corresponding binary operation for each token, if it exists.
            let operation = match &self.prev_token.token {
                Token::Assign => None,
                Token::AddAssign => Some(BinaryOperation::Add),
                Token::SubAssign => Some(BinaryOperation::Sub),
                Token::MulAssign => Some(BinaryOperation::Mul),
                Token::DivAssign => Some(BinaryOperation::Div),
                Token::RemAssign => Some(BinaryOperation::Rem),
                Token::PowAssign => Some(BinaryOperation::Pow),
                Token::OrAssign => Some(BinaryOperation::Or),
                Token::AndAssign => Some(BinaryOperation::And),
                Token::BitAndAssign => Some(BinaryOperation::BitwiseAnd),
                Token::BitOrAssign => Some(BinaryOperation::BitwiseOr),
                Token::BitXorAssign => Some(BinaryOperation::Xor),
                Token::ShrAssign => Some(BinaryOperation::Shr),
                Token::ShlAssign => Some(BinaryOperation::Shl),
                _ => unreachable!("`parse_assign_statement` shouldn't produce this"),
            };

            let value = self.parse_expression()?;
            self.expect(&Token::Semicolon)?;

            // Construct the span for the statement.
            let span = place.span() + value.span();

            // Simplify complex assignments into simple assignments.
            // For example, `x += 1` becomes `x = x + 1`, while simple assignments like `x = y` remain unchanged.
            let value = match operation {
                None => value,
                Some(op) => Expression::Binary(BinaryExpression {
                    left: Box::new(place.clone()),
                    right: Box::new(value),
                    op,
                    span,
                }),
            };

            Ok(Statement::Assign(Box::new(AssignStatement { span, place, value })))
        } else {
            // Parse the expression as a statement.
            let end = self.expect(&Token::Semicolon)?;
            Ok(Statement::Expression(ExpressionStatement {
                span: place.span() + end,
                expression: place,
            }))
        }
    }

    /// Returns a [`Block`] AST node if the next tokens represent a block of statements.
    pub(super) fn parse_block(&mut self) -> Result<Block> {
        self.parse_list(Delimiter::Brace, None, |p| p.parse_statement().map(Some))
            .map(|(statements, _, span)| Block { statements, span })
    }

    /// Returns a [`ReturnStatement`] AST node if the next tokens represent a return statement.
    fn parse_return_statement(&mut self) -> Result<ReturnStatement> {
        let start = self.expect(&Token::Return)?;
        let expression = match self.token.token {
            // If the next token is a semicolon, implicitly return a unit expression, `()`.
            Token::Semicolon => Expression::Unit(UnitExpression { span: self.token.span }),
            // Otherwise, attempt to parse an expression.
            _ => self.parse_expression()?,
        };
        self.expect(&Token::Semicolon)?;
        let span = start + expression.span();
        Ok(ReturnStatement { span, expression })
    }

    /// Returns a [`FinalizeStatement`] AST node if the next tokens represent a finalize statement.
    fn parse_finalize_statement(&mut self) -> Result<FinalizeStatement> {
        self.expect(&Token::Async)?;
        let start = self.expect(&Token::Finalize)?;
        let (arguments, _, span) = self.parse_paren_comma_list(|p| p.parse_expression().map(Some))?;
        self.expect(&Token::Semicolon)?;
        let span = start + span;
        Ok(FinalizeStatement { span, arguments })
    }

    /// Returns a [`DecrementStatement`] AST node if the next tokens represent a decrement statement.
    fn parse_decrement_statement(&mut self) -> Result<DecrementStatement> {
        let start = self.expect(&Token::Decrement)?;
        self.expect(&Token::LeftParen)?;
        let mapping = self.expect_identifier()?;
        self.expect(&Token::Comma)?;
        let index = self.parse_expression()?;
        self.expect(&Token::Comma)?;
        let amount = self.parse_expression()?;
        self.eat(&Token::Comma);
        let end = self.expect(&Token::RightParen)?;
        self.expect(&Token::Semicolon)?;
        let span = start + end;
        Ok(DecrementStatement {
            mapping,
            index,
            amount,
            span,
        })
    }

    /// Returns an [`IncrementStatement`] AST node if the next tokens represent an increment statement.
    fn parse_increment_statement(&mut self) -> Result<IncrementStatement> {
        let start = self.expect(&Token::Increment)?;
        self.expect(&Token::LeftParen)?;
        let mapping = self.expect_identifier()?;
        self.expect(&Token::Comma)?;
        let index = self.parse_expression()?;
        self.expect(&Token::Comma)?;
        let amount = self.parse_expression()?;
        self.eat(&Token::Comma);
        let end = self.expect(&Token::RightParen)?;
        self.expect(&Token::Semicolon)?;
        let span = start + end;
        Ok(IncrementStatement {
            mapping,
            index,
            amount,
            span,
        })
    }

    /// Returns a [`ConditionalStatement`] AST node if the next tokens represent a conditional statement.
    fn parse_conditional_statement(&mut self) -> Result<ConditionalStatement> {
        let start = self.expect(&Token::If)?;
        self.disallow_struct_construction = true;
        let expr = self.parse_conditional_expression()?;
        self.disallow_struct_construction = false;
        let body = self.parse_block()?;
        let next = if self.eat(&Token::Else) {
            let s = self.parse_statement()?;
            if !matches!(s, Statement::Block(_) | Statement::Conditional(_)) {
                self.emit_err(ParserError::unexpected_statement(&s, "Block or Conditional", s.span()));
            }
            Some(Box::new(s))
        } else {
            None
        };

        Ok(ConditionalStatement {
            span: start + next.as_ref().map(|x| x.span()).unwrap_or(body.span),
            condition: expr,
            then: body,
            otherwise: next,
        })
    }

    /// Returns an [`IterationStatement`] AST node if the next tokens represent an iteration statement.
    fn parse_loop_statement(&mut self) -> Result<IterationStatement> {
        let start_span = self.expect(&Token::For)?;
        let ident = self.expect_identifier()?;
        self.expect(&Token::Colon)?;
        let type_ = self.parse_type()?;
        self.expect(&Token::In)?;

        // Parse iteration range.
        let start = self.parse_expression()?;
        self.expect(&Token::DotDot)?;
        self.disallow_struct_construction = true;
        let stop = self.parse_conditional_expression()?;
        self.disallow_struct_construction = false;

        let block = self.parse_block()?;

        Ok(IterationStatement {
            span: start_span + block.span,
            variable: ident,
            type_: type_.0,
            start,
            start_value: Default::default(),
            stop,
            stop_value: Default::default(),
            inclusive: false,
            block,
        })
    }

    /// Returns a [`ConsoleStatement`] AST node if the next tokens represent a console statement.
    fn parse_console_statement(&mut self) -> Result<ConsoleStatement> {
        let keyword = self.expect(&Token::Console)?;
        self.expect(&Token::Dot)?;
        let identifier = self.expect_identifier()?;
        let (span, function) = match identifier.name {
            sym::assert => {
                self.expect(&Token::LeftParen)?;
                let expr = self.parse_expression()?;
                self.expect(&Token::RightParen)?;
                (keyword + expr.span(), ConsoleFunction::Assert(expr))
            }
            sym::assert_eq => {
                self.expect(&Token::LeftParen)?;
                let left = self.parse_expression()?;
                self.expect(&Token::Comma)?;
                let right = self.parse_expression()?;
                self.expect(&Token::RightParen)?;
                (left.span() + right.span(), ConsoleFunction::AssertEq(left, right))
            }
            sym::assert_neq => {
                self.expect(&Token::LeftParen)?;
                let left = self.parse_expression()?;
                self.expect(&Token::Comma)?;
                let right = self.parse_expression()?;
                self.expect(&Token::RightParen)?;
                (left.span() + right.span(), ConsoleFunction::AssertNeq(left, right))
            }
            symbol => {
                // Not sure what it is, assume it's `log`.
                self.emit_err(ParserError::unexpected_ident(
                    symbol,
                    &["assert", "assert_eq", "assert_neq"],
                    identifier.span,
                ));
                (
                    Default::default(),
                    ConsoleFunction::Assert(Expression::Err(ErrExpression {
                        span: Default::default(),
                    })),
                )
            }
        };
        self.expect(&Token::Semicolon)?;

        Ok(ConsoleStatement {
            span: keyword + span,
            function,
        })
    }

    /// Returns a [`DefinitionStatement`] AST node if the next tokens represent a definition statement.
    pub(super) fn parse_definition_statement(&mut self) -> Result<DefinitionStatement> {
        self.expect_any(&[Token::Let, Token::Const])?;
        let decl_span = self.prev_token.span;
        let decl_type = match &self.prev_token.token {
            Token::Let => DeclarationType::Let,
            Token::Const => DeclarationType::Const,
            _ => unreachable!("parse_definition_statement_ shouldn't produce this"),
        };

        // Parse variable name and type.
        let place = self.parse_expression()?;
        self.expect(&Token::Colon)?;
        let type_ = self.parse_type()?.0;

        self.expect(&Token::Assign)?;
        let value = self.parse_expression()?;
        self.expect(&Token::Semicolon)?;

        Ok(DefinitionStatement {
            span: decl_span + value.span(),
            declaration_type: decl_type,
            place,
            type_,
            value,
        })
    }
}
