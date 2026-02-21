// Copyright (C) 2019-2026 Provable Inc.
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

//! Lexer for the rowan-based Leo parser.
//!
//! This module provides a logos-based lexer that produces tokens suitable for
//! use with rowan's GreenNodeBuilder. All tokens, including whitespace and
//! comments (trivia), are explicitly represented.

use crate::{SyntaxKind, SyntaxKind::*};
use logos::Logos;
use rowan::{TextRange, TextSize};

/// A token produced by the lexer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Token {
    /// The kind of token.
    pub kind: SyntaxKind,
    /// The length in bytes of the token text.
    pub len: u32,
}

/// The kind of lexer error, carrying any structured data needed for diagnostics.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LexErrorKind {
    /// A digit was invalid for the given radix (e.g. `9` in an octal literal).
    InvalidDigit { digit: char, radix: u32, token: String },
    /// A token could not be lexed at all.
    CouldNotLex { content: String },
    /// A Unicode bidi override code point was encountered.
    BidiOverride,
}

/// An error encountered during lexing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LexError {
    /// The text range where the error occurred.
    pub range: TextRange,
    /// Structured error kind.
    pub kind: LexErrorKind,
}

/// Callback for parsing block comments.
///
/// Block comments can't be matched with a simple regex due to the need to find
/// the closing `*/`. This also detects bidi override characters for security.
/// Always returns true to produce a token; unterminated comments are detected
/// by checking if the slice ends with `*/` in the lex() function.
fn comment_block(lex: &mut logos::Lexer<LogosToken>) -> bool {
    let mut last_asterisk = false;
    for (index, c) in lex.remainder().char_indices() {
        if c == '*' {
            last_asterisk = true;
        } else if c == '/' && last_asterisk {
            lex.bump(index + 1);
            return true;
        } else if matches!(c, '\u{202A}'..='\u{202E}' | '\u{2066}'..='\u{2069}') {
            // Bidi character detected - end the comment token here
            // so we can report that error separately.
            lex.bump(index);
            return true;
        } else {
            last_asterisk = false;
        }
    }
    // Unterminated block comment - consume all remaining input
    let remaining = lex.remainder().len();
    lex.bump(remaining);
    true
}

/// Internal logos token enum.
///
/// This is mapped to `SyntaxKind` during lexing. We use a separate enum here
/// because logos requires ownership of the token type during lexing.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Logos)]
#[logos(skip r"")] // Don't skip anything - we want all tokens for lossless parsing
enum LogosToken {
    // =========================================================================
    // Trivia
    // =========================================================================
    #[regex(r"[ \t\f]+")]
    Whitespace,

    #[regex(r"\r?\n")]
    Linebreak,

    // Comments don't include line breaks or bidi characters
    #[regex(r"//[^\r\n\u{202A}-\u{202E}\u{2066}-\u{2069}]*")]
    CommentLine,

    #[token(r"/*", comment_block)]
    CommentBlock,

    // =========================================================================
    // Literals
    // =========================================================================
    // Address literals: aleo1...
    // We lex any length and validate later
    #[regex(r"aleo1[a-z0-9]*")]
    AddressLiteral,

    // Integer literals with various radixes
    // The regex includes type suffixes (u8, i32, field, etc.) to lex as a single token.
    // We use permissive patterns that allow invalid digits (e.g., G in hex) so we can
    // report specific validation errors rather than failing to lex.
    #[regex(r"0x[0-9A-Za-z_]+([ui](8|16|32|64|128)|field|group|scalar)?", priority = 3)]
    #[regex(r"0o[0-9A-Za-z_]+([ui](8|16|32|64|128)|field|group|scalar)?", priority = 3)]
    #[regex(r"0b[0-9A-Za-z_]+([ui](8|16|32|64|128)|field|group|scalar)?", priority = 3)]
    #[regex(r"[0-9][0-9A-Za-z_]*([ui](8|16|32|64|128)|field|group|scalar)?")]
    Integer,

    #[regex(r#""[^"]*""#)]
    StaticString,

