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

//! Syntax kind definitions for the rowan-based Leo parser.
//!
//! This module defines a flat `SyntaxKind` enum containing all token and node
//! types used in the Leo syntax tree. The enum is `#[repr(u16)]` for
//! compatibility with rowan's internal representation.

// Re-export the variants to make the `SyntaxKind` helper method implementations
// a bit less noisey.
use SyntaxKind::*;

/// All syntax kinds for Leo tokens and nodes.
///
/// This enum is intentionally flat (not nested) to satisfy rowan's requirement
/// for a `#[repr(u16)]` type. Categories are indicated by comments and helper
/// methods like `is_trivia()` and `is_keyword()`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u16)]
#[allow(non_camel_case_types)]
pub enum SyntaxKind {
    // ==========================================================================
    // Special
    // ==========================================================================
    /// Error node for wrapping parse errors and invalid tokens.
    ERROR = 0,
    /// End of file marker.
    EOF,

    // ==========================================================================
    // Trivia (whitespace and comments)
    // ==========================================================================
    /// Horizontal whitespace: spaces, tabs, form feeds.
    WHITESPACE,
    /// Line breaks: \n or \r\n.
    LINEBREAK,
    /// Line comment: // ...
    COMMENT_LINE,
    /// Block comment: /* ... */
    COMMENT_BLOCK,

    // ==========================================================================
    // Literals
    // ==========================================================================
    /// Integer literal: 123, 0xFF, 0b101, 0o77
    INTEGER,
    /// String literal: "..."
    STRING,
    /// Address literal: aleo1...
    ADDRESS_LIT,

    // ==========================================================================
    // Identifiers
    // ==========================================================================
    /// Identifier: foo, Bar, _baz
    /// Note: Complex identifiers (paths, program IDs, locators) are deferred
    /// to Phase 2. The lexer produces simple IDENT tokens; the parser handles
    /// disambiguation of foo::bar, foo.aleo, foo.aleo/bar patterns.
    IDENT,

    // ==========================================================================
    // Keywords - Literals
    // ==========================================================================
    /// `true`
    KW_TRUE,
    /// `false`
    KW_FALSE,
    /// `none`
    KW_NONE,

    // ==========================================================================
    // Keywords - Types
    // ==========================================================================
    /// `address`
    KW_ADDRESS,
    /// `bool`
    KW_BOOL,
    /// `field`
    KW_FIELD,
    /// `group`
    KW_GROUP,
    /// `scalar`
    KW_SCALAR,
    /// `signature`
    KW_SIGNATURE,
    /// `string`
    KW_STRING,
    /// `record`
    KW_RECORD,
    /// `Final`
    KW_FINAL_UPPER,
    /// `i8`
    KW_I8,
    /// `i16`
    KW_I16,
    /// `i32`
    KW_I32,
    /// `i64`
    KW_I64,
    /// `i128`
    KW_I128,
    /// `u8`
    KW_U8,
    /// `u16`
    KW_U16,
    /// `u32`
    KW_U32,
    /// `u64`
    KW_U64,
    /// `u128`
    KW_U128,

    // ==========================================================================
    // Keywords - Control Flow
    // ==========================================================================
    /// `if`
    KW_IF,
    /// `else`
    KW_ELSE,
    /// `for`
    KW_FOR,
    /// `in`
    KW_IN,
    /// `return`
    KW_RETURN,

    // ==========================================================================
    // Keywords - Declarations
    // ==========================================================================
    /// `let`
    KW_LET,
    /// `const`
    KW_CONST,
    /// `constant`
    KW_CONSTANT,
    /// `final`
    KW_FINAL,
    /// `fn`
    KW_FN,
    /// `Fn`
    KW_FN_UPPER,
    /// `struct`
    KW_STRUCT,
    /// `constructor`
    KW_CONSTRUCTOR,

