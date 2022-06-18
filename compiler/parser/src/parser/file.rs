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
        let mut circuits = IndexMap::new();

        while self.has_next() {
            match &self.token.token {
                Token::Circuit => {
                    let (id, circuit) = self.parse_circuit()?;
                    circuits.insert(id, circuit);
                }
                Token::Const if self.peek_is_function() => {
                    let (id, function) = self.parse_function()?;
                    functions.insert(id, function);
                }
                Token::Ident(sym::test) => return Err(ParserError::test_function(self.token.span).into()),
                // Const functions share the first token with the global Const.
                Token::Function => {
                    let (id, function) = self.parse_function()?;
                    functions.insert(id, function);
                }

                _ => return Err(Self::unexpected_item(&self.token).into()),
            }
        }
        Ok(Program {
            name: String::new(),
            expected_input: Vec::new(),
            functions,
            circuits,
        })
    }

    fn unexpected_item(token: &SpannedToken) -> ParserError {
        ParserError::unexpected(
            &token.token,
            [Token::Function, Token::Circuit, Token::Ident(sym::test)]
                .iter()
                .map(|x| format!("'{}'", x))
                .collect::<Vec<_>>()
                .join(", "),
            token.span,
        )
    }

    /// Returns a [`CircuitMember`] AST node if the next tokens represent a circuit member variable
    /// or circuit member function or circuit member constant.
    pub fn parse_circuit_declaration(&mut self) -> Result<(Vec<CircuitMember>, Span)> {
        let mut members = Vec::new();

        let (mut semi_colons, mut commas) = (false, false);

        while !self.check(&Token::RightCurly) {
            members.push(if self.peek_is_function() {
                // function
                self.parse_member_function_declaration()?
            } else if self.eat(&Token::Static) {
                // static const
                self.parse_const_member_variable_declaration()?
            } else {
                // variable
                let variable = self.parse_member_variable_declaration()?;

                if self.eat(&Token::Semicolon) {
                    if commas {
                        self.emit_err(ParserError::mixed_commas_and_semicolons(self.token.span));
                    }
                    semi_colons = true;
                }

                if self.eat(&Token::Comma) {
                    if semi_colons {
                        self.emit_err(ParserError::mixed_commas_and_semicolons(self.token.span));
                    }
                    commas = true;
                }

                variable
            });
        }
        let span = self.expect(&Token::RightCurly)?;

        self.ban_mixed_member_order(&members);

        Ok((members, span))
    }

    /// Emits errors if order isn't `consts variables functions`.
    fn ban_mixed_member_order(&self, members: &[CircuitMember]) {
        let mut had_var = false;
        let mut had_fun = false;
        for member in members {
            match member {
                CircuitMember::CircuitConst(id, _, e) if had_var => {
                    self.emit_err(ParserError::member_const_after_var(id.span() + e.span()));
                }
                CircuitMember::CircuitConst(id, _, e) if had_fun => {
                    self.emit_err(ParserError::member_const_after_fun(id.span() + e.span()));
                }
                CircuitMember::CircuitVariable(id, _) if had_fun => {
                    self.emit_err(ParserError::member_var_after_fun(id.span()));
                }
                CircuitMember::CircuitConst(..) => {}
                CircuitMember::CircuitVariable(..) => had_var = true,
                CircuitMember::CircuitFunction(..) => had_fun = true,
            }
        }
    }

    /// Parses `IDENT: TYPE`.
    fn parse_typed_field_name(&mut self) -> Result<(Identifier, Type)> {
        let name = self.expect_ident()?;
        self.expect(&Token::Colon)?;
        let type_ = self.parse_all_types()?.0;

        Ok((name, type_))
    }

    /// Returns a [`CircuitMember`] AST node if the next tokens represent a circuit member static constant.
    pub fn parse_const_member_variable_declaration(&mut self) -> Result<CircuitMember> {
        self.expect(&Token::Static)?;
        self.expect(&Token::Const)?;

        // `IDENT: TYPE = EXPR`:
        let (name, type_) = self.parse_typed_field_name()?;
        self.expect(&Token::Assign)?;
        let expr = self.parse_expression()?;

        self.expect(&Token::Semicolon)?;

        Ok(CircuitMember::CircuitConst(name, type_, expr))
    }

    /// Returns a [`CircuitMember`] AST node if the next tokens represent a circuit member variable.
    pub fn parse_member_variable_declaration(&mut self) -> Result<CircuitMember> {
        let (name, type_) = self.parse_typed_field_name()?;

        Ok(CircuitMember::CircuitVariable(name, type_))
    }

    /// Returns a [`CircuitMember`] AST node if the next tokens represent a circuit member function.
    pub fn parse_member_function_declaration(&mut self) -> Result<CircuitMember> {
        if self.peek_is_function() {
            // CAUTION: function members are unstable for testnet3.
            let function = self.parse_function()?;

            return Err(ParserError::circuit_functions_unstable(function.1.span()).into())
            // Ok(CircuitMember::CircuitFunction(Box::new(function.1)))
        } else {
            return Err(Self::unexpected_item(&self.token).into());
        }
    }

    /// Returns an [`(Identifier, Function)`] ast node if the next tokens represent a circuit declaration.
    pub(super) fn parse_circuit(&mut self) -> Result<(Identifier, Circuit)> {
        let start = self.expect(&Token::Circuit)?;
        let circuit_name = self.expect_ident()?;

        self.expect(&Token::LeftCurly)?;
        let (members, end) = self.parse_circuit_declaration()?;

        Ok((
            circuit_name.clone(),
            Circuit {
                identifier: circuit_name,
                members,
                span: start + end,
            },
        ))
    }

    /// Returns a [`ParamMode`] AST node if the next tokens represent a function parameter mode.
    pub(super) fn parse_function_parameter_mode(&mut self) -> Result<ParamMode> {
        let public = self.eat(&Token::Public).then(|| self.prev_token.span);
        let constant = self.eat(&Token::Constant).then(|| self.prev_token.span);
        let const_ = self.eat(&Token::Const).then(|| self.prev_token.span);

        if let Some(span) = const_ {
            self.emit_warning(ParserWarning::const_parameter_or_input(span));
        }

        match (public, constant, const_) {
            (None, Some(_), None) => Ok(ParamMode::Const),
            (None, None, Some(_)) => Ok(ParamMode::Const),
            (None, None, None) => Ok(ParamMode::Private),
            (Some(_), None, None) => Ok(ParamMode::Public),
            (Some(m1), Some(m2), None) | (Some(m1), None, Some(m2)) | (None, Some(m1), Some(m2)) => {
                Err(ParserError::inputs_multiple_variable_types_specified(m1 + m2).into())
            }
            (Some(m1), Some(m2), Some(m3)) => {
                Err(ParserError::inputs_multiple_variable_types_specified(m1 + m2 + m3).into())
            }
        }
    }

    /// Returns a [`FunctionInput`] AST node if the next tokens represent a function parameter.
    fn parse_function_parameter(&mut self) -> Result<FunctionInput> {
        let mode = self.parse_function_parameter_mode()?;

        let name = self.expect_ident()?;

        self.expect(&Token::Colon)?;
        let type_ = self.parse_all_types()?.0;
        Ok(FunctionInput::Variable(FunctionInputVariable::new(
            name, mode, type_, name.span,
        )))
    }

    /// Returns `true` if the next token is Function or if it is a Const followed by Function.
    /// Returns `false` otherwise.
    fn peek_is_function(&self) -> bool {
        matches!(
            (&self.token.token, self.look_ahead(1, |t| &t.token)),
            (Token::Function, _) | (Token::Const, Token::Function)
        )
    }

    /// Returns an [`(Identifier, Function)`] AST node if the next tokens represent a function name
    /// and function definition.
    fn parse_function(&mut self) -> Result<(Identifier, Function)> {
        // Parse `function IDENT`.
        let start = self.expect(&Token::Function)?;
        let name = self.expect_ident()?;

        // Parse parameters.
        let (inputs, ..) = self.parse_paren_comma_list(|p| p.parse_function_parameter().map(Some))?;

        // Parse return type.
        self.expect(&Token::Arrow)?;
        self.disallow_circuit_construction = true;
        let output = self.parse_all_types()?.0;
        self.disallow_circuit_construction = false;

        // Parse the function body.
        let block = self.parse_block()?;

        Ok((
            name,
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
}
