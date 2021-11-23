#  Leo Parser
[![Crates.io](https://img.shields.io/crates/v/leo-parser.svg?color=neon)](https://crates.io/crates/leo-parser)
[![Authors](https://img.shields.io/badge/authors-Aleo-orange.svg)](../AUTHORS)
[![License](https://img.shields.io/badge/License-GPLv3-blue.svg)](./LICENSE.md)

This directory contains the code to tokenize, lex and parse Leo files to the [Leo AST](./../ast/README.md).

## Tokenizer

The tokenizer contains all tokens in Leo.
It also decides which tokens are keywords.
Meaning that keywords are a subset of tokens.
The lexer goes through character by character as bytes, and converts the bytes into the tokens.

### Tokens

Bolded ones are also keywords.

#### Literals
- CommentLine
- CommentBlock
- StringLit
- Ident
- Int
- **True**
- **False**
- AddressLit
- CharLit

#### Symbols
- At
- Not
- And (`&&`)
- Ampersand (`&`)
- Or
- Eq
- NotEq
- Lt
- LtEq
- Gt
- GtEq
- Add
- Minus
- Mul
- Div
- Exp
- Assign
- AddEq
- MinusEq
- MulEq
- DivEq
- ExpEq
- LeftParen
- RightParen
- LeftSquare
- RightSquare
- LeftCurly
- RightCurly
- Comma
- Dot
- DotDot
- DotDotDot
- Semicolon
- Colon
- DoubleColon
- Question
- Arrow
- Underscore

#### Types
- **U8**
- **U16**
- **U32**
- **U64**
- **U128**
- **I8**
- **I16**
- **I32**
- **I64**
- **I128**
- **Field**
- **Group**
- **Bool**
- **Address**
- **Char**
- **BigSelf**

#### Words
- **Input**
- **LittleSelf**
- **Import**
- **As**
- **Struct**
- **Console**
- **Const**
- **Else**
- **For**
- **Function**
- **If**
- **In**
- **Let**
- **Return**
- **Static**
- **String**

#### Meta
- Eof

## Parser

The parser converts the tokens to the [Leo AST](./../ast/README.md).

The parser is broken down to different files that correspond to different aspects of the AST:

- [File](./src/parser/file.rs) - Parses the top level nodes in Leo.
- [Types](./src/parser/type_.rs) - Parses the type declarations in Leo.
- [Statements](./src/parser/statement.rs) - Parses the different kinds of statements.
- [Expressions](./src/parser/expression.rs) - Parses the different kinds of expressions.
  
  For more information on those please read the Leo AST README, linked above.

## Grammar Relation

All function and token names are as close as possible to the [Leo Grammar](./../grammar/README.md)
