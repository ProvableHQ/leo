//! Format display functions for zokrates_program types.
//!
//! @file zokrates_program.rs
//! @author Collin Chin <collin@aleo.org>
//! @date 2020

use crate::program::{
    Assignee, Expression, Function, FunctionName, Integer, Parameter, RangeOrExpression,
    SpreadOrExpression, Statement, Struct, StructField, Type, Variable,
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

impl<'ast, F: Field + PrimeField> fmt::Display for RangeOrExpression<F> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RangeOrExpression::Range(ref from, ref to) => write!(
                f,
                "{}..{}",
                from.as_ref()
                    .map(|e| format!("{}", e))
                    .unwrap_or("".to_string()),
                to.as_ref()
                    .map(|e| format!("{}", e))
                    .unwrap_or("".to_string())
            ),
            RangeOrExpression::Expression(ref e) => write!(f, "{}", e),
        }
    }
}

impl<F: Field + PrimeField> fmt::Display for SpreadOrExpression<F> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SpreadOrExpression::Spread(ref spread) => write!(f, "...{}", spread),
            SpreadOrExpression::Expression(ref expression) => write!(f, "{}", expression),
        }
    }
}

impl<'ast, F: Field + PrimeField> fmt::Display for Expression<F> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            // Variables
            Expression::Variable(ref variable) => write!(f, "{}", variable),

            // Values
            Expression::Integer(ref integer) => write!(f, "{}", integer),
            Expression::FieldElement(ref fe) => write!(f, "{}", fe),
            Expression::Boolean(ref bool) => write!(f, "{}", bool),

            // Number operations
            Expression::Add(ref left, ref right) => write!(f, "{} + {}", left, right),
            Expression::Sub(ref left, ref right) => write!(f, "{} - {}", left, right),
            Expression::Mul(ref left, ref right) => write!(f, "{} * {}", left, right),
            Expression::Div(ref left, ref right) => write!(f, "{} / {}", left, right),
            Expression::Pow(ref left, ref right) => write!(f, "{} ** {}", left, right),

            // Boolean operations
            Expression::Not(ref expression) => write!(f, "!{}", expression),
            Expression::Or(ref lhs, ref rhs) => write!(f, "{} || {}", lhs, rhs),
            Expression::And(ref lhs, ref rhs) => write!(f, "{} && {}", lhs, rhs),
            Expression::Eq(ref lhs, ref rhs) => write!(f, "{} == {}", lhs, rhs),
            Expression::Geq(ref lhs, ref rhs) => write!(f, "{} >= {}", lhs, rhs),
            Expression::Gt(ref lhs, ref rhs) => write!(f, "{} > {}", lhs, rhs),
            Expression::Leq(ref lhs, ref rhs) => write!(f, "{} <= {}", lhs, rhs),
            Expression::Lt(ref lhs, ref rhs) => write!(f, "{} < {}", lhs, rhs),

            // Conditionals
            Expression::IfElse(ref first, ref second, ref third) => {
                write!(f, "if {} then {} else {} fi", first, second, third)
            }

            Expression::Array(ref array) => {
                write!(f, "[")?;
                for (i, e) in array.iter().enumerate() {
                    write!(f, "{}", e)?;
                    if i < array.len() - 1 {
                        write!(f, ", ")?;
                    }
                }
                write!(f, "]")
            }
            Expression::ArrayAccess(ref array, ref index) => write!(f, "{}[{}]", array, index),

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
            Type::U32 => write!(f, "u32"),
            Type::FieldElement => write!(f, "fe"),
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
        let visibility = if self.private { "private" } else { "public" };
        write!(f, "{}: {} {}", self.variable, visibility, self.ty,)
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
            "({}): ->({})\n{}",
            self.parameters
                .iter()
                .map(|x| format!("{}", x))
                .collect::<Vec<_>>()
                .join(","),
            self.returns
                .iter()
                .map(|r| format!("{}", r))
                .collect::<Vec<_>>()
                .join(","),
            self.statements
                .iter()
                .map(|x| format!("\t{}", x))
                .collect::<Vec<_>>()
                .join("\n"),
        )
    }
}

impl<F: Field + PrimeField> fmt::Debug for Function<F> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "({}): -> ({})\n{}",
            self.parameters
                .iter()
                .map(|x| format!("{}", x))
                .collect::<Vec<_>>()
                .join(","),
            self.returns
                .iter()
                .map(|r| format!("{}", r))
                .collect::<Vec<_>>()
                .join(","),
            self.statements
                .iter()
                .map(|x| format!("\t{}", x))
                .collect::<Vec<_>>()
                .join("\n"),
        )
    }
}

// impl<F: Field + PrimeField> fmt::Display for IntegerSpreadOrExpression<F> {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         match *self {
//             IntegerSpreadOrExpression::Spread(ref spread) => write!(f, "...{}", spread),
//             IntegerSpreadOrExpression::Expression(ref expression) => write!(f, "{}", expression),
//         }
//     }
// }

