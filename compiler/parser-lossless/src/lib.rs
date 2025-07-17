// Copyright (C) 2019-2025 Provable Inc.
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

//! The lossless syntax tree and parser for Leo.

use itertools::Itertools as _;
use lalrpop_util::lalrpop_mod;

use leo_errors::{Handler, LeoError, ParserError, Result};
use leo_span::Span;

lalrpop_mod!(pub grammar);

pub mod tokens;

use tokens::*;

/// A tag indicating the nature of a syntax node.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SyntaxKind {
    Whitespace,
    Linebreak,
    CommentLine,
    CommentBlock,

    Expression(ExpressionKind),
    StructMemberInitializer,

    Statement(StatementKind),
    Type(TypeKind),
    Token,

    Annotation,
    AnnotationMember,
    AnnotationList,

    Parameter,
    ParameterList,
    FunctionOutput,
    FunctionOutputs,
    Function,
    Constructor,

    ConstParameter,
    ConstParameterList,
    ConstArgumentList,

    StructDeclaration,
    StructMemberDeclaration,
    StructMemberDeclarationList,

    Mapping,

    GlobalConst,

    Import,
    MainContents,
    ModuleContents,
    ProgramDeclaration,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum IntegerLiteralKind {
    U8,
    U16,
    U32,
    U64,
    U128,

    I8,
    I16,
    I32,
    I64,
    I128,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum IntegerTypeKind {
    U8,
    U16,
    U32,
    U64,
    U128,

    I8,
    I16,
    I32,
    I64,
    I128,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TypeKind {
    Address,
    Array,
    Boolean,
    Composite,
    Field,
    Future,
    Group,
    Identifier,
    Integer(IntegerTypeKind),
    Mapping,
    Scalar,
    Signature,
    String,
    Tuple,
    Numeric,
    Unit,
}

impl From<TypeKind> for SyntaxKind {
    fn from(value: TypeKind) -> Self {
        SyntaxKind::Type(value)
    }
}

impl From<IntegerTypeKind> for TypeKind {
    fn from(value: IntegerTypeKind) -> Self {
        TypeKind::Integer(value)
    }
}

impl From<IntegerTypeKind> for SyntaxKind {
    fn from(value: IntegerTypeKind) -> Self {
        SyntaxKind::Type(TypeKind::Integer(value))
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExpressionKind {
    ArrayAccess,
    AssociatedConstant,
    AssociatedFunctionCall,
    Async,
    Array,
    Binary,
    Call,
    Cast,
    Path,
    Literal(LiteralKind),
    Locator,
    MemberAccess,
    MethodCall,
    Parenthesized,
    Repeat,
    // program.id, block.height, etc
    SpecialAccess,
    Struct,
    Ternary,
    Tuple,
    TupleAccess,
    Unary,
    Unit,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LiteralKind {
    Address,
    Boolean,
    Field,
    Group,
    Integer(IntegerLiteralKind),
    Scalar,
    Unsuffixed,
    String,
}

impl From<ExpressionKind> for SyntaxKind {
    fn from(value: ExpressionKind) -> Self {
        SyntaxKind::Expression(value)
    }
}

impl From<LiteralKind> for ExpressionKind {
    fn from(value: LiteralKind) -> Self {
        ExpressionKind::Literal(value)
    }
}

impl From<LiteralKind> for SyntaxKind {
    fn from(value: LiteralKind) -> Self {
        SyntaxKind::Expression(ExpressionKind::Literal(value))
    }
}

impl From<IntegerLiteralKind> for LiteralKind {
    fn from(value: IntegerLiteralKind) -> Self {
        LiteralKind::Integer(value)
    }
}

impl From<IntegerLiteralKind> for ExpressionKind {
    fn from(value: IntegerLiteralKind) -> Self {
        ExpressionKind::Literal(LiteralKind::Integer(value))
    }
}

impl From<IntegerLiteralKind> for SyntaxKind {
    fn from(value: IntegerLiteralKind) -> Self {
        SyntaxKind::Expression(ExpressionKind::Literal(LiteralKind::Integer(value)))
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StatementKind {
    Assert,
    AssertEq,
    AssertNeq,
    Assign,
    Block,
    Conditional,
    Const,
    Definition,
    Expression,
    Iteration,
    Return,
}

impl From<StatementKind> for SyntaxKind {
    fn from(value: StatementKind) -> Self {
        SyntaxKind::Statement(value)
    }
}

/// An untyped node in the lossless syntax tree.
#[derive(Debug, Clone)]
pub struct SyntaxNode<'a> {
    /// A tag indicating the nature of the node.
    pub kind: SyntaxKind,
    /// The text from the source if applicable.
    pub text: &'a str,
    pub span: leo_span::Span,
    pub children: Vec<SyntaxNode<'a>>,
}

impl<'a> SyntaxNode<'a> {
    fn new_token(kind: SyntaxKind, token: LalrToken<'a>, children: Vec<Self>) -> Self {
        Self { kind, text: token.text, span: token.span, children }
    }

    fn new(kind: impl Into<SyntaxKind>, children: impl IntoIterator<Item = Self>) -> Self {
        let children: Vec<Self> = children.into_iter().collect();
        let lo = children.first().unwrap().span.lo;
        let hi = children.last().unwrap().span.hi;
        let span = leo_span::Span { lo, hi };
        Self { kind: kind.into(), text: "", span, children }
    }

    fn suffixed_literal(integer: LalrToken<'a>, suffix: LalrToken<'a>, children: Vec<Self>) -> Self {
        let kind: SyntaxKind = match suffix.token {
            Token::Field => LiteralKind::Field.into(),
            Token::Group => LiteralKind::Group.into(),
            Token::Scalar => LiteralKind::Scalar.into(),
            Token::I8 => IntegerLiteralKind::I8.into(),
            Token::I16 => IntegerLiteralKind::I16.into(),
            Token::I32 => IntegerLiteralKind::I32.into(),
            Token::I64 => IntegerLiteralKind::I64.into(),
            Token::I128 => IntegerLiteralKind::I128.into(),
            Token::U8 => IntegerLiteralKind::U8.into(),
            Token::U16 => IntegerLiteralKind::U16.into(),
            Token::U32 => IntegerLiteralKind::U32.into(),
            Token::U64 => IntegerLiteralKind::U64.into(),
            Token::U128 => IntegerLiteralKind::U128.into(),
            x => panic!("Error in grammar.lalrpop: {x:?}"),
        };

        let lo = integer.span.lo;
        let hi = suffix.span.hi;
        let span = leo_span::Span { lo, hi };

        Self { kind, text: integer.text, span, children }
    }

    fn binary_expression(lhs: Self, op: Self, rhs: Self) -> Self {
        let span = leo_span::Span { lo: lhs.span.lo, hi: rhs.span.hi };
        let children = vec![lhs, op, rhs];
        SyntaxNode { kind: ExpressionKind::Binary.into(), text: "", span, children }
    }
}

fn two_path_components(text: &str) -> Option<(&str, &str)> {
    let mut iter = text.split("::");

    match (iter.next(), iter.next(), iter.next()) {
        (Some(first), Some(second), None) => Some((first, second)),
        _ => None,
    }
}

pub fn parse_expression<'a>(handler: Handler, source: &'a str, start_pos: u32) -> Result<SyntaxNode<'a>> {
    let parser = grammar::ExprParser::new();
    parse_general(handler.clone(), source, start_pos, |lexer| parser.parse(&handler, lexer))
}

pub fn parse_statement<'a>(handler: Handler, source: &'a str, start_pos: u32) -> Result<SyntaxNode<'a>> {
    let parser = grammar::StatementParser::new();
    parse_general(handler.clone(), source, start_pos, |lexer| parser.parse(&handler, lexer))
}

pub fn parse_module<'a>(handler: Handler, source: &'a str, start_pos: u32) -> Result<SyntaxNode<'a>> {
    let parser = grammar::ModuleContentsParser::new();
    parse_general(handler.clone(), source, start_pos, |lexer| parser.parse(&handler, lexer))
}

