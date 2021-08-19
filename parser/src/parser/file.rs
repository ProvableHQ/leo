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

use tendril::format_tendril;

use leo_errors::{ParserError, Result, Span};

use crate::KEYWORD_TOKENS;

use super::*;

impl ParserContext {
    ///
    /// Returns a [`Program`] AST if all tokens can be consumed and represent a valid Leo program.
    ///
    pub fn parse_program(&mut self) -> Result<Program> {
        let mut import_statements = Vec::new();
        let mut aliases = IndexMap::new();
        let mut circuits = IndexMap::new();
        let mut functions = IndexMap::new();
        let mut global_consts = IndexMap::new();
        // let mut tests = IndexMap::new();

        while self.has_next() {
            let token = self.peek()?;
            match &token.token {
                Token::Import => {
                    import_statements.push(self.parse_import_statement()?);
                }
                Token::Circuit => {
                    let (id, circuit) = self.parse_circuit()?;
                    circuits.insert(id, circuit);
                }
                Token::Function | Token::At => {
                    let (id, function) = self.parse_function_declaration()?;
                    functions.insert(id, function);
                }
                Token::Ident(ident) if ident.as_ref() == "test" => {
                    return Err(ParserError::test_function(&token.span).into());
                    // self.expect(Token::Test)?;
                    // let (id, function) = self.parse_function_declaration()?;
                    // tests.insert(id, TestFunction {
                    //     function,
                    //     input_file: None,
                    // });
                }
                Token::Const => {
                    let (name, global_const) = self.parse_global_const_declaration()?;
                    global_consts.insert(name, global_const);
                }
                Token::Type => {
                    let (name, alias) = self.parse_type_alias()?;
                    aliases.insert(name, alias);
                }
                _ => {
                    return Err(ParserError::unexpected(
                        &token.token,
                        [
                            Token::Import,
                            Token::Circuit,
                            Token::Function,
                            Token::Ident("test".into()),
                            Token::At,
                        ]
                        .iter()
                        .map(|x| format!("'{}'", x))
                        .collect::<Vec<_>>()
                        .join(", "),
                        &token.span,
                    )
                    .into());
                }
            }
        }
        Ok(Program {
            name: String::new(),
            expected_input: Vec::new(),
            import_statements,
            imports: IndexMap::new(),
            aliases,
            circuits,
            functions,
            global_consts,
        })
    }

    ///
    /// Returns an [`Annotation`] AST node if the next tokens represent a supported annotation.
    ///
    pub fn parse_annotation(&mut self) -> Result<Annotation> {
        let start = self.expect(Token::At)?;
        let name = self.expect_ident()?;
        if name.name.as_ref() == "context" {
            return Err(ParserError::context_annotation(&name.span).into());
        }

        assert_no_whitespace(&start, &name.span, &name.name, "@")?;

        let end_span;
        let arguments = if self.eat(Token::LeftParen).is_some() {
            let mut args = Vec::new();
            let mut comma = false;
            loop {
                if let Some(end) = self.eat(Token::RightParen) {
                    if comma {
                        return Err(ParserError::unexpected(
                            Token::RightParen,
                            [Token::Ident("identifier".into()), Token::Int("number".into())]
                                .iter()
                                .map(|x| format!("'{}'", x))
                                .collect::<Vec<_>>()
                                .join(", "),
                            &end.span,
                        )
                        .into());
                    }
                    end_span = end.span;
                    break;
                }
                comma = false;
                if let Some(ident) = self.eat_identifier() {
                    args.push(ident.name);
                } else if let Some((int, _)) = self.eat_int() {
                    args.push(int.value);
                } else {
                    let token = self.peek()?;
                    return Err(ParserError::unexpected_str(&token.token, "ident or int", &token.span).into());
                }
                if self.eat(Token::Comma).is_none() && !comma {
                    end_span = self.expect(Token::RightParen)?;
                    break;
                }
                comma = true;
            }
            args
        } else {
            end_span = name.span.clone();
            Vec::new()
        };
        Ok(Annotation {
            name,
            arguments,
            span: start + end_span,
        })
    }