    // ==========================================================================
    // Keywords - Program Structure
    // ==========================================================================
    /// `program`
    KW_PROGRAM,
    /// `import`
    KW_IMPORT,
    /// `mapping`
    KW_MAPPING,
    /// `storage`
    KW_STORAGE,
    /// `network`
    KW_NETWORK,
    /// `aleo`
    KW_ALEO,
    /// `script`
    KW_SCRIPT,
    /// `block`
    KW_BLOCK,

    // ==========================================================================
    // Keywords - Visibility & Assertions
    // ==========================================================================
    /// `public`
    KW_PUBLIC,
    /// `private`
    KW_PRIVATE,
    /// `as`
    KW_AS,
    /// `self`
    KW_SELF,
    /// `assert`
    KW_ASSERT,
    /// `assert_eq`
    KW_ASSERT_EQ,
    /// `assert_neq`
    KW_ASSERT_NEQ,

    // ==========================================================================
    // Punctuation - Delimiters
    // ==========================================================================
    /// `(`
    L_PAREN,
    /// `)`
    R_PAREN,
    /// `[`
    L_BRACKET,
    /// `]`
    R_BRACKET,
    /// `{`
    L_BRACE,
    /// `}`
    R_BRACE,

    // ==========================================================================
    // Punctuation - Separators
    // ==========================================================================
    /// `,`
    COMMA,
    /// `.`
    DOT,
    /// `..`
    DOT_DOT,
    /// `;`
    SEMICOLON,
    /// `:`
    COLON,
    /// `::`
    COLON_COLON,
    /// `?`
    QUESTION,
    /// `->`
    ARROW,
    /// `=>`
    FAT_ARROW,
    /// `_`
    UNDERSCORE,
    /// `@`
    AT,

    // ==========================================================================
    // Operators - Assignment
    // ==========================================================================
    /// `=`
    EQ,
    /// `+=`
    PLUS_EQ,
    /// `-=`
    MINUS_EQ,
    /// `*=`
    STAR_EQ,
    /// `/=`
    SLASH_EQ,
    /// `%=`
    PERCENT_EQ,
    /// `**=`
    STAR2_EQ,
    /// `&&=`
    AMP2_EQ,
    /// `||=`
    PIPE2_EQ,
    /// `&=`
    AMP_EQ,
    /// `|=`
    PIPE_EQ,
    /// `^=`
    CARET_EQ,
    /// `<<=`
    SHL_EQ,
    /// `>>=`
    SHR_EQ,

    // ==========================================================================
    // Operators - Arithmetic
    // ==========================================================================
    /// `+`
    PLUS,
    /// `-`
    MINUS,
    /// `*`
    STAR,
    /// `/`
    SLASH,
    /// `%`
    PERCENT,
    /// `**`
    STAR2,

    // ==========================================================================
    // Operators - Comparison
    // ==========================================================================
    /// `==`
    EQ2,
    /// `!=`
    BANG_EQ,
    /// `<`
    LT,
    /// `<=`
    LT_EQ,
    /// `>`
    GT,
    /// `>=`
    GT_EQ,

    // ==========================================================================
    // Operators - Logical
    // ==========================================================================
    /// `&&`
    AMP2,
    /// `||`
    PIPE2,
    /// `!`
    BANG,

    // ==========================================================================
    // Operators - Bitwise
    // ==========================================================================
    /// `&`
    AMP,
    /// `|`
    PIPE,
    /// `^`
    CARET,
    /// `<<`
    SHL,
    /// `>>`
    SHR,

    // ==========================================================================
    // Composite Nodes - Top Level
    // ==========================================================================
    /// Root node of the syntax tree.
    ROOT,
    /// Program declaration: `program foo.aleo { ... }`
    PROGRAM_DECL,
    /// Import statement: `import foo.aleo;`
    IMPORT,
    /// Main file contents.
    MAIN_CONTENTS,
    /// Module file contents.
    MODULE_CONTENTS,

