//! Abstract syntax tree (ast) representation from language.pest.
//!
//! @file zokrates_program.rs
//! @author Howard Wu <howard@aleo.org>
//! @date 2020

use from_pest::{ConversionError, FromPest, Void};
use pest::{
    error::Error,
    iterators::{Pair, Pairs},
    prec_climber::{Assoc, Operator, PrecClimber},
    Parser, Span,
};
use pest_ast::FromPest;
use std::fmt;
use std::fmt::Formatter;

#[derive(Parser)]
#[grammar = "language.pest"]
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
                Rule::expression_inline_struct => {
                    println!("struct inline");
                    Expression::StructInline(
                        StructInlineExpression::from_pest(&mut pair.into_inner()).unwrap(),
                    )
                },
                Rule::expression_array_inline => {
                    println!("array inline");
                    Expression::ArrayInline(
                        ArrayInlineExpression::from_pest(&mut pair.into_inner()).unwrap()
                    )
                },
                Rule::expression_array_initializer => {
                    println!("array initializer");
                    Expression::ArrayInitializer(
                        ArrayInitializerExpression::from_pest(&mut pair.into_inner()).unwrap()
                    )
                },
                Rule::expression_conditional => {
                    println!("conditional expression");
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
                Rule::expression_increment => {
                    println!("expression increment");
                    let span = next.as_span();
                    let mut inner = next.into_inner();
                    let expression = parse_term(inner.next().unwrap());
                    let operation = match inner.next().unwrap().as_rule() {
                        Rule::operation_post_increment => Increment::from_pest(&mut pair.into_inner().next().unwrap().into_inner()).unwrap(),
                        rule => unreachable!("`expression_increment` should yield `operation_post_increment`, found {:#?}", rule)
                    };
                    Expression::Increment(IncrementExpression { operation, expression, span })
                },
                Rule::expression_decrement => {
                    println!("expression decrement");
                    let span = next.as_span();
                    let mut inner = next.into_inner();
                    let expression = parse_term(inner.next().unwrap());
                    let operation = match inner.next().unwrap().as_rule() {
                        Rule::operation_post_decrement => Decrement::from_pest(&mut pair.into_inner().next().unwrap().into_inner()).unwrap(),
                        rule => unreachable!("`expression_decrement` should yield `operation_post_decrement`, found {:#?}", rule)
                    };
                    Expression::Decrement(DecrementExpression { operation, expression, span })
                },
                Rule::expression_postfix => {
                    println!("postfix expression");
                    Expression::Postfix(
                        PostfixExpression::from_pest(&mut pair.into_inner()).unwrap(),
                    )
                }
                Rule::expression_primitive => {
                    let next = next.into_inner().next().unwrap();
                    match next.as_rule() {
                        Rule::value => Expression::Value(
                            Value::from_pest(&mut pair.into_inner().next().unwrap().into_inner()).unwrap()
                        ),
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

// Types

#[derive(Debug, FromPest, PartialEq, Clone)]
#[pest_ast(rule(Rule::ty_bool))]
pub struct BooleanType<'ast> {
    #[pest_ast(outer())]
    pub span: Span<'ast>,
}

#[derive(Debug, FromPest, PartialEq, Clone)]
#[pest_ast(rule(Rule::ty_field))]
pub struct FieldType<'ast> {
    #[pest_ast(outer())]
    pub span: Span<'ast>,
}

#[derive(Debug, FromPest, PartialEq, Clone)]
#[pest_ast(rule(Rule::ty_struct))]
pub struct StructType<'ast> {
    pub variable: Variable<'ast>,
    #[pest_ast(outer())]
    pub span: Span<'ast>,
}

#[derive(Debug, FromPest, PartialEq, Clone)]
#[pest_ast(rule(Rule::ty_basic))]
pub enum BasicType<'ast> {
    Field(FieldType<'ast>),
    Boolean(BooleanType<'ast>),
}

#[derive(Debug, FromPest, PartialEq, Clone)]
#[pest_ast(rule(Rule::ty_basic_or_struct))]
pub enum BasicOrStructType<'ast> {
    Struct(StructType<'ast>),
    Basic(BasicType<'ast>),
}

#[derive(Debug, FromPest, PartialEq, Clone)]
#[pest_ast(rule(Rule::ty_array))]
pub struct ArrayType<'ast> {
    pub ty: BasicType<'ast>,
    pub dimensions: Vec<Expression<'ast>>,
    #[pest_ast(outer())]
    pub span: Span<'ast>,
}

#[derive(Debug, FromPest, PartialEq, Clone)]
#[pest_ast(rule(Rule::ty))]
pub enum Type<'ast> {
    Basic(BasicType<'ast>),
    Array(ArrayType<'ast>),
    Struct(StructType<'ast>),
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

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::operation_post_increment))]
pub struct Increment<'ast> {
    #[pest_ast(outer())]
    pub span: Span<'ast>,
}

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::operation_post_decrement))]
pub struct Decrement<'ast> {
    #[pest_ast(outer())]
    pub span: Span<'ast>,
}

// Binary Operations

#[derive(Debug, PartialEq, Clone)]
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

