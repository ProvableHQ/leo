//! Abstract syntax tree (ast) representation from leo.pest.

use from_pest::{ConversionError, FromPest, Void};
use pest::{
    error::Error,
    iterators::{Pair, Pairs},
    prec_climber::{Assoc, Operator, PrecClimber},
    Parser, Span,
};
use pest_ast::FromPest;
use std::fmt;

#[derive(Parser)]
#[grammar = "leo.pest"]
pub struct LanguageParser;

pub fn parse(input: &str) -> Result<Pairs<Rule>, Error<Rule>> {
    LanguageParser::parse(Rule::file, input)
}

fn span_into_string(span: Span) -> String {
    span.as_str().to_string()
}

lazy_static! {
    static ref PRECEDENCE_CLIMBER: PrecClimber<Rule> = precedence_climber();
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

// Unary Operations

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::operation_pre_not))]
pub struct Not<'ast> {
    #[pest_ast(outer())]
    pub span: Span<'ast>,
}

// #[derive(Clone, Debug, FromPest, PartialEq)]
// #[pest_ast(rule(Rule::operation_post_increment))]
// pub struct Increment<'ast> {
//     #[pest_ast(outer())]
//     pub span: Span<'ast>,
// }
//
// #[derive(Clone, Debug, FromPest, PartialEq)]
// #[pest_ast(rule(Rule::operation_post_decrement))]
// pub struct Decrement<'ast> {
//     #[pest_ast(outer())]
//     pub span: Span<'ast>,
// }

// Binary Operations

#[derive(Clone, Debug, PartialEq)]
pub enum BinaryOperator {
    Or,
    And,
    Eq,
    Neq,
    Geq,
    Gt,
    Leq,
    Lt,
    Add,
    Sub,
    Mul,
    Div,
    Pow,
}

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::assign))]
pub struct Assign {}

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::operation_add_assign))]
pub struct AddAssign {}

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::operation_sub_assign))]
pub struct SubAssign {}

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::operation_mul_assign))]
pub struct MulAssign {}

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::operation_div_assign))]
pub struct DivAssign {}

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::operation_pow_assign))]
pub struct PowAssign {}

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::operation_assign))]
pub enum OperationAssign {
    Assign(Assign),
    AddAssign(AddAssign),
    SubAssign(SubAssign),
    MulAssign(MulAssign),
    DivAssign(DivAssign),
    PowAssign(PowAssign),
}

// Types

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
#[pest_ast(rule(Rule::type_integer))]
pub enum IntegerType {
    U8Type(U8Type),
    U16Type(U16Type),
    U32Type(U32Type),
    U64Type(U64Type),
    U128Type(U128Type),
}

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::type_field))]
pub struct FieldType<'ast> {
    #[pest_ast(outer())]
    pub span: Span<'ast>,
}

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::type_group))]
pub struct GroupType<'ast> {
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
#[pest_ast(rule(Rule::type_circuit))]
pub struct CircuitType<'ast> {
    pub variable: Variable<'ast>,
    #[pest_ast(outer())]
    pub span: Span<'ast>,
}

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::type_basic))]
pub enum BasicType<'ast> {
    Integer(IntegerType),
    Field(FieldType<'ast>),
    Group(GroupType<'ast>),
    Boolean(BooleanType<'ast>),
}

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::type_array))]
pub struct ArrayType<'ast> {
    pub _type: BasicType<'ast>,
    pub dimensions: Vec<Value<'ast>>,
    #[pest_ast(outer())]
    pub span: Span<'ast>,
}

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::_type))]
pub enum Type<'ast> {
    Basic(BasicType<'ast>),
    Array(ArrayType<'ast>),
    Circuit(CircuitType<'ast>),
}

impl<'ast> fmt::Display for Type<'ast> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Type::Basic(ref _type) => write!(f, "basic"),
            Type::Array(ref _type) => write!(f, "array"),
            Type::Circuit(ref _type) => write!(f, "struct"),
        }
    }
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

