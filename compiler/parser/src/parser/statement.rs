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

use leo_errors::{ParserError, ParserWarning, Result};
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
            Token::If => Ok(Statement::Conditional(self.parse_conditional_statement()?)),
            Token::For => Ok(Statement::Iteration(Box::new(self.parse_loop_statement()?))),
            Token::Assert | Token::AssertEq | Token::AssertNeq => Ok(self.parse_assert_statement()?),
            Token::Let => Ok(Statement::Definition(self.parse_definition_statement()?)),
            Token::Const => Ok(Statement::Const(self.parse_const_declaration_statement()?)),
            Token::LeftCurly => Ok(Statement::Block(self.parse_block()?)),
            Token::Console => Err(ParserError::console_statements_are_not_yet_supported(self.token.span).into()),
            Token::Finalize => Err(ParserError::finalize_statements_are_deprecated(self.token.span).into()),
            _ => Ok(self.parse_assign_statement()?),
        }
    }

    /// Returns a [`AssertStatement`] AST node if the next tokens represent an assertion statement.
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
        Ok(Statement::Assert(AssertStatement { variant, span, id: self.node_builder.next_id() }))
    }

    /// Returns a [`AssignStatement`] AST node if the next tokens represent a assign, otherwise expects an expression statement.
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

            // Construct a copy of the lhs with a unique id.
            let mut left = place.clone();
            left.set_id(self.node_builder.next_id());

            // Simplify complex assignments into simple assignments.
            // For example, `x += 1` becomes `x = x + 1`, while simple assignments like `x = y` remain unchanged.
            let value = match operation {
                None => value,
                Some(op) => Expression::Binary(BinaryExpression {
                    left: Box::new(left),
                    right: Box::new(value),
                    op,
                    span,
                    id: self.node_builder.next_id(),
                }),
            };

            Ok(Statement::Assign(Box::new(AssignStatement { span, place, value, id: self.node_builder.next_id() })))
        } else {
            // Check for `increment` and `decrement` statements. If found, emit a deprecation warning.
            if let Expression::Call(call_expression) = &place {
                match *call_expression.function {
                    Expression::Identifier(Identifier { name: sym::decrement, .. }) => {
                        self.emit_warning(ParserWarning::deprecated(
                            "decrement",
                            "Use `Mapping::{get, get_or_use, set, remove, contains}` for manipulating on-chain mappings.",
                            place.span(),
                        ));
                    }
                    Expression::Identifier(Identifier { name: sym::increment, .. }) => {
                        self.emit_warning(ParserWarning::deprecated(
                            "increment",
                            "Use `Mapping::{get, get_or_use, set, remove, contains}` for manipulating on-chain mappings.",
                            place.span(),
                        ));
                    }
                    _ => (),
                }
            }

            // Parse the expression as a statement.
            let end = self.expect(&Token::Semicolon)?;
            Ok(Statement::Expression(ExpressionStatement {
                span: place.span() + end,
                expression: place,
                id: self.node_builder.next_id(),
            }))
        }
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
            Token::Semicolon | Token::Then => {
                Expression::Unit(UnitExpression { span: self.token.span, id: self.node_builder.next_id() })
            }
            // Otherwise, attempt to parse an expression.
            _ => self.parse_expression()?,
        };

        let finalize_args = match self.token.token {
            Token::Then => {
                // Parse `then`.
                self.expect(&Token::Then)?;
                // Parse `finalize`.
                self.expect(&Token::Finalize)?;
                // Parse finalize arguments if they exist.
                match self.token.token {
                    Token::Semicolon => Some(vec![]),
                    Token::LeftParen => Some(self.parse_paren_comma_list(|p| p.parse_expression().map(Some))?.0),
                    _ => Some(vec![self.parse_expression()?]),
                }
            }
            _ => None,
        };
        let end = self.expect(&Token::Semicolon)?;
        let span = start + end;
        Ok(ReturnStatement { span, expression, finalize_arguments: finalize_args, id: self.node_builder.next_id() })
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
            id: self.node_builder.next_id(),
        })
    }

    /// Returns a [`ConsoleStatement`] AST node if the next tokens represent a console statement.
    #[allow(dead_code)]
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
                        id: self.node_builder.next_id(),
                    })),
                )
            }
        };
        self.expect(&Token::Semicolon)?;

        Ok(ConsoleStatement { span: keyword + span, function, id: self.node_builder.next_id() })
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

    /// Returns a [`DefinitionStatement`] AST node if the next tokens represent a definition statement.
    pub(super) fn parse_definition_statement(&mut self) -> Result<DefinitionStatement> {
        self.expect(&Token::Let)?;
        let decl_span = self.prev_token.span;
        let decl_type = match &self.prev_token.token {
            Token::Let => DeclarationType::Let,
            // Note: Reserving for `constant` declarations.
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
            id: self.node_builder.next_id(),
        })
    }
}
