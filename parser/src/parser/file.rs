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

use crate::KEYWORD_TOKENS;

use super::*;

impl ParserContext {
    ///
    /// Returns a [`Program`] AST if all tokens can be consumed and represent a valid Leo program.
    ///
    pub fn parse_program(&mut self) -> SyntaxResult<Program> {
        let mut imports = Vec::new();
        let mut circuits = IndexMap::new();
        let mut functions = IndexMap::new();
        let mut global_consts = IndexMap::new();
        // let mut tests = IndexMap::new();

        while self.has_next() {
            let token = self.peek()?;
            match &token.token {
                Token::Import => {
                    imports.push(self.parse_import()?);
                }
                Token::Circuit => {
                    let (id, circuit) = self.parse_circuit()?;
                    circuits.insert(id, circuit);
                }
                Token::Function | Token::At => {
                    let (id, function) = self.parse_function()?;
                    functions.insert(id, function);
                }
                Token::Ident(ident) if ident == "test" => {
                    return Err(SyntaxError::DeprecatedError(DeprecatedError::test_function(
                        &token.span,
                    )));
                    // self.expect(Token::Test)?;
                    // let (id, function) = self.parse_function()?;
                    // tests.insert(id, TestFunction {
                    //     function,
                    //     input_file: None,
                    // });
                }
                Token::Const => {
                    let statement = self.parse_definition_statement()?;
                    let variable_names = statement
                        .variable_names
                        .iter()
                        .fold("".to_string(), |joined, variable_name| {
                            format!("{}, {}", joined, variable_name.identifier.name)
                        });
                    global_consts.insert(variable_names, statement);
                }
                _ => {
                    return Err(SyntaxError::unexpected(
                        &token.token,
                        &[
                            Token::Import,
                            Token::Circuit,
                            Token::Function,
                            Token::Ident("test".to_string()),
                            Token::At,
                        ],
                        &token.span,
                    ));
                }
            }
        }
        Ok(Program {
            name: String::new(),
            expected_input: Vec::new(),
            imports,
            circuits,
            functions,
            global_consts,
        })
    }

