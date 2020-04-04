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
                }
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
                Rule::expression => Expression::from_pest(&mut pair.into_inner()).unwrap(), // Parenthesis case

                // Rule::expression_increment => {
                //     let span = next.as_span();
                //     let mut inner = next.into_inner();
                //     let expression = parse_expression_term(inner.next().unwrap());
                //     let operation = match inner.next().unwrap().as_rule() {
                //         Rule::operation_post_increment => Increment::from_pest(&mut pair.into_inner().next().unwrap().into_inner()).unwrap(),
                //         rule => unreachable!("`expression_increment` should yield `operation_post_increment`, found {:#?}", rule)
                //     };
                //     Expression::Increment(IncrementExpression { operation, expression, span })
                // },
                // Rule::expression_decrement => {
                //     let span = next.as_span();
                //     let mut inner = next.into_inner();
                //     let expression = parse_expression_term(inner.next().unwrap());
                //     let operation = match inner.next().unwrap().as_rule() {
                //         Rule::operation_post_decrement => Decrement::from_pest(&mut pair.into_inner().next().unwrap().into_inner()).unwrap(),
                //         rule => unreachable!("`expression_decrement` should yield `operation_post_decrement`, found {:#?}", rule)
                //     };
                //     Expression::Decrement(DecrementExpression { operation, expression, span })
                // },

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

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::file))]
pub struct File<'ast> {
    pub statements: Vec<Statement<'ast>>,
    pub eoi: EOI,
    #[pest_ast(outer())]
    pub span: Span<'ast>,
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
//     pub operation: Increment<'ast>,
//     #[pest_ast(outer())]
//     pub span: Span<'ast>,
// }
//
// #[derive(Clone, Debug, FromPest, PartialEq)]
// #[pest_ast(rule(Rule::expression_decrement))]
// pub struct DecrementExpression<'ast> {
//     pub expression: Box<Expression<'ast>>,
//     pub operation: Decrement<'ast>,
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

// #[derive(Clong, Debug, PartialEq)]
// pub struct IdentifierExpression<'ast> {
//     pub value: String,
//     pub span: Span<'ast>,
// }

#[derive(Clone, Debug, PartialEq)]
pub enum Expression<'ast> {
    Value(Value<'ast>),
    Variable(Variable<'ast>),
    Not(NotExpression<'ast>),
    Binary(BinaryExpression<'ast>),
    // Increment(IncrementExpression<'ast>),
    // Decrement(DecrementExpression<'ast>),
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

    pub fn span(&self) -> &Span<'ast> {
        match self {
            Expression::Value(expression) => &expression.span(),
            Expression::Variable(expression) => &expression.span,
            Expression::Not(expression) => &expression.span,
            Expression::Binary(expression) => &expression.span,
            // Expression::Increment(expression) => &expression.span,
            // Expression::Decrement(expression) => &expression.span,
        }
    }
}

impl<'ast> fmt::Display for Expression<'ast> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Expression::Value(ref expression) => write!(f, "{}", expression),
            Expression::Variable(ref expression) => write!(f, "{}", expression),
            Expression::Not(ref expression) => write!(f, "{}", expression.expression),
            Expression::Binary(ref expression) => {
                write!(f, "{} == {}", expression.left, expression.right)
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

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::statement))]
pub enum Statement<'ast> {
    Assign(AssignStatement<'ast>),
    Return(ReturnStatement<'ast>),
}

impl<'ast> fmt::Display for Statement<'ast> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Statement::Assign(ref statement) => write!(f, "{}", statement),
            Statement::Return(ref statement) => write!(f, "{}", statement),
        }
    }
}

// Utilities

#[derive(Debug, FromPest, PartialEq, Clone)]
#[pest_ast(rule(Rule::EOI))]
pub struct EOI;
