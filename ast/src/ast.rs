//! Abstract syntax tree (ast) representation from leo.pest.
use crate::{
    circuits::Circuit,
    common::{
        Identifier,
        Mutable,
        Visibility
    },
    expressions::{
        ArrayInlineExpression,
        ArrayInitializerExpression,
        CircuitInlineExpression,
        Expression,
        TernaryExpression,
        NotExpression,
        PostfixExpression
    },
    imports::Import,
    operations::{
        BinaryOperation,
        NotOperation,
    },
    statements::Statement,
    types::Type,
    values::Value,
};

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

pub(crate) fn span_into_string(span: Span) -> String {
    span.as_str().to_string()
}

lazy_static! {
    static ref PRECEDENCE_CLIMBER: PrecClimber<Rule> = precedence_climber();
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
    pub identifier: Identifier<'ast>,
    #[pest_ast(outer())]
    pub span: Span<'ast>,
}

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::access_static_member))]
pub struct StaticMemberAccess<'ast> {
    pub identifier: Identifier<'ast>,
    #[pest_ast(outer())]
    pub span: Span<'ast>,
}

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::access))]
pub enum Access<'ast> {
    Array(ArrayAccess<'ast>),
    Call(CallAccess<'ast>),
    Object(MemberAccess<'ast>),
    StaticObject(StaticMemberAccess<'ast>),
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
            AssigneeAccess::Member(ref member) => write!(f, ".{}", member.identifier),
        }
    }
}

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::assignee))]
pub struct Assignee<'ast> {
    pub identifier: Identifier<'ast>,
    pub accesses: Vec<AssigneeAccess<'ast>>,
    #[pest_ast(outer())]
    pub span: Span<'ast>,
}

impl<'ast> fmt::Display for Assignee<'ast> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.identifier)?;
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

// Expressions

fn precedence_climber() -> PrecClimber<Rule> {
    PrecClimber::new(vec![
        Operator::new(Rule::operation_or, Assoc::Left),
        Operator::new(Rule::operation_and, Assoc::Left),
        Operator::new(Rule::operation_eq, Assoc::Left)
            | Operator::new(Rule::operation_ne, Assoc::Left),
        Operator::new(Rule::operation_ge, Assoc::Left)
            | Operator::new(Rule::operation_gt, Assoc::Left)
            | Operator::new(Rule::operation_le, Assoc::Left)
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
                Rule::expression_circuit_inline => {
                    Expression::CircuitInline(
                        CircuitInlineExpression::from_pest(&mut pair.into_inner()).unwrap(),
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
                        Rule::operation_not => NotOperation::from_pest(&mut pair.into_inner().next().unwrap().into_inner()).unwrap(),
                        rule => unreachable!("`expression_not` should yield `operation_pre_not`, found {:#?}", rule)
                    };
                    let expression = parse_term(inner.next().unwrap());
                    Expression::Not(NotExpression { operation, expression, span })
                },
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
                        Rule::identifier => Expression::Identifier(
                            Identifier::from_pest(&mut pair.into_inner().next().unwrap().into_inner()).unwrap(),
                        ),
                        rule => unreachable!("`expression_primitive` should contain one of [`value`, `identifier`], found {:#?}", rule)
                    }
                },
                rule => unreachable!("`term` should contain one of ['value', 'identifier', 'expression', 'expression_not', 'expression_increment', 'expression_decrement'], found {:#?}", rule)
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
        Rule::operation_or => Expression::binary(BinaryOperation::Or, lhs, rhs, span),
        Rule::operation_and => Expression::binary(BinaryOperation::And, lhs, rhs, span),
        Rule::operation_eq => Expression::binary(BinaryOperation::Eq, lhs, rhs, span),
        Rule::operation_ne => Expression::binary(BinaryOperation::Ne, lhs, rhs, span),
        Rule::operation_ge => Expression::binary(BinaryOperation::Ge, lhs, rhs, span),
        Rule::operation_gt => Expression::binary(BinaryOperation::Gt, lhs, rhs, span),
        Rule::operation_le => Expression::binary(BinaryOperation::Le, lhs, rhs, span),
        Rule::operation_lt => Expression::binary(BinaryOperation::Lt, lhs, rhs, span),
        Rule::operation_add => Expression::binary(BinaryOperation::Add, lhs, rhs, span),
        Rule::operation_sub => Expression::binary(BinaryOperation::Sub, lhs, rhs, span),
        Rule::operation_mul => Expression::binary(BinaryOperation::Mul, lhs, rhs, span),
        Rule::operation_div => Expression::binary(BinaryOperation::Div, lhs, rhs, span),
        Rule::operation_pow => Expression::binary(BinaryOperation::Pow, lhs, rhs, span),
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

// Functions

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::input_model))]
pub struct InputModel<'ast> {
    pub mutable: Option<Mutable>,
    pub identifier: Identifier<'ast>,
    pub visibility: Option<Visibility>,
    pub _type: Type<'ast>,
    #[pest_ast(outer())]
    pub span: Span<'ast>,
}

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::function_definition))]
pub struct Function<'ast> {
    pub function_name: Identifier<'ast>,
    pub parameters: Vec<InputModel<'ast>>,
    pub returns: Vec<Type<'ast>>,
    pub statements: Vec<Statement<'ast>>,
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

// Tests

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::test))]
pub struct Test<'ast> {
    pub function: Function<'ast>,
    #[pest_ast(outer())]
    pub span: Span<'ast>,
}

// File

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::file))]
pub struct File<'ast> {
    pub imports: Vec<Import<'ast>>,
    pub circuits: Vec<Circuit<'ast>>,
    pub functions: Vec<Function<'ast>>,
    pub tests: Vec<Test<'ast>>,
    pub eoi: EOI,
    #[pest_ast(outer())]
    pub span: Span<'ast>,
}
