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
    Token::AddEq,
    Token::MinusEq,
    Token::MulEq,
    Token::DivEq,
    Token::ExpEq,
];

impl ParserContext<'_> {
    ///
    /// Returns an [`Identifier`] AST node if the given [`Expression`] AST node evaluates to an
    /// identifier access. The access is stored in the given accesses.
    ///
    pub fn construct_assignee_access(expr: Expression, accesses: &mut Vec<AssigneeAccess>) -> Result<Identifier> {
        let identifier;
        match expr {
            Expression::Access(access) => match access {
                AccessExpression::Member(expr) => {
                    identifier = Self::construct_assignee_access(*expr.inner, accesses)?;
                    accesses.push(AssigneeAccess::Member(expr.name));
                }
                AccessExpression::Tuple(expr) => {
                    identifier = Self::construct_assignee_access(*expr.tuple, accesses)?;
                    accesses.push(AssigneeAccess::Tuple(expr.index, expr.span));
                }
                AccessExpression::ArrayRange(expr) => {
                    identifier = Self::construct_assignee_access(*expr.array, accesses)?;
                    accesses.push(AssigneeAccess::ArrayRange(
                        expr.left.map(|x| *x),
                        expr.right.map(|x| *x),
                    ));
                }
                AccessExpression::Array(expr) => {
                    identifier = Self::construct_assignee_access(*expr.array, accesses)?;
                    accesses.push(AssigneeAccess::ArrayIndex(*expr.index));
                }
                _ => return Err(ParserError::invalid_assignment_target(access.span()).into()),
            },

            Expression::Identifier(id) => identifier = id,
            _ => return Err(ParserError::invalid_assignment_target(expr.span()).into()),
        }
        Ok(identifier)
    }

    ///
    /// Returns an [`Assignee`] AST node from the given [`Expression`] AST node with accesses.
    ///
    pub fn construct_assignee(expr: Expression) -> Result<Assignee> {
        let expr_span = expr.span().clone();
        let mut accesses = Vec::new();
        let identifier = Self::construct_assignee_access(expr, &mut accesses)?;

        Ok(Assignee {
            span: expr_span,
            identifier,
            accesses,
        })
    }

    ///
    /// Returns a [`Statement`] AST node if the next tokens represent a statement.
    ///
    pub fn parse_statement(&mut self) -> Result<Statement> {
        match &self.peek()?.token {
            Token::Return => Ok(Statement::Return(self.parse_return_statement()?)),
            Token::If => Ok(Statement::Conditional(self.parse_conditional_statement()?)),
            Token::For => Ok(Statement::Iteration(Box::new(self.parse_loop_statement()?))),
            Token::Console => Ok(Statement::Console(self.parse_console_statement()?)),
            Token::Let | Token::Const => Ok(Statement::Definition(self.parse_definition_statement()?)),
            Token::LeftCurly => Ok(Statement::Block(self.parse_block()?)),
            _ => Ok(self.parse_assign_statement()?),
        }
    }

    ///
    /// Returns a [`Block`] AST node if the next tokens represent a assign, or expression statement.
    ///
    pub fn parse_assign_statement(&mut self) -> Result<Statement> {
        let expr = self.parse_expression()?;

        if let Some(operator) = self.eat_any(ASSIGN_TOKENS) {
            let value = self.parse_expression()?;
            let assignee = Self::construct_assignee(expr)?;
            self.expect(Token::Semicolon)?;
            Ok(Statement::Assign(Box::new(AssignStatement {
                span: &assignee.span + value.span(),
                assignee,
                operation: match operator.token {
                    Token::Assign => AssignOperation::Assign,
                    Token::AddEq => AssignOperation::Add,
                    Token::MinusEq => AssignOperation::Sub,
                    Token::MulEq => AssignOperation::Mul,
                    Token::DivEq => AssignOperation::Div,
                    Token::ExpEq => AssignOperation::Pow,
                    _ => unreachable!("parse_assign_statement_ shouldn't produce this"),
                },
                value,
            })))
        } else {
            self.expect(Token::Semicolon)?;
            Ok(Statement::Expression(ExpressionStatement {
                span: expr.span().clone(),
                expression: expr,
            }))
        }
    }

    ///
    /// Returns a [`Block`] AST node if the next tokens represent a block of statements.
    ///
    pub fn parse_block(&mut self) -> Result<Block> {
        let start = self.expect(Token::LeftCurly)?;

        let mut statements = Vec::new();
        loop {
            match self.eat(Token::RightCurly) {
                None => {
                    statements.push(self.parse_statement()?);
                }
                Some(end) => {
                    return Ok(Block {
                        span: start + end.span,
                        statements,
                    });
                }
            }
        }
    }

    ///
    /// Returns a [`ReturnStatement`] AST node if the next tokens represent a return statement.
    ///
    pub fn parse_return_statement(&mut self) -> Result<ReturnStatement> {
        let start = self.expect(Token::Return)?;
        let expr = self.parse_expression()?;
        self.expect(Token::Semicolon)?;

        Ok(ReturnStatement {
            span: &start + expr.span(),
            expression: expr,
        })
    }

    /// Returns a [`ConditionalStatement`] AST node if the next tokens represent a conditional statement.
    pub fn parse_conditional_statement(&mut self) -> Result<ConditionalStatement> {
        let start = self.expect(Token::If)?;
        self.fuzzy_struct_state = true;
        let expr = self.parse_conditional_expression()?;
        self.fuzzy_struct_state = false;
        let body = self.parse_block()?;
        let next = if self.eat(Token::Else).is_some() {
            let s = self.parse_statement()?;
            if !matches!(s, Statement::Block(_) | Statement::Conditional(_)) {
                self.emit_err(ParserError::unexpected_statement(&s, "Block or Conditional", s.span()));
            }
            Some(Box::new(s))
        } else {
            None
        };

        Ok(ConditionalStatement {
            span: &start + next.as_ref().map(|x| x.span()).unwrap_or(&body.span),
            condition: expr,
            block: body,
            next,
        })
    }

    /// Returns an [`IterationStatement`] AST node if the next tokens represent an iteration statement.
    pub fn parse_loop_statement(&mut self) -> Result<IterationStatement> {
        let start_span = self.expect(Token::For)?;
        let ident = self.expect_ident()?;
        self.expect(Token::In)?;

        // Parse iteration range.
        let start = self.parse_expression()?;
        self.expect(Token::DotDot)?;
        let inclusive = self.eat(Token::Assign).is_some();
        self.fuzzy_struct_state = true;
        let stop = self.parse_conditional_expression()?;
        self.fuzzy_struct_state = false;

        let block = self.parse_block()?;

        Ok(IterationStatement {
            span: start_span + block.span.clone(),
            variable: ident,
            start,
            stop,
            inclusive,
            block,
        })
    }

    /// Returns a [`ConsoleArgs`] AST node if the next tokens represent a formatted string.
    pub fn parse_console_args(&mut self) -> Result<ConsoleArgs> {
        let mut string = None;
        let (parameters, _, span) = self.parse_paren_comma_list(|p| {
            if string.is_none() {
                let SpannedToken { token, span } = p.expect_any()?;
                string = Some(match token {
                    Token::StringLit(chars) => chars,
                    _ => {
                        p.emit_err(ParserError::unexpected_str(token, "formatted string", &span));
                        Vec::new()
                    }
                });
                Ok(None)
            } else {
                p.parse_expression().map(Some)
            }
        })?;

        Ok(ConsoleArgs {
            string: string.unwrap_or_default(),
            span,
            parameters,
        })
    }

    /// Returns a [`ConsoleStatement`] AST node if the next tokens represent a console statement.
    pub fn parse_console_statement(&mut self) -> Result<ConsoleStatement> {
        let keyword = self.expect(Token::Console)?;
        self.expect(Token::Dot)?;
        let function = self.expect_ident()?;
        let function = match function.name {
            sym::assert => {
                self.expect(Token::LeftParen)?;
                let expr = self.parse_expression()?;
                self.expect(Token::RightParen)?;
                ConsoleFunction::Assert(expr)
            }
            sym::error => ConsoleFunction::Error(self.parse_console_args()?),
            sym::log => ConsoleFunction::Log(self.parse_console_args()?),
            x => {
                // Not sure what it is, assume it's `log`.
                self.emit_err(ParserError::unexpected_ident(
                    x,
                    &["assert", "error", "log"],
                    &function.span,
                ));
                ConsoleFunction::Log(self.parse_console_args()?)
            }
        };
        self.expect(Token::Semicolon)?;

        Ok(ConsoleStatement {
            span: &keyword + function.span(),
            function,
        })
    }

    /// Returns a [`VariableName`] AST node if the next tokens represent a variable name with
    /// valid keywords.
    pub fn parse_variable_name(&mut self, span: &SpannedToken) -> Result<VariableName> {
        let mutable = self.eat(Token::Mut);

        if let Some(mutable) = &mutable {
            self.emit_err(ParserError::let_mut_statement(&(&mutable.span + &span.span)));
        }

        let name = self.expect_ident()?;
        Ok(VariableName {
            span: name.span.clone(),
            mutable: matches!(span.token, Token::Let),
            identifier: name,
        })
    }

    /// Returns a [`DefinitionStatement`] AST node if the next tokens represent a definition statement.
    pub fn parse_definition_statement(&mut self) -> Result<DefinitionStatement> {
        let declare = self.expect_oneof(&[Token::Let, Token::Const])?;

        // Parse variable names.
        let (variable_names, parened) = if self.peek_is_left_par() {
            (
                self.parse_paren_comma_list(|p| p.parse_variable_name(&declare).map(Some))
                    .map(|(vars, ..)| vars)?,
                true,
            )
        } else {
            (vec![self.parse_variable_name(&declare)?], false)
        };

        // Parse an optional type ascription.
        let type_ = self
            .eat(Token::Colon)
            .map(|_| self.parse_type().map(|t| t.0))
            .transpose()?;

        self.expect(Token::Assign)?;
        let expr = self.parse_expression()?;
        self.expect(Token::Semicolon)?;

        Ok(DefinitionStatement {
            span: &declare.span + expr.span(),
            declaration_type: match declare.token {
                Token::Let => Declare::Let,
                Token::Const => Declare::Const,
                _ => unreachable!("parse_definition_statement_ shouldn't produce this"),
            },
            variable_names,
            parened,
            type_,
            value: expr,
        })
    }
}
