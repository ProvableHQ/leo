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

use leo_errors::{ParserError, ParserWarning, Result};
use leo_span::sym;

impl ParserContext<'_> {
    /// Returns a [`Program`] AST if all tokens can be consumed and represent a valid Leo program.
    pub fn parse_program(&mut self) -> Result<Program> {
        let mut functions = IndexMap::new();

        while self.has_next() {
            match &self.token.token {
                Token::Ident(sym::test) => return Err(ParserError::test_function(&self.token.span).into()),
                // Const functions share the first token with the global Const.
                Token::Const if self.peek_is_function() => {
                    let (id, function) = self.parse_function_declaration()?;
                    functions.insert(id, function);
                }
                Token::Function => {
                    let (id, function) = self.parse_function_declaration()?;
                    functions.insert(id, function);
                }
                _ => return Err(Self::unexpected_item(&self.token).into()),
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

    /// Returns a [`ParamMode`] AST node if the next tokens represent a function parameter mode.
    pub fn parse_function_parameter_mode(&mut self) -> Result<ParamMode> {
        let public = self.eat(&Token::Public).then(|| self.prev_token.span);
        let constant = self.eat(&Token::Constant).then(|| self.prev_token.span);
        let const_ = self.eat(&Token::Const).then(|| self.prev_token.span);

        if let Some(span) = &const_ {
            self.emit_warning(ParserWarning::const_parameter_or_input(span));
        }

        match (public, constant, const_) {
            (None, Some(_), None) => Ok(ParamMode::Constant),
            (None, None, Some(_)) => Ok(ParamMode::Constant),
            (None, None, None) => Ok(ParamMode::Private),
            (Some(_), None, None) => Ok(ParamMode::Public),
            (Some(m1), Some(m2), None) | (Some(m1), None, Some(m2)) | (None, Some(m1), Some(m2)) => {
                Err(ParserError::inputs_multiple_variable_types_specified(&(m1 + m2)).into())
            }
            (Some(m1), Some(m2), Some(m3)) => {
                Err(ParserError::inputs_multiple_variable_types_specified(&(m1 + m2 + m3)).into())
            }
        }
    }

    /// Returns a [`FunctionInput`] AST node if the next tokens represent a function parameter.
    pub fn parse_function_parameter(&mut self) -> Result<FunctionInput> {
        let mode = self.parse_function_parameter_mode()?;
        let mutable = self.eat(&Token::Mut).then(|| self.prev_token.clone());

        let name = self.expect_ident()?;

        if let Some(mutable) = &mutable {
            self.emit_err(ParserError::mut_function_input(&(mutable.span + name.span)));
        }

        self.expect(&Token::Colon)?;
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
        // Parse `function IDENT`.
        let start = self.expect(&Token::Function)?;
        let name = self.expect_ident()?;

        // Parse parameters.
        let (inputs, ..) = self.parse_paren_comma_list(|p| p.parse_function_parameter().map(Some))?;

        // Parse return type.
        self.expect(&Token::Arrow)?;
        let output = self.parse_type()?.0;

        // Parse the function body.
        let block = self.parse_block()?;

        Ok((
            name.clone(),
            Function {
                identifier: name,
                input: inputs,
                output,
                span: start + block.span,
                block,
                core_mapping: <_>::default(),
            },
        ))
    }

    /// Returns an [`(String, DefinitionStatement)`] AST node if the next tokens represent a global
    /// constant declaration.
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