    ///
    /// Returns a vector of [`PackageAccess`] AST nodes if the next tokens represent package access
    /// expressions within an import statement.
    ///
    pub fn parse_package_accesses(&mut self, span: &Span) -> Result<Vec<PackageAccess>> {
        let mut out = Vec::new();
        self.expect(Token::LeftParen)?;
        while self.eat(Token::RightParen).is_none() {
            let access = self.parse_package_access()?;
            out.push(access);
            if self.eat(Token::Comma).is_none() {
                self.expect(Token::RightParen)?;
                break;
            }
        }

        if out.is_empty() {
            return Err(ParserError::invalid_import_list(span).into());
        }

        Ok(out)
    }

    ///
    /// Returns a [`PackageAccess`] AST node if the next tokens represent a package access expression
    /// within an import statement.
    ///
    pub fn parse_package_access(&mut self) -> Result<PackageAccess> {
        if let Some(SpannedToken { span, .. }) = self.eat(Token::Mul) {
            Ok(PackageAccess::Star { span })
        } else {
            let mut name = self.expect_ident()?;

            // Allow dashes in the accessed members (should only be used for directories).
            // If imported member does not exist, code will fail on ASG level.
            if let Token::Minus = self.peek_token().as_ref() {
                let span = self.expect(Token::Minus)?;
                name.span = name.span + span;
                let next = self.expect_ident()?;
                name.span = name.span + next.span;
                name.name = format_tendril!("{}-{}", name.name, next.name);
            }

            if self.peek_token().as_ref() == &Token::Dot {
                self.backtrack(SpannedToken {
                    token: Token::Ident(name.name),
                    span: name.span,
                });
                Ok(match self.parse_package_path()? {
                    PackageOrPackages::Package(p) => PackageAccess::SubPackage(Box::new(p)),
                    PackageOrPackages::Packages(p) => PackageAccess::Multiple(p),
                })
            } else if self.eat(Token::As).is_some() {
                let alias = self.expect_ident()?;
                Ok(PackageAccess::Symbol(ImportSymbol {
                    span: &name.span + &alias.span,
                    symbol: name,
                    alias: Some(alias),
                }))
            } else {
                Ok(PackageAccess::Symbol(ImportSymbol {
                    span: name.span.clone(),
                    symbol: name,
                    alias: None,
                }))
            }
        }
    }

    ///
    /// Returns an [`Identifier`] AST node if the next tokens represent a valid package name.
    ///
    pub fn parse_package_name(&mut self) -> Result<Identifier> {
        // Build the package name, starting with valid characters up to a dash `-` (Token::Minus).
        let mut base = self.expect_loose_identifier()?;

        // Build the rest of the package name including dashes.
        loop {
            match &self.peek_token().as_ref() {
                Token::Minus => {
                    let span = self.expect(Token::Minus)?;
                    base.span = base.span + span;
                    let next = self.expect_loose_identifier()?;
                    base.name = format_tendril!("{}-{}", base.name, next.name);
                    base.span = base.span + next.span;
                }
                Token::Int(_) => {
                    let (num, span) = self.eat_int().unwrap();
                    base.name = format_tendril!("{}{}", base.name, num.value);
                    base.span = base.span + span;
                }
                Token::Ident(_) => {
                    let next = self.expect_ident()?;
                    base.name = format_tendril!("{}{}", base.name, next.name);
                    base.span = base.span + next.span;
                }
                x if KEYWORD_TOKENS.contains(x) => {
                    let next = self.expect_loose_identifier()?;
                    base.name = format_tendril!("{}{}", base.name, next.name);
                    base.span = base.span + next.span;
                }
                _ => break,
            }
        }

        // Return an error if the package name contains a keyword.
        if let Some(token) = KEYWORD_TOKENS.iter().find(|x| x.to_string() == base.name.as_ref()) {
            return Err(ParserError::unexpected_str(token, "package name", &base.span).into());
        }

        // Return an error if the package name contains invalid characters.
        if !base
            .name
            .chars()
            .all(|x| x.is_ascii_lowercase() || x.is_ascii_digit() || x == '-' || x == '_')
        {
            return Err(ParserError::invalid_package_name(&base.span).into());
        }

        // Return the package name.
        Ok(base)
    }

