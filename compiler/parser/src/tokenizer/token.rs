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

use std::fmt;

use serde::{Deserialize, Serialize};

use leo_span::{Symbol, sym};

/// Represents all valid Leo syntax tokens.
///
/// The notion of 'token' here is a bit more general than in the ABNF grammar:
/// since it includes comments and whitespace,
/// it corresponds to the notion of 'lexeme' in the ABNF grammar.
/// There are also a few other differences, noted in comments below.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Token {
    // Comments
    CommentLine(String),  // the string includes the starting '//' and the ending line feed
    CommentBlock(String), // the string includes the starting '/*' and the ending '*/'

    // Whitespace (we do not distinguish among different kinds here)
    WhiteSpace,

    // Literals (= atomic literals and numerals in the ABNF grammar)
    // The string in Integer(String) consists of digits
    // The string in AddressLit(String) has the form `aleo1...`.
    True,
    False,
    Integer(String), // = numeral (including tuple index) in the ABNF grammar
    AddressLit(String),
    StaticString(String),
    // The numeric literals in the ABNF grammar, which consist of numerals followed by types,
    // are represented not as single tokens here,
    // but as two separate tokens (one for the numeral and one for the type),
    // enforcing, during parsing, the absence of whitespace or comments between those two tokens
    // (see the parse_primary_expression function).

    // Identifiers
    Identifier(Symbol),

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
    BitXor,
    BitXorAssign,
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
    Shl,
    ShlAssign,
    Shr,
    ShrAssign,
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
    Underscore,
    At, // @ is not a symbol token in the ABNF grammar (see explanation about annotations below)

    // The ABNF grammar has annotations as tokens,
    // defined as @ immediately followed by an identifier.
    // Here instead we regard the @ sign alone as a token (see `At` above),
    // and we lex it separately from the identifier that is supposed to follow it in an annotation.
    // When parsing annotations, we check that there is no whitespace or comments
    // between the @ and the identifier, thus eventually complying to the ABNF grammar.
    // See the parse_annotation function.

    // Type keywords
    Address,
    Bool,
    Field,
    Group,
    I8,
    I16,
    I32,
    I64,
    I128,
    Record,
    Scalar,
    Signature,
    String,
    Struct,
    U8,
    U16,
    U32,
    U64,
    U128,

    // Other keywords
    Aleo,
    As,
    Assert,
    AssertEq,
    AssertNeq,
    Async,
    Block,
    Const,
    Constant,
    Constructor,
    Else,
    Fn,
    For,
    Function,
    Future,
    If,
    Import,
    In,
    Inline,
    Let,
    Mapping,
    Network,
    Private,
    Program,
    Public,
    Return,
    Script,
    SelfLower,
    Transition,

    // Meta tokens
    Eof, // used to signal end-of-file, not an actual token of the language
    Leo, // only used for error messages, not an actual keyword
}

macro_rules! keyword_map {
    ($($token:ident => $symbol:ident),* $(,)?) => {
        pub const KEYWORD_TOKENS: &[Token] = &[
            $(Token::$token),*
        ];

        impl Token {
            pub fn is_keyword(&self) -> bool {
                matches!(self, $(Token::$token)|*)
            }

            pub fn keyword_to_symbol(&self) -> Option<Symbol> {
                match self {
                    $(Token::$token => Some(sym::$symbol),)*
                    _ => None,
                }
            }

            pub fn symbol_to_keyword(symbol: Symbol) -> Option<Self> {
                match symbol {
                    $(sym::$symbol => Some(Token::$token),)*
                    _ => None,
                }
            }
        }
    }
}

