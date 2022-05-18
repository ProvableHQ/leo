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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Char {
    Scalar(char),
    NonScalar(u32),
}

impl From<Char> for leo_ast::Char {
    fn from(val: Char) -> Self {
        match val {
            Char::Scalar(c) => Self::Scalar(c),
            Char::NonScalar(c) => Self::NonScalar(c),
        }
    }
}

impl fmt::Display for Char {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Scalar(c) => write!(f, "{}", c),
            Self::NonScalar(c) => write!(f, "{:X}", c),
        }
    }
}

/// Represents all valid Leo syntax tokens.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Token {
    // Lexical Grammar
    // Literals
    CommentLine(String),
    CommentBlock(String),
    StringLit(Vec<leo_ast::Char>),
    Ident(Symbol),
    Int(String),
    True,
    False,
    AddressLit(String),
    WhiteSpace,

    // Symbols
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
    LeftParen,
    RightParen,
    LeftSquare,
    RightSquare,
    LeftCurly,
    RightCurly,
    Comma,
    Dot,
    DotDot,
    Semicolon,
    Colon,
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

    // Regular Keywords
    Console,
    /// Const variable and a const function.
    Const,
    /// Constant parameter
    Constant,
    Else,
    For,
    Function,
    If,
    In,
    Let,
    /// For public inputs.
    Public,
    Return,

    // Meta Tokens
    Eof,
}

/// Represents all valid Leo keyword tokens.
pub const KEYWORD_TOKENS: &[Token] = &[
    Token::Address,
    Token::Bool,
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
    Token::In,
    Token::Let,
    Token::Public,
    Token::Return,
    Token::True,
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
            Token::Bool => sym::bool,
            Token::Console => sym::console,
            Token::Const => sym::Const,
            Token::Constant => sym::Constant,
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
            Token::In => sym::In,
            Token::Let => sym::Let,
            Token::Public => sym::Public,
            Token::Return => sym::Return,
            Token::True => sym::True,
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
            WhiteSpace => write!(f, "whitespace"),

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
            LeftParen => write!(f, "("),
            RightParen => write!(f, ")"),
            LeftSquare => write!(f, "["),
            RightSquare => write!(f, "]"),
            LeftCurly => write!(f, "{{"),
            RightCurly => write!(f, "}}"),
            Comma => write!(f, ","),
            Dot => write!(f, "."),
            DotDot => write!(f, ".."),
            Semicolon => write!(f, ";"),
            Colon => write!(f, ":"),
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
            Console => write!(f, "console"),
            Const => write!(f, "const"),
            Constant => write!(f, "constant"),
            Else => write!(f, "else"),
            For => write!(f, "for"),
            Function => write!(f, "function"),
            If => write!(f, "if"),
            In => write!(f, "in"),
            Let => write!(f, "let"),
            Public => write!(f, "public"),
            Return => write!(f, "return"),
            Eof => write!(f, "<eof>"),
        }
    }
}