pub fn parse_main<'a>(handler: Handler, source: &'a str, start_pos: u32) -> Result<SyntaxNode<'a>> {
    let parser = grammar::MainContentsParser::new();
    parse_general(handler.clone(), source, start_pos, |lexer| parser.parse(&handler, lexer))
}

fn check_identifier(token: &LalrToken<'_>, handler: &Handler) {
    const MAX_IDENTIFIER_LEN: usize = 31usize;
    if token.token == Token::IdVariants(IdVariants::Identifier) {
        if token.text.len() > MAX_IDENTIFIER_LEN {
            handler.emit_err(leo_errors::ParserError::identifier_too_long(
                token.text,
                token.text.len(),
                MAX_IDENTIFIER_LEN,
                token.span,
            ));
        }
        // These are reserved for compiler-generated names.
        if token.text.contains("__") {
            handler.emit_err(ParserError::identifier_cannot_contain_double_underscore(token.text, token.span));
        }
    }
}

fn parse_general<'a>(
    handler: Handler,
    source: &'a str,
    start_pos: u32,
    parse: impl FnOnce(
        &mut Lexer<'a>,
    ) -> Result<SyntaxNode<'a>, lalrpop_util::ParseError<usize, LalrToken<'a>, &'static str>>,
) -> Result<SyntaxNode<'a>> {
    let mut lexer = tokens::Lexer::new(source, start_pos, handler.clone());
    match parse(&mut lexer) {
        Ok(val) => {
            handler.last_err()?;
            Ok(val)
        }
        Err(e) => {
            if matches!(e, lalrpop_util::ParseError::UnrecognizedEof { .. }) {
                // We don't want to redundantly report the EOF error, when the meaningfull
                // errors are recorded in the handler.
                handler.last_err()?;
            }
            Err(convert(e, source, start_pos))
        }
    }
}

// We can't implement From<lalrpop_util::ParseError> since both that
// trait and leo_errors::Error are defined in other crates.
fn convert(
    error: lalrpop_util::ParseError<usize, LalrToken<'_>, &'static str>,
    source: &str,
    start_pos: u32,
) -> LeoError {
    match error {
        lalrpop_util::ParseError::UnrecognizedToken { token, expected } => {
            let expected = expected.iter().flat_map(|s| tokens::Token::str_user(s)).format(", ");
            ParserError::unexpected(token.1.text, expected, token.1.span).into()
        }
        lalrpop_util::ParseError::UnrecognizedEof { location, .. } => {
            let (lo, hi) = if source.is_empty() {
                (start_pos, start_pos)
            } else if location >= source.len() + start_pos as usize {
                // Generally lalrpop reports the `location` for this error as
                // one character past the end of the source. So let's
                // back up one character.
                // Can't just subtract 1 as we may not be on a character boundary.
                let lo = source.char_indices().last().unwrap().0 as u32 + start_pos;
                (lo, lo + 1)
            } else {
                (location as u32, location as u32 + 1)
            };
            ParserError::unexpected_eof(Span { lo, hi }).into()
        }
        x => panic!("ERR: {x:?}"),
    }
}
