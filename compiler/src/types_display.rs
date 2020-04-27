//! Format display functions for zokrates_program types.

use crate::{
    Assignee, ConditionalNestedOrEnd, ConditionalStatement, Expression, Function, FunctionName,
    Integer, Parameter, RangeOrExpression, SpreadOrExpression, Statement, Struct, StructField,
    Type, Variable,
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

impl<F: Field + PrimeField> fmt::Display for ConditionalNestedOrEnd<F> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ConditionalNestedOrEnd::Nested(ref nested) => write!(f, "else {}", nested),
            ConditionalNestedOrEnd::End(ref statements) => {
                write!(f, "else {{\n")?;
                for statement in statements.iter() {
                    write!(f, "\t\t{}\n", statement)?;
                }
                write!(f, "\t}}")
            }
        }
    }
}

impl<F: Field + PrimeField> fmt::Display for ConditionalStatement<F> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "if ({}) {{\n", self.condition)?;
        for statement in self.statements.iter() {
            write!(f, "\t\t{}\n", statement)?;
        }
        match self.next.clone() {
            Some(n_or_e) => write!(f, "\t}} {}", n_or_e),
            None => write!(f, "\t}}"),
        }
    }
}

impl<F: Field + PrimeField> fmt::Display for Statement<F> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Statement::Return(ref statements) => {
                write!(f, "return ")?;
                for (i, value) in statements.iter().enumerate() {
                    write!(f, "{}", value)?;
                    if i < statements.len() - 1 {
                        write!(f, ", ")?;
                    }
                }
                write!(f, "\n")
            }
            Statement::Definition(ref ty, ref assignee, ref statement) => {
                write!(f, "{} {} = {};", ty, assignee, statement)
            }
            Statement::Assign(ref variable, ref statement) => {
                write!(f, "{} = {};", variable, statement)
            }
            Statement::MultipleAssign(ref assignees, ref function) => {
                for (i, id) in assignees.iter().enumerate() {
                    write!(f, "{}", id)?;
                    if i < assignees.len() - 1 {
                        write!(f, ", ")?;
                    }
                }
                write!(f, " = {};", function)
            }
            Statement::Conditional(ref statement) => write!(f, "{}", statement),
            Statement::For(ref var, ref start, ref stop, ref list) => {
                write!(f, "for {} in {}..{} do\n", var, start, stop)?;
                for l in list {
                    write!(f, "\t\t{}\n", l)?;
                }
                write!(f, "\tendfor;")
            }
        }
    }
}

impl<F: Field + PrimeField> fmt::Debug for Statement<F> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Statement::Return(ref statements) => {
                write!(f, "return ")?;
                for (i, value) in statements.iter().enumerate() {
                    write!(f, "{}", value)?;
                    if i < statements.len() - 1 {
                        write!(f, ", ")?;
                    }
                }
                write!(f, "\n")
            }
            Statement::Definition(ref ty, ref assignee, ref statement) => {
                write!(f, "{} {} = {};", ty, assignee, statement)
            }
            Statement::Assign(ref variable, ref statement) => {
                write!(f, "{} = {};", variable, statement)
            }
            Statement::MultipleAssign(ref assignees, ref function) => {
                for (i, id) in assignees.iter().enumerate() {
                    write!(f, "{}", id)?;
                    if i < assignees.len() - 1 {
                        write!(f, ", ")?;
                    }
                }
                write!(f, " = {};", function)
            }
            Statement::Conditional(ref statement) => write!(f, "{}", statement),
            Statement::For(ref var, ref start, ref stop, ref list) => {
                write!(f, "for {} in {}..{} do\n", var, start, stop)?;
                for l in list {
                    write!(f, "\t\t{}\n", l)?;
                }
                write!(f, "\tendfor;")
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