    ///
    /// Returns a [`PackageOrPackages`] AST node if the next tokens represent a valid package import
    /// with accesses.
    ///
    pub fn parse_package_path(&mut self) -> Result<PackageOrPackages> {
        let package_name = self.parse_package_name()?;
        self.expect(Token::Dot)?;
        if self.peek()?.token == Token::LeftParen {
            let accesses = self.parse_package_accesses(&package_name.span)?;
            Ok(PackageOrPackages::Packages(Packages {
                span: &package_name.span + accesses.last().map(|x| x.span()).unwrap_or(&package_name.span),
                name: package_name,
                accesses,
            }))
        } else {
            let access = self.parse_package_access()?;
            Ok(PackageOrPackages::Package(Package {
                span: &package_name.span + access.span(),
                name: package_name,
                access,
            }))
        }
    }

    ///
    /// Returns a [`ImportStatement`] AST node if the next tokens represent an import statement.
    ///
    pub fn parse_import_statement(&mut self) -> Result<ImportStatement> {
        self.expect(Token::Import)?;
        let package_or_packages = self.parse_package_path()?;
        self.expect(Token::Semicolon)?;
        Ok(ImportStatement {
            span: package_or_packages.span().clone(),
            package_or_packages,
        })
    }

    ///
    /// Returns a [`CircuitMember`] AST node if the next tokens represent a circuit member variable
    /// or circuit member function.
    ///
    pub fn parse_circuit_declaration(&mut self) -> Result<Vec<CircuitMember>> {
        let mut members = Vec::new();
        let peeked = &self.peek()?.token;
        let mut last_variable = peeked == &Token::Function || peeked == &Token::At;
        let (mut semi_colons, mut commas) = (false, false);
        while self.eat(Token::RightCurly).is_none() {
            if !last_variable {
                let (variable, last) = self.parse_member_variable_declaration()?;

                members.push(variable);

                let peeked = &self.peek()?;
                if peeked.token == Token::Semicolon {
                    if commas {
                        return Err(ParserError::mixed_commas_and_semicolons(&peeked.span).into());
                    }

                    semi_colons = true;
                    self.expect(Token::Semicolon)?;
                } else {
                    if semi_colons {
                        return Err(ParserError::mixed_commas_and_semicolons(&peeked.span).into());
                    }

                    commas = true;
                    self.eat(Token::Comma);
                }

                if last {
                    last_variable = last;
                }
            } else {
                let function = self.parse_member_function_declaration()?;
                members.push(function);
            }
        }

        Ok(members)
    }

    ///
    /// Returns a [`CircuitMember`] AST node if the next tokens represent a circuit member variable.
    ///
    pub fn parse_member_variable_declaration(&mut self) -> Result<(CircuitMember, bool)> {
        let name = self.expect_ident()?;
        self.expect(Token::Colon)?;
        let type_ = self.parse_type()?.0;

        let peeked = &self.peek()?.token;
        if peeked == &Token::Function || peeked == &Token::At || peeked == &Token::RightCurly {
            return Ok((CircuitMember::CircuitVariable(name, type_), true));
        } else if peeked == &Token::Comma || peeked == &Token::Semicolon {
            let peeked = &self.peek_next()?.token;
            if peeked == &Token::Function || peeked == &Token::At || peeked == &Token::RightCurly {
                return Ok((CircuitMember::CircuitVariable(name, type_), true));
            }
        }

        Ok((CircuitMember::CircuitVariable(name, type_), false))
    }

