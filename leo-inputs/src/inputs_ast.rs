//! Abstract syntax tree (ast) representation from leo-inputs.pest.

use pest::{error::Error, iterators::Pairs, Parser, Span};
use pest_ast::FromPest;

#[derive(Parser)]
#[grammar = "leo-inputs.pest"]
pub struct LanguageParser;

pub fn parse(input: &str) -> Result<Pairs<Rule>, Error<Rule>> {
    LanguageParser::parse(Rule::file, input)
}

fn span_into_string(span: Span) -> String {
    span.as_str().to_string()
}

// Visibility

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::visibility_public))]
pub struct Public {}

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::visibility_private))]
pub struct Private {}

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::visibility))]
pub enum Visibility {
    Public(Public),
    Private(Private),
}

// Types

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::type_u32))]
pub struct U32Type<'ast> {
    #[pest_ast(outer())]
    pub span: Span<'ast>,
}

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::type_field))]
pub struct FieldType<'ast> {
    #[pest_ast(outer())]
    pub span: Span<'ast>,
}

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::type_bool))]
pub struct BooleanType<'ast> {
    #[pest_ast(outer())]
    pub span: Span<'ast>,
}

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::type_basic))]
pub enum BasicType<'ast> {
    U32(U32Type<'ast>),
    Field(FieldType<'ast>),
    Boolean(BooleanType<'ast>),
}

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::type_struct))]
pub struct StructType<'ast> {
    pub variable: Variable<'ast>,
    #[pest_ast(outer())]
    pub span: Span<'ast>,
}

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::type_array))]
pub struct ArrayType<'ast> {
    pub _type: BasicType<'ast>,
    pub count: Value<'ast>,
    #[pest_ast(outer())]
    pub span: Span<'ast>,
}

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::_type))]
pub enum Type<'ast> {
    Basic(BasicType<'ast>),
    Array(ArrayType<'ast>),
    Struct(StructType<'ast>),
}

// Values

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::value_number))]
pub struct Number<'ast> {
    #[pest_ast(outer(with(span_into_string)))]
    pub value: String,
    #[pest_ast(outer())]
    pub span: Span<'ast>,
}

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::value_u32))]
pub struct U32<'ast> {
    pub number: Number<'ast>,
    pub _type: Option<U32Type<'ast>>,
    #[pest_ast(outer())]
    pub span: Span<'ast>,
}

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::value_field))]
pub struct Field<'ast> {
    pub number: Number<'ast>,
    pub _type: FieldType<'ast>,
    #[pest_ast(outer())]
    pub span: Span<'ast>,
}

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::value_boolean))]
pub struct Boolean<'ast> {
    #[pest_ast(outer(with(span_into_string)))]
    pub value: String,
    #[pest_ast(outer())]
    pub span: Span<'ast>,
}

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::value))]
pub enum Value<'ast> {
    Field(Field<'ast>),
    Boolean(Boolean<'ast>),
    U32(U32<'ast>),
}

// Variables

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::variable))]
pub struct Variable<'ast> {
    #[pest_ast(outer(with(span_into_string)))]
    pub value: String,
    #[pest_ast(outer())]
    pub span: Span<'ast>,
}

// Arrays

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::expression_array_inline))]
pub struct ArrayInlineExpression<'ast> {
    pub expressions: Vec<Expression<'ast>>,
    #[pest_ast(outer())]
    pub span: Span<'ast>,
}

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::expression_array_initializer))]
pub struct ArrayInitializerExpression<'ast> {
    pub expression: Box<Expression<'ast>>,
    pub count: Value<'ast>,
    #[pest_ast(outer())]
    pub span: Span<'ast>,
}

// Structs

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::inline_struct_member))]
pub struct InlineStructMember<'ast> {
    pub variable: Variable<'ast>,
    pub expression: Expression<'ast>,
    #[pest_ast(outer())]
    pub span: Span<'ast>,
}

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::expression_inline_struct))]
pub struct StructInlineExpression<'ast> {
    pub variable: Variable<'ast>,
    pub members: Vec<InlineStructMember<'ast>>,
    #[pest_ast(outer())]
    pub span: Span<'ast>,
}

// Expressions

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::expression))]
pub enum Expression<'ast> {
    StructInline(StructInlineExpression<'ast>),
    ArrayInline(ArrayInlineExpression<'ast>),
    ArrayInitializer(ArrayInitializerExpression<'ast>),
    Value(Value<'ast>),
    Variable(Variable<'ast>),
}

// Functions

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::parameter))]
pub struct Parameter<'ast> {
    pub variable: Variable<'ast>,
    pub visibility: Option<Visibility>,
    pub _type: Type<'ast>,
    #[pest_ast(outer())]
    pub span: Span<'ast>,
}

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::function_name))]
pub struct FunctionName<'ast> {
    #[pest_ast(outer(with(span_into_string)))]
    pub value: String,
    #[pest_ast(outer())]
    pub span: Span<'ast>,
}

// Sections

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::header))]
pub struct Header<'ast> {
    pub function_name: FunctionName<'ast>,
    #[pest_ast(outer())]
    pub span: Span<'ast>,
}

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::assignment))]
pub struct Assignment<'ast> {
    pub parameter: Parameter<'ast>,
    pub expression: Expression<'ast>,
    #[pest_ast(outer())]
    pub span: Span<'ast>,
}

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::section))]
pub struct Section<'ast> {
    pub header: Header<'ast>,
    pub assignments: Vec<Assignment<'ast>>,
    #[pest_ast(outer())]
    pub span: Span<'ast>,
}

// Utilities

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::EOI))]
pub struct EOI;

// File

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::file))]
pub struct File<'ast> {
    pub sections: Vec<Section<'ast>>,
    pub eoi: EOI,
    #[pest_ast(outer())]
    pub span: Span<'ast>,
}
