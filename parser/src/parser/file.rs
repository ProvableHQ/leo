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
    pub fn parse_program(&mut self) -> SyntaxResult<Program> {
        let mut imports = vec![];
        let mut circuits = IndexMap::new();
        let mut functions = IndexMap::new();
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
            expected_input: vec![],
            imports,
            circuits,
            functions,
        })
    }

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
            let mut args = vec![];
            loop {
                if let Some(end) = self.eat(Token::RightParen) {
                    end_span = end.span;
                    break;
                }
                if let Some(ident) = self.eat_ident() {
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
            vec![]
        };
        Ok(Annotation {
            name,
            arguments,
            span: start + end_span,
        })
    }

    pub fn parse_package_accesses(&mut self) -> SyntaxResult<Vec<PackageAccess>> {
        let mut out = vec![];
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

    pub fn parse_package_name(&mut self) -> SyntaxResult<Identifier> {
        let mut base = self.expect_loose_ident()?;
        while let Some(token) = self.eat(Token::Minus) {
            if token.span.line_start == base.span.line_stop && token.span.col_start == base.span.col_stop {
                base.name += "-";
                base.span = base.span + token.span;
                let next = self.expect_loose_ident()?;
                base.name += &next.name;
                base.span = base.span + next.span;
            } else {
                break;
            }
        }
        if let Some(token) = KEYWORD_TOKENS.iter().find(|x| x.to_string() == base.name) {
            return Err(SyntaxError::unexpected_str(token, "package name", &base.span));
        }
        if !base
            .name
            .chars()
            .all(|x| x.is_ascii_lowercase() || x.is_ascii_digit() || x == '-' || x == '_')
        {
            return Err(SyntaxError::invalid_package_name(&base.span));
        }
        Ok(base)
    }

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

    pub fn parse_import(&mut self) -> SyntaxResult<ImportStatement> {
        self.expect(Token::Import)?;
        let package_or_packages = self.parse_package_or_packages()?;
        self.expect(Token::Semicolon)?;
        Ok(ImportStatement {
            span: package_or_packages.span().clone(),
            package_or_packages,
        })
    }

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

    pub fn parse_circuit(&mut self) -> SyntaxResult<(Identifier, Circuit)> {
        self.expect(Token::Circuit)?;
        let name = self.expect_ident()?;
        self.expect(Token::LeftCurly)?;
        let mut members = vec![];
        while self.eat(Token::RightCurly).is_none() {
            let member = self.parse_circuit_member()?;
            members.push(member);
        }
        Ok((name.clone(), Circuit {
            circuit_name: name,
            members,
        }))
    }

    pub fn parse_function_input(&mut self) -> SyntaxResult<FunctionInput> {
        if let Some(token) = self.eat(Token::Input) {
            return Ok(FunctionInput::InputKeyword(InputKeyword { span: token.span }));
        }
        let const_ = self.eat(Token::Const);
        let mutable = self.eat(Token::Mut);
        let name = if let Some(token) = self.eat(Token::LittleSelf) {
            Identifier {
                name: token.token.to_string(),
                span: token.span,
            }
        } else {
            self.expect_ident()?
        };
        if name.name == "self" {
            if const_.is_some() {
                //error
            }
            if let Some(mutable) = &mutable {
                return Ok(FunctionInput::MutSelfKeyword(MutSelfKeyword {
                    span: &mutable.span + &name.span,
                }));
            }
            return Ok(FunctionInput::SelfKeyword(SelfKeyword { span: name.span }));
        }
        self.expect(Token::Colon)?;
        let type_ = self.parse_type()?.0;
        Ok(FunctionInput::Variable(FunctionInputVariable {
            const_: const_.is_some(),
            mutable: mutable.is_some(),
            type_,
            span: name.span.clone(),
            identifier: name,
        }))
    }

    pub fn parse_function(&mut self) -> SyntaxResult<(Identifier, Function)> {
        let mut annotations = vec![];
        while self.peek()?.token == Token::At {
            annotations.push(self.parse_annotation()?);
        }
        let start = self.expect(Token::Function)?;
        let name = self.expect_ident()?;
        self.expect(Token::LeftParen)?;
        let mut inputs = vec![];
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