impl<'ast> fmt::Display for Number<'ast> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::value_integer))]
pub struct Integer<'ast> {
    pub number: Number<'ast>,
    pub _type: Option<IntegerType>,
    #[pest_ast(outer())]
    pub span: Span<'ast>,
}

impl<'ast> fmt::Display for Integer<'ast> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.number)
    }
}

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::value_field))]
pub struct Field<'ast> {
    pub number: Number<'ast>,
    pub _type: FieldType<'ast>,
    #[pest_ast(outer())]
    pub span: Span<'ast>,
}

impl<'ast> fmt::Display for Field<'ast> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.number)
    }
}

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::value_group))]
pub struct Group<'ast> {
    pub number: Number<'ast>,
    pub _type: GroupType<'ast>,
    #[pest_ast(outer())]
    pub span: Span<'ast>,
}

impl<'ast> fmt::Display for Group<'ast> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.number)
    }
}

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::value_boolean))]
pub struct Boolean<'ast> {
    #[pest_ast(outer(with(span_into_string)))]
    pub value: String,
    #[pest_ast(outer())]
    pub span: Span<'ast>,
}

impl<'ast> fmt::Display for Boolean<'ast> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::value))]
pub enum Value<'ast> {
    Integer(Integer<'ast>),
    Field(Field<'ast>),
    Group(Group<'ast>),
    Boolean(Boolean<'ast>),
}

impl<'ast> Value<'ast> {
    pub fn span(&self) -> &Span<'ast> {
        match self {
            Value::Integer(value) => &value.span,
            Value::Field(value) => &value.span,
            Value::Group(value) => &value.span,
            Value::Boolean(value) => &value.span,
        }
    }
}

impl<'ast> fmt::Display for Value<'ast> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Value::Integer(ref value) => write!(f, "{}", value),
            Value::Field(ref value) => write!(f, "{}", value),
            Value::Group(ref value) => write!(f, "{}", value),
            Value::Boolean(ref value) => write!(f, "{}", value),
        }
    }
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

impl<'ast> fmt::Display for Variable<'ast> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[derive(Debug, FromPest, PartialEq, Clone)]
#[pest_ast(rule(Rule::optionally_typed_variable))]
pub struct OptionallyTypedVariable<'ast> {
    pub _type: Option<Type<'ast>>,
    pub variable: VariableWithMutability<'ast>,
    #[pest_ast(outer())]
    pub span: Span<'ast>,
}

impl<'ast> fmt::Display for OptionallyTypedVariable<'ast> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.variable)
    }
}

// Mutability

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::mutable))]
pub struct Mutable {}

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::variable_with_mutability))]
pub struct VariableWithMutability<'ast> {
    pub mutable: Option<Mutable>,
    pub variable: Variable<'ast>,
    #[pest_ast(outer())]
    pub span: Span<'ast>,
}

impl<'ast> fmt::Display for VariableWithMutability<'ast> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.variable)
    }
}