// Represents all valid Leo keyword tokens.
// This also includes the boolean literals `true` and `false`,
// unlike the ABNF grammar, which classifies them as literals and not keywords.
// But for the purposes of our lexer implementation,
// it is fine to include the boolean literals in this list.
keyword_map! {
    Address    => address,
    Aleo       => aleo,
    As         => As,
    Assert     => assert,
    AssertEq   => assert_eq,
    AssertNeq  => assert_neq,
    Async      => Async,   // if you need it
    Block      => block,
    Bool       => bool,
    Const      => Const,
    Constant   => constant,
    Constructor => constructor,
    Else       => Else,
    False      => False,
    Field      => field,
    Fn         => Fn,
    For        => For,
    Function   => function,
    Future     => Future,
    Group      => group,
    I8         => i8,
    I16        => i16,
    I32        => i32,
    I64        => i64,
    I128       => i128,
    If         => If,
    Import     => import,
    In         => In,
    Inline     => inline,
    Let        => Let,
    Leo        => leo,
    Mapping    => mapping,
    Network    => network,
    Private    => private,
    Program    => program,
    Public     => public,
    Record     => record,
    Return     => Return,
    Scalar     => scalar,
    Script     => script,
    SelfLower  => SelfLower,
    Signature  => signature,
    String     => string,
    Struct     => Struct,
    Transition => transition,
    True       => True,
    U8         => u8,
    U16        => u16,
    U32        => u32,
    U64        => u64,
    U128       => u128,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Token::*;
        match self {
            CommentLine(s) => write!(f, "{s}"),
            CommentBlock(s) => write!(f, "{s}"),

            WhiteSpace => write!(f, "whitespace"),

            True => write!(f, "true"),
            False => write!(f, "false"),
            Integer(s) => write!(f, "{s}"),
            AddressLit(s) => write!(f, "{s}"),
            StaticString(s) => write!(f, "\"{s}\""),

            Identifier(s) => write!(f, "{s}"),

            Not => write!(f, "!"),
            And => write!(f, "&&"),
            AndAssign => write!(f, "&&="),
            Or => write!(f, "||"),
            OrAssign => write!(f, "||="),
            BitAnd => write!(f, "&"),
            BitAndAssign => write!(f, "&="),
            BitOr => write!(f, "|"),
            BitOrAssign => write!(f, "|="),
            BitXor => write!(f, "^"),
            BitXorAssign => write!(f, "^="),
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
            Shl => write!(f, "<<"),
            ShlAssign => write!(f, "<<="),
            Shr => write!(f, ">>"),
            ShrAssign => write!(f, ">>="),
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
            Underscore => write!(f, "_"),
            At => write!(f, "@"),

            Address => write!(f, "address"),
            Bool => write!(f, "bool"),
            Field => write!(f, "field"),
            Group => write!(f, "group"),
            I8 => write!(f, "i8"),
            I16 => write!(f, "i16"),
            I32 => write!(f, "i32"),
            I64 => write!(f, "i64"),
            I128 => write!(f, "i128"),
            Record => write!(f, "record"),
            Scalar => write!(f, "scalar"),
            Signature => write!(f, "signature"),
            String => write!(f, "string"),
            Struct => write!(f, "struct"),
            U8 => write!(f, "u8"),
            U16 => write!(f, "u16"),
            U32 => write!(f, "u32"),
            U64 => write!(f, "u64"),
            U128 => write!(f, "u128"),

            Aleo => write!(f, "aleo"),
            As => write!(f, "as"),
            Assert => write!(f, "assert"),
            AssertEq => write!(f, "assert_eq"),
            AssertNeq => write!(f, "assert_neq"),
            Async => write!(f, "async"),
            Block => write!(f, "block"),
            Const => write!(f, "const"),
            Constant => write!(f, "constant"),
            Constructor => write!(f, "constructor"),
            Else => write!(f, "else"),
            Fn => write!(f, "Fn"),
            For => write!(f, "for"),
            Function => write!(f, "function"),
            Future => write!(f, "Future"),
            If => write!(f, "if"),
            Import => write!(f, "import"),
            In => write!(f, "in"),
            Inline => write!(f, "inline"),
            Let => write!(f, "let"),
            Mapping => write!(f, "mapping"),
            Network => write!(f, "network"),
            Private => write!(f, "private"),
            Program => write!(f, "program"),
            Public => write!(f, "public"),
            Return => write!(f, "return"),
            Script => write!(f, "script"),
            SelfLower => write!(f, "self"),
            Transition => write!(f, "transition"),

            Eof => write!(f, "<eof>"),
            Leo => write!(f, "leo"),
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