    // ==========================================================================
    // Composite Nodes - Declarations
    // ==========================================================================
    /// Function definition.
    FUNCTION_DEF,
    /// Constructor definition.
    CONSTRUCTOR_DEF,
    /// Struct definition.
    STRUCT_DEF,
    /// Record definition.
    RECORD_DEF,
    /// Struct member declaration.
    STRUCT_MEMBER,
    /// Mapping definition.
    MAPPING_DEF,
    /// Storage definition.
    STORAGE_DEF,
    /// Global constant definition.
    GLOBAL_CONST,

    // ==========================================================================
    // Composite Nodes - Function Parts
    // ==========================================================================
    /// Annotation: `@foo`
    ANNOTATION,
    /// Parameter in a function signature.
    PARAM,
    /// Parameter list: `(a: u32, b: u32)`
    PARAM_LIST,
    /// Function output type.
    RETURN_TYPE,
    /// Const generic parameter.
    CONST_PARAM,
    /// Const generic parameter list.
    CONST_PARAM_LIST,
    /// Const generic argument list.
    CONST_ARG_LIST,

    // ==========================================================================
    // Composite Nodes - Statements
    // ==========================================================================
    /// Let statement: `let x = ...;`
    LET_STMT,
    /// Const statement: `const x = ...;`
    CONST_STMT,
    /// Return statement: `return ...;`
    RETURN_STMT,
    /// Expression statement: `foo();`
    EXPR_STMT,
    /// Assignment statement: `x = ...;`
    ASSIGN_STMT,
    /// If statement: `if ... { } else { }`
    IF_STMT,
    /// For loop: `for i in 0..10 { }`
    FOR_STMT,
    /// Block: `{ ... }`
    BLOCK,
    /// Assert statement: `assert(...);`
    ASSERT_STMT,
    /// Assert equals statement: `assert_eq(...);`
    ASSERT_EQ_STMT,
    /// Assert not equals statement: `assert_neq(...);`
    ASSERT_NEQ_STMT,

    // ==========================================================================
    // Composite Nodes - Patterns
    // ==========================================================================
    /// Identifier pattern: `x`
    IDENT_PATTERN,
    /// Tuple pattern: `(a, b, c)`
    TUPLE_PATTERN,
    /// Wildcard pattern: `_`
    WILDCARD_PATTERN,

    // ==========================================================================
    // Composite Nodes - Expressions
    // ==========================================================================
    /// Binary expression: `a + b`
    BINARY_EXPR,
    /// Unary expression: `!a`, `-a`
    UNARY_EXPR,
    /// Function call: `foo(a, b)`
    CALL_EXPR,
    /// Method call: `a.foo(b)`
    METHOD_CALL_EXPR,
    /// Member access: `a.b`
    FIELD_EXPR,
    /// Array/tuple index: `a[0]`
    INDEX_EXPR,
    /// Cast expression: `a as u32`
    CAST_EXPR,
    /// Ternary expression: `a ? b : c`
    TERNARY_EXPR,
    /// Array literal: `[1, 2, 3]`
    ARRAY_EXPR,
    /// Tuple literal: `(1, 2, 3)`
    TUPLE_EXPR,
    /// Struct literal: `Foo { a: 1, b: 2 }`
    STRUCT_EXPR,
    /// Struct field initializer: `a: 1`
    STRUCT_FIELD_INIT,
    /// Path expression: `foo::bar`
    PATH_EXPR,
    /// Parenthesized expression: `(a + b)`
    PAREN_EXPR,
    /// Literal expression (wraps INTEGER, STRING, ADDRESS_LIT, or keywords).
    LITERAL,
    /// Repeat expression: `[0u8; 32]`
    REPEAT_EXPR,
    /// Async expression: `async foo()`
    FINAL_EXPR,
    /// Associated function call: `Foo::bar()`
    ASSOC_FN_EXPR,
    /// Associated constant: `Foo::BAR`
    ASSOC_CONST_EXPR,
    /// Locator expression: `foo.aleo/bar`
    LOCATOR_EXPR,
    /// Tuple access: `a.0`
    TUPLE_ACCESS_EXPR,
    /// Intrinsic expression: `_foo()`
    INTRINSIC_EXPR,
    /// Unit expression: `()`
    UNIT_EXPR,

