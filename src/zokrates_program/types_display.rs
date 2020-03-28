//! Format display functions for zokrates_program types.
//!
//! @file zokrates_program.rs
//! @author Collin Chin <collin@aleo.org>
//! @date 2020

use crate::zokrates_program::{Expression, ExpressionList, Statement, Variable};

use std::fmt;

impl<'ast> fmt::Display for Variable<'ast> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.id)
    }
}

impl<'ast> fmt::Display for Expression<'ast> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Expression::Identifier(ref s) => write!(f, "Identifier({:?})", s),
            Expression::Boolean(ref b) => write!(f, "Boolean({:?})", b),
            Expression::Field(ref s) => write!(f, "Field({:?})", s),
            Expression::Variable(ref v) => write!(f, "{}", v),
            Expression::Not(ref e) => write!(f, "{}", e),
            Expression::Or(ref lhs, ref rhs) => write!(f, "{} || {}", lhs, rhs),
            Expression::And(ref lhs, ref rhs) => write!(f, "{} && {}", lhs, rhs),
            Expression::Eq(ref lhs, ref rhs) => write!(f, "{} == {}", lhs, rhs),
            Expression::Neq(ref lhs, ref rhs) => write!(f, "{} != {}", lhs, rhs),
            Expression::Geq(ref lhs, ref rhs) => write!(f, "{} >= {}", lhs, rhs),
            Expression::Gt(ref lhs, ref rhs) => write!(f, "{} > {}", lhs, rhs),
            Expression::Leq(ref lhs, ref rhs) => write!(f, "{} <= {}", lhs, rhs),
            Expression::Lt(ref lhs, ref rhs) => write!(f, "{} < {}", lhs, rhs),
            Expression::Add(ref lhs, ref rhs) => write!(f, "{} + {}", lhs, rhs),
            Expression::Sub(ref lhs, ref rhs) => write!(f, "{} - {}", lhs, rhs),
            Expression::Mul(ref lhs, ref rhs) => write!(f, "{} * {}", lhs, rhs),
            Expression::Div(ref lhs, ref rhs) => write!(f, "{} / {}", lhs, rhs),
            Expression::Pow(ref lhs, ref rhs) => write!(f, "{} ** {}", lhs, rhs),
        }
    }
}

impl<'ast> fmt::Display for ExpressionList<'ast> {
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

impl<'ast> fmt::Display for Statement<'ast> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Statement::Return(ref expressions) => write!(f, "return {}", expressions),
            Statement::Declaration(ref variable) => write!(f, "{}", variable),
            Statement::Definition(ref variable, ref expression) => {
                write!(f, "{} = {}", variable, expression)
            }
        }
    }
}
