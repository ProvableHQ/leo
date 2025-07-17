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

use lalrpop_util::lalrpop_mod;

use leo_errors::{Handler, Result};

lalrpop_mod!(pub grammar);

pub mod tokens;

use tokens::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SyntaxKind {
    Whitespace,
    Linebreak,
    CommentLine,
    CommentBlock,

    Expression(ExpressionKind),
    StructMemberInitializer,

    Statement(StatementKind),
    Type(TypeKind),
    Token,

    Annotation,
    AnnotationMember,
    AnnotationList,

    Parameter,
    ParameterList,
    FunctionOutput,
    FunctionOutputs,
    Function,
    Constructor,

    ConstParameter,
    ConstParameterList,
    ConstArgumentList,

    StructDeclaration,
    StructMemberDeclaration,
    StructMemberDeclarationList,

    Mapping,

    GlobalConst,

    Import,
    MainContents,
    ModuleContents,
    ProgramDeclaration,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum IntegerLiteralKind {
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
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum IntegerTypeKind {
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
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TypeKind {
    /// The `address` type.
    Address,
    /// The array type.
    Array,
    /// The `bool` type.
    Boolean,
    /// The `struct` type.
    Composite,
    /// The `field` type.
    Field,
    /// The `future` type.
    Future,
    /// The `group` type.
    Group,
    /// A reference to a built in type.
    Identifier,
    /// An integer type.
    Integer(IntegerTypeKind),
    /// A mapping type.
    Mapping,
    /// The `scalar` type.
    Scalar,
    /// The `signature` type.
    Signature,
    /// The `string` type.
    String,
    /// A static tuple of at least one type.
    Tuple,
    /// Numeric type which should be resolved to `Field`, `Group`, `Integer(_)`, or `Scalar`.
    Numeric,
    /// The `unit` type.
    Unit,
}

impl From<TypeKind> for SyntaxKind {
    fn from(value: TypeKind) -> Self {
        SyntaxKind::Type(value)
    }
}

impl From<IntegerTypeKind> for TypeKind {
    fn from(value: IntegerTypeKind) -> Self {
        TypeKind::Integer(value)
    }
}

impl From<IntegerTypeKind> for SyntaxKind {
    fn from(value: IntegerTypeKind) -> Self {
        SyntaxKind::Type(TypeKind::Integer(value))
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExpressionKind {
    /// An array access, e.g. `arr[i]`.
    ArrayAccess,
    /// An associated constant; e.g., `group::GEN`.
    AssociatedConstant,
    /// An associated function; e.g., `BHP256::hash_to_field`.
    AssociatedFunctionCall,
    /// An array expression, e.g., `[true, false, true, false]`.
    Array,
    /// A binary expression, e.g., `42 + 24`.
    Binary,
    /// A call expression, e.g., `my_fun(args)`.
    Call,
    /// A cast expression, e.g., `42u32 as u8`.
    Cast,
    /// A path.
    Path,
    /// A literal expression.
    Literal(LiteralKind),
    /// A locator expression, e.g., `hello.aleo/foo`.
    Locator,
    /// An access of a struct member, e.g. `struc.member`.
    MemberAccess,
    MethodCall,
    Parenthesized,
    /// An array expression constructed from one repeated element, e.g., `[1u32; 5]`.
    Repeat,
    // program.id, block.height, etc
    SpecialAccess,
    /// An expression constructing a struct like `Foo { bar: 42, baz }`.
    Struct,
    /// A ternary conditional expression `cond ? if_expr : else_expr`.
    Ternary,
    /// A tuple expression e.g., `(foo, 42, true)`.
    Tuple,
    /// A tuple access expression e.g., `foo.2`.
    TupleAccess,
    /// An unary expression.
    Unary,
    /// A unit expression e.g. `()`
    Unit,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LiteralKind {
    Address,
    /// A boolean literal, either `true` or `false`.
    Boolean,
    /// A field literal, e.g., `42field`.
    /// A signed number followed by the keyword `field`.
    Field,
    /// A group literal, eg `42group`.
    Group,
    /// An integer literal, e.g., `42u32`.
    Integer(IntegerLiteralKind),
    /// A scalar literal, e.g. `1scalar`.
    /// An unsigned number followed by the keyword `scalar`.
    Scalar,
    /// An unsuffixed literal, e.g. `42` (without a type suffix)
    Unsuffixed,
    /// A string literal, e.g., `"foobar"`.
    String,
}

impl From<ExpressionKind> for SyntaxKind {
    fn from(value: ExpressionKind) -> Self {
        SyntaxKind::Expression(value)
    }
}

impl From<LiteralKind> for ExpressionKind {
    fn from(value: LiteralKind) -> Self {
        ExpressionKind::Literal(value)
    }
}

impl From<LiteralKind> for SyntaxKind {
    fn from(value: LiteralKind) -> Self {
        SyntaxKind::Expression(ExpressionKind::Literal(value))
    }
}

impl From<IntegerLiteralKind> for LiteralKind {
    fn from(value: IntegerLiteralKind) -> Self {
        LiteralKind::Integer(value)
    }
}

impl From<IntegerLiteralKind> for ExpressionKind {
    fn from(value: IntegerLiteralKind) -> Self {
        ExpressionKind::Literal(LiteralKind::Integer(value))
    }
}

impl From<IntegerLiteralKind> for SyntaxKind {
    fn from(value: IntegerLiteralKind) -> Self {
        SyntaxKind::Expression(ExpressionKind::Literal(LiteralKind::Integer(value)))
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StatementKind {
    Assert,
    AssertEq,
    AssertNeq,
    Assign,
    Block,
    Conditional,
    Const,
    Definition,
    Expression,
    Iteration,
    Return,
}

impl From<StatementKind> for SyntaxKind {
    fn from(value: StatementKind) -> Self {
        SyntaxKind::Statement(value)
    }
}

#[derive(Debug, Clone)]
pub struct SyntaxNode<'a> {
    pub kind: SyntaxKind,
    pub text: &'a str,
    pub span: leo_span::Span,
    pub children: Vec<SyntaxNode<'a>>,
}

impl<'a> SyntaxNode<'a> {
    fn new_token(kind: SyntaxKind, token: LalrToken<'a>, children: Vec<Self>) -> Self {
        Self { kind, text: token.text, span: token.span, children }
    }

    fn new(kind: impl Into<SyntaxKind>, children: impl IntoIterator<Item = Self>) -> Self {
        let children: Vec<Self> = children.into_iter().collect();
        let lo = children.first().unwrap().span.lo;
        let hi = children.last().unwrap().span.hi;
        let span = leo_span::Span { lo, hi };
        Self { kind: kind.into(), text: "", span, children }
    }

    fn suffixed_literal(integer: LalrToken<'a>, suffix: LalrToken<'a>) -> Self {
        let kind: SyntaxKind = match suffix.token {
            Token::Field => LiteralKind::Field.into(),
            Token::Group => LiteralKind::Group.into(),
            Token::Scalar => LiteralKind::Scalar.into(),
            Token::I8 => IntegerLiteralKind::I8.into(),
            Token::I16 => IntegerLiteralKind::I16.into(),
            Token::I32 => IntegerLiteralKind::I32.into(),
            Token::I64 => IntegerLiteralKind::I64.into(),
            Token::I128 => IntegerLiteralKind::I128.into(),
            Token::U8 => IntegerLiteralKind::U8.into(),
            Token::U16 => IntegerLiteralKind::U16.into(),
            Token::U32 => IntegerLiteralKind::U32.into(),
            Token::U64 => IntegerLiteralKind::U64.into(),
            Token::U128 => IntegerLiteralKind::U128.into(),
            x => panic!("Error in grammar.lalrpop: {x:?}"),
        };

        let span = leo_span::Span { lo: integer.span.lo, hi: suffix.span.hi };

        Self { kind, text: integer.text, span, children: Vec::new() }
    }

    fn suffixed_literal2(integer: LalrToken<'a>, suffix: LalrToken<'a>, children: Vec<Self>) -> Self {
        let kind: SyntaxKind = match suffix.token {
            Token::Field => LiteralKind::Field.into(),
            Token::Group => LiteralKind::Group.into(),
            Token::Scalar => LiteralKind::Scalar.into(),
            Token::I8 => IntegerLiteralKind::I8.into(),
            Token::I16 => IntegerLiteralKind::I16.into(),
            Token::I32 => IntegerLiteralKind::I32.into(),
            Token::I64 => IntegerLiteralKind::I64.into(),
            Token::I128 => IntegerLiteralKind::I128.into(),
            Token::U8 => IntegerLiteralKind::U8.into(),
            Token::U16 => IntegerLiteralKind::U16.into(),
            Token::U32 => IntegerLiteralKind::U32.into(),
            Token::U64 => IntegerLiteralKind::U64.into(),
            Token::U128 => IntegerLiteralKind::U128.into(),
            x => panic!("Error in grammar.lalrpop: {x:?}"),
        };

        let lo = integer.span.lo;
        let hi = suffix.span.hi;
        let span = leo_span::Span { lo, hi };

        Self { kind, text: integer.text, span, children }
    }

    fn binary_expression(lhs: Self, op: Self, rhs: Self) -> Self {
        let span = leo_span::Span { lo: lhs.span.lo, hi: rhs.span.hi };
        let children = vec![lhs, op, rhs];
        SyntaxNode { kind: ExpressionKind::Binary.into(), text: "", span, children }
    }

    fn unary_expression(op: LalrToken<'a>, ys: Vec<Self>, operand: Self) -> Self {
        let span = leo_span::Span { lo: op.span.lo, hi: operand.span.hi };
        let op_node = SyntaxNode { kind: ExpressionKind::Unary.into(), text: op.text, span: op.span, children: ys };
        let children = vec![op_node, operand];
        SyntaxNode { kind: ExpressionKind::Unary.into(), text: "", span, children }
    }
}

fn two_path_components(text: &str) -> Option<(&str, &str)> {
    let mut iter = text.split("::");

    match (iter.next(), iter.next(), iter.next()) {
        (Some(first), Some(second), None) => Some((first, second)),
        _ => None,
    }
}

pub fn parse_expression<'a>(_handler: Handler, source: &'a str, start_pos: u32) -> Result<SyntaxNode<'a>> {
    let mut lexer = tokens::Lexer::new(source, start_pos);
    let expr_node = grammar::ExprParser::new().parse(&mut lexer).unwrap();
    Ok(expr_node)
}

pub fn parse_statement<'a>(_handler: Handler, source: &'a str, start_pos: u32) -> Result<SyntaxNode<'a>> {
    let mut lexer = tokens::Lexer::new(source, start_pos);
    let statement_node = grammar::StatementParser::new().parse(&mut lexer).unwrap();
    Ok(statement_node)
}

pub fn parse_module<'a>(_handler: Handler, source: &'a str, start_pos: u32) -> Result<SyntaxNode<'a>> {
    let mut lexer = tokens::Lexer::new(source, start_pos);
    let module_node = grammar::ModuleContentsParser::new().parse(&mut lexer).unwrap();
    Ok(module_node)
}

pub fn parse_main<'a>(_handler: Handler, source: &'a str, start_pos: u32) -> Result<SyntaxNode<'a>> {
    let mut lexer = tokens::Lexer::new(source, start_pos);
    let main_contents = grammar::MainContentsParser::new().parse(&mut lexer).unwrap();
    Ok(main_contents)
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_simple() {
        const TEXT: &str = "43u32 * 1field ** /* abc */ 2field//    \n";
        let mut lexer = tokens::Lexer::new(TEXT, 0);
        let x = grammar::ExprParser::new().parse(&mut lexer).unwrap();

        println!("{x:#?}");
    }

    #[test]
    fn test_simple2() {
        const TEXT: &str = "


program test.aleo {
    async transition main() -> bool {

    }

    async function finalize_main() {

    }

    function main() -> bool {

    }
    async function main(a: foo, b: bar) -> baz {

    }


}        
";
        let mut lexer = tokens::Lexer::new(TEXT, 0);
        let x = grammar::MainContentsParser::new().parse(&mut lexer).unwrap();

        println!("{x:#?}");
    }
}