    // ==========================================================================
    // Composite Nodes - Types
    // ==========================================================================
    /// Named/path type: `u32`, `Foo`, `foo::Bar`
    TYPE_PATH,
    /// Array type: `[u32; 10]`
    TYPE_ARRAY,
    /// Tuple type: `(u32, u32)`
    TYPE_TUPLE,
    /// Optional type: `u32?` (Future feature)
    TYPE_OPTIONAL,
    /// Final type: `Final<Foo>`
    TYPE_FINAL,
    /// Mapping type in storage.
    TYPE_MAPPING,

    // ==========================================================================
    // Composite Nodes - Other
    // ==========================================================================
    /// Argument list: `(a, b, c)`
    ARG_LIST,
    /// Name reference (identifier in expression context).
    NAME_REF,
    /// Name definition (identifier in binding context).
    NAME,
    /// Visibility modifier: `public`, `private`
    VISIBILITY,

    // Sentinel for bounds checking (must be last)
    #[doc(hidden)]
    __LAST,
}

impl SyntaxKind {
    /// Check if this is a trivia token (whitespace or comment).
    pub fn is_trivia(self) -> bool {
        matches!(self, WHITESPACE | LINEBREAK | COMMENT_LINE | COMMENT_BLOCK)
    }

    /// Check if this is a keyword.
    pub fn is_keyword(self) -> bool {
        matches!(
            self,
            KW_TRUE
                | KW_FALSE
                | KW_NONE
                | KW_ADDRESS
                | KW_BOOL
                | KW_FIELD
                | KW_GROUP
                | KW_SCALAR
                | KW_SIGNATURE
                | KW_STRING
                | KW_RECORD
                | KW_FINAL_UPPER
                | KW_I8
                | KW_I16
                | KW_I32
                | KW_I64
                | KW_I128
                | KW_U8
                | KW_U16
                | KW_U32
                | KW_U64
                | KW_U128
                | KW_IF
                | KW_ELSE
                | KW_FOR
                | KW_IN
                | KW_RETURN
                | KW_LET
                | KW_CONST
                | KW_CONSTANT
                | KW_FN
                | KW_FINAL
                | KW_FN_UPPER
                | KW_STRUCT
                | KW_CONSTRUCTOR
                | KW_PROGRAM
                | KW_IMPORT
                | KW_MAPPING
                | KW_STORAGE
                | KW_NETWORK
                | KW_ALEO
                | KW_SCRIPT
                | KW_BLOCK
                | KW_PUBLIC
                | KW_PRIVATE
                | KW_AS
                | KW_SELF
                | KW_ASSERT
                | KW_ASSERT_EQ
                | KW_ASSERT_NEQ
        )
    }

    /// Check if this is a type keyword.
    pub fn is_type_keyword(self) -> bool {
        matches!(
            self,
            KW_ADDRESS
                | KW_BOOL
                | KW_FIELD
                | KW_GROUP
                | KW_SCALAR
                | KW_SIGNATURE
                | KW_STRING
                | KW_FINAL_UPPER
                | KW_I8
                | KW_I16
                | KW_I32
                | KW_I64
                | KW_I128
                | KW_U8
                | KW_U16
                | KW_U32
                | KW_U64
                | KW_U128
        )
    }

    /// Check if this is a literal token.
    pub fn is_literal(self) -> bool {
        matches!(self, INTEGER | STRING | ADDRESS_LIT | KW_TRUE | KW_FALSE | KW_NONE)
    }

    /// Check if this is a punctuation token.
    pub fn is_punctuation(self) -> bool {
        matches!(
            self,
            L_PAREN
                | R_PAREN
                | L_BRACKET
                | R_BRACKET
                | L_BRACE
                | R_BRACE
                | COMMA
                | DOT
                | DOT_DOT
                | SEMICOLON
                | COLON
                | COLON_COLON
                | QUESTION
                | ARROW
                | FAT_ARROW
                | UNDERSCORE
                | AT
        )
    }

