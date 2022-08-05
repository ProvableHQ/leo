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
use crate::parse_ast;
use leo_errors::{CompilerError, ParserError, ParserWarning, Result};
use leo_span::source_map::FileName;
use leo_span::sym;
use leo_span::symbol::with_session_globals;

use std::fs;

impl ParserContext<'_> {
    /// Returns a [`Program`] AST if all tokens can be consumed and represent a valid Leo program.
    pub fn parse_program(&mut self) -> Result<Program> {
        let mut imports = IndexMap::new();
        let mut functions = IndexMap::new();
        let mut circuits = IndexMap::new();

        // TODO: Condense logic
        while self.has_next() {
            match &self.token.token {
                Token::Import => {
                    let (id, import) = self.parse_import()?;
                    imports.insert(id, import);
                }
                Token::Circuit | Token::Record => {
                    let (id, circuit) = self.parse_circuit()?;
                    circuits.insert(id, circuit);
                }
                Token::At => {
                    let (id, function) = self.parse_function()?;
                    functions.insert(id, function);
                }
                Token::Const if self.peek_is_function() => {
                    let (id, function) = self.parse_function()?;
                    functions.insert(id, function);
                }
                Token::Identifier(sym::test) => return Err(ParserError::test_function(self.token.span).into()),
                Token::Function => {
                    let (id, function) = self.parse_function()?;
                    functions.insert(id, function);
                }
                _ => return Err(Self::unexpected_item(&self.token).into()),
            }
        }
        Ok(Program {
            name: String::new(),
            network: String::new(),
            expected_input: Vec::new(),
            imports,
            functions,
            circuits,
        })
    }

    fn unexpected_item(token: &SpannedToken) -> ParserError {
        ParserError::unexpected(
            &token.token,
            [Token::Function, Token::Circuit, Token::Identifier(sym::test)]
                .iter()
                .map(|x| format!("'{}'", x))
                .collect::<Vec<_>>()
                .join(", "),
            token.span,
        )
    }

    /// Parses an import statement `import foo.leo;`.
    pub(super) fn parse_import(&mut self) -> Result<(Identifier, Program)> {
        // Parse `import`.
        let _start = self.expect(&Token::Import)?;

        // Parse `foo`.
        let import_name = self.expect_identifier()?;

        // Parse `.leo`.
        self.expect(&Token::Dot)?;
        let leo_file_extension = self.expect_identifier()?;

        // Throw error for non-leo files.
        if leo_file_extension.name.ne(&sym::leo) {
            return Err(ParserError::leo_imports_only(leo_file_extension, self.token.span).into());
        }
        let _end = self.expect(&Token::Semicolon)?;

        // Tokenize and parse import file.
        // Todo: move this to a different module.
        let mut import_file_path =
            std::env::current_dir().map_err(|err| CompilerError::cannot_open_cwd(err, self.token.span))?;
        import_file_path.push("imports");
        import_file_path.push(format!("{}.leo", import_name.name));

        // Throw an error if the import file doesn't exist.
        if !import_file_path.exists() {
            return Err(CompilerError::import_not_found(import_file_path.display(), self.prev_token.span).into());
        }

        // Read the import file into string.
        // Todo: protect against cyclic imports.
        let program_string =
            fs::read_to_string(&import_file_path).map_err(|e| CompilerError::file_read_error(&import_file_path, e))?;

        // Create import file name.
        let name: FileName = FileName::Real(import_file_path);

        // Register the source (`program_string`) in the source map.
        let prg_sf = with_session_globals(|s| s.source_map.new_source(&program_string, name));

        // Use the parser to construct the imported abstract syntax tree (ast).
        let program_ast = parse_ast(self.handler, &prg_sf.src, prg_sf.start_pos)?;

        Ok((import_name, program_ast.into_repr()))
    }

    /// Returns a [`Vec<CircuitMember>`] AST node if the next tokens represent a circuit member variable
    /// or circuit member function or circuit member constant.
    fn parse_circuit_members(&mut self) -> Result<(Vec<CircuitMember>, Span)> {
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

        Ok((members, span))
    }

    /// Parses `IDENT: TYPE`.
    pub(super) fn parse_typed_ident(&mut self) -> Result<(Identifier, Type)> {
        let name = self.expect_identifier()?;
        self.expect(&Token::Colon)?;
        let type_ = self.parse_type()?.0;

        Ok((name, type_))
    }

    /// Returns a [`CircuitMember`] AST node if the next tokens represent a circuit member static constant.
    fn parse_const_member_variable_declaration(&mut self) -> Result<CircuitMember> {
        self.expect(&Token::Static)?;
        self.expect(&Token::Const)?;

        // `IDENT: TYPE = EXPR`:
        let (_name, _type_) = self.parse_typed_ident()?;
        self.expect(&Token::Assign)?;
        let expr = self.parse_expression()?;

        self.expect(&Token::Semicolon)?;

        // CAUTION: function members are unstable for testnet3.
        Err(ParserError::circuit_constants_unstable(expr.span()).into())

        // Ok(CircuitMember::CircuitConst(name, type_, expr))
    }

    /// Returns a [`CircuitMember`] AST node if the next tokens represent a circuit member variable.
    fn parse_member_variable_declaration(&mut self) -> Result<CircuitMember> {
        let (name, type_) = self.parse_typed_ident()?;

        Ok(CircuitMember::CircuitVariable(name, type_))
    }

    /// Returns a [`CircuitMember`] AST node if the next tokens represent a circuit member function.
    fn parse_member_function_declaration(&mut self) -> Result<CircuitMember> {
        if self.peek_is_function() {
            // CAUTION: function members are unstable for testnet3.
            let function = self.parse_function()?;

            Err(ParserError::circuit_functions_unstable(function.1.span()).into())
            // Ok(CircuitMember::CircuitFunction(Box::new(function.1)))
        } else {
            Err(Self::unexpected_item(&self.token).into())
        }
    }

    /// Parses a circuit or record definition, e.g., `circit Foo { ... }` or `record Foo { ... }`.
    pub(super) fn parse_circuit(&mut self) -> Result<(Identifier, Circuit)> {
        let is_record = matches!(&self.token.token, Token::Record);
        let start = self.expect_any(&[Token::Circuit, Token::Record])?;
        let circuit_name = self.expect_identifier()?;

        self.expect(&Token::LeftCurly)?;
        let (members, end) = self.parse_circuit_members()?;

        Ok((
            circuit_name,
            Circuit {
                identifier: circuit_name,
                members,
                is_record,
                span: start + end,
            },
        ))
    }

    /// Returns a [`ParamMode`] AST node if the next tokens represent a function parameter mode.
    pub(super) fn parse_function_parameter_mode(&mut self) -> Result<ParamMode> {
        let private = self.eat(&Token::Private).then(|| self.prev_token.span);
        let public = self.eat(&Token::Public).then(|| self.prev_token.span);
        let constant = self.eat(&Token::Constant).then(|| self.prev_token.span);
        let const_ = self.eat(&Token::Const).then(|| self.prev_token.span);

        if let Some(span) = const_ {
            self.emit_warning(ParserWarning::const_parameter_or_input(span));
        }

        match (private, public, constant, const_) {
            (None, None, None, None) => Ok(ParamMode::None),
            (Some(_), None, None, None) => Ok(ParamMode::Private),
            (None, Some(_), None, None) => Ok(ParamMode::Public),
            (None, None, Some(_), None) => Ok(ParamMode::Const),
            (None, None, None, Some(_)) => Ok(ParamMode::Const),
            _ => {
                let mut spans = [private, public, constant, const_].into_iter().flatten();

                // There must exist at least one mode, since the none case is handled above.
                let starting_span = spans.next().unwrap();
                // Sum the spans.
                let summed_span = spans.fold(starting_span, |span, next| span + next);
                // Emit an error.
                Err(ParserError::inputs_multiple_variable_types_specified(summed_span).into())
            }
        }
    }

    /// Returns a [`FunctionInput`] AST node if the next tokens represent a function parameter.
    fn parse_function_parameter(&mut self) -> Result<FunctionInput> {
        let mode = self.parse_function_parameter_mode()?;
        let (name, type_) = self.parse_typed_ident()?;
        Ok(FunctionInput::new(name, mode, type_, name.span))
    }

    /// Returns `true` if the next token is Function or if it is a Const followed by Function.
    /// Returns `false` otherwise.
    fn peek_is_function(&self) -> bool {
        matches!(
            (&self.token.token, self.look_ahead(1, |t| &t.token)),
            (Token::Function, _) | (Token::Const, Token::Function)
        )
    }

    /// Returns an [`Annotation`] AST node if the next tokens represent an annotation.
    fn parse_annotation(&mut self) -> Result<Annotation> {
        // Parse the `@` symbol and identifier.
        let start = self.expect(&Token::At)?;
        let identifier = self.expect_identifier()?;
        let span = start + identifier.span;

        // TODO: Verify that this check is sound.
        // Check that there is no whitespace in between the `@` symbol and identifier.
        match identifier.span.hi.0 - start.lo.0 > 1 + identifier.name.to_string().len() as u32 {
            true => Err(ParserError::space_in_annotation(span).into()),
            false => Ok(Annotation { identifier, span }),
        }
    }

    /// Returns an [`(Identifier, Function)`] AST node if the next tokens represent a function name
    /// and function definition.
    fn parse_function(&mut self) -> Result<(Identifier, Function)> {
        // TODO: Handle dangling annotations.
        // TODO: Handle duplicate annotations.
        // Parse annotations, if they exist.
        let mut annotations = Vec::new();
        while self.look_ahead(0, |t| &t.token) == &Token::At {
            annotations.push(self.parse_annotation()?)
        }
        // Parse `function IDENT`.
        let start = self.expect(&Token::Function)?;
        let name = self.expect_identifier()?;

        // Parse parameters.
        let (inputs, ..) = self.parse_paren_comma_list(|p| p.parse_function_parameter().map(Some))?;

        // Parse return type.
        self.expect(&Token::Arrow)?;
        self.disallow_circuit_construction = true;
        let output = self.parse_type()?.0;
        self.disallow_circuit_construction = false;

        // Parse the function body.
        let block = self.parse_block()?;

        Ok((
            name,
            Function {
                annotations,
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