// Access

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::from_expression))]
pub struct FromExpression<'ast>(pub Expression<'ast>);

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::to_expression))]
pub struct ToExpression<'ast>(pub Expression<'ast>);

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::range))]
pub struct Range<'ast> {
    pub from: Option<FromExpression<'ast>>,
    pub to: Option<ToExpression<'ast>>,
    #[pest_ast(outer())]
    pub span: Span<'ast>,
}

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::range_or_expression))]
pub enum RangeOrExpression<'ast> {
    Range(Range<'ast>),
    Expression(Expression<'ast>),
}

impl<'ast> fmt::Display for RangeOrExpression<'ast> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RangeOrExpression::Expression(ref expression) => write!(f, "{}", expression),
            RangeOrExpression::Range(ref range) => write!(
                f,
                "{}..{}",
                range
                    .from
                    .as_ref()
                    .map(|e| e.0.to_string())
                    .unwrap_or("".to_string()),
                range
                    .to
                    .as_ref()
                    .map(|e| e.0.to_string())
                    .unwrap_or("".to_string())
            ),
        }
    }
}

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::access_call))]
pub struct CallAccess<'ast> {
    pub expressions: Vec<Expression<'ast>>,
    #[pest_ast(outer())]
    pub span: Span<'ast>,
}

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::access_array))]
pub struct ArrayAccess<'ast> {
    pub expression: RangeOrExpression<'ast>,
    #[pest_ast(outer())]
    pub span: Span<'ast>,
}

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::access_member))]
pub struct MemberAccess<'ast> {
    pub variable: Variable<'ast>,
    #[pest_ast(outer())]
    pub span: Span<'ast>,
}

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::access))]
pub enum Access<'ast> {
    Array(ArrayAccess<'ast>),
    Call(CallAccess<'ast>),
    Member(MemberAccess<'ast>),
}

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::expression_postfix))]
pub struct PostfixExpression<'ast> {
    pub variable: Variable<'ast>,
    pub accesses: Vec<Access<'ast>>,
    #[pest_ast(outer())]
    pub span: Span<'ast>,
}

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::assignee_access))]
pub enum AssigneeAccess<'ast> {
    Array(ArrayAccess<'ast>),
    Member(MemberAccess<'ast>),
}

impl<'ast> fmt::Display for AssigneeAccess<'ast> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AssigneeAccess::Array(ref array) => write!(f, "[{}]", array.expression),
            AssigneeAccess::Member(ref member) => write!(f, ".{}", member.variable),
        }
    }
}

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::assignee))]
pub struct Assignee<'ast> {
    pub variable: Variable<'ast>,
    pub accesses: Vec<AssigneeAccess<'ast>>,
    #[pest_ast(outer())]
    pub span: Span<'ast>,
}

impl<'ast> fmt::Display for Assignee<'ast> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.variable)?;
        for (i, access) in self.accesses.iter().enumerate() {
            write!(f, "{}", access)?;
            if i < self.accesses.len() - 1 {
                write!(f, ", ")?;
            }
        }
        write!(f, "")
    }
}

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::spread))]
pub struct Spread<'ast> {
    pub expression: Expression<'ast>,
    #[pest_ast(outer())]
    pub span: Span<'ast>,
}

impl<'ast> fmt::Display for Spread<'ast> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "...{}", self.expression)
    }
}

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::spread_or_expression))]
pub enum SpreadOrExpression<'ast> {
    Spread(Spread<'ast>),
    Expression(Expression<'ast>),
}

impl<'ast> fmt::Display for SpreadOrExpression<'ast> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SpreadOrExpression::Spread(ref spread) => write!(f, "{}", spread),
            SpreadOrExpression::Expression(ref expression) => write!(f, "{}", expression),
        }
    }
}

// Arrays

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::expression_array_inline))]
pub struct ArrayInlineExpression<'ast> {
    pub expressions: Vec<SpreadOrExpression<'ast>>,
    #[pest_ast(outer())]
    pub span: Span<'ast>,
}

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::expression_array_initializer))]
pub struct ArrayInitializerExpression<'ast> {
    pub expression: Box<SpreadOrExpression<'ast>>,
    pub count: Value<'ast>,
    #[pest_ast(outer())]
    pub span: Span<'ast>,
}

// Circuits

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::circuit_object))]
pub struct CircuitObject<'ast> {
    pub variable: Variable<'ast>,
    pub _type: Type<'ast>,
    #[pest_ast(outer())]
    pub span: Span<'ast>,
}

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::circuit_definition))]
pub struct Circuit<'ast> {
    pub variable: Variable<'ast>,
    pub fields: Vec<CircuitObject<'ast>>,
    #[pest_ast(outer())]
    pub span: Span<'ast>,
}

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::inline_circuit_member))]
pub struct InlineCircuitMember<'ast> {
    pub variable: Variable<'ast>,
    pub expression: Expression<'ast>,
    #[pest_ast(outer())]
    pub span: Span<'ast>,
}

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::expression_inline_circuit))]
pub struct CircuitInlineExpression<'ast> {
    pub variable: Variable<'ast>,
    pub members: Vec<InlineCircuitMember<'ast>>,
    #[pest_ast(outer())]
    pub span: Span<'ast>,
}

