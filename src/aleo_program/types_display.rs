//! Format display functions for zokrates_program types.
//!
//! @file zokrates_program.rs
//! @author Collin Chin <collin@aleo.org>
//! @date 2020

use crate::aleo_program::{
    Assignee, BooleanExpression, BooleanSpreadOrExpression, Expression, Function, FunctionName,
    Integer, IntegerExpression, IntegerRangeOrExpression, IntegerSpreadOrExpression, Parameter,
    Statement, Struct, StructField, Type, Variable,
};

use snarkos_models::curves::{Field, PrimeField};
use std::fmt;

impl<F: Field + PrimeField> fmt::Display for Variable<F> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}
impl<F: Field + PrimeField> fmt::Debug for Variable<F> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl fmt::Display for Integer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Integer::U32(ref num) => write!(f, "{}", num),
        }
    }
}

impl<F: Field + PrimeField> fmt::Display for IntegerSpreadOrExpression<F> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            IntegerSpreadOrExpression::Spread(ref spread) => write!(f, "...{}", spread),
            IntegerSpreadOrExpression::Expression(ref expression) => write!(f, "{}", expression),
        }
    }
}

impl<'ast, F: Field + PrimeField> fmt::Display for IntegerExpression<F> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            IntegerExpression::Variable(ref variable) => write!(f, "{}", variable),
            IntegerExpression::Number(ref number) => write!(f, "{}", number),
            IntegerExpression::Add(ref lhs, ref rhs) => write!(f, "{} + {}", lhs, rhs),
            IntegerExpression::Sub(ref lhs, ref rhs) => write!(f, "{} - {}", lhs, rhs),
            IntegerExpression::Mul(ref lhs, ref rhs) => write!(f, "{} * {}", lhs, rhs),
            IntegerExpression::Div(ref lhs, ref rhs) => write!(f, "{} / {}", lhs, rhs),
            IntegerExpression::Pow(ref lhs, ref rhs) => write!(f, "{} ** {}", lhs, rhs),
            IntegerExpression::IfElse(ref a, ref b, ref c) => {
                write!(f, "if {} then {} else {} fi", a, b, c)
            }
            IntegerExpression::Array(ref array) => {
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

impl<F: Field + PrimeField> fmt::Display for BooleanSpreadOrExpression<F> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            BooleanSpreadOrExpression::Spread(ref spread) => write!(f, "...{}", spread),
            BooleanSpreadOrExpression::Expression(ref expression) => write!(f, "{}", expression),
        }
    }
}

impl<'ast, F: Field + PrimeField> fmt::Display for BooleanExpression<F> {
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

impl<'ast, F: Field + PrimeField> fmt::Display for IntegerRangeOrExpression<F> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            IntegerRangeOrExpression::Range(ref from, ref to) => write!(
                f,
                "{}..{}",
                from.as_ref()
                    .map(|e| e.to_string())
                    .unwrap_or("".to_string()),
                to.as_ref().map(|e| e.to_string()).unwrap_or("".to_string())
            ),
            IntegerRangeOrExpression::Expression(ref e) => write!(f, "{}", e),
        }
    }
}

impl<'ast, F: Field + PrimeField> fmt::Display for Expression<F> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Expression::Integer(ref integer_expression) => write!(f, "{}", integer_expression),
            Expression::FieldElement(ref _field_expression) => {
                unimplemented!("field elem not impl ")
            }
            Expression::Boolean(ref boolean_expression) => write!(f, "{}", boolean_expression),
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

impl<F: Field + PrimeField> fmt::Display for Assignee<F> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Assignee::Variable(ref variable) => write!(f, "{}", variable),
            Assignee::Array(ref array, ref index) => write!(f, "{}[{}]", array, index),
            Assignee::StructMember(ref struct_variable, ref member) => {
                write!(f, "{}.{}", struct_variable, member)
            }
        }
    }
}

impl<F: Field + PrimeField> fmt::Display for Statement<F> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Statement::Definition(ref variable, ref statement) => {
                write!(f, "{} = {}", variable, statement)
            }
            Statement::For(ref var, ref start, ref stop, ref list) => {
                write!(f, "for {} in {}..{} do\n", var, start, stop)?;
                for l in list {
                    write!(f, "\t\t{}\n", l)?;
                }
                write!(f, "\tendfor")
            }
            Statement::Return(ref statements) => {
                statements.iter().for_each(|statement| {
                    write!(f, "return {}", statement).unwrap();
                });
                write!(f, "\n")
            }
        }
    }
}

impl<F: Field + PrimeField> fmt::Debug for Statement<F> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Statement::Definition(ref variable, ref statement) => {
                write!(f, "{} = {}", variable, statement)
            }
            Statement::For(ref var, ref start, ref stop, ref list) => {
                write!(f, "for {:?} in {:?}..{:?} do\n", var, start, stop)?;
                for l in list {
                    write!(f, "\t\t{:?}\n", l)?;
                }
                write!(f, "\tendfor")
            }
            Statement::Return(ref statements) => {
                statements.iter().for_each(|statement| {
                    write!(f, "return {}", statement).unwrap();
                });
                write!(f, "\n")
            }
        }
    }
}

impl<F: Field + PrimeField> fmt::Display for Type<F> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Type::FieldElement => unimplemented!("field type unimpl"),
            Type::U32 => write!(f, "field"),
            Type::Boolean => write!(f, "bool"),
            Type::Struct(ref variable) => write!(f, "{}", variable),
            Type::Array(ref array, ref count) => write!(f, "[{}; {}]", array, count),
        }
    }
}

impl<F: Field + PrimeField> fmt::Display for StructField<F> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} : {}", self.ty, self.variable)
    }
}

impl<F: Field + PrimeField> fmt::Debug for Struct<F> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "struct {} {{ \n", self.variable)?;
        for field in self.fields.iter() {
            write!(f, "    {}\n", field)?;
        }
        write!(f, "}}")
    }
}

impl<F: Field + PrimeField> fmt::Display for Parameter<F> {
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

impl<F: Field + PrimeField> fmt::Debug for Parameter<F> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Parameter(variable: {:?})", self.ty)
    }
}

impl fmt::Debug for FunctionName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<F: Field + PrimeField> fmt::Display for Function<F> {
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

impl<F: Field + PrimeField> fmt::Debug for Function<F> {
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
