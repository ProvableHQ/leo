//! Format display functions for Leo types.

use crate::{
    Assignee, ConditionalNestedOrEnd, ConditionalStatement, Expression, FieldElement, Function,
    FunctionName, InputModel, InputValue, Integer, IntegerType, RangeOrExpression,
    SpreadOrExpression, Statement, Struct, StructField, Type, Variable,
};

use snarkos_models::curves::{Group, Field, PrimeField};
use std::fmt;

impl<G: Group, F: Field + PrimeField> fmt::Display for Variable<G, F> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}
impl<G: Group, F: Field + PrimeField> fmt::Debug for Variable<G, F> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl fmt::Display for Integer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.to_usize(), self.get_type())
    }
}

impl<F: Field + PrimeField> FieldElement<F> {
    fn format(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            FieldElement::Constant(ref constant) => write!(f, "{}", constant),
            FieldElement::Allocated(ref option, ref _r1cs_var) => {
                if option.is_some() {
                    write!(f, "{}", option.unwrap())
                } else {
                    write!(f, "allocated fe")
                }
            }
        }
    }
}

impl<F: Field + PrimeField> fmt::Display for FieldElement<F> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.format(f)
    }
}

impl<F: Field + PrimeField> fmt::Debug for FieldElement<F> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.format(f)
    }
}

impl<'ast, G: Group, F: Field + PrimeField> fmt::Display for RangeOrExpression<G, F> {
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

impl<G: Group, F: Field + PrimeField> fmt::Display for SpreadOrExpression<G, F> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SpreadOrExpression::Spread(ref spread) => write!(f, "...{}", spread),
            SpreadOrExpression::Expression(ref expression) => write!(f, "{}", expression),
        }
    }
}

impl<'ast, G: Group, F: Field + PrimeField> fmt::Display for Expression<G, F> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            // Variables
            Expression::Variable(ref variable) => write!(f, "{}", variable),

            // Values
            Expression::Integer(ref integer) => write!(f, "{}", integer),
            Expression::FieldElement(ref fe) => write!(f, "{}", fe),
            Expression::GroupElement(ref gr) => write!(f, "{}", gr),
            Expression::Boolean(ref bool) => write!(f, "{}", bool.get_value().unwrap()),

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

            // Arrays
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

            // Structs
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

            // Function calls
            Expression::FunctionCall(ref function, ref arguments) => {
                write!(f, "{}(", function,)?;
                for (i, param) in arguments.iter().enumerate() {
                    write!(f, "{}", param)?;
                    if i < arguments.len() - 1 {
                        write!(f, ", ")?;
                    }
                }
                write!(f, ")")
            }
        }
    }
}

impl<G: Group, F: Field + PrimeField> fmt::Display for Assignee<G, F> {
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

impl<G: Group, F: Field + PrimeField> fmt::Display for ConditionalNestedOrEnd<G, F> {
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

impl<G: Group, F: Field + PrimeField> fmt::Display for ConditionalStatement<G, F> {
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

impl<G: Group, F: Field + PrimeField> fmt::Display for Statement<G, F> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Statement::Return(ref statements) => {
                write!(f, "return (")?;
                for (i, value) in statements.iter().enumerate() {
                    write!(f, "{}", value)?;
                    if i < statements.len() - 1 {
                        write!(f, ", ")?;
                    }
                }
                write!(f, ")\n")
            }
            Statement::Definition(ref assignee, ref ty, ref expression) => match ty {
                Some(ref ty) => write!(f, "let {}: {} = {};", assignee, ty, expression),
                None => write!(f, "let {} = {};", assignee, expression),
            },
            Statement::Assign(ref variable, ref statement) => {
                write!(f, "{} = {};", variable, statement)
            }
            Statement::MultipleAssign(ref assignees, ref function) => {
                write!(f, "let (")?;
                for (i, id) in assignees.iter().enumerate() {
                    write!(f, "{}", id)?;
                    if i < assignees.len() - 1 {
                        write!(f, ", ")?;
                    }
                }
                write!(f, ") = {};", function)
            }
            Statement::Conditional(ref statement) => write!(f, "{}", statement),
            Statement::For(ref var, ref start, ref stop, ref list) => {
                write!(f, "for {} in {}..{} {{\n", var, start, stop)?;
                for l in list {
                    write!(f, "\t\t{}\n", l)?;
                }
                write!(f, "\t}}")
            }
            Statement::AssertEq(ref left, ref right) => {
                write!(f, "assert_eq({}, {});", left, right)
            }
            Statement::Expression(ref expression) => write!(f, "{};", expression),
        }
    }
}