// Expressions

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::expression_not))]
pub struct NotExpression<'ast> {
    pub operation: Not<'ast>,
    pub expression: Box<Expression<'ast>>,
    #[pest_ast(outer())]
    pub span: Span<'ast>,
}

// #[derive(Clone, Debug, FromPest, PartialEq)]
// #[pest_ast(rule(Rule::expression_increment))]
// pub struct IncrementExpression<'ast> {
//     pub expression: Box<Expression<'ast>>,
//     #[pest_ast(outer())]
//     pub span: Span<'ast>,
// }
//
// #[derive(Clone, Debug, FromPest, PartialEq)]
// #[pest_ast(rule(Rule::expression_decrement))]
// pub struct DecrementExpression<'ast> {
//     pub expression: Box<Expression<'ast>>,
//     #[pest_ast(outer())]
//     pub span: Span<'ast>,
// }

#[derive(Clone, Debug, PartialEq)]
pub struct BinaryExpression<'ast> {
    pub operation: BinaryOperator,
    pub left: Box<Expression<'ast>>,
    pub right: Box<Expression<'ast>>,
    pub span: Span<'ast>,
}

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::expression_conditional))]
pub struct TernaryExpression<'ast> {
    pub first: Box<Expression<'ast>>,
    pub second: Box<Expression<'ast>>,
    pub third: Box<Expression<'ast>>,
    #[pest_ast(outer())]
    pub span: Span<'ast>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Expression<'ast> {
    Value(Value<'ast>),
    Variable(Variable<'ast>),
    Not(NotExpression<'ast>),
    // Increment(IncrementExpression<'ast>),
    // Decrement(DecrementExpression<'ast>),
    Binary(BinaryExpression<'ast>),
    Ternary(TernaryExpression<'ast>),
    ArrayInline(ArrayInlineExpression<'ast>),
    ArrayInitializer(ArrayInitializerExpression<'ast>),
    CircuitInline(CircuitInlineExpression<'ast>),
    Postfix(PostfixExpression<'ast>),
}

impl<'ast> Expression<'ast> {
    pub fn binary(
        operation: BinaryOperator,
        left: Box<Expression<'ast>>,
        right: Box<Expression<'ast>>,
        span: Span<'ast>,
    ) -> Self {
        Expression::Binary(BinaryExpression {
            operation,
            left,
            right,
            span,
        })
    }

    pub fn ternary(
        first: Box<Expression<'ast>>,
        second: Box<Expression<'ast>>,
        third: Box<Expression<'ast>>,
        span: Span<'ast>,
    ) -> Self {
        Expression::Ternary(TernaryExpression {
            first,
            second,
            third,
            span,
        })
    }

    pub fn span(&self) -> &Span<'ast> {
        match self {
            Expression::Value(expression) => &expression.span(),
            Expression::Variable(expression) => &expression.span,
            Expression::Not(expression) => &expression.span,
            // Expression::Increment(expression) => &expression.span,
            // Expression::Decrement(expression) => &expression.span,
            Expression::Binary(expression) => &expression.span,
            Expression::Ternary(expression) => &expression.span,
            Expression::ArrayInline(expression) => &expression.span,
            Expression::ArrayInitializer(expression) => &expression.span,
            Expression::CircuitInline(expression) => &expression.span,
            Expression::Postfix(expression) => &expression.span,
        }
    }
}

impl<'ast> fmt::Display for Expression<'ast> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Expression::Value(ref expression) => write!(f, "{}", expression),
            Expression::Variable(ref expression) => write!(f, "{}", expression),
            Expression::Not(ref expression) => write!(f, "!{}", expression.expression),
            // Expression::Increment(ref expression) => write!(f, "{}++", expression.expression),
            // Expression::Decrement(ref expression) => write!(f, "{}--", expression.expression),
            Expression::Binary(ref expression) => {
                write!(f, "{} == {}", expression.left, expression.right)
            }
            Expression::Ternary(ref expression) => write!(
                f,
                "if {} ? {} : {}",
                expression.first, expression.second, expression.third
            ),
            Expression::ArrayInline(ref expression) => {
                for (i, spread_or_expression) in expression.expressions.iter().enumerate() {
                    write!(f, "{}", spread_or_expression)?;
                    if i < expression.expressions.len() - 1 {
                        write!(f, ", ")?;
                    }
                }
                write!(f, "")
            }
            Expression::ArrayInitializer(ref expression) => {
                write!(f, "[{} ; {}]", expression.expression, expression.count)
            }
            Expression::CircuitInline(ref expression) => {
                write!(f, "inline circuit display not impl {}", expression.variable)
            }
            Expression::Postfix(ref expression) => {
                write!(f, "Postfix display not impl {}", expression.variable)
            }
        }
    }
}

fn precedence_climber() -> PrecClimber<Rule> {
    PrecClimber::new(vec![
        Operator::new(Rule::operation_or, Assoc::Left),
        Operator::new(Rule::operation_and, Assoc::Left),
        Operator::new(Rule::operation_eq, Assoc::Left)
            | Operator::new(Rule::operation_neq, Assoc::Left),
        Operator::new(Rule::operation_geq, Assoc::Left)
            | Operator::new(Rule::operation_gt, Assoc::Left)
            | Operator::new(Rule::operation_leq, Assoc::Left)
            | Operator::new(Rule::operation_lt, Assoc::Left),
        Operator::new(Rule::operation_add, Assoc::Left)
            | Operator::new(Rule::operation_sub, Assoc::Left),
        Operator::new(Rule::operation_mul, Assoc::Left)
            | Operator::new(Rule::operation_div, Assoc::Left),
        Operator::new(Rule::operation_pow, Assoc::Left),
    ])
}

fn parse_term(pair: Pair<Rule>) -> Box<Expression> {
    Box::new(match pair.as_rule() {
        Rule::expression_term => {
            let clone = pair.clone();
            let next = clone.into_inner().next().unwrap();
            match next.as_rule() {
                Rule::expression => Expression::from_pest(&mut pair.into_inner()).unwrap(), // Parenthesis case
                Rule::expression_inline_circuit => {
                    Expression::CircuitInline(
                        CircuitInlineExpression::from_pest(&mut pair.into_inner()).unwrap(),
                    )
                },
                Rule::expression_array_inline => {
                    Expression::ArrayInline(
                        ArrayInlineExpression::from_pest(&mut pair.into_inner()).unwrap()
                    )
                },
                Rule::expression_array_initializer => {
                    Expression::ArrayInitializer(
                        ArrayInitializerExpression::from_pest(&mut pair.into_inner()).unwrap()
                    )
                },
                Rule::expression_conditional => {
                    Expression::Ternary(
                        TernaryExpression::from_pest(&mut pair.into_inner()).unwrap(),
                    )
                },
                Rule::expression_not => {
                    let span = next.as_span();
                    let mut inner = next.into_inner();
                    let operation = match inner.next().unwrap().as_rule() {
                        Rule::operation_pre_not => Not::from_pest(&mut pair.into_inner().next().unwrap().into_inner()).unwrap(),
                        rule => unreachable!("`expression_not` should yield `operation_pre_not`, found {:#?}", rule)
                    };
                    let expression = parse_term(inner.next().unwrap());
                    Expression::Not(NotExpression { operation, expression, span })
                },
                // Rule::expression_increment => {
                //     println!("expression increment");
                //     let span = next.as_span();
                //     let mut inner = next.into_inner();
                //     let expression = parse_term(inner.next().unwrap());
                //     // let operation = match inner.next().unwrap().as_rule() {
                //     //     Rule::operation_post_increment => Increment::from_pest(&mut pair.into_inner().next().unwrap().into_inner()).unwrap(),
                //     //     rule => unreachable!("`expression_increment` should yield `operation_post_increment`, found {:#?}", rule)
                //     // };
                //     Expression::Increment(IncrementExpression { expression, span })
                // },
                // Rule::expression_decrement => {
                //     println!("expression decrement");
                //     let span = next.as_span();
                //     let mut inner = next.into_inner();
                //     let expression = parse_term(inner.next().unwrap());
                //     // let operation = match inner.next().unwrap().as_rule() {
                //     //     Rule::operation_post_decrement => Decrement::from_pest(&mut pair.into_inner().next().unwrap().into_inner()).unwrap(),
                //     //     rule => unreachable!("`expression_decrement` should yield `operation_post_decrement`, found {:#?}", rule)
                //     // };
                //     Expression::Decrement(DecrementExpression { expression, span })
                // },
                Rule::expression_postfix => {
                    Expression::Postfix(
                        PostfixExpression::from_pest(&mut pair.into_inner()).unwrap(),
                    )
                }
                Rule::expression_primitive => {
                    let next = next.into_inner().next().unwrap();
                    match next.as_rule() {
                        Rule::value => {
                            Expression::Value(
                                Value::from_pest(&mut pair.into_inner().next().unwrap().into_inner()).unwrap()
                            )
                        },
                        Rule::variable => Expression::Variable(
                            Variable::from_pest(&mut pair.into_inner().next().unwrap().into_inner()).unwrap(),
                        ),
                        rule => unreachable!("`expression_primitive` should contain one of [`value`, `variable`], found {:#?}", rule)
                    }
                },

                rule => unreachable!("`term` should contain one of ['value', 'variable', 'expression', 'expression_not', 'expression_increment', 'expression_decrement'], found {:#?}", rule)
            }
        }
        rule => unreachable!(
            "`parse_expression_term` should be invoked on `Rule::expression_term`, found {:#?}",
            rule
        ),
    })
}

fn binary_expression<'ast>(
    lhs: Box<Expression<'ast>>,
    pair: Pair<'ast, Rule>,
    rhs: Box<Expression<'ast>>,
) -> Box<Expression<'ast>> {
    let (start, _) = lhs.span().clone().split();
    let (_, end) = rhs.span().clone().split();
    let span = start.span(&end);

    Box::new(match pair.as_rule() {
        Rule::operation_or => Expression::binary(BinaryOperator::Or, lhs, rhs, span),
        Rule::operation_and => Expression::binary(BinaryOperator::And, lhs, rhs, span),
        Rule::operation_eq => Expression::binary(BinaryOperator::Eq, lhs, rhs, span),
        Rule::operation_neq => Expression::binary(BinaryOperator::Neq, lhs, rhs, span),
        Rule::operation_geq => Expression::binary(BinaryOperator::Geq, lhs, rhs, span),
        Rule::operation_gt => Expression::binary(BinaryOperator::Gt, lhs, rhs, span),
        Rule::operation_leq => Expression::binary(BinaryOperator::Leq, lhs, rhs, span),
        Rule::operation_lt => Expression::binary(BinaryOperator::Lt, lhs, rhs, span),
        Rule::operation_add => Expression::binary(BinaryOperator::Add, lhs, rhs, span),
        Rule::operation_sub => Expression::binary(BinaryOperator::Sub, lhs, rhs, span),
        Rule::operation_mul => Expression::binary(BinaryOperator::Mul, lhs, rhs, span),
        Rule::operation_div => Expression::binary(BinaryOperator::Div, lhs, rhs, span),
        Rule::operation_pow => Expression::binary(BinaryOperator::Pow, lhs, rhs, span),
        _ => unreachable!(),
    })
}

impl<'ast> FromPest<'ast> for Expression<'ast> {
    type Rule = Rule;
    type FatalError = Void;

    fn from_pest(pest: &mut Pairs<'ast, Rule>) -> Result<Self, ConversionError<Void>> {
        let mut clone = pest.clone();
        let pair = clone.next().ok_or(::from_pest::ConversionError::NoMatch)?;
        match pair.as_rule() {
            Rule::expression => {
                // Transfer iterated state to pest.
                *pest = clone;
                Ok(*PRECEDENCE_CLIMBER.climb(pair.into_inner(), parse_term, binary_expression))
            }
            _ => Err(ConversionError::NoMatch),
        }
    }
}

// Statements

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::statement_return))]
pub struct ReturnStatement<'ast> {
    pub expressions: Vec<Expression<'ast>>,
    #[pest_ast(outer())]
    pub span: Span<'ast>,
}

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::conditional_nested_or_end))]
pub enum ConditionalNestedOrEnd<'ast> {
    Nested(Box<ConditionalStatement<'ast>>),
    End(Vec<Statement<'ast>>),
}

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::statement_conditional))]
pub struct ConditionalStatement<'ast> {
    pub condition: Expression<'ast>,
    pub statements: Vec<Statement<'ast>>,
    pub next: Option<ConditionalNestedOrEnd<'ast>>,
    #[pest_ast(outer())]
    pub span: Span<'ast>,
}

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::statement_for))]
pub struct ForStatement<'ast> {
    pub index: Variable<'ast>,
    pub start: Expression<'ast>,
    pub stop: Expression<'ast>,
    pub statements: Vec<Statement<'ast>>,
    #[pest_ast(outer())]
    pub span: Span<'ast>,
}

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::statement_multiple_assignment))]
pub struct MultipleAssignmentStatement<'ast> {
    pub assignees: Vec<OptionallyTypedVariable<'ast>>,
    pub function_name: Variable<'ast>,
    pub arguments: Vec<Expression<'ast>>,
    #[pest_ast(outer())]
    pub span: Span<'ast>,
}

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::statement_definition))]
pub struct DefinitionStatement<'ast> {
    pub variable: VariableWithMutability<'ast>,
    pub _type: Option<Type<'ast>>,
    pub expression: Expression<'ast>,
    #[pest_ast(outer())]
    pub span: Span<'ast>,
}

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::statement_assign))]
pub struct AssignStatement<'ast> {
    pub assignee: Assignee<'ast>,
    pub assign: OperationAssign,
    pub expression: Expression<'ast>,
    #[pest_ast(outer())]
    pub span: Span<'ast>,
}

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::assert_eq))]
pub struct AssertEq<'ast> {
    pub left: Expression<'ast>,
    pub right: Expression<'ast>,
    #[pest_ast(outer())]
    pub span: Span<'ast>,
}

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::statement_assert))]
pub enum AssertStatement<'ast> {
    AssertEq(AssertEq<'ast>),
}

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::statement_expression))]
pub struct ExpressionStatement<'ast> {
    pub expression: Expression<'ast>,
    #[pest_ast(outer())]
    pub span: Span<'ast>,
}

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::statement))]
pub enum Statement<'ast> {
    Return(ReturnStatement<'ast>),
    Definition(DefinitionStatement<'ast>),
    Assign(AssignStatement<'ast>),
    MultipleAssignment(MultipleAssignmentStatement<'ast>),
    Conditional(ConditionalStatement<'ast>),
    Iteration(ForStatement<'ast>),
    Assert(AssertStatement<'ast>),
    Expression(ExpressionStatement<'ast>),
}

impl<'ast> fmt::Display for ReturnStatement<'ast> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (i, expression) in self.expressions.iter().enumerate() {
            write!(f, "{}", expression)?;
            if i < self.expressions.len() - 1 {
                write!(f, ", ")?;
            }
        }
        write!(f, "")
    }
}

impl<'ast> fmt::Display for ConditionalNestedOrEnd<'ast> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ConditionalNestedOrEnd::Nested(ref nested) => write!(f, "else {}", nested),
            ConditionalNestedOrEnd::End(ref statements) => {
                write!(f, "else {{\n \t{:#?}\n }}", statements)
            }
        }
    }
}

impl<'ast> fmt::Display for ConditionalStatement<'ast> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "if ({}) {{\n", self.condition)?;
        write!(f, "\t{:#?}\n", self.statements)?;
        self.next
            .as_ref()
            .map(|n_or_e| write!(f, "}} {}", n_or_e))
            .unwrap_or(write!(f, "}}"))
    }
}