    /// Check if this is an operator token.
    pub fn is_operator(self) -> bool {
        matches!(
            self,
            EQ | PLUS_EQ
                | MINUS_EQ
                | STAR_EQ
                | SLASH_EQ
                | PERCENT_EQ
                | STAR2_EQ
                | AMP2_EQ
                | PIPE2_EQ
                | AMP_EQ
                | PIPE_EQ
                | CARET_EQ
                | SHL_EQ
                | SHR_EQ
                | PLUS
                | MINUS
                | STAR
                | SLASH
                | PERCENT
                | STAR2
                | EQ2
                | BANG_EQ
                | LT
                | LT_EQ
                | GT
                | GT_EQ
                | AMP2
                | PIPE2
                | BANG
                | AMP
                | PIPE
                | CARET
                | SHL
                | SHR
        )
    }
}

impl From<SyntaxKind> for rowan::SyntaxKind {
    fn from(kind: SyntaxKind) -> Self {
        Self(kind as u16)
    }
}

/// Lookup table for converting raw u16 values back to SyntaxKind.
/// This avoids unsafe transmute by using an explicit array.
const SYNTAX_KIND_TABLE: &[SyntaxKind] = &[
    ERROR,
    EOF,
    WHITESPACE,
    LINEBREAK,
    COMMENT_LINE,
    COMMENT_BLOCK,
    INTEGER,
    STRING,
    ADDRESS_LIT,
    IDENT,
    KW_TRUE,
    KW_FALSE,
    KW_NONE,
    KW_ADDRESS,
    KW_BOOL,
    KW_FIELD,
    KW_GROUP,
    KW_SCALAR,
    KW_SIGNATURE,
    KW_STRING,
    KW_RECORD,
    KW_FINAL_UPPER,
    KW_I8,
    KW_I16,
    KW_I32,
    KW_I64,
    KW_I128,
    KW_U8,
    KW_U16,
    KW_U32,
    KW_U64,
    KW_U128,
    KW_IF,
    KW_ELSE,
    KW_FOR,
    KW_IN,
    KW_RETURN,
    KW_LET,
    KW_CONST,
    KW_CONSTANT,
    KW_FINAL,
    KW_FN,
    KW_FN_UPPER,
    KW_STRUCT,
    KW_CONSTRUCTOR,
    KW_PROGRAM,
    KW_IMPORT,
    KW_MAPPING,
    KW_STORAGE,
    KW_NETWORK,
    KW_ALEO,
    KW_SCRIPT,
    KW_BLOCK,
    KW_PUBLIC,
    KW_PRIVATE,
    KW_AS,
    KW_SELF,
    KW_ASSERT,
    KW_ASSERT_EQ,
    KW_ASSERT_NEQ,
    L_PAREN,
    R_PAREN,
    L_BRACKET,
    R_BRACKET,
    L_BRACE,
    R_BRACE,
    COMMA,
    DOT,
    DOT_DOT,
    SEMICOLON,
    COLON,
    COLON_COLON,
    QUESTION,
    ARROW,
    FAT_ARROW,
    UNDERSCORE,
    AT,
    EQ,
    PLUS_EQ,
    MINUS_EQ,
    STAR_EQ,
    SLASH_EQ,
    PERCENT_EQ,
    STAR2_EQ,
    AMP2_EQ,
    PIPE2_EQ,
    AMP_EQ,
    PIPE_EQ,
    CARET_EQ,
    SHL_EQ,
    SHR_EQ,
    PLUS,
    MINUS,
    STAR,
    SLASH,
    PERCENT,
    STAR2,
    EQ2,
    BANG_EQ,
    LT,
    LT_EQ,
    GT,
    GT_EQ,
    AMP2,
    PIPE2,
    BANG,
    AMP,
    PIPE,
    CARET,
    SHL,
    SHR,
    ROOT,
    PROGRAM_DECL,
    IMPORT,
    MAIN_CONTENTS,
    MODULE_CONTENTS,
    FUNCTION_DEF,
    CONSTRUCTOR_DEF,
    STRUCT_DEF,
    RECORD_DEF,
    STRUCT_MEMBER,
    MAPPING_DEF,
    STORAGE_DEF,
    GLOBAL_CONST,
    ANNOTATION,
    PARAM,
    PARAM_LIST,
    RETURN_TYPE,
    CONST_PARAM,
    CONST_PARAM_LIST,
    CONST_ARG_LIST,
    LET_STMT,
    CONST_STMT,
    RETURN_STMT,
    EXPR_STMT,
    ASSIGN_STMT,
    IF_STMT,
    FOR_STMT,
    BLOCK,
    ASSERT_STMT,
    ASSERT_EQ_STMT,
    ASSERT_NEQ_STMT,
    IDENT_PATTERN,
    TUPLE_PATTERN,
    WILDCARD_PATTERN,
    BINARY_EXPR,
    UNARY_EXPR,
    CALL_EXPR,
    METHOD_CALL_EXPR,
    FIELD_EXPR,
    INDEX_EXPR,
    CAST_EXPR,
    TERNARY_EXPR,
    ARRAY_EXPR,
    TUPLE_EXPR,
    STRUCT_EXPR,
    STRUCT_FIELD_INIT,
    PATH_EXPR,
    PAREN_EXPR,
    LITERAL,
    REPEAT_EXPR,
    FINAL_EXPR,
    ASSOC_FN_EXPR,
    ASSOC_CONST_EXPR,
    LOCATOR_EXPR,
    TUPLE_ACCESS_EXPR,
    INTRINSIC_EXPR,
    UNIT_EXPR,
    TYPE_PATH,
    TYPE_ARRAY,
    TYPE_TUPLE,
    TYPE_OPTIONAL,
    TYPE_FINAL,
    TYPE_MAPPING,
    ARG_LIST,
    NAME_REF,
    NAME,
    VISIBILITY,
    __LAST,
];