impl fmt::Display for IntegerType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            IntegerType::U8 => write!(f, "u8"),
            IntegerType::U16 => write!(f, "u16"),
            IntegerType::U32 => write!(f, "u32"),
            IntegerType::U64 => write!(f, "u64"),
            IntegerType::U128 => write!(f, "u128"),
        }
    }
}

impl<G: Group, F: Field + PrimeField> fmt::Display for Type<G, F> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Type::IntegerType(ref integer_type) => write!(f, "{}", integer_type),
            Type::FieldElement => write!(f, "field"),
            Type::GroupElement => write!(f, "group"),
            Type::Boolean => write!(f, "bool"),
            Type::Struct(ref variable) => write!(f, "{}", variable),
            Type::Array(ref array, ref dimensions) => {
                write!(f, "{}", *array)?;
                for row in dimensions {
                    write!(f, "[{}]", row)?;
                }
                write!(f, "")
            }
        }
    }
}

impl<G: Group, F: Field + PrimeField> fmt::Display for StructField<G, F> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.variable, self._type)
    }
}

impl<G: Group, F: Field + PrimeField> Struct<G, F> {
    fn format(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "struct {} {{ \n", self.variable)?;
        for field in self.fields.iter() {
            write!(f, "    {}\n", field)?;
        }
        write!(f, "}}")
    }
}

// impl<G: Group, F: Field + PrimeField> fmt::Display for Struct<G, F> {// uncomment when we no longer print out Program
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         self.format(f)
//     }
// }

impl<G: Group, F: Field + PrimeField> fmt::Debug for Struct<G, F> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.format(f)
    }
}

impl<G: Group, F: Field + PrimeField> fmt::Display for InputModel<G, F> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let visibility = if self.private { "private" } else { "public" };
        write!(f, "{}: {} {}", self.variable, visibility, self._type,)
    }
}

impl<G: Group, F: Field + PrimeField> fmt::Display for InputValue<G, F> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            InputValue::Integer(ref integer) => write!(f, "{}", integer),
            InputValue::Field(ref field) => write!(f, "{}", field),
            InputValue::Boolean(ref bool) => write!(f, "{}", bool),
            InputValue::Array(ref array) => {
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

impl FunctionName {
    fn format(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl fmt::Display for FunctionName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.format(f)
    }
}

impl fmt::Debug for FunctionName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.format(f)
    }
}

impl<G: Group, F: Field + PrimeField> Function<G, F> {
    fn format(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "function {}", self.function_name)?;
        let parameters = self
            .inputs
            .iter()
            .map(|x| format!("{}", x))
            .collect::<Vec<_>>()
            .join(",");
        let returns = self
            .returns
            .iter()
            .map(|r| format!("{}", r))
            .collect::<Vec<_>>()
            .join(",");
        let statements = self
            .statements
            .iter()
            .map(|s| format!("\t{}\n", s))
            .collect::<Vec<_>>()
            .join("");
        if self.returns.len() == 0 {
            write!(f, "({}) {{\n{}}}", parameters, statements,)
        } else if self.returns.len() == 1 {
            write!(f, "({}) -> {} {{\n{}}}", parameters, returns, statements,)
        } else {
            write!(f, "({}) -> ({}) {{\n{}}}", parameters, returns, statements,)
        }
    }
}

// impl<G: Group, F: Field + PrimeField> fmt::Display for Function<G, F> {// uncomment when we no longer print out Program
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         self.format(f)
//     }
// }

impl<G: Group, F: Field + PrimeField> fmt::Debug for Function<G, F> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.format(f)
    }
}