    // =========================================================================
    // Identifiers and Keywords
    // =========================================================================
    // Note: Complex identifiers (paths like foo::bar, program IDs like foo.aleo,
    // locators like foo.aleo/bar) are deferred to Phase 2. The lexer produces
    // simple tokens; the parser handles disambiguation.
    //
    // We need special cases for `group::abc`, `signature::abc`, and `Future::abc`
    // as otherwise these would be keywords followed by ::.
    #[regex(r"group::[a-zA-Z][a-zA-Z0-9_]*")]
    #[regex(r"signature::[a-zA-Z][a-zA-Z0-9_]*")]
    #[regex(r"Future::[a-zA-Z][a-zA-Z0-9_]*")]
    PathSpecial,

    // Identifiers starting with underscore (intrinsic names)
    #[regex(r"_[a-zA-Z][a-zA-Z0-9_]*")]
    IdentIntrinsic,

    // Regular identifiers (keywords are matched by checking the slice)
    #[regex(r"[a-zA-Z][a-zA-Z0-9_]*")]
    Ident,

    // =========================================================================
    // Operators (multi-character first for correct priority)
    // =========================================================================
    #[token("**=")]
    PowAssign,
    #[token("&&=")]
    AndAssign,
    #[token("||=")]
    OrAssign,
    #[token("<<=")]
    ShlAssign,
    #[token(">>=")]
    ShrAssign,

    #[token("**")]
    Pow,
    #[token("&&")]
    And,
    #[token("||")]
    Or,
    #[token("<<")]
    Shl,
    #[token(">>")]
    Shr,
    #[token("==")]
    EqEq,
    #[token("!=")]
    NotEq,
    #[token("<=")]
    LtEq,
    #[token(">=")]
    GtEq,
    #[token("+=")]
    AddAssign,
    #[token("-=")]
    SubAssign,
    #[token("*=")]
    MulAssign,
    #[token("/=")]
    DivAssign,
    #[token("%=")]
    RemAssign,
    #[token("&=")]
    BitAndAssign,
    #[token("|=")]
    BitOrAssign,
    #[token("^=")]
    BitXorAssign,

    #[token("->")]
    Arrow,
    #[token("=>")]
    FatArrow,
    #[token("..=")]
    DotDotEq,
    #[token("..")]
    DotDot,
    #[token("::")]
    ColonColon,

    // Single character operators and punctuation
    #[token("=")]
    Eq,
    #[token("!")]
    Bang,
    #[token("<")]
    Lt,
    #[token(">")]
    Gt,
    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("*")]
    Star,
    #[token("/")]
    Slash,
    #[token("%")]
    Percent,
    #[token("&")]
    Amp,
    #[token("|")]
    Pipe,
    #[token("^")]
    Caret,

    // =========================================================================
    // Punctuation
    // =========================================================================
    #[token("(")]
    LParen,
    #[token(")")]
    RParen,
    #[token("[")]
    LBracket,
    #[token("]")]
    RBracket,
    #[token("{")]
    LBrace,
    #[token("}")]
    RBrace,
    #[token(",")]
    Comma,
    #[token(".")]
    Dot,
    #[token(";")]
    Semicolon,
    #[token(":")]
    Colon,
    #[token("?")]
    Question,
    #[token("_")]
    Underscore,
    #[token("@")]
    At,

    // =========================================================================
    // Security
    // =========================================================================
    // Unicode bidirectional control characters are a security risk.
    // We detect them so we can report an error.
    #[regex(r"[\u{202A}-\u{202E}\u{2066}-\u{2069}]")]
    Bidi,
}

