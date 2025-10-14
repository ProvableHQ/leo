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

use leo_errors::{Handler, ParserError};
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

fn comment_block(lex: &mut logos::Lexer<Token>) -> bool {
    let mut last_asterisk = false;
    for (index, c) in lex.remainder().char_indices() {
        if c == '*' {
            last_asterisk = true;
        } else if c == '/' && last_asterisk {
            lex.bump(index + 1);
            return true;
        } else if matches!(c,
            '\u{202A}'..='\u{202E}' |
            '\u{2066}'..='\u{2069}'
        ) {
            // It's a bidi character - end the comment token
            // so we can report that error.
            lex.bump(index);
            return true;
        } else {
            last_asterisk = false;
        }
    }
    false
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Logos)]
pub enum Token {
    #[regex(r"[ \t\f]+")]
    Whitespace,

    #[regex(r"\r?\n")]
    Linebreak,

    // Comments don't include line breaks or bidi characters.
    #[regex(r"//[^\r\n\u{202A}-\u{202E}\u{2066}-\u{2069}]*")]
    CommentLine,

    // Can't match block comments in a regex without lazy quantifiers,
    // so use a callback.
    #[token(r"/*", comment_block)]
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
    // We need to special case `group::abc` and `signature::abc` as otherwise these are keywords.
    #[token(r"group::[a-zA-Z][a-zA-Z0-9_]*", |_| IdVariants::Path)]
    #[token(r"signature::[a-zA-Z][a-zA-Z0-9_]*", |_| IdVariants::Path)]
    #[token(r"Future::[a-zA-Z][a-zA-Z0-9_]*", |_| IdVariants::Path)]
    IdVariants(IdVariants),

    // Address literals should have exactly 58 characters, but we lex other lengths
    // and flag an error later.
    #[regex(r"aleo1[a-z0-9]*")]
    AddressLiteral,

    // As with the previous parser, avoid lowercase letters to avoid ambiguity with the `field` postfix.
    // Allow invalid digits for each radix so we can report an error about them later.
    #[regex(r"0x[0-9A-Z_]+")]
    #[regex(r"0o[0-9A-Z_]+")]
    #[regex(r"0b[0-9A-Z_]+")]
    #[regex(r"[0-9][0-9A-Z_]*")]
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
    #[token("none")]
    None,
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

    // Unicode bidirectional control characters are a potential risk in
    // source. We detect them so we can report them as an error.
    #[regex(r"[\u{202A}-\u{202E}\u{2066}-\u{2069}]")]
    Bidi,

    // This token is never produced; we use it in grammar.lalrpop
    // to ensure a given production doesn't happen.
    Never,
}

