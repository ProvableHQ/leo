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

use logos::Logos;
use std::sync::LazyLock;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IdVariants {
    Identifier,
    Path,
    ProgramId,
    Locator,
}

fn id_variant(lex: &mut logos::Lexer<Token>) -> IdVariants {
    // Use LazyLock to not recompile these regexes every time.
    static REGEX_LOCATOR: LazyLock<regex::Regex> =
        LazyLock::new(|| regex::Regex::new(r"^\.aleo/[a-zA-Z][a-zA-Z0-9_]*").unwrap());
    static REGEX_PROGRAM_ID: LazyLock<regex::Regex> = LazyLock::new(|| regex::Regex::new(r"^\.aleo\b").unwrap());
    static REGEX_PATH: LazyLock<regex::Regex> =
        LazyLock::new(|| regex::Regex::new(r"^(?:::[a-zA-Z][a-zA-Z0-9_]*)+").unwrap());

    if let Some(found) = REGEX_LOCATOR.find(lex.remainder()) {
        lex.bump(found.len());
        IdVariants::Locator
    } else if let Some(found) = REGEX_PROGRAM_ID.find(lex.remainder()) {
        lex.bump(found.len());
        IdVariants::ProgramId
    } else if let Some(found) = REGEX_PATH.find(lex.remainder()) {
        lex.bump(found.len());
        IdVariants::Path
    } else {
        IdVariants::Identifier
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Logos)]
pub enum Token {
    #[regex(r"[ \t\f]+")]
    Whitespace,

    #[regex(r"\r?\n")]
    Linebreak,

    #[regex(r"//[^\r\n]*")]
    CommentLine,

    #[regex(r"/\*([^*]|\*[^\/])*\*/")]
    CommentBlock,

    // We want to lex these four categories as separate token types:
    // 1. identifiers like `abc`
    // 2. paths like `abc::def::ghi`
    // 3. program ids like `abc.aleo`
    // 4. locators like `abc.aleo/def`
    // We can't do this directly with logos regexes due to the lack of backtracking.
    // So we do it with this callback.
    //
    // As an alternative design, we could simply treat the individual components of these as separate tokens,
    // so that `abc.aleo/def` would be tokenized as `[abc, ., aleo, /, def]`. This is challenging to handle
    // with an LR(1) parser - we potentially get shift-reduce conflicts and other ambiguities between
    // member accesses, program ids, tuple accesses, etc. We could make it work but let's just cut to the
    // chase here.
    #[regex(r"[a-zA-Z][a-zA-Z0-9_]*", id_variant)]
    IdVariants(IdVariants),

    #[regex(r"aleo1[a-z0-9]{58}")]
    AddressLiteral,

    #[regex(r"-?(?:0x[0-9a-fA-F]+|0o[0-7]+|0b[01]+|\d+)")]
    Integer,

    #[regex(r#""[^"]*""#)]
    StaticString,

    // Symbols
    #[token("=")]
    Assign,
    #[token("!")]
    Not,
    #[token("&&")]
    And,
    #[token("&&=")]
    AndAssign,
    #[token("||")]
    Or,
    #[token("||=")]
    OrAssign,
    #[token("&")]
    BitAnd,
    #[token("&=")]
    BitAndAssign,
    #[token("|")]
    BitOr,
    #[token("|=")]
    BitOrAssign,
    #[token("^")]
    BitXor,
    #[token("^=")]
    BitXorAssign,
    #[token("==")]
    Eq,
    #[token("!=")]
    NotEq,
    #[token("<")]
    Lt,
    #[token("<=")]
    LtEq,
    #[token(">")]
    Gt,
    #[token(">=")]
    GtEq,
    #[token("+")]
    Add,
    #[token("+=")]
    AddAssign,
    #[token("-")]
    Sub,
    #[token("-=")]
    SubAssign,
    #[token("*")]
    Mul,
    #[token("*=")]
    MulAssign,
    #[token("/")]
    Div,
    #[token("/=")]
    DivAssign,
    #[token("**")]
    Pow,
    #[token("**=")]
    PowAssign,
    #[token("%")]
    Rem,
    #[token("%=")]
    RemAssign,
    #[token("<<")]
    Shl,
    #[token("<<=")]
    ShlAssign,
    #[token(">>")]
    Shr,
    #[token(">>=")]
    ShrAssign,
    #[token("(")]
    LeftParen,
    #[token(")")]
    RightParen,
    #[token("[")]
    LeftSquare,
    #[token("]")]
    RightSquare,
    #[token("{")]
    LeftCurly,
    #[token("}")]
    RightCurly,
    #[token(",")]
    Comma,
    #[token(".")]
    Dot,
    #[token("..")]
    DotDot,
    #[token(";")]
    Semicolon,
    #[token(":")]
    Colon,
    #[token("::")]
    DoubleColon,
    #[token("?")]
    Question,
    #[token("->")]
    Arrow,
    #[token("=>")]
    BigArrow,
    #[token("_")]
    Underscore,
    #[token("@")]
    At,

