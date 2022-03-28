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
use crate::KEYWORD_TOKENS;

use leo_errors::{ParserError, Result};
use leo_span::sym;

impl ParserContext<'_> {
    ///
    /// Returns a [`Program`] AST if all tokens can be consumed and represent a valid Leo program.
    ///
    pub fn parse_program(&mut self) -> Result<Program> {
        let mut import_statements = Vec::new();
        let mut aliases = IndexMap::new();
        let mut functions = IndexMap::new();
        let mut global_consts = IndexMap::new();

        while self.has_next() {
            let token = self.peek()?;
            match &token.token {
                Token::Import => {
                    import_statements.push(self.parse_import_statement()?);
                }
                Token::Ident(ident) => match *ident {
                    sym::test => return Err(ParserError::test_function(&token.span).into()),
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
            import_statements,
            imports: IndexMap::new(),
            aliases,
            functions,
            global_consts,
        })
    }

    fn unexpected_item(token: &SpannedToken) -> ParserError {
        ParserError::unexpected(
            &token.token,
            [
                Token::Import,
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

    /// Returns an [`Identifier`] AST node if the next tokens represent a valid package name.
    pub fn parse_package_name(&mut self) -> Result<Identifier> {
        // Build the package name, starting with valid characters up to a dash `-` (Token::Minus).
        let base = self.expect_loose_identifier()?;

        // Return an error if the package name contains a keyword.
        if let Some(token) = KEYWORD_TOKENS.iter().find(|x| x.keyword_to_symbol() == Some(base.name)) {
            self.emit_err(ParserError::unexpected_str(token, "package name", &base.span));
        }

        // Return the package name.
        Ok(base)
    }

    /// Returns an [`ImportTree`] AST node if the next tokens represent a valid package import
    /// with accesses.
    // Public solely for writing import parsing tests.
    pub fn parse_import_tree(&mut self) -> Result<ImportTree> {
        // Parse the first part of the path.
        let first_name = self.parse_package_name()?;
        let start = first_name.span.clone();
        let mut base = vec![first_name];

        let make = |base, end, kind| {
            let span = start + end;
            ImportTree { base, span, kind }
        };

        // Paths are separated by `.`s.
        while self.eat(Token::Dot).is_some() {
            if self.peek_is_left_par() {
                // Encountered `.(`, so we have a nested import. Recurse!
                let (tree, _, end) = self.parse_paren_comma_list(|p| p.parse_import_tree().map(Some))?;

                if tree.is_empty() {
                    self.emit_err(ParserError::invalid_import_list(&end));
                }

                return Ok(make(base, end, ImportTreeKind::Nested { tree }));
            } else if let Some(SpannedToken { span, .. }) = self.eat(Token::Mul) {
                // Encountered `.*`, so we have a glob import.
                return Ok(make(base, span.clone(), ImportTreeKind::Glob { span }));
            }

            // Parse another path segment.
            base.push(self.parse_package_name()?);
        }

        let (end, alias) = if self.eat(Token::As).is_some() {
            // Encountered `as`, so interpret as `path as rename`.
            let alias = self.expect_ident()?;
            (alias.span.clone(), Some(alias))
        } else {
            (base.last().unwrap().span.clone(), None)
        };
        Ok(make(base, end, ImportTreeKind::Leaf { alias }))
    }

    /// Returns a [`ImportStatement`] AST node if the next tokens represent an import statement.
    pub fn parse_import_statement(&mut self) -> Result<ImportStatement> {
        self.expect(Token::Import)?;
        let tree = self.parse_import_tree()?;
        self.expect(Token::Semicolon)?;
        Ok(ImportStatement {
            span: tree.span.clone(),
            tree,
        })
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
