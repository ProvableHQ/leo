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

use leo_span::{sym, Symbol};

use serde::{Deserialize, Serialize};
use std::fmt;
use tendril::StrTendril;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Char {
    Scalar(char),
    NonScalar(u32),
}

#[allow(clippy::from_over_into)]
impl Into<leo_ast::Char> for Char {
    fn into(self) -> leo_ast::Char {
        match self {
            Self::Scalar(c) => leo_ast::Char::Scalar(c),
            Self::NonScalar(c) => leo_ast::Char::NonScalar(c),
        }
    }
}

impl fmt::Display for Char {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Scalar(c) => write!(f, "{}", c),
            Self::NonScalar(c) => write!(f, "{}", c),
        }
    }
}

/// Represents all valid Leo syntax tokens.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Token {
    // Lexical Grammar
    // Literals
    CommentLine(#[serde(with = "leo_span::tendril_json")] StrTendril),
    CommentBlock(#[serde(with = "leo_span::tendril_json")] StrTendril),
    StringLit(Vec<leo_ast::Char>),
    Ident(Symbol),
    Int(#[serde(with = "leo_span::tendril_json")] StrTendril),
    True,
    False,
    AddressLit(#[serde(with = "leo_span::tendril_json")] StrTendril),
    CharLit(Char),

    // Symbols
    At,
    Not,
    And,
    Or,
    Eq,
    NotEq,
    Lt,
    LtEq,
    Gt,
    GtEq,
    Add,
    Minus,
    Mul,
    Div,
    Exp,
    Assign,
    AddEq,
    MinusEq,
    MulEq,
    DivEq,
    ExpEq,
    LeftParen,
    RightParen,
    LeftSquare,
    RightSquare,
    LeftCurly,
    RightCurly,
    Comma,
    Dot,
    DotDot,
    DotDotDot,
    Semicolon,
    Colon,
    DoubleColon,
    Question,
    Arrow,
    Underscore,

    // Syntactic Grammr
    // Types
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
    Field,
    Group,
    Bool,
    Address,
    Char,
    BigSelf,

    // primary expresion
    Input,
    LittleSelf,

    // Import
    Import,

    // Regular Keywords
    As,
    Circuit,
    Console,
    /// Const variable and a const function.
    Const,
    Else,
    For,
    Function,
    If,
    In,
    Let,
    Mut,
    /// Represents `&`.
    /// Used for `Reference` and `BitAnd`.
    Ampersand,
    Return,
    Static,
    Type,

    // Meta Tokens
    Eof,
}

/// Represents all valid Leo keyword tokens.
pub const KEYWORD_TOKENS: &[Token] = &[
    Token::Address,
    Token::As,
    Token::Bool,
    Token::Char,
    Token::Circuit,
    Token::Console,
    Token::Const,
    Token::Else,
    Token::False,
    Token::Field,
    Token::For,
    Token::Function,
    Token::Group,
    Token::I8,
    Token::I16,
    Token::I32,
    Token::I64,
    Token::I128,
    Token::If,
    Token::Import,
    Token::In,
    Token::Input,
    Token::Let,
    Token::Mut,
    Token::Ampersand,
    Token::Return,
    Token::BigSelf,
    Token::LittleSelf,
    Token::Static,
    Token::True,
    Token::Type,
    Token::U8,
    Token::U16,
    Token::U32,
    Token::U64,
    Token::U128,
];

impl Token {
    /// Returns `true` if the `self` token equals a Leo keyword.
    pub fn is_keyword(&self) -> bool {
        KEYWORD_TOKENS.contains(self)
    }

    /// Converts `self` to the corresponding `Symbol` if it `is_keyword`.
    pub fn keyword_to_symbol(&self) -> Option<Symbol> {
        Some(match self {
            Token::Address => sym::address,
            Token::As => sym::As,
            Token::Bool => sym::bool,
            Token::Char => sym::char,
            Token::Circuit => sym::circuit,
            Token::Console => sym::console,
            Token::Const => sym::Const,
            Token::Else => sym::Else,
            Token::False => sym::False,
            Token::Field => sym::field,
            Token::For => sym::For,
            Token::Function => sym::function,
            Token::Group => sym::group,
            Token::I8 => sym::i8,
            Token::I16 => sym::i16,
            Token::I32 => sym::i32,
            Token::I64 => sym::i64,
            Token::I128 => sym::i128,
            Token::If => sym::If,
            Token::Import => sym::import,
            Token::In => sym::In,
            Token::Input => sym::input,
            Token::Let => sym::Let,
            Token::Mut => sym::Mut,
            Token::Ampersand => sym::Ampersand,
            Token::Return => sym::Return,
            Token::BigSelf => sym::SelfUpper,
            Token::LittleSelf => sym::SelfLower,
            Token::Static => sym::Static,
            Token::True => sym::True,
            Token::Type => sym::Type,
            Token::U8 => sym::u8,
            Token::U16 => sym::u16,
            Token::U32 => sym::u32,
            Token::U64 => sym::u64,
            Token::U128 => sym::u128,
            _ => return None,
        })
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Token::*;
        match self {
            CommentLine(s) => write!(f, "{}", s),
            CommentBlock(s) => write!(f, "{}", s),
            StringLit(string) => {
                write!(f, "\"")?;
                for character in string.iter() {
                    write!(f, "{}", character)?;
                }
                write!(f, "\"")
            }
            Ident(s) => write!(f, "{}", s),
            Int(s) => write!(f, "{}", s),
            True => write!(f, "true"),
            False => write!(f, "false"),
            AddressLit(s) => write!(f, "{}", s),
            CharLit(s) => write!(f, "{}", s),

            At => write!(f, "@"),

            Not => write!(f, "!"),
            And => write!(f, "&&"),
            Or => write!(f, "||"),
            Eq => write!(f, "=="),
            NotEq => write!(f, "!="),
            Lt => write!(f, "<"),
            LtEq => write!(f, "<="),
            Gt => write!(f, ">"),
            GtEq => write!(f, ">="),
            Add => write!(f, "+"),
            Minus => write!(f, "-"),
            Mul => write!(f, "*"),
            Div => write!(f, "/"),
            Exp => write!(f, "**"),
            Assign => write!(f, "="),
            AddEq => write!(f, "+="),
            MinusEq => write!(f, "-="),
            MulEq => write!(f, "*="),
            DivEq => write!(f, "/="),
            ExpEq => write!(f, "**="),
            LeftParen => write!(f, "("),
            RightParen => write!(f, ")"),
            LeftSquare => write!(f, "["),
            RightSquare => write!(f, "]"),
            LeftCurly => write!(f, "{{"),
            RightCurly => write!(f, "}}"),
            Comma => write!(f, ","),
            Dot => write!(f, "."),
            DotDot => write!(f, ".."),
            DotDotDot => write!(f, "..."),
            Semicolon => write!(f, ";"),
            Colon => write!(f, ":"),
            DoubleColon => write!(f, "::"),
            Question => write!(f, "?"),
            Arrow => write!(f, "->"),
            Underscore => write!(f, "_"),

            U8 => write!(f, "u8"),
            U16 => write!(f, "u16"),
            U32 => write!(f, "u32"),
            U64 => write!(f, "u64"),
            U128 => write!(f, "u128"),
            I8 => write!(f, "i8"),
            I16 => write!(f, "i16"),
            I32 => write!(f, "i32"),
            I64 => write!(f, "i64"),
            I128 => write!(f, "i128"),
            Field => write!(f, "field"),
            Group => write!(f, "group"),
            Bool => write!(f, "bool"),
            Address => write!(f, "address"),
            Char => write!(f, "char"),
            BigSelf => write!(f, "Self"),

            Input => write!(f, "input"),
            LittleSelf => write!(f, "self"),

            Import => write!(f, "import"),

            As => write!(f, "as"),
            Circuit => write!(f, "circuit"),
            Console => write!(f, "console"),
            Const => write!(f, "const"),
            Else => write!(f, "else"),
            For => write!(f, "for"),
            Function => write!(f, "function"),
            If => write!(f, "if"),
            In => write!(f, "in"),
            Let => write!(f, "let"),
            Mut => write!(f, "mut"),
            Ampersand => write!(f, "&"), // Used for `Reference` and `BitAnd`
            Return => write!(f, "return"),
            Static => write!(f, "static"),
            Type => write!(f, "type"),
            Eof => write!(f, ""),
        }
    }
}
