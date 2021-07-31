// Copyright (C) 2019-2021 Aleo Systems Inc.
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

use leo_errors::{LeoError, ParserError};

const ASSIGN_TOKENS: &[Token] = &[
    Token::Assign,
    Token::AddEq,
    Token::MinusEq,
    Token::MulEq,
    Token::DivEq,
    Token::ExpEq,
    // Token::BitAndEq,
    // Token::BitOrEq,
    // Token::BitXorEq,
    // Token::ShlEq,
    // Token::ShrEq,
    // Token::ShrSignedEq,
    // Token::ModEq,
    // Token::OrEq,
    // Token::AndEq,
];

impl ParserContext {
    ///
    /// Returns an [`Identifier`] AST node if the given [`Expression`] AST node evaluates to an
    /// identifier access. The access is stored in the given accesses.
    ///
    pub fn construct_assignee_access(expr: Expression, accesses: &mut Vec<AssigneeAccess>) -> SyntaxResult<Identifier> {
        let identifier;
        match expr {
            Expression::CircuitMemberAccess(expr) => {
                identifier = Self::construct_assignee_access(*expr.circuit, accesses)?;
                accesses.push(AssigneeAccess::Member(expr.name));
            }
            Expression::TupleAccess(expr) => {
                identifier = Self::construct_assignee_access(*expr.tuple, accesses)?;
                accesses.push(AssigneeAccess::Tuple(expr.index, expr.span));
            }
            Expression::ArrayRangeAccess(expr) => {
                identifier = Self::construct_assignee_access(*expr.array, accesses)?;
                accesses.push(AssigneeAccess::ArrayRange(
                    expr.left.map(|x| *x),
                    expr.right.map(|x| *x),
                ));
            }
            Expression::ArrayAccess(expr) => {
                identifier = Self::construct_assignee_access(*expr.array, accesses)?;
                accesses.push(AssigneeAccess::ArrayIndex(*expr.index));
            }
            Expression::Identifier(id) => identifier = id,
            _ => return Err(LeoError::from(ParserError::invalid_assignment_target(expr.span()))),
        }
        Ok(identifier)
    }

