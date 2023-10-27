// Copyright (C) 2019-2023 Aleo Systems Inc.
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

/// Represents all valid Leo syntax tokens.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Token {
    // Lexical Grammar
    // Literals
    CommentLine(String),
    CommentBlock(String),
    StaticString(String),
    Identifier(Symbol),
    Integer(String),
    True,
    False,
    AddressLit(String),
    WhiteSpace,

    // Symbols
    Not,
    And,
    AndAssign,
    Or,
    OrAssign,
    BitAnd,
    BitAndAssign,
    BitOr,
    BitOrAssign,
    Eq,
    NotEq,
    Lt,
    LtEq,
    Gt,
    GtEq,
    Add,
    AddAssign,
    Sub,
    SubAssign,
    Mul,
    MulAssign,
    Div,
    DivAssign,
    Pow,
    PowAssign,
    Rem,
    RemAssign,
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
    DoubleColon,
    Question,
    Arrow,
    BigArrow,
    Shl,
    ShlAssign,
    Shr,
    ShrAssign,
    Underscore,
    BitXor,
    BitXorAssign,
    At,

    // Syntactic Grammar
    // Types
    Address,
    Bool,
    Field,
    Group,
    Scalar,
    Signature,
    String,
    I8,
    I16,
    I32,
    I64,
    I128,
    U8,
    U16,
    U32,
    U64,
    U128,
    Record,

    // Regular Keywords
    As,
    Assert,
    AssertEq,
    AssertNeq,
    Console,
    Const,
    Constant,
    Else,
    Finalize,
    For,
    Function,
    If,
    Import,
    In,
    Inline,
    Let,
    Mapping,
    Private,
    Program,

    // Public inputs.
    Public,
    Return,
    SelfLower,
    Struct,
    Then,
    Transition,

    // Meta Tokens
    Block,
    Eof,
    Leo,
}