// Values

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
#[pest_ast(rule(Rule::value_field))]
pub struct Field<'ast> {
    #[pest_ast(outer(with(span_into_string)))]
    pub value: String,
    #[pest_ast(outer())]
    pub span: Span<'ast>,
}

impl<'ast> fmt::Display for Field<'ast> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::value))]
pub enum Value<'ast> {
    Boolean(Boolean<'ast>),
    Field(Field<'ast>),
}

impl<'ast> Value<'ast> {
    pub fn span(&self) -> &Span<'ast> {
        match self {
            Value::Boolean(value) => &value.span,
            Value::Field(value) => &value.span,
        }
    }
}

impl<'ast> fmt::Display for Value<'ast> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Value::Boolean(ref value) => write!(f, "{}", value),
            Value::Field(ref value) => write!(f, "{}", value),
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

// Access

#[derive(Debug, FromPest, PartialEq, Clone)]
#[pest_ast(rule(Rule::from_expression))]
pub struct FromExpression<'ast>(pub Expression<'ast>);

#[derive(Debug, FromPest, PartialEq, Clone)]
#[pest_ast(rule(Rule::to_expression))]
pub struct ToExpression<'ast>(pub Expression<'ast>);

#[derive(Debug, FromPest, PartialEq, Clone)]
#[pest_ast(rule(Rule::range))]
pub struct Range<'ast> {
    pub from: Option<FromExpression<'ast>>,
    pub to: Option<ToExpression<'ast>>,
    #[pest_ast(outer())]
    pub span: Span<'ast>,
}

#[derive(Debug, FromPest, PartialEq, Clone)]
#[pest_ast(rule(Rule::range_or_expression))]
pub enum RangeOrExpression<'ast> {
    Range(Range<'ast>),
    Expression(Expression<'ast>),
}

// #[derive(Debug, FromPest, PartialEq, Clone)]
// #[pest_ast(rule(Rule::call_access))]
// pub struct CallAccess<'ast> {
//     pub expressions: Vec<Expression<'ast>>,
//     #[pest_ast(outer())]
//     pub span: Span<'ast>,
// }

#[derive(Debug, FromPest, PartialEq, Clone)]
#[pest_ast(rule(Rule::access_array))]
pub struct ArrayAccess<'ast> {
    pub expression: RangeOrExpression<'ast>,
    #[pest_ast(outer())]
    pub span: Span<'ast>,
}

// #[derive(Debug, FromPest, PartialEq, Clone)]
// #[pest_ast(rule(Rule::member_access))]
// pub struct MemberAccess<'ast> {
//     pub id: IdentifierExpression<'ast>,
//     #[pest_ast(outer())]
//     pub span: Span<'ast>,
// }

#[derive(Debug, FromPest, PartialEq, Clone)]
#[pest_ast(rule(Rule::access))]
pub enum Access<'ast> {
    // Call(CallAccess<'ast>),
    Select(ArrayAccess<'ast>),
    // Member(MemberAccess<'ast>),
}

#[derive(Debug, FromPest, PartialEq, Clone)]
#[pest_ast(rule(Rule::expression_postfix))]
pub struct PostfixExpression<'ast> {
    pub variable: Variable<'ast>,
    pub accesses: Vec<Access<'ast>>,
    #[pest_ast(outer())]
    pub span: Span<'ast>,
}

#[derive(Debug, FromPest, PartialEq, Clone)]
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

#[derive(Debug, FromPest, PartialEq, Clone)]
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

#[derive(Debug, FromPest, PartialEq, Clone)]
#[pest_ast(rule(Rule::expression_array_inline))]
pub struct ArrayInlineExpression<'ast> {
    pub expressions: Vec<SpreadOrExpression<'ast>>,
    #[pest_ast(outer())]
    pub span: Span<'ast>,
}

#[derive(Debug, FromPest, PartialEq, Clone)]
#[pest_ast(rule(Rule::expression_array_initializer))]
pub struct ArrayInitializerExpression<'ast> {
    pub expression: Box<Expression<'ast>>,
    pub value: Value<'ast>,
    #[pest_ast(outer())]
    pub span: Span<'ast>,
}

// Structs

#[derive(Debug, FromPest, PartialEq, Clone)]
#[pest_ast(rule(Rule::struct_field))]
pub struct StructField<'ast> {
    pub ty: Type<'ast>,
    pub variable: Variable<'ast>,
    #[pest_ast(outer())]
    pub span: Span<'ast>,
}

#[derive(Debug, FromPest, PartialEq, Clone)]
#[pest_ast(rule(Rule::struct_definition))]
pub struct Struct<'ast> {
    pub variable: Variable<'ast>,
    pub fields: Vec<StructField<'ast>>,
    #[pest_ast(outer())]
    pub span: Span<'ast>,
}

#[derive(Debug, FromPest, PartialEq, Clone)]
#[pest_ast(rule(Rule::inline_struct_member))]
pub struct InlineStructMember<'ast> {
    pub variable: Variable<'ast>,
    pub expression: Expression<'ast>,
    #[pest_ast(outer())]
    pub span: Span<'ast>,
}