    ///
    /// Returns an [`Assignee`] AST node from the given [`Expression`] AST node with accesses.
    ///
    pub fn construct_assignee(expr: Expression) -> SyntaxResult<Assignee> {
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
    pub fn parse_statement(&mut self) -> SyntaxResult<Statement> {
        match &self.peek()?.token {
            Token::Return => Ok(Statement::Return(self.parse_return_statement()?)),
            Token::If => Ok(Statement::Conditional(self.parse_conditional_statement()?)),
            Token::For => Ok(Statement::Iteration(self.parse_loop_statement()?)),
            Token::Console => Ok(Statement::Console(self.parse_console_statement()?)),
            Token::Let | Token::Const => Ok(Statement::Definition(self.parse_definition_statement()?)),
            Token::LeftCurly => Ok(Statement::Block(self.parse_block()?)),
            _ => Ok(self.parse_assign_statement()?),
        }
    }

    ///
    /// Returns a [`Block`] AST node if the next tokens represent a assign, or expression statement.
    ///
    pub fn parse_assign_statement(&mut self) -> SyntaxResult<Statement> {
        let expr = self.parse_expression()?;

        if let Some(operator) = self.eat_any(ASSIGN_TOKENS) {
            let value = self.parse_expression()?;
            let assignee = Self::construct_assignee(expr)?;
            self.expect(Token::Semicolon)?;
            Ok(Statement::Assign(AssignStatement {
                span: &assignee.span + value.span(),
                assignee,
                operation: match operator.token {
                    Token::Assign => AssignOperation::Assign,
                    Token::AddEq => AssignOperation::Add,
                    Token::MinusEq => AssignOperation::Sub,
                    Token::MulEq => AssignOperation::Mul,
                    Token::DivEq => AssignOperation::Div,
                    Token::ExpEq => AssignOperation::Pow,
                    // Token::OrEq => AssignOperation::Or,
                    // Token::AndEq => AssignOperation::And,
                    // Token::BitOrEq => AssignOperation::BitOr,
                    // Token::BitAndEq => AssignOperation::BitAnd,
                    // Token::BitXorEq => AssignOperation::BitXor,
                    // Token::ShrEq => AssignOperation::Shr,
                    // Token::ShrSignedEq => AssignOperation::ShrSigned,
                    // Token::ShlEq => AssignOperation::Shl,
                    // Token::ModEq => AssignOperation::Mod,
                    _ => unimplemented!(),
                },
                value,
            }))
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
    pub fn parse_block(&mut self) -> SyntaxResult<Block> {
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
    pub fn parse_return_statement(&mut self) -> SyntaxResult<ReturnStatement> {
        let start = self.expect(Token::Return)?;
        let expr = self.parse_expression()?;
        self.expect(Token::Semicolon)?;

        Ok(ReturnStatement {
            span: &start + expr.span(),
            expression: expr,
        })
    }

    ///
    /// Returns a [`ConditionalStatement`] AST node if the next tokens represent a conditional statement.
    ///
    pub fn parse_conditional_statement(&mut self) -> SyntaxResult<ConditionalStatement> {
        let start = self.expect(Token::If)?;
        self.fuzzy_struct_state = true;
        let expr = self.parse_conditional_expression()?;
        self.fuzzy_struct_state = false;
        let body = self.parse_block()?;
        let next = if self.eat(Token::Else).is_some() {
            let s = self.parse_statement()?;
            match s {
                Statement::Block(_) | Statement::Conditional(_) => Some(Box::new(s)),
                s => {
                    return Err(LeoError::from(ParserError::unexpected_statement(
                        s.to_string(),
                        "Block or Conditional",
                        s.span(),
                    )));
                }
            }
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

    ///
    /// Returns an [`IterationStatement`] AST node if the next tokens represent an iteration statement.
    ///
    pub fn parse_loop_statement(&mut self) -> SyntaxResult<IterationStatement> {
        let start_span = self.expect(Token::For)?;
        let ident = self.expect_ident()?;
        self.expect(Token::In)?;
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

    ///
    /// Returns a [`ConsoleArgs`] AST node if the next tokens represent a formatted string.
    ///
    pub fn parse_console_args(&mut self) -> SyntaxResult<ConsoleArgs> {
        let start_span;
        let string = match self.expect_any()? {
            SpannedToken {
                token: Token::StringLit(chars),
                span,
            } => {
                start_span = span;
                chars
            }
            SpannedToken { token, span } => {
                return Err(LeoError::from(ParserError::unexpected_str(
                    token.to_string(),
                    "formatted string",
                    &span,
                )));
            }
        };

        // let parts = FormatStringPart::from_string(string);

        let mut parameters = Vec::new();
        while self.eat(Token::Comma).is_some() {
            let param = self.parse_expression()?;
            parameters.push(param);
        }

        Ok(ConsoleArgs {
            string,
            span: &start_span + parameters.last().map(|x| x.span()).unwrap_or(&start_span),
            parameters,
        })
    }

    ///
    /// Returns a [`ConsoleStatement`] AST node if the next tokens represent a console statement.
    ///
    pub fn parse_console_statement(&mut self) -> SyntaxResult<ConsoleStatement> {
        let keyword = self.expect(Token::Console)?;
        self.expect(Token::Dot)?;
        let function = self.expect_ident()?;
        self.expect(Token::LeftParen)?;
        let function = match &*function.name {
            "assert" => {
                let expr = self.parse_expression()?;
                ConsoleFunction::Assert(expr)
            }
            "error" => ConsoleFunction::Error(self.parse_console_args()?),
            "log" => ConsoleFunction::Log(self.parse_console_args()?),
            x => {
                return Err(LeoError::from(ParserError::unexpected_ident(
                    &x,
                    &["assert", "error", "log"],
                    &function.span,
                )));
            }
        };
        self.expect(Token::RightParen)?;
        self.expect(Token::Semicolon)?;

        Ok(ConsoleStatement {
            span: &keyword + function.span(),
            function,
        })
    }

    ///
    /// Returns a [`VariableName`] AST node if the next tokens represent a variable name with
    /// valid keywords.
    ///
    pub fn parse_variable_name(&mut self, span: &SpannedToken) -> SyntaxResult<VariableName> {
        let mutable = self.eat(Token::Mut);
        if let Some(mutable) = &mutable {
            return Err(LeoError::from(ParserError::let_mut_statement(
                &(&mutable.span + &span.span),
            )));
        }

        let name = self.expect_ident()?;
        Ok(VariableName {
            span: name.span.clone(),
            mutable: matches!(span.token, Token::Let),
            identifier: name,
        })
    }

    ///
    /// Returns a [`DefinitionStatement`] AST node if the next tokens represent a definition statement.
    ///
    pub fn parse_definition_statement(&mut self) -> SyntaxResult<DefinitionStatement> {
        let declare = self.expect_oneof(&[Token::Let, Token::Const])?;
        let mut variable_names = Vec::new();

        let next = self.eat(Token::LeftParen);
        variable_names.push(self.parse_variable_name(&declare)?);
        if next.is_some() {
            let mut eaten_ending = false;
            while self.eat(Token::Comma).is_some() {
                if self.eat(Token::RightParen).is_some() {
                    eaten_ending = true;
                    break;
                }
                variable_names.push(self.parse_variable_name(&declare)?);
            }
            if !eaten_ending {
                self.expect(Token::RightParen)?;
            }
        }

        let type_ = if self.eat(Token::Colon).is_some() {
            Some(self.parse_type()?.0)
        } else {
            None
        };

        self.expect(Token::Assign)?;
        let expr = self.parse_expression()?;
        self.expect(Token::Semicolon)?;

        Ok(DefinitionStatement {
            span: &declare.span + expr.span(),
            declaration_type: match declare.token {
                Token::Let => Declare::Let,
                Token::Const => Declare::Const,
                _ => unimplemented!(),
            },
            variable_names,
            type_,
            value: expr,
        })
    }
}