    ///
    /// Returns a [`CircuitMember`] AST node if the next tokens represent a circuit member function.
    ///
    pub fn parse_member_function_declaration(&mut self) -> Result<CircuitMember> {
        let peeked = self.peek()?.clone();
        if peeked.token == Token::Function || peeked.token == Token::At {
            let function = self.parse_function_declaration()?;
            Ok(CircuitMember::CircuitFunction(function.1))
        } else {
            return Err(ParserError::unexpected(
                &peeked.token,
                [Token::Function, Token::At]
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
    pub fn parse_circuit(&mut self) -> Result<(String, Circuit)> {
        self.expect(Token::Circuit)?;
        let name = self.expect_ident()?;
        self.expect(Token::LeftCurly)?;
        let members = self.parse_circuit_declaration()?;

        Ok((
            name.name.to_string(),
            Circuit {
                circuit_name: name,
                core_mapping: std::cell::RefCell::new(None),
                members,
            },
        ))
    }

    ///
    /// Returns a [`FunctionInput`] AST node if the next tokens represent a function parameter.
    ///
    pub fn parse_function_parameters(&mut self) -> Result<FunctionInput> {
        let const_ = self.eat(Token::Const);
        let mutable = self.eat(Token::Mut);
        let mut name = if let Some(token) = self.eat(Token::LittleSelf) {
            Identifier {
                name: token.token.to_string().into(),
                span: token.span,
            }
        } else {
            self.expect_ident()?
        };
        if name.name.as_ref() == "self" {
            if let Some(mutable) = &mutable {
                // Handle `mut self`.
                name.span = &mutable.span + &name.span;
                name.name = "mut self".to_string().into();
                return Ok(FunctionInput::MutSelfKeyword(MutSelfKeyword { identifier: name }));
            } else if let Some(const_) = &const_ {
                // Handle `const self`.
                name.span = &const_.span + &name.span;
                name.name = "const self".to_string().into();
                return Ok(FunctionInput::ConstSelfKeyword(ConstSelfKeyword { identifier: name }));
            }
            // Handle `self`.
            return Ok(FunctionInput::SelfKeyword(SelfKeyword { identifier: name }));
        }

        if let Some(mutable) = &mutable {
            return Err(ParserError::mut_function_input(&(&mutable.span + &name.span)).into());
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

    ///
    /// Returns an [`(Identifier, Function)`] AST node if the next tokens represent a function name
    /// and function definition.
    ///
    pub fn parse_function_declaration(&mut self) -> Result<(String, Function)> {
        let mut annotations = Vec::new();
        while self.peek_token().as_ref() == &Token::At {
            annotations.push(self.parse_annotation()?);
        }
        let start = self.expect(Token::Function)?;
        let name = self.expect_ident()?;
        self.expect(Token::LeftParen)?;
        let mut inputs = Vec::new();
        while self.eat(Token::RightParen).is_none() {
            let input = self.parse_function_parameters()?;
            inputs.push(input);
            if self.eat(Token::Comma).is_none() {
                self.expect(Token::RightParen)?;
                break;
            }
        }
        let output = if self.eat(Token::Arrow).is_some() {
            Some(self.parse_type()?.0)
        } else {
            None
        };
        let block = self.parse_block()?;
        Ok((
            name.name.to_string(),
            Function {
                annotations,
                identifier: name,
                input: inputs,
                output,
                span: start + block.span.clone(),
                block,
            },
        ))
    }

    ///
    /// Returns an [`(String, DefinitionStatement)`] AST node if the next tokens represent a global
    /// const definition statement and assignment.
    ///
    pub fn parse_global_const_declaration(&mut self) -> Result<(String, DefinitionStatement)> {
        let statement = self.parse_definition_statement()?;
        let variable_names = statement
            .variable_names
            .iter()
            .map(|variable_name| variable_name.identifier.name.to_string())
            .collect::<Vec<String>>()
            .join(",");

        Ok((variable_names, statement))
    }

    ///
    /// Returns an [`(String, Alias)`] AST node if the next tokens represent a global
    /// const definition statement and assignment.
    ///
    pub fn parse_type_alias(&mut self) -> Result<(String, Alias)> {
        self.expect(Token::Type)?;
        let name = self.expect_ident()?;
        self.expect(Token::Assign)?;
        let (type_, _) = self.parse_type()?;
        self.expect(Token::Semicolon)?;

        Ok((
            name.name.to_string(),
            Alias {
                represents: type_,
                name,
            },
        ))
    }
}