#[derive(Debug, FromPest, PartialEq, Clone)]
#[pest_ast(rule(Rule::expression_inline_struct))]
pub struct StructInlineExpression<'ast> {
    pub variable: Variable<'ast>,
    pub members: Vec<InlineStructMember<'ast>>,
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

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::expression_increment))]
pub struct IncrementExpression<'ast> {
    pub expression: Box<Expression<'ast>>,
    pub operation: Increment<'ast>,
    #[pest_ast(outer())]
    pub span: Span<'ast>,
}

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::expression_decrement))]
pub struct DecrementExpression<'ast> {
    pub expression: Box<Expression<'ast>>,
    pub operation: Decrement<'ast>,
    #[pest_ast(outer())]
    pub span: Span<'ast>,
}

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
    Increment(IncrementExpression<'ast>),
    Decrement(DecrementExpression<'ast>),
    Binary(BinaryExpression<'ast>),
    Ternary(TernaryExpression<'ast>),
    ArrayInline(ArrayInlineExpression<'ast>),
    ArrayInitializer(ArrayInitializerExpression<'ast>),
    StructInline(StructInlineExpression<'ast>),
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
            Expression::Increment(expression) => &expression.span,
            Expression::Decrement(expression) => &expression.span,
            Expression::Binary(expression) => &expression.span,
            Expression::Ternary(expression) => &expression.span,
            Expression::ArrayInline(expression) => &expression.span,
            Expression::ArrayInitializer(expression) => &expression.span,
            Expression::StructInline(expression) => &expression.span,
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
            Expression::Increment(ref expression) => write!(f, "{}++", expression.expression),
            Expression::Decrement(ref expression) => write!(f, "{}--", expression.expression),
            Expression::Binary(ref expression) => {
                write!(f, "{} == {}", expression.left, expression.right)
            }
            Expression::Ternary(ref expression) => write!(
                f,
                "if {} then {} else {} fi",
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
                write!(f, "{} = {}", expression.value, expression.expression)
            }
            Expression::StructInline(ref expression) => {
                write!(f, "inline struct display not impl {}", expression.variable)
            }
            Expression::Postfix(ref expression) => {
                write!(f, "Postfix display not impl {}", expression.variable)
            }
        }
    }
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

#[derive(Debug, FromPest, PartialEq, Clone)]
#[pest_ast(rule(Rule::statement_assign))]
pub struct AssignStatement<'ast> {
    pub variable: Variable<'ast>,
    pub expression: Expression<'ast>,
    #[pest_ast(outer())]
    pub span: Span<'ast>,
}

impl<'ast> fmt::Display for AssignStatement<'ast> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.expression)
    }
}

#[derive(Debug, FromPest, PartialEq, Clone)]
#[pest_ast(rule(Rule::statement_return))]
pub struct ReturnStatement<'ast> {
    pub expressions: Vec<Expression<'ast>>,
    #[pest_ast(outer())]
    pub span: Span<'ast>,
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

#[derive(Debug, FromPest, PartialEq, Clone)]
#[pest_ast(rule(Rule::statement_definition))]
pub struct DefinitionStatement<'ast> {
    pub ty: Type<'ast>,
    pub variable: Variable<'ast>,
    pub expression: Expression<'ast>,
    #[pest_ast(outer())]
    pub span: Span<'ast>,
}

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::statement))]
pub enum Statement<'ast> {
    Assign(AssignStatement<'ast>),
    Definition(DefinitionStatement<'ast>),
    Return(ReturnStatement<'ast>),
}

impl<'ast> fmt::Display for DefinitionStatement<'ast> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.expression)
    }
}

impl<'ast> fmt::Display for Statement<'ast> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Statement::Assign(ref statement) => write!(f, "{}", statement),
            Statement::Definition(ref statement) => write!(f, "{}", statement),
            Statement::Return(ref statement) => write!(f, "{}", statement),
        }
    }
}

// Functions

#[derive(Debug, FromPest, PartialEq, Clone)]
#[pest_ast(rule(Rule::parameter))]
pub struct Parameter<'ast> {
    pub visibility: Option<Visibility>,
    pub ty: Type<'ast>,
    pub variable: Variable<'ast>,
    #[pest_ast(outer())]
    pub span: Span<'ast>,
}

#[derive(Debug, FromPest, PartialEq, Clone)]
#[pest_ast(rule(Rule::function_definition))]
pub struct Function<'ast> {
    pub variable: Variable<'ast>,
    pub parameters: Vec<Parameter<'ast>>,
    pub returns: Vec<Type<'ast>>,
    pub statements: Vec<Statement<'ast>>,
    #[pest_ast(outer())]
    pub span: Span<'ast>,
}

// File

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::file))]
pub struct File<'ast> {
    pub structs: Vec<Struct<'ast>>,
    pub functions: Vec<Function<'ast>>,
    pub statements: Vec<Statement<'ast>>,
    pub eoi: EOI,
    #[pest_ast(outer())]
    pub span: Span<'ast>,
}

// Utilities

#[derive(Debug, FromPest, PartialEq, Clone)]
#[pest_ast(rule(Rule::EOI))]
pub struct EOI;