/// Convert an identifier slice to the appropriate SyntaxKind (keyword or IDENT).
fn ident_to_kind(s: &str) -> SyntaxKind {
    match s {
        // Literal keywords
        "true" => KW_TRUE,
        "false" => KW_FALSE,
        "none" => KW_NONE,
        // Type keywords
        "address" => KW_ADDRESS,
        "bool" => KW_BOOL,
        "field" => KW_FIELD,
        "group" => KW_GROUP,
        "scalar" => KW_SCALAR,
        "signature" => KW_SIGNATURE,
        "string" => KW_STRING,
        "record" => KW_RECORD,
        "i8" => KW_I8,
        "i16" => KW_I16,
        "i32" => KW_I32,
        "i64" => KW_I64,
        "i128" => KW_I128,
        "u8" => KW_U8,
        "u16" => KW_U16,
        "u32" => KW_U32,
        "u64" => KW_U64,
        "u128" => KW_U128,
        // Control flow keywords
        "if" => KW_IF,
        "else" => KW_ELSE,
        "for" => KW_FOR,
        "in" => KW_IN,
        "return" => KW_RETURN,
        // Declaration keywords
        "let" => KW_LET,
        "const" => KW_CONST,
        "constant" => KW_CONSTANT,
        "final" => KW_FINAL,
        "Final" => KW_FINAL_UPPER,
        "fn" => KW_FN,
        "Fn" => KW_FN_UPPER,
        "struct" => KW_STRUCT,
        "constructor" => KW_CONSTRUCTOR,
        "interface" => KW_INTERFACE,
        // Program structure keywords
        "program" => KW_PROGRAM,
        "import" => KW_IMPORT,
        "mapping" => KW_MAPPING,
        "storage" => KW_STORAGE,
        "network" => KW_NETWORK,
        "aleo" => KW_ALEO,
        "script" => KW_SCRIPT,
        "block" => KW_BLOCK,
        // Visibility & assertion keywords
        "public" => KW_PUBLIC,
        "private" => KW_PRIVATE,
        "as" => KW_AS,
        "self" => KW_SELF,
        "assert" => KW_ASSERT,
        "assert_eq" => KW_ASSERT_EQ,
        "assert_neq" => KW_ASSERT_NEQ,
        // Not a keyword
        _ => IDENT,
    }
}

/// Strip integer type suffix from a string, returning the numeric part.
fn strip_int_suffix(s: &str) -> Option<&str> {
    // Check for integer type suffixes (longest first to match correctly)
    let suffixes = ["u128", "i128", "u64", "i64", "u32", "i32", "u16", "i16", "u8", "i8"];
    for suffix in suffixes {
        if let Some(prefix) = s.strip_suffix(suffix) {
            return Some(prefix);
        }
    }
    None
}

/// Validate integer literal digits for the appropriate radix.
/// Adds errors to the error vector if invalid digits are found.
fn validate_integer_digits(text: &str, offset: usize, errors: &mut Vec<LexError>) {
    // Strip type suffix if present (field, group, scalar, or integer types)
    let num_part = text
        .strip_suffix("field")
        .or_else(|| text.strip_suffix("group"))
        .or_else(|| text.strip_suffix("scalar"))
        .or_else(|| strip_int_suffix(text))
        .unwrap_or(text);

    // Determine the radix and get the digit part
    let (digits, radix, _prefix_len): (&str, u32, usize) = if let Some(s) = num_part.strip_prefix("0x") {
        (s, 16, 2)
    } else if let Some(s) = num_part.strip_prefix("0X") {
        (s, 16, 2)
    } else if let Some(s) = num_part.strip_prefix("0o") {
        (s, 8, 2)
    } else if let Some(s) = num_part.strip_prefix("0O") {
        (s, 8, 2)
    } else if let Some(s) = num_part.strip_prefix("0b") {
        (s, 2, 2)
    } else if let Some(s) = num_part.strip_prefix("0B") {
        (s, 2, 2)
    } else {
        // Decimal - no prefix
        (num_part, 10, 0)
    };

    // Find the first invalid digit
    for (_, c) in digits.char_indices() {
        if c == '_' {
            continue; // Underscores are allowed
        }
        if !c.is_digit(radix) {
            // Found an invalid digit - span covers the entire numeric part (like LALRPOP)
            let error_end = offset + num_part.len();
            errors.push(LexError {
                range: TextRange::new(TextSize::new(offset as u32), TextSize::new(error_end as u32)),
                kind: LexErrorKind::InvalidDigit { digit: c, radix, token: num_part.to_string() },
            });
            return; // Only report the first invalid digit
        }
    }
}

