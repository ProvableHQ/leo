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

use serde::{Deserialize, Serialize};
use std::fmt;
use tendril::StrTendril;

/// Parts of a formatted string for logging to the console.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum FormatStringPart {
    Const(#[serde(with = "leo_ast::common::tendril_json")] StrTendril),
    Container,
}

impl fmt::Display for FormatStringPart {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FormatStringPart::Const(c) => write!(f, "{}", c),
            FormatStringPart::Container => write!(f, "{{}}"),
        }
    }
}

/// Represents all valid Leo syntax tokens.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Token {
    // Lexical Grammar
    // Literals
    CommentLine(#[serde(with = "leo_ast::common::tendril_json")] StrTendril),
    CommentBlock(#[serde(with = "leo_ast::common::tendril_json")] StrTendril),
    FormatString(Vec<FormatStringPart>),
    Ident(#[serde(with = "leo_ast::common::tendril_json")] StrTendril),
    Int(#[serde(with = "leo_ast::common::tendril_json")] StrTendril),
    True,
    False,
    AddressLit(#[serde(with = "leo_ast::common::tendril_json")] StrTendril),
    CharLit(char),

    At,

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
    Const,
    Else,
    For,
    Function,
    If,
    In,
    Let,
    Mut,
    Return,
    Static,
    String,
    // Not yet in ABNF
    // BitAnd,
    // BitAndEq,
    // BitOr,
    // BitOrEq,
    // BitXor,
    // BitXorEq,
    // BitNot,
    // Shl,
    // ShlEq,
    // Shr,
    // ShrEq,
    // ShrSigned,
    // ShrSignedEq,
    // Mod,
    // ModEq,
    // OrEq,
    // AndEq,

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
    Token::Return,
    Token::BigSelf,
    Token::LittleSelf,
    Token::Static,
    Token::String,
    Token::True,
    Token::U8,
    Token::U16,
    Token::U32,
    Token::U64,
    Token::U128,
];

impl Token {
    ///
    /// Returns `true` if the `self` token equals a Leo keyword.
    ///
    pub fn is_keyword(&self) -> bool {
        KEYWORD_TOKENS.iter().any(|x| x == self)
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Token::*;
        match self {
            CommentLine(s) => write!(f, "{}", s),
            CommentBlock(s) => write!(f, "{}", s),
            FormatString(parts) => {
                // todo escapes
                write!(f, "\"")?;
                for part in parts.iter() {
                    part.fmt(f)?;
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
            Return => write!(f, "return"),
            Static => write!(f, "static"),
            String => write!(f, "string"),
            Eof => write!(f, ""),
            // BitAnd => write!(f, "&"),
            // BitAndEq => write!(f, "&="),
            // BitOr => write!(f, "|"),
            // BitOrEq => write!(f, "|="),
            // BitXor => write!(f, "^"),
            // BitXorEq => write!(f, "^="),
            // BitNot => write!(f, "~"),
            // Shl => write!(f, "<<"),
            // ShlEq => write!(f, "<<="),
            // Shr => write!(f, ">>"),
            // ShrEq => write!(f, ">>="),
            // ShrSigned => write!(f, ">>>"),
            // ShrSignedEq => write!(f, ">>>="),
            // Mod => write!(f, "%"),
            // ModEq => write!(f, "%="),
            // OrEq => write!(f, "||="),
            // AndEq => write!(f, "&&="),
        }
    }
}
