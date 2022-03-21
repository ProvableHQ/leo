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
        let mut aliases = IndexMap::new();
        let mut circuits = IndexMap::new();
        let mut functions = IndexMap::new();
        let mut global_consts = IndexMap::new();

        while self.has_next() {
            let token = self.peek()?;
            match &token.token {
                Token::Import => {
                    unimplemented!("Import statements not stable yet")
                }
                Token::Circuit => {
                    self.expect(Token::Circuit)?;
                    let (id, circuit) = self.parse_circuit()?;
                    circuits.insert(id, circuit);
                }
                Token::Ident(ident) => match *ident {
                    sym::test => return Err(ParserError::test_function(&token.span).into()),
                    kw @ (sym::Struct | sym::Class) => {
                        self.emit_err(ParserError::unexpected(kw, "circuit", &token.span));
                        self.bump().unwrap();
                        let (id, circuit) = self.parse_circuit()?;
                        circuits.insert(id, circuit);
                    }
                    _ => return Err(Self::unexpected_item(token).into()),
                },
                // Const functions share the first token with the global Const.
                Token::Const if self.peek_is_function()? => {
                    let (id, function) = self.parse_function_declaration()?;
                    functions.insert(id, function);
                }
                Token::Const => {
                    let (name, global_const) = self.parse_global_const_declaration()?;
                    global_consts.insert(name, global_const);
                }
                Token::Function | Token::At => {
                    let (id, function) = self.parse_function_declaration()?;
                    functions.insert(id, function);
                }
                Token::Type => {
                    let (name, alias) = self.parse_type_alias()?;
                    aliases.insert(name, alias);
                }
                _ => return Err(Self::unexpected_item(token).into()),
            }
        }
        Ok(Program {
            name: String::new(),
            expected_input: Vec::new(),
            aliases,
            circuits,
            functions,
            global_consts,
        })
    }

    fn unexpected_item(token: &SpannedToken) -> ParserError {
        ParserError::unexpected(
            &token.token,
            [
                Token::Import,
                Token::Circuit,
                Token::Function,
                Token::Ident(sym::test),
                Token::At,
            ]
            .iter()
            .map(|x| format!("'{}'", x))
            .collect::<Vec<_>>()
            .join(", "),
            &token.span,
        )
    }

    /// Returns an [`Annotation`] AST node if the next tokens represent a supported annotation.
    pub fn parse_annotation(&mut self) -> Result<Annotation> {
        let start = self.expect(Token::At)?;
        let name = self.parse_annotation_name()?;

        assert_no_whitespace(&start, &name.span, &name.name.as_str(), "@")?;

        let (end_span, arguments) = if self.peek_is_left_par() {
            let (args, _, span) = self.parse_paren_comma_list(|p| {
                Ok(if let Some(ident) = p.eat_identifier() {
                    Some(ident.name)
                } else {
                    let token = p.expect_any()?;
                    p.emit_err(ParserError::unexpected_str(&token.token, "ident", &token.span));
                    None
                })
            })?;
            (span, args)
        } else {
            (name.span.clone(), Vec::new())
        };
        Ok(Annotation {
            name,
            arguments,
            span: start + end_span,
        })
    }

    /// Parses `foo` in an annotation `@foo . That is, the name of the annotation.
    fn parse_annotation_name(&mut self) -> Result<Identifier> {
        let mut name = self.expect_ident()?;

        // Recover `context` instead of `test`.
        if name.name == sym::context {
            self.emit_err(ParserError::context_annotation(&name.span));
            name.name = sym::test;
        }

        Ok(name)
    }

    /// Returns a [`CircuitMember`] AST node if the next tokens represent a circuit member variable
    /// or circuit member function or circuit member constant.
    pub fn parse_circuit_declaration(&mut self) -> Result<Vec<CircuitMember>> {
        let mut members = Vec::new();

        let (mut semi_colons, mut commas) = (false, false);

        while self.eat(Token::RightCurly).is_none() {
            members.push(if self.peek_is_function()? {
                // function
                self.parse_member_function_declaration()?
            } else if *self.peek_token() == Token::Static {
                // static const
                self.parse_const_member_variable_declaration()?
            } else {
                // variable
                let variable = self.parse_member_variable_declaration()?;

                if let Some(semi) = self.eat(Token::Semicolon) {
                    if commas {
                        self.emit_err(ParserError::mixed_commas_and_semicolons(&semi.span));
                    }
                    semi_colons = true;
                }

                if let Some(comma) = self.eat(Token::Comma) {
                    if semi_colons {
                        self.emit_err(ParserError::mixed_commas_and_semicolons(&comma.span));
                    }
                    commas = true;
                }

                variable
            });
        }

        self.ban_mixed_member_order(&members);

        Ok(members)
    }

    /// Emits errors if order isn't `consts variables functions`.
    fn ban_mixed_member_order(&self, members: &[CircuitMember]) {
        let mut had_var = false;
        let mut had_fun = false;
        for member in members {
            match member {
                CircuitMember::CircuitConst(id, _, e) if had_var => {
                    self.emit_err(ParserError::member_const_after_var(&(id.span() + e.span())));
                }
                CircuitMember::CircuitConst(id, _, e) if had_fun => {
                    self.emit_err(ParserError::member_const_after_fun(&(id.span() + e.span())));
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
        self.expect(Token::Colon)?;
        let type_ = self.parse_type()?.0;

        Ok((name, type_))
    }

    /// Returns a [`CircuitMember`] AST node if the next tokens represent a circuit member static constant.
    pub fn parse_const_member_variable_declaration(&mut self) -> Result<CircuitMember> {
        self.expect(Token::Static)?;
        self.expect(Token::Const)?;

        // `IDENT: TYPE = EXPR`:
        let (name, type_) = self.parse_typed_field_name()?;
        self.expect(Token::Assign)?;
        let literal = self.parse_primary_expression()?;

        self.expect(Token::Semicolon)?;

        Ok(CircuitMember::CircuitConst(name, type_, literal))
    }

    ///
    /// Returns a [`CircuitMember`] AST node if the next tokens represent a circuit member variable.
    ///
    pub fn parse_member_variable_declaration(&mut self) -> Result<CircuitMember> {
        let (name, type_) = self.parse_typed_field_name()?;

        Ok(CircuitMember::CircuitVariable(name, type_))
    }

    /// Returns a [`CircuitMember`] AST node if the next tokens represent a circuit member function.
    pub fn parse_member_function_declaration(&mut self) -> Result<CircuitMember> {
        let peeked = self.peek()?.clone();
        if self.peek_is_function()? {
            let function = self.parse_function_declaration()?;
            Ok(CircuitMember::CircuitFunction(Box::new(function.1)))
        } else {
            return Err(ParserError::unexpected(
                &peeked.token,
                [Token::Function, Token::At, Token::Const]
                    .iter()
                    .map(|x| format!("'{}'", x))
                    .collect::<Vec<_>>()
                    .join(", "),
                &peeked.span,
            )
            .into());
        }
    }

    ///
    /// Returns an [`(Identifier, Circuit)`] tuple of AST nodes if the next tokens represent a
    /// circuit name and definition statement.
    ///
    pub fn parse_circuit(&mut self) -> Result<(Identifier, Circuit)> {
        let name = if let Some(ident) = self.eat_identifier() {
            ident
        } else if let Some(scalar_type) = self.eat_any(crate::type_::TYPE_TOKENS) {
            Identifier {
                name: scalar_type.token.keyword_to_symbol().unwrap(),
                span: scalar_type.span,
            }
        } else {
            let next = self.peek()?;
            return Err(ParserError::unexpected_str(&next.token, "ident", &next.span).into());
        };

        self.expect(Token::LeftCurly)?;
        let members = self.parse_circuit_declaration()?;

        Ok((
            name.clone(),
            Circuit {
                circuit_name: name,
                members,
            },
        ))
    }

    ///
    /// Returns a [`FunctionInput`] AST node if the next tokens represent a function parameter.
    ///
    pub fn parse_function_parameters(&mut self, first: bool) -> Result<FunctionInput> {
        let const_ = self.eat(Token::Const);
        let mutable = self.eat(Token::Mut);
        let reference = self.eat(Token::Ampersand);
        let mut name = if let Some(token) = self.eat(Token::LittleSelf) {
            Identifier {
                name: sym::SelfLower,
                span: token.span,
            }
        } else {
            self.expect_ident()?
        };
        if name.name == sym::SelfLower {
            if !first {
                return Err(ParserError::parser_self_outside_first_argument().into());
            } else if let Some(mutable) = &mutable {
                self.emit_err(ParserError::mut_self_parameter(&(&mutable.span + &name.span)));
                return Ok(Self::build_ref_self(name, mutable));
            } else if let Some(reference) = &reference {
                // Handle `&self`.
                return Ok(Self::build_ref_self(name, reference));
            } else if let Some(const_) = &const_ {
                // Handle `const self`.
                name.span = &const_.span + &name.span;
                name.name = Symbol::intern("const self");
                return Ok(FunctionInput::ConstSelfKeyword(ConstSelfKeyword { identifier: name }));
            }
            // Handle `self`.
            return Ok(FunctionInput::SelfKeyword(SelfKeyword { identifier: name }));
        }

        if let Some(mutable) = &mutable {
            self.emit_err(ParserError::mut_function_input(&(&mutable.span + &name.span)));
        }

        self.expect(Token::Colon)?;
        let type_ = self.parse_type()?.0;
        Ok(FunctionInput::Variable(FunctionInputVariable {
            const_: const_.is_some(),
            mutable: const_.is_none(),
            type_,
            span: name.span.clone(),
            identifier: name,
        }))
    }

    /// Builds a function parameter `&self`.
    fn build_ref_self(mut name: Identifier, reference: &SpannedToken) -> FunctionInput {
        name.span = &reference.span + &name.span;
        // FIXME(Centril): This should be *two* tokens, NOT one!
        name.name = Symbol::intern("&self");
        FunctionInput::RefSelfKeyword(RefSelfKeyword { identifier: name })
    }

    /// Returns an [`(Identifier, Function)`] AST node if the next tokens represent a function name
    /// and function definition.
    pub fn parse_function_declaration(&mut self) -> Result<(Identifier, Function)> {
        // Parse any annotations.
        let mut annotations = IndexMap::new();
        while self.peek_token().as_ref() == &Token::At {
            let annotation = self.parse_annotation()?;
            annotations.insert(annotation.name.name, annotation);
        }

        // Parse optional const modifier.
        let const_ = self.eat(Token::Const).is_some();

        // Parse `function IDENT`.
        let start = self.expect(Token::Function)?;
        let name = self.expect_ident()?;

        // Parse parameters.
        let mut first = true;
        let (inputs, ..) = self.parse_paren_comma_list(|p| {
            let param = p.parse_function_parameters(first).map(Some);
            first = false;
            param
        })?;

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
                annotations,
                identifier: name,
                input: inputs,
                const_,
                output,
                span: start + block.span.clone(),
                block,
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

    ///
    /// Returns a [`(String, Alias)`] AST node if the next tokens represent a type alias declaration.
    ///
    pub fn parse_type_alias(&mut self) -> Result<(Identifier, Alias)> {
        let start = self.expect(Token::Type)?;
        let name = self.expect_ident()?;
        self.expect(Token::Assign)?;
        let (represents, _) = self.parse_type()?;
        let span = start + self.expect(Token::Semicolon)?;

        Ok((name.clone(), Alias { represents, span, name }))
    }
}
