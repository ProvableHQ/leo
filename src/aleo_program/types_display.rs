//! Format display functions for zokrates_program types.
//!
//! @file zokrates_program.rs
//! @author Collin Chin <collin@aleo.org>
//! @date 2020

use crate::aleo_program::{
    BooleanExpression, BooleanSpread, BooleanSpreadOrExpression, Expression, FieldExpression,
    FieldRangeOrExpression, FieldSpread, FieldSpreadOrExpression, Function, Parameter, Statement,
    Struct, StructField, Type, Variable,
};

use std::fmt;

impl fmt::Display for Variable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl fmt::Debug for Variable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl fmt::Display for FieldSpread {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "...{}", self.0)
    }
}

impl fmt::Display for FieldSpreadOrExpression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            FieldSpreadOrExpression::Spread(ref spread) => write!(f, "{}", spread),
            FieldSpreadOrExpression::FieldExpression(ref expression) => write!(f, "{}", expression),
        }
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
            FieldExpression::IfElse(ref a, ref b, ref c) => {
                write!(f, "if {} then {} else {} fi", a, b, c)
            }
            FieldExpression::Array(ref array) => {
                write!(f, "[")?;
                for (i, e) in array.iter().enumerate() {
                    write!(f, "{}", e)?;
                    if i < array.len() - 1 {
                        write!(f, ", ")?;
                    }
                }
                write!(f, "]")
            }
        }
    }
}

impl fmt::Display for BooleanSpread {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "...{}", self.0)
    }
}

impl fmt::Display for BooleanSpreadOrExpression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            BooleanSpreadOrExpression::Spread(ref spread) => write!(f, "{}", spread),
            BooleanSpreadOrExpression::BooleanExpression(ref expression) => {
                write!(f, "{}", expression)
            }
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
            BooleanExpression::IfElse(ref a, ref b, ref c) => {
                write!(f, "if {} then {} else {} fi", a, b, c)
            }
            BooleanExpression::Array(ref array) => {
                write!(f, "[")?;
                for (i, e) in array.iter().enumerate() {
                    write!(f, "{}", e)?;
                    if i < array.len() - 1 {
                        write!(f, ", ")?;
                    }
                }
                write!(f, "]")
            }
        }
    }
}

impl<'ast> fmt::Display for FieldRangeOrExpression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            FieldRangeOrExpression::Range(ref from, ref to) => write!(
                f,
                "{}..{}",
                from.as_ref()
                    .map(|e| e.to_string())
                    .unwrap_or("".to_string()),
                to.as_ref().map(|e| e.to_string()).unwrap_or("".to_string())
            ),
            FieldRangeOrExpression::FieldExpression(ref e) => write!(f, "{}", e),
        }
    }
}

impl<'ast> fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Expression::Boolean(ref boolean_expression) => write!(f, "{}", boolean_expression),
            Expression::FieldElement(ref field_expression) => write!(f, "{}", field_expression),
            Expression::Variable(ref variable) => write!(f, "{}", variable),
            Expression::Struct(ref var, ref members) => {
                write!(f, "{} {{", var)?;
                for (i, member) in members.iter().enumerate() {
                    write!(f, "{}: {}", member.variable, member.expression)?;
                    if i < members.len() - 1 {
                        write!(f, ", ")?;
                    }
                }
                write!(f, "}}")
            }
            Expression::ArrayAccess(ref array, ref index) => write!(f, "{}[{}]", array, index),
            Expression::StructMemberAccess(ref struct_variable, ref member) => {
                write!(f, "{}.{}", struct_variable, member)
            }
            Expression::FunctionCall(ref function, ref arguments) => {
                write!(f, "{}(", function,)?;
                for (i, param) in arguments.iter().enumerate() {
                    write!(f, "{}", param)?;
                    if i < arguments.len() - 1 {
                        write!(f, ", ")?;
                    }
                }
                write!(f, ")")
            } // _ => unimplemented!("can't display expression yet"),
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
                write!(f, "\n")
            }
            Statement::Definition(ref variable, ref statement) => {
                write!(f, "{} = {}", variable, statement)
            }
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
                write!(f, "\n")
            }
            Statement::Definition(ref variable, ref statement) => {
                write!(f, "{} = {}", variable, statement)
            }
        }
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Type::Boolean => write!(f, "bool"),
            Type::FieldElement => write!(f, "field"),
            Type::Struct(ref variable) => write!(f, "{}", variable),
            Type::Array(ref array, ref count) => write!(f, "[{}; {}]", array, count),
        }
    }
}

impl fmt::Display for StructField {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} : {}", self.ty, self.variable)
    }
}

impl fmt::Debug for Struct {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "struct {} {{ \n", self.variable)?;
        for field in self.fields.iter() {
            write!(f, "    {}\n", field)?;
        }
        write!(f, "}}")
    }
}

impl fmt::Display for Parameter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // let visibility = if self.private { "private " } else { "" };
        write!(
            f,
            "{} {}",
            // visibility,
            self.ty,
            self.variable
        )
    }
}

impl fmt::Debug for Parameter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Parameter(variable: {:?})", self.ty)
    }
}

impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "({}):\n{}",
            self.parameters
                .iter()
                .map(|x| format!("{}", x))
                .collect::<Vec<_>>()
                .join(","),
            self.statements
                .iter()
                .map(|x| format!("\t{}", x))
                .collect::<Vec<_>>()
                .join("\n")
        )
    }
}

impl fmt::Debug for Function {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "({}):\n{}",
            self.parameters
                .iter()
                .map(|x| format!("{}", x))
                .collect::<Vec<_>>()
                .join(","),
            self.statements
                .iter()
                .map(|x| format!("\t{}", x))
                .collect::<Vec<_>>()
                .join("\n")
        )
    }
}