impl<'ast> fmt::Display for ForStatement<'ast> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "for {} in {}..{} {{ {:#?} }}",
            self.index, self.start, self.stop, self.statements
        )
    }
}

impl<'ast> fmt::Display for MultipleAssignmentStatement<'ast> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (i, id) in self.assignees.iter().enumerate() {
            write!(f, "{}", id)?;
            if i < self.assignees.len() - 1 {
                write!(f, ", ")?;
            }
        }
        write!(f, " = {}", self.function_name)
    }
}

impl<'ast> fmt::Display for DefinitionStatement<'ast> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self._type {
            Some(ref _type) => write!(
                f,
                "let {} : {} = {};",
                self.variable, _type, self.expression
            ),
            None => write!(f, "let {} = {}", self.variable, self.expression),
        }
    }
}

impl<'ast> fmt::Display for AssignStatement<'ast> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} = {};", self.assignee, self.expression)
    }
}

impl<'ast> fmt::Display for AssertStatement<'ast> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AssertStatement::AssertEq(ref assert) => {
                write!(f, "assert_eq({}, {});", assert.left, assert.right)
            }
        }
    }
}

impl<'ast> fmt::Display for Statement<'ast> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Statement::Return(ref statement) => write!(f, "{}", statement),
            Statement::Definition(ref statement) => write!(f, "{}", statement),
            Statement::Assign(ref statement) => write!(f, "{}", statement),
            Statement::MultipleAssignment(ref statement) => write!(f, "{}", statement),
            Statement::Conditional(ref statement) => write!(f, "{}", statement),
            Statement::Iteration(ref statement) => write!(f, "{}", statement),
            Statement::Assert(ref statement) => write!(f, "{}", statement),
            Statement::Expression(ref statement) => write!(f, "{}", statement.expression),
        }
    }
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

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::function_definition))]
pub struct Function<'ast> {
    pub function_name: FunctionName<'ast>,
    pub parameters: Vec<Parameter<'ast>>,
    pub returns: Vec<Type<'ast>>,
    pub statements: Vec<Statement<'ast>>,
    #[pest_ast(outer())]
    pub span: Span<'ast>,
}

// Utilities

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::EOI))]
pub struct EOI;

// Imports

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::import_source))]
pub struct ImportSource<'ast> {
    #[pest_ast(outer(with(span_into_string)))]
    pub value: String,
    #[pest_ast(outer())]
    pub span: Span<'ast>,
}

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::import_symbol))]
pub struct ImportSymbol<'ast> {
    pub value: Variable<'ast>,
    pub alias: Option<Variable<'ast>>,
    #[pest_ast(outer())]
    pub span: Span<'ast>,
}

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::import))]
pub struct Import<'ast> {
    pub source: ImportSource<'ast>,
    pub symbols: Vec<ImportSymbol<'ast>>,
    #[pest_ast(outer())]
    pub span: Span<'ast>,
}

// File

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::file))]
pub struct File<'ast> {
    pub imports: Vec<Import<'ast>>,
    pub structs: Vec<Circuit<'ast>>,
    pub functions: Vec<Function<'ast>>,
    pub eoi: EOI,
    #[pest_ast(outer())]
    pub span: Span<'ast>,
}
