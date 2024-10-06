// Copyright (C) 2019-2024 Aleo Systems Inc.
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
use leo_ast::{Annotation, Identifier, Input, Mode, Output, ProgramId, Type, Variant};
use leo_errors::{ParserError, Result};
#[derive(Clone)]
pub enum LeoBlock {
    Import,
    Program,
    None
}
#[derive(Clone)]
pub enum ProgramBlock {
    Mapping,
    Const,
    Composite,
    None
}

impl<N: Network> ParserContext<'_, N> {
    /// Returns a [`Program`] AST if all tokens can be consumed and represent a valid Leo program.
    pub fn parse_program(&mut self) -> Result<Program> {
        let mut imports = IndexMap::new();
        let mut program_scopes = IndexMap::new();
        let mut comments = vec![];
        // TODO: Remove restrictions on multiple program scopes
        let mut parsed_program_scope = false;
        let mut block = LeoBlock::None;
        while self.has_next() {
            match &self.token.token {
                Token::Import => {
                    let (id, import) = self.parse_import()?;
                    imports.insert(id, (import.0, import.1, comments.clone(), Comment::None));
                    comments.clear();
                    block = LeoBlock::Import;
                }
                Token::Program => {
                    match parsed_program_scope {
                        // Only one program scope is allowed per file.
                        true => {
                            return Err(ParserError::only_one_program_scope_is_allowed(self.token.span).into());
                        }
                        false => {
                            parsed_program_scope = true;
                            let program_scope = self.parse_program_scope()?;
                            program_scopes.insert(program_scope.program_id.name.name,(program_scope, comments.clone()));
                            comments.clear();
                            block = LeoBlock::Program;
                        }
                    }
                }
                Token::CommentLine(string) => {
                    comments.push(Comment::CommentLine(string.clone()));
                    self.bump();
                }
                Token::CommentBlock(string) => {
                    comments.push(Comment::CommentBlock(string.clone()));
                    self.bump();
                }
                Token::_CommentLine(_) | Token::_CommentBlock(_) => {
                    if matches!(block, LeoBlock::Import) {
                        if let Some((key, value)) = imports.last() {
                            imports.insert(
                                key.clone(), 
                                (value.0.clone(), value.1, value.2.clone(), self.get_comment(self.token.token.clone()))
                            );
                        }
                    }
                    self.bump();
                }
                _ => return Err(Self::unexpected_item(&self.token, &[Token::Import, Token::Program]).into()),
            }
        }

        // Requires that at least one program scope is present.
        if !parsed_program_scope {
            return Err(ParserError::missing_program_scope(self.token.span).into());
        }
        Ok(Program { imports, stubs: IndexMap::new(), program_scopes})
    }

    fn unexpected_item(token: &SpannedToken, expected: &[Token]) -> ParserError {
        ParserError::unexpected(
            &token.token,
            expected.iter().map(|x| format!("'{x}'")).collect::<Vec<_>>().join(", "),
            token.span,
        )
    }

    // TODO: remove import resolution from parser.
    /// Parses an import statement `import foo.leo;`.
    pub(super) fn parse_import(&mut self) -> Result<(Symbol, (Program, Span))> {
        // Parse `import`.
        let start = self.expect(&Token::Import)?;

        // Parse `foo`.
        let import_name = self.expect_identifier()?;

        // Parse `.`.
        self.expect(&Token::Dot)?;

        // Parse network, which currently must be `aleo`.
        if !self.eat(&Token::Aleo) {
            // Throw error for non-aleo networks.
            return Err(ParserError::invalid_network(self.token.span).into());
        }

        let end = self.expect(&Token::Semicolon)?;

        // Return the import name and the span.
        Ok((import_name.name, (Program::default(), start + end)))
    }

    /// Parses a program scope `program foo.aleo { ... }`.
    fn parse_program_scope(&mut self) -> Result<ProgramScope> {
        // Parse `program` keyword.
        let start = self.expect(&Token::Program)?;

        // Parse the program name.
        let name = self.expect_identifier()?;

        // Set the program name in the context.
        self.program_name = Some(name.name);

        // Parse the `.`.
        self.expect(&Token::Dot)?;

        // Parse the program network, which must be `aleo`, otherwise throw parser error.
        self.expect(&Token::Aleo).map_err(|_| ParserError::invalid_network(self.token.span))?;

        // Construct the program id.
        let program_id =
            ProgramId { name, network: Identifier::new(Symbol::intern("aleo"), self.node_builder.next_id()) };

        // Parse `{`.
        self.expect(&Token::LeftCurly)?;

        let mut header_comment = Comment::None;
        let mut comments = vec![];
        
        let mut block: ProgramBlock = ProgramBlock::None;
        // Parse the body of the program scope.
        let mut consts: Vec<(Symbol, (ConstDeclaration,  Vec<Comment>))> = Vec::new();
        let (mut transitions, mut functions) = (Vec::new(), Vec::new());
        let mut structs: Vec<(Symbol, (Composite, Vec<Comment>))> = Vec::new();
        let mut mappings: Vec<(Symbol, (Mapping, Vec<Comment>))> = Vec::new();

        while self.has_next() {
            match &self.token.token {
                Token::CommentLine(string) => {
                    comments.push(Comment::CommentLine(string.clone()));
                    self.bump();
                }
                Token::CommentBlock(string) => {
                    comments.push(Comment::CommentBlock(string.clone()));
                    self.bump();
                }
                Token::_CommentLine(_) | Token::_CommentBlock(_) => {
                    let comment =  self.get_comment(self.token.token.clone());
                    if matches!(block, ProgramBlock::Const) {
                        if let Some((_, value)) = consts.last_mut() {
                            value.0.comment = comment;
                        }
                    }else if matches!(block, ProgramBlock::Mapping) {
                        if let Some((_, value)) = mappings.last_mut() {
                            value.0.comment = comment;
                        }
                    }else if matches!(block, ProgramBlock::Composite) {
                        if let Some((_, value)) = structs.last_mut() {
                            value.0.comment = comment;
                        }
                    }else if header_comment == Comment::None{
                        header_comment = comment.clone();
                    }
                    self.bump();
                }
                
                Token::Const => {
                    let declaration = self.parse_const_declaration_statement()?;
                    consts.push((Symbol::intern(&declaration.place.to_string()), (declaration, comments.clone())));
                    comments.clear();
                    block = ProgramBlock::Const;
                }
                Token::Struct | Token::Record => {
                    let (id, struct_) = self.parse_struct()?;
                    structs.push((id, (struct_, comments.clone())));
                    comments.clear();
                    block = ProgramBlock::Composite;
                }
                Token::Mapping => {
                    let (id, mapping) = self.parse_mapping()?;
                    mappings.push((id, (mapping, comments.clone())));
                    comments.clear();
                    block = ProgramBlock::Mapping;
                }
                Token::At | Token::Async | Token::Function | Token::Transition | Token::Inline => {
                    let (id, function) = self.parse_function()?;

                    // Partition into transitions and functions so that we don't have to sort later.
                    if function.variant.is_transition() {
                        transitions.push((id, (function, comments.clone())));
                    } else {
                        functions.push((id, (function, comments.clone())));
                    }
                }
                Token::RightCurly => break,
                _ => {
                    return Err(Self::unexpected_item(&self.token, &[
                        Token::Const,
                        Token::Struct,
                        Token::Record,
                        Token::Mapping,
                        Token::At,
                        Token::Async,
                        Token::Function,
                        Token::Transition,
                        Token::Inline,
                        Token::RightCurly,
                    ])
                    .into());
                }
            }
        }

        // Parse `}`.
        let end = self.expect(&Token::RightCurly)?;

        Ok(ProgramScope {
            program_id,
            consts,
        //    functions: vec![],
            functions: [transitions, functions].concat(),
            structs,
            mappings,
            span: start + end,
            comment: header_comment.clone()
        })
    }

    /// Returns a [`Vec<Member>`] AST node if the next tokens represent a struct member.
    fn parse_struct_members(&mut self) -> Result<(Vec<(Member, Vec<Comment>)>, Span, Comment)> {
        let mut members: Vec<(Member, Vec<Comment>)> = Vec::new();
        let mut header_comment = Comment::None;
        let mut comments: Vec<Comment> = vec![];
        let (mut semi_colons, mut commas) = (false, false);
        while !self.check(&Token::RightCurly) {
            if self.is_inline_comment(self.token.token.clone()) {
                if header_comment == Comment::None {
                    header_comment = self.get_comment(self.token.token.clone());
                }else {
                    if let Some(member) = members.last_mut() {
                        member.0.comment = self.get_comment(self.token.token.clone());
                    }
                }
                self.bump();
                continue;
            }
            while self.is_comment(self.token.token.clone()) {
                comments.push(self.get_comment(self.token.token.clone()));
                self.bump();
            }
            let variable: Member = self.parse_member_variable_declaration()?;
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
            members.push((variable, comments.clone()));
            comments.clear();
        }
        let span = self.expect(&Token::RightCurly)?;
        Ok((members, span, header_comment))
    }

    /// Parses `IDENT: TYPE`.
    pub(super) fn parse_typed_ident(&mut self) -> Result<(Identifier, Type, Span)> {
        let name = self.expect_identifier()?;
        self.expect(&Token::Colon)?;
        let (type_, span) = self.parse_type()?;

        Ok((name, type_, name.span + span))
    }

    /// Returns a [`Member`] AST node if the next tokens represent a struct member variable.
    fn parse_member_variable_declaration(&mut self) -> Result<Member> {
        let mode = self.parse_mode()?;
        let (identifier, type_, span) = self.parse_typed_ident()?;
        Ok(Member { mode, identifier, type_, span, id: self.node_builder.next_id(), comment: Comment::None })
    }

    /// Parses a struct or record definition, e.g., `struct Foo { ... }` or `record Foo { ... }`.
    pub(super) fn parse_struct(&mut self) -> Result<(Symbol, Composite)> {
        let is_record = matches!(&self.token.token, Token::Record);
        let start = self.expect_any(&[Token::Struct, Token::Record])?;
        // Check if using external type
        let file_type = self.look_ahead(1, |t| &t.token);
        if self.token.token == Token::Dot && (file_type == &Token::Aleo) {
            return Err(ParserError::cannot_declare_external_struct(self.token.span).into());
        }

        let struct_name = self.expect_identifier()?;

        self.expect(&Token::LeftCurly)?;
        let (members, end, comment) = self.parse_struct_members()?;
        // Only provide a program name for records.
        let external = if is_record { self.program_name } else { None };
        Ok((struct_name.name, Composite {
            identifier: struct_name,
            members,
            external,
            is_record,
            span: start + end,
            id: self.node_builder.next_id(),
            comment
        }))
    }

    /// Parses a mapping declaration, e.g. `mapping balances: address => u128`.
    pub(super) fn parse_mapping(&mut self) -> Result<(Symbol, Mapping)> {
        let start = self.expect(&Token::Mapping)?;
        let identifier = self.expect_identifier()?;
        self.expect(&Token::Colon)?;
        let (key_type, _) = self.parse_type()?;
        self.expect(&Token::BigArrow)?;
        let (value_type, _) = self.parse_type()?;
        let end = self.expect(&Token::Semicolon)?;
        Ok((identifier.name, Mapping {
            identifier,
            key_type,
            value_type,
            span: start + end,
            id: self.node_builder.next_id(),
            comment: Comment::None
        }))
    }

    // TODO: Return a span associated with the mode.
    /// Returns a [`ParamMode`] AST node if the next tokens represent a function parameter mode.
    pub(super) fn parse_mode(&mut self) -> Result<Mode> {
        let private = self.eat(&Token::Private).then_some(self.prev_token.span);
        let public = self.eat(&Token::Public).then_some(self.prev_token.span);
        let constant = self.eat(&Token::Constant).then_some(self.prev_token.span);

        match (private, public, constant) {
            (None, None, None) => Ok(Mode::None),
            (Some(_), None, None) => Ok(Mode::Private),
            (None, Some(_), None) => Ok(Mode::Public),
            (None, None, Some(_)) => Ok(Mode::Constant),
            _ => {
                let mut spans = [private, public, constant].into_iter().flatten();

                // There must exist at least one mode, since the none case is handled above.
                let starting_span = spans.next().unwrap();
                // Sum the spans.
                let summed_span = spans.fold(starting_span, |span, next| span + next);
                // Emit an error.
                Err(ParserError::inputs_multiple_variable_modes_specified(summed_span).into())
            }
        }
    }

    /// Returns an [`Input`] AST node if the next tokens represent a function input.
    fn parse_input(&mut self) -> Result<Input> {
        let mode = self.parse_mode()?;
        let name = self.expect_identifier()?;
        self.expect(&Token::Colon)?;

        let type_ = self.parse_type()?.0;

        Ok(Input { identifier: name, mode, type_, span: name.span, id: self.node_builder.next_id() })
    }

    /// Returns an [`Output`] AST node if the next tokens represent a function output.
    fn parse_output(&mut self) -> Result<Output> {
        let mode = self.parse_mode()?;
        let (type_, span) = self.parse_type()?;
        Ok(Output { mode, type_, span, id: self.node_builder.next_id() })
    }

    /// Returns an [`Annotation`] AST node if the next tokens represent an annotation.
    fn parse_annotation(&mut self) -> Result<Annotation> {
        // Parse the `@` symbol and identifier.
        let start = self.expect(&Token::At)?;
        let identifier = match self.token.token {
            Token::Program => {
                Identifier { name: sym::program, span: self.expect(&Token::Program)?, id: self.node_builder.next_id() }
            }
            _ => self.expect_identifier()?,
        };
        let span = start + identifier.span;

        // TODO: Verify that this check is sound.
        // Check that there is no whitespace or comments in between the `@` symbol and identifier.
        match identifier.span.hi.0 - start.lo.0 > 1 + identifier.name.to_string().len() as u32 {
            true => Err(ParserError::space_in_annotation(span).into()),
            false => Ok(Annotation { identifier, span, id: self.node_builder.next_id() }),
        }
    }

    /// Returns an [`(Identifier, Function)`] AST node if the next tokens represent a function name
    /// and function definition.
    fn parse_function(&mut self) -> Result<(Symbol, Function)> {
        // TODO: Handle dangling annotations.
        // Parse annotations, if they exist.
        let mut annotations = Vec::new();
        while self.look_ahead(0, |t| &t.token) == &Token::At {
            annotations.push(self.parse_annotation()?)
        }
        // Parse a potential async signifier.
        let (is_async, start_async) =
            if self.token.token == Token::Async { (true, self.expect(&Token::Async)?) } else { (false, Span::dummy()) };
        // Parse `<variant> IDENT`, where `<variant>` is `function`, `transition`, or `inline`.
        let (variant, start) = match self.token.token.clone() {
            Token::Inline => (Variant::Inline, self.expect(&Token::Inline)?),
            Token::Function => {
                (if is_async { Variant::AsyncFunction } else { Variant::Function }, self.expect(&Token::Function)?)
            }
            Token::Transition => (
                if is_async { Variant::AsyncTransition } else { Variant::Transition },
                self.expect(&Token::Transition)?,
            ),
            _ => self.unexpected("'function', 'transition', or 'inline'")?,
        };
        let name = self.expect_identifier()?;

        // Parse parameters.
        let (inputs, ..) = self.parse_paren_comma_list(|p| p.parse_input().map(Some))?;

        // Parse return type.
        let output = match self.eat(&Token::Arrow) {
            false => vec![],
            true => {
                self.disallow_struct_construction = true;
                let output = match self.peek_is_left_par() {
                    true => self.parse_paren_comma_list(|p| p.parse_output().map(Some))?.0,
                    false => vec![self.parse_output()?],
                };
                self.disallow_struct_construction = false;
                output
            }
        };

        // Parse the function body, which must be a block,
        // but we also allow no body and a semicolon instead (e.g. `fn foo(a:u8);`).
        let (_has_empty_block, block) = match &self.token.token {
            Token::LeftCurly => (false, self.parse_block()?),
            Token::Semicolon => {
                let semicolon = self.expect(&Token::Semicolon)?;
                // TODO: make a distinction between empty block and no block (i.e. semicolon)
                (true, Block { statements: Vec::new(), span: semicolon, id: self.node_builder.next_id() })
            }
            _ => self.unexpected("block or semicolon")?,
        };

        let span = if start_async == Span::dummy() { start + block.span } else { start_async + block.span };

        Ok((
            name.name,
            Function::new(annotations, variant, name, inputs, output, block, span, self.node_builder.next_id()),
        ))
    }

    pub fn get_comment(&mut self, token: Token) -> Comment{
        match token {
            Token::_CommentLine(string) => Comment::_CommentLine(string),
            Token::_CommentBlock(string) => Comment::_CommentBlock(string),
            Token::CommentLine(string) => Comment::CommentLine(string),
            Token::CommentBlock(string) => Comment::CommentBlock(string),
            _ =>  Comment::None
        }
    }
    pub fn is_inline_comment(&mut self, token: Token) -> bool {
        match token {
            Token::_CommentLine(_) => true,
            Token::_CommentBlock(_) => true,
            _ => false
        }
    }
    pub fn is_comment(&mut self, token: Token) -> bool {
        match token {
            Token::CommentLine(_) => true,
            Token::CommentBlock(_) => true,
            _ => false
        }
    }
}

use leo_span::{sym, Symbol};