/// Represents all valid Leo keyword tokens.
/// This differs from the ABNF grammar for the following reasons:
/// Adding true and false to the keywords of the ABNF grammar makes the lexical grammar ambiguous,
/// because true and false are also boolean literals, which are different tokens from keywords.
pub const KEYWORD_TOKENS: &[Token] = &[
    Token::Address,
    Token::As,
    Token::Assert,
    Token::AssertEq,
    Token::AssertNeq,
    Token::Bool,
    Token::Console,
    Token::Const,
    Token::Constant,
    Token::Else,
    Token::False,
    Token::Field,
    Token::Finalize,
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
    Token::Inline,
    Token::Let,
    Token::Mapping,
    Token::Private,
    Token::Program,
    Token::Public,
    Token::Record,
    Token::Return,
    Token::SelfLower,
    Token::Signature,
    Token::Scalar,
    Token::String,
    Token::Struct,
    Token::Then,
    Token::Transition,
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
            Token::As => sym::As,
            Token::Assert => sym::assert,
            Token::AssertEq => sym::assert_eq,
            Token::AssertNeq => sym::assert_neq,
            Token::Block => sym::block,
            Token::Bool => sym::bool,
            Token::Console => sym::console,
            Token::Const => sym::Const,
            Token::Constant => sym::constant,
            Token::Else => sym::Else,
            Token::False => sym::False,
            Token::Field => sym::field,
            Token::Finalize => sym::finalize,
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
            Token::Inline => sym::inline,
            Token::Let => sym::Let,
            Token::Leo => sym::leo,
            Token::Mapping => sym::mapping,
            Token::Private => sym::private,
            Token::Program => sym::program,
            Token::Public => sym::public,
            Token::Record => sym::record,
            Token::Return => sym::Return,
            Token::Scalar => sym::scalar,
            Token::Signature => sym::signature,
            Token::SelfLower => sym::SelfLower,
            Token::String => sym::string,
            Token::Struct => sym::Struct,
            Token::Then => sym::then,
            Token::Transition => sym::transition,
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
            CommentLine(s) => write!(f, "{s}"),
            CommentBlock(s) => write!(f, "{s}"),
            StaticString(s) => write!(f, "\"{s}\""),
            Identifier(s) => write!(f, "{s}"),
            Integer(s) => write!(f, "{s}"),
            True => write!(f, "true"),
            False => write!(f, "false"),
            AddressLit(s) => write!(f, "{s}"),
            WhiteSpace => write!(f, "whitespace"),

            Not => write!(f, "!"),
            And => write!(f, "&&"),
            AndAssign => write!(f, "&&="),
            Or => write!(f, "||"),
            OrAssign => write!(f, "||="),
            BitAnd => write!(f, "&"),
            BitAndAssign => write!(f, "&="),
            BitOr => write!(f, "|"),
            BitOrAssign => write!(f, "|="),
            Eq => write!(f, "=="),
            NotEq => write!(f, "!="),
            Lt => write!(f, "<"),
            LtEq => write!(f, "<="),
            Gt => write!(f, ">"),
            GtEq => write!(f, ">="),
            Add => write!(f, "+"),
            AddAssign => write!(f, "+="),
            Sub => write!(f, "-"),
            SubAssign => write!(f, "-="),
            Mul => write!(f, "*"),
            MulAssign => write!(f, "*="),
            Div => write!(f, "/"),
            DivAssign => write!(f, "/="),
            Pow => write!(f, "**"),
            PowAssign => write!(f, "**="),
            Rem => write!(f, "%"),
            RemAssign => write!(f, "%="),
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
            DoubleColon => write!(f, "::"),
            Question => write!(f, "?"),
            Arrow => write!(f, "->"),
            BigArrow => write!(f, "=>"),
            Shl => write!(f, "<<"),
            ShlAssign => write!(f, "<<="),
            Shr => write!(f, ">>"),
            ShrAssign => write!(f, ">>="),
            Underscore => write!(f, "_"),
            BitXor => write!(f, "^"),
            BitXorAssign => write!(f, "^="),
            At => write!(f, "@"),

            Address => write!(f, "address"),
            Bool => write!(f, "bool"),
            Field => write!(f, "field"),
            Group => write!(f, "group"),
            Scalar => write!(f, "scalar"),
            Signature => write!(f, "signature"),
            String => write!(f, "string"),
            I8 => write!(f, "i8"),
            I16 => write!(f, "i16"),
            I32 => write!(f, "i32"),
            I64 => write!(f, "i64"),
            I128 => write!(f, "i128"),
            U8 => write!(f, "u8"),
            U16 => write!(f, "u16"),
            U32 => write!(f, "u32"),
            U64 => write!(f, "u64"),
            U128 => write!(f, "u128"),
            Record => write!(f, "record"),

            As => write!(f, "as"),
            Assert => write!(f, "assert"),
            AssertEq => write!(f, "assert_eq"),
            AssertNeq => write!(f, "assert_neq"),
            Console => write!(f, "console"),
            Const => write!(f, "const"),
            Constant => write!(f, "constant"),
            Else => write!(f, "else"),
            Finalize => write!(f, "finalize"),
            For => write!(f, "for"),
            Function => write!(f, "function"),
            If => write!(f, "if"),
            Import => write!(f, "import"),
            In => write!(f, "in"),
            Inline => write!(f, "inline"),
            Let => write!(f, "let"),
            Mapping => write!(f, "mapping"),
            Private => write!(f, "private"),
            Program => write!(f, "program"),
            Public => write!(f, "public"),
            Return => write!(f, "return"),
            SelfLower => write!(f, "self"),
            Struct => write!(f, "struct"),
            Then => write!(f, "then"),
            Transition => write!(f, "transition"),
            Block => write!(f, "block"),
            Leo => write!(f, "leo"),
            Eof => write!(f, "<eof>"),
        }
    }
}

/// Describes delimiters of a token sequence.
#[derive(Copy, Clone)]
pub enum Delimiter {
    /// `( ... )`
    Parenthesis,
    /// `{ ... }`
    Brace,
    /// `[ ... ]`
    Bracket,
}

impl Delimiter {
    /// Returns the open/close tokens that the delimiter corresponds to.
    pub fn open_close_pair(self) -> (Token, Token) {
        match self {
            Self::Parenthesis => (Token::LeftParen, Token::RightParen),
            Self::Brace => (Token::LeftCurly, Token::RightCurly),
            Self::Bracket => (Token::LeftSquare, Token::RightSquare),
        }
    }
}