    ///
    /// Returns an [`Annotation`] AST node if the next tokens represent a supported annotation.
    ///
    pub fn parse_annotation(&mut self) -> SyntaxResult<Annotation> {
        let start = self.expect(Token::At)?;
        let name = self.expect_ident()?;
        if name.name == "context" {
            return Err(SyntaxError::DeprecatedError(DeprecatedError::context_annotation(
                &name.span,
            )));
        }
        let end_span;
        let arguments = if self.eat(Token::LeftParen).is_some() {
            let mut args = Vec::new();
            loop {
                if let Some(end) = self.eat(Token::RightParen) {
                    end_span = end.span;
                    break;
                }
                if let Some(ident) = self.eat_identifier() {
                    args.push(ident.name);
                } else if let Some((int, _)) = self.eat_int() {
                    args.push(int.value);
                } else {
                    let token = self.peek()?;
                    return Err(SyntaxError::unexpected_str(&token.token, "ident or int", &token.span));
                }
                if self.eat(Token::Comma).is_none() {
                    end_span = self.expect(Token::RightParen)?;
                    break;
                }
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
    pub fn parse_package_accesses(&mut self) -> SyntaxResult<Vec<PackageAccess>> {
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
        Ok(out)
    }

    ///
    /// Returns a [`PackageAccess`] AST node if the next tokens represent a package access expression
    /// within an import statement.
    ///
    pub fn parse_package_access(&mut self) -> SyntaxResult<PackageAccess> {
        if let Some(SpannedToken { span, .. }) = self.eat(Token::Mul) {
            Ok(PackageAccess::Star(span))
        } else {
            let name = self.expect_ident()?;
            if self.peek()?.token == Token::Dot {
                self.backtrack(SpannedToken {
                    token: Token::Ident(name.name),
                    span: name.span,
                });
                Ok(match self.parse_package_or_packages()? {
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
    pub fn parse_package_name(&mut self) -> SyntaxResult<Identifier> {
        // Build the package name, starting with valid characters up to a dash `-` (Token::Minus).
        let mut base = self.expect_loose_identifier()?;

        // Build the rest of the package name including dashes.
        loop {
            match &self.peek()?.token {
                Token::Minus => {
                    let span = self.expect(Token::Minus)?;
                    base.name += "-";
                    base.span = base.span + span;
                    let next = self.expect_loose_identifier()?;
                    base.name += &next.name;
                    base.span = base.span + next.span;
                }
                Token::Int(_) => {
                    let (num, span) = self.eat_int().unwrap();
                    base.name += &num.value;
                    base.span = base.span + span;
                }
                Token::Ident(_) => {
                    let next = self.expect_ident()?;
                    base.name += &next.name;
                    base.span = base.span + next.span;
                }
                x if KEYWORD_TOKENS.contains(&x) => {
                    let next = self.expect_loose_identifier()?;
                    base.name += &next.name;
                    base.span = base.span + next.span;
                }
                _ => break,
            }
        }

        // Return an error if the package name contains a keyword.
        if let Some(token) = KEYWORD_TOKENS.iter().find(|x| x.to_string() == base.name) {
            return Err(SyntaxError::unexpected_str(token, "package name", &base.span));
        }

        // Return an error if the package name contains invalid characters.
        if !base
            .name
            .chars()
            .all(|x| x.is_ascii_lowercase() || x.is_ascii_digit() || x == '-' || x == '_')
        {
            return Err(SyntaxError::invalid_package_name(&base.span));
        }

        // Return the package name.
        Ok(base)
    }

    ///
    /// Returns a [`PackageOrPackages`] AST node if the next tokens represent a valid package import
    /// with accesses.
    ///
    pub fn parse_package_or_packages(&mut self) -> SyntaxResult<PackageOrPackages> {
        let package_name = self.parse_package_name()?;
        self.expect(Token::Dot)?;
        if self.peek()?.token == Token::LeftParen {
            let accesses = self.parse_package_accesses()?;
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
    pub fn parse_import(&mut self) -> SyntaxResult<ImportStatement> {
        self.expect(Token::Import)?;
        let package_or_packages = self.parse_package_or_packages()?;
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
    pub fn parse_circuit_member(&mut self) -> SyntaxResult<CircuitMember> {
        let peeked = &self.peek()?.token;
        if peeked == &Token::Function || peeked == &Token::At {
            let function = self.parse_function()?;
            Ok(CircuitMember::CircuitFunction(function.1))
        } else {
            // circuit variable
            let name = self.expect_ident()?;
            self.expect(Token::Colon)?;
            let type_ = self.parse_type()?.0;
            self.eat(Token::Comma);
            Ok(CircuitMember::CircuitVariable(name, type_))
        }
    }

    ///
    /// Returns an [`(Identifier, Circuit)`] tuple of AST nodes if the next tokens represent a
    /// circuit name and definition statement.
    ///
    pub fn parse_circuit(&mut self) -> SyntaxResult<(Identifier, Circuit)> {
        self.expect(Token::Circuit)?;
        let name = self.expect_ident()?;
        self.expect(Token::LeftCurly)?;
        let mut members = Vec::new();
        while self.eat(Token::RightCurly).is_none() {
            let member = self.parse_circuit_member()?;
            members.push(member);
        }
        Ok((name.clone(), Circuit {
            circuit_name: name,
            members,
        }))
    }

    ///
    /// Returns a [`FunctionInput`] AST node if the next tokens represent a function parameter.
    ///
    pub fn parse_function_input(&mut self) -> SyntaxResult<FunctionInput> {
        if let Some(token) = self.eat(Token::Input) {
            return Ok(FunctionInput::InputKeyword(InputKeyword {
                identifier: Identifier {
                    name: token.token.to_string(),
                    span: token.span,
                },
            }));
        }
        let const_ = self.eat(Token::Const);
        let mutable = self.eat(Token::Mut);
        let mut name = if let Some(token) = self.eat(Token::LittleSelf) {
            Identifier {
                name: token.token.to_string(),
                span: token.span,
            }
        } else {
            self.expect_ident()?
        };
        if name.name == "self" {
            if let Some(mutable) = &mutable {
                name.span = &mutable.span + &name.span;
                name.name = "mut self".to_string();
                return Ok(FunctionInput::MutSelfKeyword(MutSelfKeyword { identifier: name }));
            } else if let Some(const_) = &const_ {
                name.span = &const_.span + &name.span;
                name.name = "const self".to_string();
                return Ok(FunctionInput::ConstSelfKeyword(ConstSelfKeyword { identifier: name }));
            }
            return Ok(FunctionInput::SelfKeyword(SelfKeyword { identifier: name }));
        }

        if let Some(mutable) = &mutable {
            return Err(SyntaxError::DeprecatedError(DeprecatedError::mut_function_input(
                &mutable.span + &name.span,
            )));
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
    pub fn parse_function(&mut self) -> SyntaxResult<(Identifier, Function)> {
        let mut annotations = Vec::new();
        while self.peek()?.token == Token::At {
            annotations.push(self.parse_annotation()?);
        }
        let start = self.expect(Token::Function)?;
        let name = self.expect_ident()?;
        self.expect(Token::LeftParen)?;
        let mut inputs = Vec::new();
        while self.eat(Token::RightParen).is_none() {
            let input = self.parse_function_input()?;
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
        Ok((name.clone(), Function {
            annotations,
            identifier: name,
            input: inputs,
            output,
            span: start + block.span.clone(),
            block,
        }))
    }
}
