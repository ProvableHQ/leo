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

use super::*;

use leo_errors::{ParserError, Result};

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
    Token::BitXorAssign,
    Token::ShrAssign,
    Token::ShlAssign,
];

impl<N: Network> ParserContext<'_, N> {
    /// Returns a [`Statement`] AST node if the next tokens represent a statement.
    pub(crate) fn parse_statement(&mut self) -> Result<Statement> {
        match &self.token.token {
            Token::Return => Ok(self.parse_return_statement()?.into()),
            Token::If => Ok(self.parse_conditional_statement()?.into()),
            Token::For => Ok(self.parse_loop_statement()?.into()),
            Token::Assert | Token::AssertEq | Token::AssertNeq => Ok(self.parse_assert_statement()?),
            Token::Let => Ok(self.parse_definition_statement()?.into()),
            Token::Const => Ok(self.parse_const_declaration_statement()?.into()),
            Token::LeftCurly => Ok(self.parse_block()?.into()),
            _ => Ok(self.parse_assign_statement()?),
        }
    }

    /// Returns an [`AssertStatement`] AST node if the next tokens represent an assertion statement.
    fn parse_assert_statement(&mut self) -> Result<Statement> {
        // Check which variant of the assert statement is being used.
        // Note that `parse_assert_statement` is called only if the next token is an assertion token.
        let is_assert = self.check(&Token::Assert);
        let is_assert_eq = self.check(&Token::AssertEq);
        let is_assert_neq = self.check(&Token::AssertNeq);
        // Parse the span of the assertion statement.
        let span = self.expect_any(&[Token::Assert, Token::AssertEq, Token::AssertNeq])?;
        // Parse the left parenthesis token.
        self.expect(&Token::LeftParen)?;
        // Parse the variant.
        let variant = match (is_assert, is_assert_eq, is_assert_neq) {
            (true, false, false) => AssertVariant::Assert(self.parse_expression()?),
            (false, true, false) => AssertVariant::AssertEq(self.parse_expression()?, {
                self.expect(&Token::Comma)?;
                self.parse_expression()?
            }),
            (false, false, true) => AssertVariant::AssertNeq(self.parse_expression()?, {
                self.expect(&Token::Comma)?;
                self.parse_expression()?
            }),
            _ => unreachable!("The call the `expect_any` ensures that only one of the three tokens is true."),
        };
        // Parse the right parenthesis token.
        self.expect(&Token::RightParen)?;
        // Parse the semicolon token.
        self.expect(&Token::Semicolon)?;

        // Return the assertion statement.
        Ok(AssertStatement { variant, span, id: self.node_builder.next_id() }.into())
    }

    /// Returns an [`AssignStatement`] AST node if the next tokens represent an assignment, otherwise expects an expression statement.
    fn parse_assign_statement(&mut self) -> Result<Statement> {
        let expression = self.parse_expression()?;
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
                _ => panic!("`parse_assign_statement` shouldn't produce this"),
            };

            let value = self.parse_expression()?;
            self.expect(&Token::Semicolon)?;

            // Construct the span for the statement.
            let span = expression.span() + value.span();

            // Construct a copy of the lhs with a unique id.
            let mut left = expression.clone();
            left.set_id(self.node_builder.next_id());

            // Simplify complex assignments into simple assignments.
            // For example, `x += 1` becomes `x = x + 1`, while simple assignments like `x = y` remain unchanged.
            let value = match operation {
                None => value,
                Some(op) => BinaryExpression { left, right: value, op, span, id: self.node_builder.next_id() }.into(),
            };

            return Ok(AssignStatement { span, place: expression, value, id: self.node_builder.next_id() }.into());
        }

        let end = self.expect(&Token::Semicolon)?;

        Ok(ExpressionStatement { span: expression.span() + end, expression, id: self.node_builder.next_id() }.into())
    }

    /// Returns a [`Block`] AST node if the next tokens represent a block of statements.
    pub(super) fn parse_block(&mut self) -> Result<Block> {
        self.parse_list(Delimiter::Brace, None, |p| p.parse_statement().map(Some)).map(|(statements, _, span)| Block {
            statements,
            span,
            id: self.node_builder.next_id(),
        })
    }

    /// Returns a [`ReturnStatement`] AST node if the next tokens represent a return statement.
    fn parse_return_statement(&mut self) -> Result<ReturnStatement> {
        let start = self.expect(&Token::Return)?;

        let expression = match self.token.token {
            // If the next token is a semicolon, implicitly return a unit expression, `()`.
            Token::Semicolon => {
                Expression::Unit(UnitExpression { span: self.token.span, id: self.node_builder.next_id() })
            }
            // Otherwise, attempt to parse an expression.
            _ => self.parse_expression()?,
        };
        let end = self.expect(&Token::Semicolon)?;
        let span = start + end;
        Ok(ReturnStatement { span, expression, id: self.node_builder.next_id() })
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
            id: self.node_builder.next_id(),
        })
    }

    /// Returns an [`IterationStatement`] AST node if the next tokens represent an iteration statement.
    fn parse_loop_statement(&mut self) -> Result<IterationStatement> {
        let start_span = self.expect(&Token::For)?;

        // Parse the iterator name
        let ident = self.expect_identifier()?;

        // The type annotation is optional
        let type_ = if self.eat(&Token::Colon) { Some(self.parse_type()?.0) } else { None };
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
            type_,
            start,
            stop,
            inclusive: false,
            block,
            id: self.node_builder.next_id(),
        })
    }

    /// Returns a [`ConstDeclaration`] AST node if the next tokens represent a const declaration statement.
    pub(super) fn parse_const_declaration_statement(&mut self) -> Result<ConstDeclaration> {
        self.expect(&Token::Const)?;
        let decl_span = self.prev_token.span;

        // Parse variable name and type.
        let (place, type_, _) = self.parse_typed_ident()?;

        self.expect(&Token::Assign)?;
        let value = self.parse_expression()?;
        self.expect(&Token::Semicolon)?;

        Ok(ConstDeclaration { span: decl_span + value.span(), place, type_, value, id: self.node_builder.next_id() })
    }

    fn parse_definition_place(&mut self) -> Result<DefinitionPlace> {
        if let Some(identifier) = self.eat_identifier() {
            return Ok(DefinitionPlace::Single(identifier));
        }

        let (identifiers, _, _) = self.parse_paren_comma_list(|p| {
            let span = p.token.span;

            let eaten = p.eat_identifier();

            if eaten.is_some() { Ok(eaten) } else { Err(ParserError::expected_identifier(span).into()) }
        })?;

        Ok(DefinitionPlace::Multiple(identifiers))
    }

    /// Returns a [`DefinitionStatement`] AST node if the next tokens represent a definition statement.
    pub(super) fn parse_definition_statement(&mut self) -> Result<DefinitionStatement> {
        self.expect(&Token::Let)?;
        let decl_span = self.prev_token.span;

        // Parse definition place which can either be an identifier or a group of identifiers.
        let place = self.parse_definition_place()?;

        // The type annotation is optional
        let type_ = if self.eat(&Token::Colon) { Some(self.parse_type()?.0) } else { None };

        self.expect(&Token::Assign)?;
        let value = self.parse_expression()?;
        self.expect(&Token::Semicolon)?;

        Ok(DefinitionStatement { span: decl_span + value.span(), place, type_, value, id: self.node_builder.next_id() })
    }
}
