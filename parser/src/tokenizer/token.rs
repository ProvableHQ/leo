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

/// Parts of a formatted string for logging to the console.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum FormattedStringPart {
    Const(String),
    Container,
}

impl fmt::Display for FormattedStringPart {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FormattedStringPart::Const(c) => write!(f, "{}", c),
            FormattedStringPart::Container => write!(f, "{{}}"),
        }
    }
}

/// Represents all valid Leo syntax tokens.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Token {
    CommentLine(String),
    CommentBlock(String),

    FormattedString(Vec<FormattedStringPart>),
    AddressLit(String),
    Ident(String),
    Int(String),
    Not,
    NotEq,
    And,
    LeftParen,
    RightParen,
    Mul,
    Exp,
    ExpEq,
    MulEq,
    Add,
    AddEq,
    Comma,
    Minus,
    MinusEq,
    Arrow,
    Dot,
    DotDot,
    DotDotDot,
    Div,
    DivEq,
    Colon,
    DoubleColon,
    Semicolon,
    Lt,
    LtEq,
    Assign,
    Eq,
    Gt,
    GtEq,
    At,
    LeftSquare,
    RightSquare,
    Address,
    As,
    Bool,
    Circuit,
    Const,
    Else,
    False,
    Field,
    For,
    Function,
    Group,
    I128,
    I64,
    I32,
    I16,
    I8,
    If,
    Import,
    In,
    Input,
    Let,
    Mut,
    Return,
    Static,
    String,
    True,
    U128,
    U64,
    U32,
    U16,
    U8,
    BigSelf,
    LittleSelf,
    Console,
    LeftCurly,
    RightCurly,
    Or,
    BitAnd,
    BitAndEq,
    BitOr,
    BitOrEq,
    BitXor,
    BitXorEq,
    BitNot,
    Shl,
    ShlEq,
    Shr,
    ShrEq,
    ShrSigned,
    ShrSignedEq,
    Mod,
    ModEq,
    OrEq,
    AndEq,
    Question,
}

/// Represents all valid Leo keyword tokens.
pub const KEYWORD_TOKENS: &[Token] = &[
    Token::Address,
    Token::As,
    Token::Bool,
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
            FormattedString(parts) => {
                // todo escapes
                write!(f, "\"")?;
                for part in parts.iter() {
                    part.fmt(f)?;
                }
                write!(f, "\"")
            }
            AddressLit(s) => write!(f, "{}", s),
            Ident(s) => write!(f, "{}", s),
            Int(s) => write!(f, "{}", s),
            CommentLine(s) => writeln!(f, "{}", s),
            CommentBlock(s) => write!(f, "{}", s),
            Not => write!(f, "!"),
            NotEq => write!(f, "!="),
            And => write!(f, "&&"),
            LeftParen => write!(f, "("),
            RightParen => write!(f, ")"),
            Mul => write!(f, "*"),
            Exp => write!(f, "**"),
            ExpEq => write!(f, "**="),
            MulEq => write!(f, "*="),
            Add => write!(f, "+"),
            AddEq => write!(f, "+="),
            Comma => write!(f, ","),
            Minus => write!(f, "-"),
            MinusEq => write!(f, "-="),
            Arrow => write!(f, "->"),
            Dot => write!(f, "."),
            DotDot => write!(f, ".."),
            DotDotDot => write!(f, "..."),
            Div => write!(f, "/"),
            DivEq => write!(f, "/="),
            Colon => write!(f, ":"),
            DoubleColon => write!(f, "::"),
            Semicolon => write!(f, ";"),
            Lt => write!(f, "<"),
            LtEq => write!(f, "<="),
            Assign => write!(f, "="),
            Eq => write!(f, "=="),
            Gt => write!(f, ">"),
            GtEq => write!(f, ">="),
            At => write!(f, "@"),
            LeftSquare => write!(f, "["),
            RightSquare => write!(f, "]"),
            Address => write!(f, "address"),
            As => write!(f, "as"),
            Bool => write!(f, "bool"),
            Circuit => write!(f, "circuit"),
            Const => write!(f, "const"),
            Else => write!(f, "else"),
            False => write!(f, "false"),
            Field => write!(f, "field"),
            For => write!(f, "for"),
            Function => write!(f, "function"),
            Group => write!(f, "group"),
            I128 => write!(f, "i128"),
            I64 => write!(f, "i64"),
            I32 => write!(f, "i32"),
            I16 => write!(f, "i16"),
            I8 => write!(f, "i8"),
            If => write!(f, "if"),
            Import => write!(f, "import"),
            In => write!(f, "in"),
            Input => write!(f, "input"),
            Let => write!(f, "let"),
            Mut => write!(f, "mut"),
            Return => write!(f, "return"),
            Static => write!(f, "static"),
            String => write!(f, "string"),
            True => write!(f, "true"),
            U128 => write!(f, "u128"),
            U64 => write!(f, "u64"),
            U32 => write!(f, "u32"),
            U16 => write!(f, "u16"),
            U8 => write!(f, "u8"),
            BigSelf => write!(f, "Self"),
            LittleSelf => write!(f, "self"),
            Console => write!(f, "console"),
            LeftCurly => write!(f, "{{"),
            RightCurly => write!(f, "}}"),
            Or => write!(f, "||"),
            BitAnd => write!(f, "&"),
            BitAndEq => write!(f, "&="),
            BitOr => write!(f, "|"),
            BitOrEq => write!(f, "|="),
            BitXor => write!(f, "^"),
            BitXorEq => write!(f, "^="),
            BitNot => write!(f, "~"),
            Shl => write!(f, "<<"),
            ShlEq => write!(f, "<<="),
            Shr => write!(f, ">>"),
            ShrEq => write!(f, ">>="),
            ShrSigned => write!(f, ">>>"),
            ShrSignedEq => write!(f, ">>>="),
            Mod => write!(f, "%"),
            ModEq => write!(f, "%="),
            OrEq => write!(f, "||="),
            AndEq => write!(f, "&&="),
            Question => write!(f, "?"),
        }
    }
}