impl Token {
    /// A `str` describing the token suitable for use in error messages.
    ///
    /// * `token_s` - The str as reported by logos.
    pub fn str_user(token_s: &str) -> Option<&'static str> {
        let v = match token_s {
            // These variants we don't want to report to the user.
            // Whitespace,
            // Linebreak,
            // CommentLine,
            // CommentBlock,
            "Identifier" => "an identifier",
            "AddressLiteral" => "an address literal",
            "ProgramId" => "a program id",

            "Integer" => "an integer literal",

            "StaticString" => "a static string",

            // Symbols
            "Assign" => "'='",
            "Not" => "'!'",
            "And" => "'&&'",
            "AndAssign" => "'&&='",
            "Or" => "'||'",
            "OrAssign" => "'||='",
            "BitAnd" => "'&'",
            "BitAndAssign" => "'&='",
            "BitOr" => "'|'",
            "BitOrAssign" => "'|='",
            "BitXor" => "'^'",
            "BitXorAssign" => "'&='",
            "Eq" => "'=='",
            "NotEq" => "'!='",
            "Lt" => "'<'",
            "LtEq" => "'<='",
            "Gt" => "'>'",
            "GtEq" => "'>='",
            "Add" => "'+'",
            "AddAssign" => "'+='",
            "Sub" => "'-'",
            "SubAssign" => "'-='",
            "Mul" => "'*'",
            "MulAssign" => "'*='",
            "Div" => "'/'",
            "DivAssign" => "'/='",
            "Pow" => "'**'",
            "PowAssign" => "'**='",
            "Rem" => "'%'",
            "RemAssign" => "'%='",
            "Shl" => "'<<'",
            "ShlAssign" => "'<<='",
            "Shr" => "'>>'",
            "ShrAssign" => "'>>='",
            "LeftParen" => "'('",
            "RightParen" => "')'",
            "LeftSquare" => "'['",
            "RightSquare" => "']'",
            "LeftCurly" => "'{'",
            "RightCurly" => "'}'",
            "Comma" => "','",
            "Dot" => "'.'",
            "DotDot" => "'..'",
            "Semicolon" => "';'",
            "Colon" => "':'",
            "DoubleColon" => "'::'",
            "Question" => "'?'",
            "Arrow" => "'->'",
            "BigArrow" => "'=>'",
            "Underscore" => "'_'",
            "At" => "'@'",

            // Keywords
            "True" => "'true'",
            "False" => "'false'",
            "Address" => "'address",
            "Bool" => "'bool'",
            "Field" => "'field'",
            "Group" => "'group'",
            "I8" => "'i8'",
            "I16" => "'i16'",
            "I32" => "'i32'",
            "I64" => "'i64'",
            "I128" => "'i128'",
            "Record" => "'record'",
            "Scalar" => "'scalar'",
            "Signature" => "'signature'",
            "String" => "a string",
            "Struct" => "'struct'",
            "U8" => "'u8'",
            "U16" => "'u16'",
            "U32" => "'u32'",
            "U64" => "'u64'",
            "U128" => "'u128'",

            "Aleo" => "'aleo'",
            "As" => "'as'",
            "Assert" => "'assert'",
            "AssertEq" => "'assert_eq'",
            "AssertNeq" => "'assert_neq'",
            "Async" => "'async'",
            "Block" => "'block'",
            "Const" => "'const'",
            "Constant" => "'constant'",
            "Constructor" => "'constructor'",
            "Else" => "'else'",
            "Fn" => "'Fn'",
            "For" => "'for'",
            "Function" => "'function'",
            "Future" => "'future'",
            "If" => "'if'",
            "Import" => "'import'",
            "In" => "'in'",
            "Inline" => "'inline'",
            "Let" => "'let'",
            "Mapping" => "'mapping'",
            "Network" => "'network'",
            "Private" => "'private'",
            "Program" => "'program'",
            "Public" => "'public'",
            "Return" => "'return'",
            "Script" => "'script'",
            "SelfLower" => "'self'",
            "Transition" => "'transition'",

            "Never" => return None,

            _ => return None,
        };
        Some(v)
    }
}

/// The token type we present to LALRPOP.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LalrToken<'a> {
    pub token: Token,
    pub text: &'a str,
    pub span: leo_span::Span,
}

/// The lexer we present to LALRPOP.
pub struct Lexer<'a> {
    logos_lexer: logos::Lexer<'a, Token>,
    start_pos: u32,
    handler: Handler,
}

impl<'a> Lexer<'a> {
    pub fn new(text: &'a str, start_pos: u32, handler: Handler) -> Self {
        Self { logos_lexer: Token::lexer(text), start_pos, handler }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = (usize, LalrToken<'a>, usize);

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.logos_lexer.next()?;
        let logos_span = self.logos_lexer.span();
        let span =
            leo_span::Span { lo: self.start_pos + logos_span.start as u32, hi: self.start_pos + logos_span.end as u32 };

        let text = self.logos_lexer.slice();

        let Ok(token) = next else {
            self.handler.emit_err(ParserError::could_not_lex_span(text.trim(), span));
            return None;
        };

        if matches!(token, Token::Bidi) {
            self.handler.emit_err(ParserError::lexer_bidi_override_span(span));
            return None;
        } else if matches!(token, Token::Integer) {
            let (s, radix) = if let Some(s) = text.strip_prefix("0x") {
                (s, 16)
            } else if let Some(s) = text.strip_prefix("0o") {
                (s, 8)
            } else if let Some(s) = text.strip_prefix("0b") {
                (s, 2)
            } else {
                (text, 10)
            };

            if let Some(c) = s.chars().find(|&c| c != '_' && !c.is_digit(radix)) {
                self.handler.emit_err(ParserError::wrong_digit_for_radix_span(c, radix, text, span));
            }
        }

        let lalr_token = LalrToken { token, text, span };

        Some((span.lo as usize, lalr_token, span.hi as usize))
    }
}