// impl<'ast, F: Field + PrimeField> fmt::Display for IntegerExpression<F> {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         match *self {
//             IntegerExpression::Variable(ref variable) => write!(f, "{}", variable),
//             IntegerExpression::Number(ref number) => write!(f, "{}", number),
//             IntegerExpression::Add(ref lhs, ref rhs) => write!(f, "{} + {}", lhs, rhs),
//             IntegerExpression::Sub(ref lhs, ref rhs) => write!(f, "{} - {}", lhs, rhs),
//             IntegerExpression::Mul(ref lhs, ref rhs) => write!(f, "{} * {}", lhs, rhs),
//             IntegerExpression::Div(ref lhs, ref rhs) => write!(f, "{} / {}", lhs, rhs),
//             IntegerExpression::Pow(ref lhs, ref rhs) => write!(f, "{} ** {}", lhs, rhs),
//             IntegerExpression::IfElse(ref a, ref b, ref c) => {
//                 write!(f, "if {} then {} else {} fi", a, b, c)
//             }
//             IntegerExpression::Array(ref array) => {
//                 write!(f, "[")?;
//                 for (i, e) in array.iter().enumerate() {
//                     write!(f, "{}", e)?;
//                     if i < array.len() - 1 {
//                         write!(f, ", ")?;
//                     }
//                 }
//                 write!(f, "]")
//             }
//         }
//     }
// }

// impl<F: Field + PrimeField> fmt::Display for FieldSpreadOrExpression<F> {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         match *self {
//             FieldSpreadOrExpression::Spread(ref spread) => write!(f, "...{}", spread),
//             FieldSpreadOrExpression::Expression(ref expression) => write!(f, "{}", expression),
//         }
//     }
// }
//
// impl<'ast, F: Field + PrimeField> fmt::Display for FieldExpression<F> {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         match *self {
//             FieldExpression::Variable(ref variable) => write!(f, "{}", variable),
//             FieldExpression::Number(ref number) => write!(f, "{}", number),
//             FieldExpression::Add(ref lhs, ref rhs) => write!(f, "{} + {}", lhs, rhs),
//             FieldExpression::Sub(ref lhs, ref rhs) => write!(f, "{} - {}", lhs, rhs),
//             FieldExpression::Mul(ref lhs, ref rhs) => write!(f, "{} * {}", lhs, rhs),
//             FieldExpression::Div(ref lhs, ref rhs) => write!(f, "{} / {}", lhs, rhs),
//             FieldExpression::Pow(ref lhs, ref rhs) => write!(f, "{} ** {}", lhs, rhs),
//             FieldExpression::IfElse(ref a, ref b, ref c) => {
//                 write!(f, "if {} then {} else {} fi", a, b, c)
//             }
//             FieldExpression::Array(ref array) => {
//                 write!(f, "[")?;
//                 for (i, e) in array.iter().enumerate() {
//                     write!(f, "{}", e)?;
//                     if i < array.len() - 1 {
//                         write!(f, ", ")?;
//                     }
//                 }
//                 write!(f, "]")
//             } // _ => unimplemented!("not all field expressions can be displayed")
//         }
//     }
// }

// impl<F: Field + PrimeField> fmt::Display for BooleanSpreadOrExpression<F> {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         match *self {
//             BooleanSpreadOrExpression::Spread(ref spread) => write!(f, "...{}", spread),
//             BooleanSpreadOrExpression::Expression(ref expression) => write!(f, "{}", expression),
//         }
//     }
// }

// impl<'ast, F: Field + PrimeField> fmt::Display for BooleanExpression<F> {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         match *self {
//             BooleanExpression::Variable(ref variable) => write!(f, "{}", variable),
//             BooleanExpression::Value(ref value) => write!(f, "{}", value),
//             BooleanExpression::Not(ref expression) => write!(f, "!{}", expression),
//             BooleanExpression::Or(ref lhs, ref rhs) => write!(f, "{} || {}", lhs, rhs),
//             BooleanExpression::And(ref lhs, ref rhs) => write!(f, "{} && {}", lhs, rhs),
//             BooleanExpression::BoolEq(ref lhs, ref rhs) => write!(f, "{} == {}", lhs, rhs),
//             BooleanExpression::IntegerEq(ref lhs, ref rhs) => write!(f, "{} == {}", lhs, rhs),
//             BooleanExpression::FieldEq(ref lhs, ref rhs) => write!(f, "{} == {}", lhs, rhs),
//             // BooleanExpression::Neq(ref lhs, ref rhs) => write!(f, "{} != {}", lhs, rhs),
//             BooleanExpression::Geq(ref lhs, ref rhs) => write!(f, "{} >= {}", lhs, rhs),
//             BooleanExpression::Gt(ref lhs, ref rhs) => write!(f, "{} > {}", lhs, rhs),
//             BooleanExpression::Leq(ref lhs, ref rhs) => write!(f, "{} <= {}", lhs, rhs),
//             BooleanExpression::Lt(ref lhs, ref rhs) => write!(f, "{} < {}", lhs, rhs),
//             BooleanExpression::IfElse(ref a, ref b, ref c) => {
//                 write!(f, "if {} then {} else {} fi", a, b, c)
//             }
//             BooleanExpression::Array(ref array) => {
//                 write!(f, "[")?;
//                 for (i, e) in array.iter().enumerate() {
//                     write!(f, "{}", e)?;
//                     if i < array.len() - 1 {
//                         write!(f, ", ")?;
//                     }
//                 }
//                 write!(f, "]")
//             }
//         }
//     }
// }
