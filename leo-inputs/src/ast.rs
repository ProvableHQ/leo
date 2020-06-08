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
#[pest_ast(rule(Rule::type_field))]
pub struct FieldType {}

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::type_group))]
pub struct GroupType {}

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::type_boolean))]
pub struct BooleanType {}

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::type_data))]
pub enum DataType {
    Integer(IntegerType),
    Field(FieldType),
    Group(GroupType),
    Boolean(BooleanType),
}

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::type_circuit))]
pub struct CircuitType<'ast> {
    pub variable: Identifier<'ast>,
    #[pest_ast(outer())]
    pub span: Span<'ast>,
}

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::type_array))]
pub struct ArrayType<'ast> {
    pub _type: DataType,
    pub dimensions: Vec<Value<'ast>>,
    #[pest_ast(outer())]
    pub span: Span<'ast>,
}

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::_type))]
pub enum Type<'ast> {
    Data(DataType),
    Array(ArrayType<'ast>),
    Circuit(CircuitType<'ast>),
}

// Values

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::value_number))]
pub struct NumberValue<'ast> {
    #[pest_ast(outer(with(span_into_string)))]
    pub value: String,
    #[pest_ast(outer())]
    pub span: Span<'ast>,
}

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::value_field))]
pub struct FieldValue<'ast> {
    pub number: NumberValue<'ast>,
    pub _type: FieldType,
    #[pest_ast(outer())]
    pub span: Span<'ast>,
}

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::value_group))]
pub struct GroupValue<'ast> {
    pub value: GroupRepresentation<'ast>,
    pub _type: GroupType,
    #[pest_ast(outer())]
    pub span: Span<'ast>,
}

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::group_single_or_tuple))]
pub enum GroupRepresentation<'ast> {
    Single(NumberValue<'ast>),
    Tuple(GroupTuple<'ast>),
}

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::group_tuple))]
pub struct GroupTuple<'ast> {
    pub x: NumberValue<'ast>,
    pub y: NumberValue<'ast>,
    #[pest_ast(outer())]
    pub span: Span<'ast>,
}

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::value_boolean))]
pub struct BooleanValue<'ast> {
    #[pest_ast(outer(with(span_into_string)))]
    pub value: String,
    #[pest_ast(outer())]
    pub span: Span<'ast>,
}
#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::type_integer))]
pub enum IntegerType {
    U8Type(U8Type),
    U16Type(U16Type),
    U32Type(U32Type),
    U64Type(U64Type),
    U128Type(U128Type),
}

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::type_u8))]
pub struct U8Type {}

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::type_u16))]
pub struct U16Type {}

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::type_u32))]
pub struct U32Type {}

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::type_u64))]
pub struct U64Type {}

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::type_u128))]
pub struct U128Type {}

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::value_integer))]
pub struct IntegerValue<'ast> {
    pub number: NumberValue<'ast>,
    pub _type: IntegerType,
    #[pest_ast(outer())]
    pub span: Span<'ast>,
}

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::value_implicit))]
pub struct NumberImplicitValue<'ast> {
    pub number: NumberValue<'ast>,
    #[pest_ast(outer())]
    pub span: Span<'ast>,
}

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::value))]
pub enum Value<'ast> {
    Integer(IntegerValue<'ast>),
    Field(FieldValue<'ast>),
    Group(GroupValue<'ast>),
    Boolean(BooleanValue<'ast>),
    Implicit(NumberImplicitValue<'ast>),
}
// Identifier

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::identifier))]
pub struct Identifier<'ast> {
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

// Circuits

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::circuit_field))]
pub struct CircuitField<'ast> {
    pub variable: Identifier<'ast>,
    pub expression: Expression<'ast>,
    #[pest_ast(outer())]
    pub span: Span<'ast>,
}

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::expression_circuit_inline))]
pub struct CircuitInlineExpression<'ast> {
    pub variable: Identifier<'ast>,
    pub members: Vec<CircuitField<'ast>>,
    #[pest_ast(outer())]
    pub span: Span<'ast>,
}

// Expressions

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::expression))]
pub enum Expression<'ast> {
    CircuitInline(CircuitInlineExpression<'ast>),
    ArrayInline(ArrayInlineExpression<'ast>),
    ArrayInitializer(ArrayInitializerExpression<'ast>),
    Value(Value<'ast>),
    Variable(Identifier<'ast>),
}

// Parameters

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::parameter))]
pub struct Parameter<'ast> {
    pub variable: Identifier<'ast>,
    pub visibility: Option<Visibility>,
    pub _type: Type<'ast>,
    #[pest_ast(outer())]
    pub span: Span<'ast>,
}

// Sections

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::section))]
pub struct Section<'ast> {
    pub header: Header<'ast>,
    pub assignments: Vec<Assignment<'ast>>,
    #[pest_ast(outer())]
    pub span: Span<'ast>,
}

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::header))]
pub struct Header<'ast> {
    pub name: Identifier<'ast>,
    #[pest_ast(outer())]
    pub span: Span<'ast>,
}

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::assignment))]
pub struct Assignment<'ast> {
    pub parameter: Parameter<'ast>,
    pub expression: Expression<'ast>,
    pub line_end: LineEnd,
    #[pest_ast(outer())]
    pub span: Span<'ast>,
}

// Utilities

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::EOI))]
pub struct EOI;

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::LINE_END))]
pub struct LineEnd;

// File

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::file))]
pub struct File<'ast> {
    pub sections: Vec<Section<'ast>>,
    pub eoi: EOI,
    #[pest_ast(outer())]
    pub span: Span<'ast>,
}