    // Keywords
    #[token("true")]
    True,
    #[token("false")]
    False,
    #[token("address")]
    Address,
    #[token("bool")]
    Bool,
    #[token("field")]
    Field,
    #[token("group")]
    Group,
    #[token("i8")]
    I8,
    #[token("i16")]
    I16,
    #[token("i32")]
    I32,
    #[token("i64")]
    I64,
    #[token("i128")]
    I128,
    #[token("record")]
    Record,
    #[token("scalar")]
    Scalar,
    #[token("signature")]
    Signature,
    #[token("string")]
    String,
    #[token("struct")]
    Struct,
    #[token("u8")]
    U8,
    #[token("u16")]
    U16,
    #[token("u32")]
    U32,
    #[token("u64")]
    U64,
    #[token("u128")]
    U128,

    #[token("aleo")]
    Aleo,
    #[token("as")]
    As,
    #[token("assert")]
    Assert,
    #[token("assert_eq")]
    AssertEq,
    #[token("assert_neq")]
    AssertNeq,
    #[token("async")]
    Async,
    #[token("block")]
    Block,
    #[token("const")]
    Const,
    #[token("constant")]
    Constant,
    #[token("constructor")]
    Constructor,
    #[token("else")]
    Else,
    #[token("Fn")]
    Fn,
    #[token("for")]
    For,
    #[token("function")]
    Function,
    #[token("Future")]
    Future,
    #[token("if")]
    If,
    #[token("import")]
    Import,
    #[token("in")]
    In,
    #[token("inline")]
    Inline,
    #[token("let")]
    Let,
    #[token("mapping")]
    Mapping,
    #[token("network")]
    Network,
    #[token("private")]
    Private,
    #[token("program")]
    Program,
    #[token("public")]
    Public,
    #[token("return")]
    Return,
    #[token("script")]
    Script,
    #[token("self")]
    SelfLower,
    #[token("transition")]
    Transition,
}

/// The token type we present to LALRPOP.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LalrToken<'a> {
    pub token: Token,
    pub text: &'a str,
    pub span: leo_span::Span,
}

pub struct Lexer<'a> {
    logos_lexer: logos::Lexer<'a, Token>,
    start_pos: u32,
}

impl<'a> Lexer<'a> {
    pub fn new(text: &'a str, start_pos: u32) -> Self {
        Self { logos_lexer: Token::lexer(text), start_pos }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = (usize, LalrToken<'a>, usize);

    fn next(&mut self) -> Option<Self::Item> {
        let token = self.logos_lexer.next()?.unwrap();
        let logos_span = self.logos_lexer.span();
        let leo_span =
            leo_span::Span { lo: self.start_pos + logos_span.start as u32, hi: self.start_pos + logos_span.end as u32 };

        Some((
            leo_span.lo as usize,
            LalrToken { token, text: self.logos_lexer.slice(), span: leo_span },
            leo_span.hi as usize,
        ))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn some_test() {
        let input = ", , ,    \t >=\n return some_symbol transition";

        let mut lexer = Token::lexer(input, 0);
        while let Some(token) = lexer.next() {
            // println!("SPAN {:?}", lexer.span());
            let slice = &input[lexer.span()];
            // println!("{token:?}");
            // println!("{slice}");
        }
    }
}