/// Lex the given source text into a sequence of tokens.
///
/// Returns a vector of tokens and any errors encountered. Even if errors
/// occur, tokens are still produced to enable error recovery in the parser.
pub fn lex(source: &str) -> (Vec<Token>, Vec<LexError>) {
    let mut tokens = Vec::new();
    let mut errors = Vec::new();
    let mut lexer = LogosToken::lexer(source);

    while let Some(result) = lexer.next() {
        let span = lexer.span();
        let len = (span.end - span.start) as u32;
        let slice = lexer.slice();

        let kind = match result {
            Ok(token) => match token {
                // Trivia
                LogosToken::Whitespace => WHITESPACE,
                LogosToken::Linebreak => LINEBREAK,
                LogosToken::CommentLine => COMMENT_LINE,
                LogosToken::CommentBlock => {
                    // Check if block comment was properly terminated
                    if !slice.ends_with("*/") {
                        let preview_len = slice.len().min(10);
                        let preview = &slice[..preview_len];
                        errors.push(LexError {
                            range: TextRange::new(
                                TextSize::new(span.start as u32),
                                TextSize::new((span.start + 2) as u32), // Just the /*
                            ),
                            kind: LexErrorKind::CouldNotLex { content: preview.to_string() },
                        });
                    }
                    COMMENT_BLOCK
                }

                // Literals
                LogosToken::AddressLiteral => ADDRESS_LIT,
                LogosToken::Integer => INTEGER,
                LogosToken::StaticString => STRING,

                // Identifiers (check for keywords)
                LogosToken::Ident => ident_to_kind(slice),
                LogosToken::IdentIntrinsic => IDENT,
                LogosToken::PathSpecial => IDENT, // Treat as IDENT for now (Phase 2)

                // Multi-char operators
                LogosToken::PowAssign => STAR2_EQ,
                LogosToken::AndAssign => AMP2_EQ,
                LogosToken::OrAssign => PIPE2_EQ,
                LogosToken::ShlAssign => SHL_EQ,
                LogosToken::ShrAssign => SHR_EQ,
                LogosToken::Pow => STAR2,
                LogosToken::And => AMP2,
                LogosToken::Or => PIPE2,
                LogosToken::Shl => SHL,
                LogosToken::Shr => SHR,
                LogosToken::EqEq => EQ2,
                LogosToken::NotEq => BANG_EQ,
                LogosToken::LtEq => LT_EQ,
                LogosToken::GtEq => GT_EQ,
                LogosToken::AddAssign => PLUS_EQ,
                LogosToken::SubAssign => MINUS_EQ,
                LogosToken::MulAssign => STAR_EQ,
                LogosToken::DivAssign => SLASH_EQ,
                LogosToken::RemAssign => PERCENT_EQ,
                LogosToken::BitAndAssign => AMP_EQ,
                LogosToken::BitOrAssign => PIPE_EQ,
                LogosToken::BitXorAssign => CARET_EQ,
                LogosToken::Arrow => ARROW,
                LogosToken::FatArrow => FAT_ARROW,
                LogosToken::DotDotEq => DOT_DOT_EQ,
                LogosToken::DotDot => DOT_DOT,
                LogosToken::ColonColon => COLON_COLON,

                // Single-char operators
                LogosToken::Eq => EQ,
                LogosToken::Bang => BANG,
                LogosToken::Lt => LT,
                LogosToken::Gt => GT,
                LogosToken::Plus => PLUS,
                LogosToken::Minus => MINUS,
                LogosToken::Star => STAR,
                LogosToken::Slash => SLASH,
                LogosToken::Percent => PERCENT,
                LogosToken::Amp => AMP,
                LogosToken::Pipe => PIPE,
                LogosToken::Caret => CARET,

                // Punctuation
                LogosToken::LParen => L_PAREN,
                LogosToken::RParen => R_PAREN,
                LogosToken::LBracket => L_BRACKET,
                LogosToken::RBracket => R_BRACKET,
                LogosToken::LBrace => L_BRACE,
                LogosToken::RBrace => R_BRACE,
                LogosToken::Comma => COMMA,
                LogosToken::Dot => DOT,
                LogosToken::Semicolon => SEMICOLON,
                LogosToken::Colon => COLON,
                LogosToken::Question => QUESTION,
                LogosToken::Underscore => UNDERSCORE,
                LogosToken::At => AT,

                // Security: bidi characters
                LogosToken::Bidi => {
                    errors.push(LexError {
                        range: TextRange::new(TextSize::new(span.start as u32), TextSize::new(span.end as u32)),
                        kind: LexErrorKind::BidiOverride,
                    });
                    ERROR
                }
            },
            Err(()) => {
                errors.push(LexError {
                    range: TextRange::new(TextSize::new(span.start as u32), TextSize::new(span.end as u32)),
                    kind: LexErrorKind::CouldNotLex { content: slice.to_string() },
                });
                ERROR
            }
        };

        // Validate integer literal digits for the appropriate radix
        if kind == INTEGER {
            validate_integer_digits(slice, span.start, &mut errors);
        }

        tokens.push(Token { kind, len });
    }

    // Add EOF token
    tokens.push(Token { kind: EOF, len: 0 });

    (tokens, errors)
}