/// Convert a raw rowan SyntaxKind to our SyntaxKind.
///
/// # Panics
/// Panics if the raw value is out of range.
pub fn syntax_kind_from_raw(raw: rowan::SyntaxKind) -> SyntaxKind {
    SYNTAX_KIND_TABLE.get(raw.0 as usize).copied().unwrap_or_else(|| panic!("invalid SyntaxKind: {}", raw.0))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn syntax_kind_table_is_correct() {
        // Verify that the table matches the enum discriminants
        for (i, &kind) in SYNTAX_KIND_TABLE.iter().enumerate() {
            assert_eq!(
                kind as u16, i as u16,
                "SYNTAX_KIND_TABLE[{i}] = {:?} has discriminant {}, expected {i}",
                kind, kind as u16
            );
        }
    }

    #[test]
    fn syntax_kind_roundtrip() {
        // Test that we can convert to rowan::SyntaxKind and back
        for &kind in SYNTAX_KIND_TABLE.iter() {
            if kind == __LAST {
                continue;
            }
            let raw: rowan::SyntaxKind = kind.into();
            let back = syntax_kind_from_raw(raw);
            assert_eq!(kind, back);
        }
    }

    #[test]
    fn is_trivia() {
        assert!(WHITESPACE.is_trivia());
        assert!(LINEBREAK.is_trivia());
        assert!(COMMENT_LINE.is_trivia());
        assert!(COMMENT_BLOCK.is_trivia());
        assert!(!IDENT.is_trivia());
        assert!(!KW_LET.is_trivia());
    }

    #[test]
    fn is_keyword() {
        assert!(KW_LET.is_keyword());
        assert!(KW_FN.is_keyword());
        assert!(KW_TRUE.is_keyword());
        assert!(!IDENT.is_keyword());
        assert!(!PLUS.is_keyword());
    }

    #[test]
    fn is_literal() {
        assert!(INTEGER.is_literal());
        assert!(STRING.is_literal());
        assert!(ADDRESS_LIT.is_literal());
        assert!(KW_TRUE.is_literal());
        assert!(KW_FALSE.is_literal());
        assert!(KW_NONE.is_literal());
        assert!(!IDENT.is_literal());
    }
}
