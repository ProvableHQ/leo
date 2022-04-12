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

impl ParserContext<'_> {
    ///
    /// Returns a [`Program`] AST if all tokens can be consumed and represent a valid Leo program.
    ///
    pub fn parse_program(&mut self) -> Result<Program> {
        let mut functions = IndexMap::new();

        while self.has_next() {
            let token = self.peek()?;
            match &token.token {
                Token::Ident(sym::test) => return Err(ParserError::test_function(&token.span).into()),
                // Const functions share the first token with the global Const.
                Token::Const if self.peek_is_function()? => {
                    let (id, function) = self.parse_function_declaration()?;
                    functions.insert(id, function);
                }
                Token::Function => {
                    let (id, function) = self.parse_function_declaration()?;
                    functions.insert(id, function);
                }
                _ => return Err(Self::unexpected_item(token).into()),
            }
        }
        Ok(Program {
            name: String::new(),
            expected_input: Vec::new(),
            functions,
        })
    }

    fn unexpected_item(token: &SpannedToken) -> ParserError {
        ParserError::unexpected(
            &token.token,
            [Token::Function, Token::Ident(sym::test)]
                .iter()
                .map(|x| format!("'{}'", x))
                .collect::<Vec<_>>()
                .join(", "),
            &token.span,
        )
    }

    ///
    /// Returns a [`ParamMode`] AST node if the next tokens represent a function parameter mode.
    ///
    pub fn parse_function_parameter_mode(&mut self) -> Result<ParamMode> {
        let public = self.eat(Token::Public);
        let constant = self.eat(Token::Constant);
        let const_ = self.eat(Token::Const);

        if const_.is_some() {
            self.emit_err(ParserError::const_parameter_or_input(&const_.as_ref().unwrap().span));
        }

        match (public, constant, const_) {
            (None, Some(_), None) => Ok(ParamMode::Constant),
            (None, None, Some(_)) => Ok(ParamMode::Constant),
            (None, None, None) => Ok(ParamMode::Private),
            (Some(_), None, None) => Ok(ParamMode::Public),
            (Some(p), None, Some(c)) => {
                Err(ParserError::inputs_multiple_variable_types_specified(&(p.span + c.span)).into())
            }
            (None, Some(c), Some(co)) => {
                Err(ParserError::inputs_multiple_variable_types_specified(&(c.span + co.span)).into())
            }
            (Some(p), Some(c), None) => {
                Err(ParserError::inputs_multiple_variable_types_specified(&(p.span + c.span)).into())
            }
            (Some(p), Some(c), Some(co)) => {
                Err(ParserError::inputs_multiple_variable_types_specified(&(p.span + c.span + co.span)).into())
            }
        }
    }

    ///
    /// Returns a [`FunctionInput`] AST node if the next tokens represent a function parameter.
    ///
    pub fn parse_function_parameters(&mut self) -> Result<FunctionInput> {
        let mode = self.parse_function_parameter_mode()?;
        let mutable = self.eat(Token::Mut);

        let name = self.expect_ident()?;

        if let Some(mutable) = &mutable {
            self.emit_err(ParserError::mut_function_input(&(&mutable.span + &name.span)));
        }

        self.expect(Token::Colon)?;
        let type_ = self.parse_type()?.0;
        Ok(FunctionInput::Variable(FunctionInputVariable::new(
            name.clone(),
            mode,
            type_,
            name.span,
        )))
    }

    /// Returns an [`(Identifier, Function)`] AST node if the next tokens represent a function name
    /// and function definition.
    pub fn parse_function_declaration(&mut self) -> Result<(Identifier, Function)> {
        // Parse optional const modifier.
        let const_ = self.eat(Token::Const).is_some();

        // Parse `function IDENT`.
        let start = self.expect(Token::Function)?;
        let name = self.expect_ident()?;

        // Parse parameters.
        let (inputs, ..) = self.parse_paren_comma_list(|p| p.parse_function_parameters().map(Some))?;

        // Parse return type.
        let output = if self.eat(Token::Arrow).is_some() {
            Some(self.parse_type()?.0)
        } else {
            None
        };

        // Parse the function body.
        let block = self.parse_block()?;

        Ok((
            name.clone(),
            Function {
                identifier: name,
                input: inputs,
                const_,
                output,
                span: start + block.span.clone(),
                block,
                core_mapping: <_>::default(),
            },
        ))
    }

    ///
    /// Returns an [`(String, DefinitionStatement)`] AST node if the next tokens represent a global
    /// constant declaration.
    ///
    pub fn parse_global_const_declaration(&mut self) -> Result<(Vec<Identifier>, DefinitionStatement)> {
        let statement = self.parse_definition_statement()?;
        let variable_names = statement
            .variable_names
            .iter()
            .map(|variable_name| variable_name.identifier.clone())
            .collect();

        Ok((variable_names, statement))
    }
}