#[cfg(test)]
mod tests {
    use super::*;
    use expect_test::{Expect, expect};

    /// Helper to format tokens for snapshot testing.
    fn check_lex(input: &str, expect: Expect) {
        let (tokens, _errors) = lex(input);
        let mut output = String::new();
        let mut offset = 0usize;
        for token in &tokens {
            let text = &input[offset..offset + token.len as usize];
            output.push_str(&format!("{:?} {:?}\n", token.kind, text));
            offset += token.len as usize;
        }
        expect.assert_eq(&output);
    }

    /// Helper to check that lexing produces expected errors.
    fn check_lex_errors(input: &str, expect: Expect) {
        let (_tokens, errors) = lex(input);
        let output = errors
            .iter()
            .map(|e| format!("{}..{}:{:?}", u32::from(e.range.start()), u32::from(e.range.end()), e.kind))
            .collect::<Vec<_>>()
            .join("\n");
        expect.assert_eq(&output);
    }

    #[test]
    fn lex_empty() {
        check_lex("", expect![[r#"
            EOF ""
        "#]]);
    }

    #[test]
    fn lex_whitespace() {
        check_lex("  \t  ", expect![[r#"
            WHITESPACE "  \t  "
            EOF ""
        "#]]);
    }

    #[test]
    fn lex_linebreaks() {
        check_lex("\n\r\n\n", expect![[r#"
            LINEBREAK "\n"
            LINEBREAK "\r\n"
            LINEBREAK "\n"
            EOF ""
"#]]);
    }

    #[test]
    fn lex_mixed_whitespace() {
        check_lex("  \n  \t\n", expect![[r#"
            WHITESPACE "  "
            LINEBREAK "\n"
            WHITESPACE "  \t"
            LINEBREAK "\n"
            EOF ""
        "#]]);
    }

    #[test]
    fn lex_line_comments() {
        check_lex("// hello\n// world", expect![[r#"
            COMMENT_LINE "// hello"
            LINEBREAK "\n"
            COMMENT_LINE "// world"
            EOF ""
        "#]]);
    }

    #[test]
    fn lex_block_comments() {
        check_lex("/* hello */ /* multi\nline */", expect![[r#"
            COMMENT_BLOCK "/* hello */"
            WHITESPACE " "
            COMMENT_BLOCK "/* multi\nline */"
            EOF ""
        "#]]);
    }

    #[test]
    fn lex_identifiers() {
        check_lex("foo Bar _baz x123", expect![[r#"
            IDENT "foo"
            WHITESPACE " "
            IDENT "Bar"
            WHITESPACE " "
            IDENT "_baz"
            WHITESPACE " "
            IDENT "x123"
            EOF ""
        "#]]);
    }

    #[test]
    fn lex_keywords() {
        check_lex("let fn if return true false", expect![[r#"
            KW_LET "let"
            WHITESPACE " "
            KW_FN "fn"
            WHITESPACE " "
            KW_IF "if"
            WHITESPACE " "
            KW_RETURN "return"
            WHITESPACE " "
            KW_TRUE "true"
            WHITESPACE " "
            KW_FALSE "false"
            EOF ""
        "#]]);
    }

    #[test]
    fn lex_type_keywords() {
        check_lex("u8 u16 u32 u64 u128 i8 i16 i32 i64 i128", expect![[r#"
            KW_U8 "u8"
            WHITESPACE " "
            KW_U16 "u16"
            WHITESPACE " "
            KW_U32 "u32"
            WHITESPACE " "
            KW_U64 "u64"
            WHITESPACE " "
            KW_U128 "u128"
            WHITESPACE " "
            KW_I8 "i8"
            WHITESPACE " "
            KW_I16 "i16"
            WHITESPACE " "
            KW_I32 "i32"
            WHITESPACE " "
            KW_I64 "i64"
            WHITESPACE " "
            KW_I128 "i128"
            EOF ""
        "#]]);
    }

    #[test]
    fn lex_more_type_keywords() {
        check_lex("bool field group scalar address signature string record", expect![[r#"
            KW_BOOL "bool"
            WHITESPACE " "
            KW_FIELD "field"
            WHITESPACE " "
            KW_GROUP "group"
            WHITESPACE " "
            KW_SCALAR "scalar"
            WHITESPACE " "
            KW_ADDRESS "address"
            WHITESPACE " "
            KW_SIGNATURE "signature"
            WHITESPACE " "
            KW_STRING "string"
            WHITESPACE " "
            KW_RECORD "record"
            EOF ""
        "#]]);
    }

    #[test]
    fn lex_integers() {
        check_lex("123 0xFF 0b101 0o77", expect![[r#"
            INTEGER "123"
            WHITESPACE " "
            INTEGER "0xFF"
            WHITESPACE " "
            INTEGER "0b101"
            WHITESPACE " "
            INTEGER "0o77"
            EOF ""
        "#]]);
    }

    #[test]
    fn lex_integers_with_underscores() {
        check_lex("1_000_000 0xFF_FF", expect![[r#"
            INTEGER "1_000_000"
            WHITESPACE " "
            INTEGER "0xFF_FF"
            EOF ""
        "#]]);
    }

    #[test]
    fn lex_address_literal() {
        check_lex("aleo1abc123", expect![[r#"
            ADDRESS_LIT "aleo1abc123"
            EOF ""
        "#]]);
    }

    #[test]
    fn lex_strings() {
        check_lex(r#""hello" "world""#, expect![[r#"
            STRING "\"hello\""
            WHITESPACE " "
            STRING "\"world\""
            EOF ""
        "#]]);
    }

    #[test]
    fn lex_punctuation() {
        check_lex("( ) [ ] { } , . ; : :: ? -> => _ @", expect![[r#"
            L_PAREN "("
            WHITESPACE " "
            R_PAREN ")"
            WHITESPACE " "
            L_BRACKET "["
            WHITESPACE " "
            R_BRACKET "]"
            WHITESPACE " "
            L_BRACE "{"
            WHITESPACE " "
            R_BRACE "}"
            WHITESPACE " "
            COMMA ","
            WHITESPACE " "
            DOT "."
            WHITESPACE " "
            SEMICOLON ";"
            WHITESPACE " "
            COLON ":"
            WHITESPACE " "
            COLON_COLON "::"
            WHITESPACE " "
            QUESTION "?"
            WHITESPACE " "
            ARROW "->"
            WHITESPACE " "
            FAT_ARROW "=>"
            WHITESPACE " "
            UNDERSCORE "_"
            WHITESPACE " "
            AT "@"
            EOF ""
        "#]]);
    }

    #[test]
    fn lex_arithmetic_operators() {
        check_lex("+ - * / % **", expect![[r#"
            PLUS "+"
            WHITESPACE " "
            MINUS "-"
            WHITESPACE " "
            STAR "*"
            WHITESPACE " "
            SLASH "/"
            WHITESPACE " "
            PERCENT "%"
            WHITESPACE " "
            STAR2 "**"
            EOF ""
        "#]]);
    }

    #[test]
    fn lex_comparison_operators() {
        check_lex("== != < <= > >=", expect![[r#"
            EQ2 "=="
            WHITESPACE " "
            BANG_EQ "!="
            WHITESPACE " "
            LT "<"
            WHITESPACE " "
            LT_EQ "<="
            WHITESPACE " "
            GT ">"
            WHITESPACE " "
            GT_EQ ">="
            EOF ""
        "#]]);
    }

    #[test]
    fn lex_logical_operators() {
        check_lex("&& || !", expect![[r#"
            AMP2 "&&"
            WHITESPACE " "
            PIPE2 "||"
            WHITESPACE " "
            BANG "!"
            EOF ""
        "#]]);
    }

    #[test]
    fn lex_bitwise_operators() {
        check_lex("& | ^ << >>", expect![[r#"
            AMP "&"
            WHITESPACE " "
            PIPE "|"
            WHITESPACE " "
            CARET "^"
            WHITESPACE " "
            SHL "<<"
            WHITESPACE " "
            SHR ">>"
            EOF ""
        "#]]);
    }

    #[test]
    fn lex_assignment_operators() {
        check_lex("= += -= *= /= %= **= &&= ||=", expect![[r#"
            EQ "="
            WHITESPACE " "
            PLUS_EQ "+="
            WHITESPACE " "
            MINUS_EQ "-="
            WHITESPACE " "
            STAR_EQ "*="
            WHITESPACE " "
            SLASH_EQ "/="
            WHITESPACE " "
            PERCENT_EQ "%="
            WHITESPACE " "
            STAR2_EQ "**="
            WHITESPACE " "
            AMP2_EQ "&&="
            WHITESPACE " "
            PIPE2_EQ "||="
            EOF ""
        "#]]);
    }

    #[test]
    fn lex_more_assignment_operators() {
        check_lex("&= |= ^= <<= >>=", expect![[r#"
            AMP_EQ "&="
            WHITESPACE " "
            PIPE_EQ "|="
            WHITESPACE " "
            CARET_EQ "^="
            WHITESPACE " "
            SHL_EQ "<<="
            WHITESPACE " "
            SHR_EQ ">>="
            EOF ""
        "#]]);
    }

    #[test]
    fn lex_dot_dot() {
        check_lex("0..10", expect![[r#"
            INTEGER "0"
            DOT_DOT ".."
            INTEGER "10"
            EOF ""
        "#]]);
    }

    #[test]
    fn lex_simple_expression() {
        check_lex("x + y * 2", expect![[r#"
            IDENT "x"
            WHITESPACE " "
            PLUS "+"
            WHITESPACE " "
            IDENT "y"
            WHITESPACE " "
            STAR "*"
            WHITESPACE " "
            INTEGER "2"
            EOF ""
        "#]]);
    }

    #[test]
    fn lex_function_call() {
        check_lex("foo(a, b)", expect![[r#"
            IDENT "foo"
            L_PAREN "("
            IDENT "a"
            COMMA ","
            WHITESPACE " "
            IDENT "b"
            R_PAREN ")"
            EOF ""
        "#]]);
    }

    #[test]
    fn lex_function_definition() {
        check_lex("fn add(x: u32) -> u32 {", expect![[r#"
            KW_FN "fn"
            WHITESPACE " "
            IDENT "add"
            L_PAREN "("
            IDENT "x"
            COLON ":"
            WHITESPACE " "
            KW_U32 "u32"
            R_PAREN ")"
            WHITESPACE " "
            ARROW "->"
            WHITESPACE " "
            KW_U32 "u32"
            WHITESPACE " "
            L_BRACE "{"
            EOF ""
        "#]]);
    }

    #[test]
    fn lex_let_statement() {
        check_lex("let x: u32 = 42;", expect![[r#"
            KW_LET "let"
            WHITESPACE " "
            IDENT "x"
            COLON ":"
            WHITESPACE " "
            KW_U32 "u32"
            WHITESPACE " "
            EQ "="
            WHITESPACE " "
            INTEGER "42"
            SEMICOLON ";"
            EOF ""
        "#]]);
    }

    #[test]
    fn lex_typed_integers() {
        // Integer literals with type suffixes should be lexed as single tokens
        check_lex("1000u32 42i64 0u8 255u128", expect![[r#"
            INTEGER "1000u32"
            WHITESPACE " "
            INTEGER "42i64"
            WHITESPACE " "
            INTEGER "0u8"
            WHITESPACE " "
            INTEGER "255u128"
            EOF ""
        "#]]);
    }

    #[test]
    fn lex_typed_integers_field() {
        // Field, group, and scalar suffixes
        check_lex("123field 456group 789scalar", expect![[r#"
            INTEGER "123field"
            WHITESPACE " "
            INTEGER "456group"
            WHITESPACE " "
            INTEGER "789scalar"
            EOF ""
        "#]]);
    }

    #[test]
    fn lex_special_paths() {
        // These are special cases where keywords are followed by ::
        check_lex("group::GEN signature::verify Future::await", expect![[r#"
            IDENT "group::GEN"
            WHITESPACE " "
            IDENT "signature::verify"
            WHITESPACE " "
            IDENT "Future::await"
            EOF ""
        "#]]);
    }

    #[test]
    fn lex_typed_integer_range() {
        // Integer with type suffix followed by range operator
        check_lex("0u8..STOP", expect![[r#"
            INTEGER "0u8"
            DOT_DOT ".."
            IDENT "STOP"
            EOF ""
        "#]]);
    }

    #[test]
    fn lex_error_unknown_char() {
        check_lex_errors("hello $ world", expect![[r#"6..7:CouldNotLex { content: "$" }"#]]);
    }

    #[test]
    fn lex_invalid_hex_digit() {
        let (tokens, errors) = lex("0xGAu32");
        assert_eq!(tokens.len(), 2); // INTEGER + EOF
        assert!(!errors.is_empty());
        assert!(matches!(errors[0].kind, LexErrorKind::InvalidDigit { digit: 'G', radix: 16, .. }));
    }

    #[test]
    fn lex_invalid_octal_digit() {
        let (_, errors) = lex("0o9u32");
        assert!(!errors.is_empty());
        assert!(matches!(errors[0].kind, LexErrorKind::InvalidDigit { digit: '9', radix: 8, .. }));
    }

    #[test]
    fn lex_invalid_binary_digit() {
        let (_, errors) = lex("0b2u32");
        assert!(!errors.is_empty());
        assert!(matches!(errors[0].kind, LexErrorKind::InvalidDigit { digit: '2', radix: 2, .. }));
    }

    #[test]
    fn lex_valid_hex_is_ok() {
        let (_, errors) = lex("0xDEADBEEFu64");
        assert!(errors.is_empty());
    }

    #[test]
    fn lex_invalid_hex_lowercase() {
        // Lowercase hex digits beyond f are invalid
        let (_, errors) = lex("0xghu32");
        assert!(!errors.is_empty());
        assert!(matches!(errors[0].kind, LexErrorKind::InvalidDigit { digit: 'g', radix: 16, .. }));
    }

    #[test]
    fn lex_bidi_override_error() {
        let (_, errors) = lex("let x\u{202E} = 1;");
        assert!(!errors.is_empty());
        assert!(matches!(errors[0].kind, LexErrorKind::BidiOverride));
    }

    #[test]
    fn lex_unclosed_block_comment() {
        let (tokens, errors) = lex("/* unclosed");
        assert!(!errors.is_empty());
        assert!(matches!(errors[0].kind, LexErrorKind::CouldNotLex { .. }));
        // Should still produce a COMMENT_BLOCK token
        assert!(tokens.iter().any(|t| t.kind == COMMENT_BLOCK));
    }

    #[test]
    fn lex_nested_comment_not_supported() {
        // Leo doesn't support nested comments - the first */ closes the comment
        let (tokens, errors) = lex("/* outer /* inner */");
        // No errors - the first */ terminates the comment
        assert!(errors.is_empty());
        // The "outer /* inner " part is the comment, then "*/" closes it
        // But actually let's check what happens...
        // "/* outer /* inner */" - this finds the first */ which is at position 18
        // So the comment is "/* outer /* inner */" which IS terminated
        assert!(tokens.iter().any(|t| t.kind == COMMENT_BLOCK));
    }

    #[test]
    fn lex_closed_comment_ok() {
        let (_, errors) = lex("/* closed */");
        assert!(errors.is_empty());
    }
}
