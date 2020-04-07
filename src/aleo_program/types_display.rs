//! Format display functions for zokrates_program types.
//!
//! @file zokrates_program.rs
//! @author Collin Chin <collin@aleo.org>
//! @date 2020

use crate::aleo_program::{BooleanExpression, Expression, FieldExpression, Statement, Variable};

use std::fmt;

impl fmt::Display for Variable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<'ast> fmt::Display for FieldExpression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            FieldExpression::Variable(ref variable) => write!(f, "{}", variable),
            FieldExpression::Number(ref number) => write!(f, "{}", number),
            FieldExpression::Add(ref lhs, ref rhs) => write!(f, "{} + {}", lhs, rhs),
            FieldExpression::Sub(ref lhs, ref rhs) => write!(f, "{} - {}", lhs, rhs),
            FieldExpression::Mul(ref lhs, ref rhs) => write!(f, "{} * {}", lhs, rhs),
            FieldExpression::Div(ref lhs, ref rhs) => write!(f, "{} / {}", lhs, rhs),
            FieldExpression::Pow(ref lhs, ref rhs) => write!(f, "{} ** {}", lhs, rhs),
            FieldExpression::IfElse(ref _a, ref _b, ref _c) => unimplemented!(),
        }
    }
}

impl<'ast> fmt::Display for BooleanExpression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            BooleanExpression::Variable(ref variable) => write!(f, "{}", variable),
            BooleanExpression::Value(ref value) => write!(f, "{}", value),
            BooleanExpression::Not(ref expression) => write!(f, "!{}", expression),
            BooleanExpression::Or(ref lhs, ref rhs) => write!(f, "{} || {}", lhs, rhs),
            BooleanExpression::And(ref lhs, ref rhs) => write!(f, "{} && {}", lhs, rhs),
            BooleanExpression::BoolEq(ref lhs, ref rhs) => write!(f, "{} == {}", lhs, rhs),
            BooleanExpression::FieldEq(ref lhs, ref rhs) => write!(f, "{} == {}", lhs, rhs),
            // BooleanExpression::Neq(ref lhs, ref rhs) => write!(f, "{} != {}", lhs, rhs),
            BooleanExpression::Geq(ref lhs, ref rhs) => write!(f, "{} >= {}", lhs, rhs),
            BooleanExpression::Gt(ref lhs, ref rhs) => write!(f, "{} > {}", lhs, rhs),
            BooleanExpression::Leq(ref lhs, ref rhs) => write!(f, "{} <= {}", lhs, rhs),
            BooleanExpression::Lt(ref lhs, ref rhs) => write!(f, "{} < {}", lhs, rhs),
        }
    }
}

impl<'ast> fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Expression::Boolean(ref boolean_expression) => write!(f, "{}", boolean_expression),
            Expression::FieldElement(ref field_expression) => write!(f, "{}", field_expression),
            Expression::Variable(ref variable) => write!(f, "{}", variable),
        }
    }
}
impl fmt::Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Statement::Return(ref statements) => {
                statements.iter().for_each(|statement| {
                    write!(f, "return {}", statement).unwrap();
                });
                write!(f, "")
            }
            _ => unimplemented!(),
        }
    }
}

impl fmt::Debug for Statement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Statement::Return(ref statements) => {
                statements.iter().for_each(|statement| {
                    write!(f, "return {}", statement).unwrap();
                });
                write!(f, "")
            }
            Statement::Definition(ref variable, ref statement) => {
                write!(f, "{} = {}", variable, statement)
            }
        }
    }
}
