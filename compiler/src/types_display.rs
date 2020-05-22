//! Format display functions for Leo types.

use crate::{
    Assignee, Circuit, CircuitMember, ConditionalNestedOrEnd, ConditionalStatement, Expression,
    Function, Identifier, InputModel, InputValue, Integer, IntegerType, RangeOrExpression,
    SpreadOrExpression, Statement, Type, Variable,
};

use snarkos_models::curves::{Field, PrimeField};
use std::fmt;

impl<NativeF: Field, F: Field + PrimeField> fmt::Display for Identifier<NativeF, F> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}
impl<NativeF: Field, F: Field + PrimeField> fmt::Debug for Identifier<NativeF, F> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl<NativeF: Field, F: Field + PrimeField> fmt::Display for Variable<NativeF, F> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.mutable {
            write!(f, "mut ")?;
        }

        write!(f, "{}", self.identifier)?;

        if self._type.is_some() {
            write!(f, ": {}", self._type.as_ref().unwrap())?;
        }

        write!(f, "")
    }
}

impl fmt::Display for Integer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.to_usize(), self.get_type())
    }
}

impl<'ast, NativeF: Field, F: Field + PrimeField> fmt::Display for RangeOrExpression<NativeF, F> {
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

impl<NativeF: Field, F: Field + PrimeField> fmt::Display for SpreadOrExpression<NativeF, F> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SpreadOrExpression::Spread(ref spread) => write!(f, "...{}", spread),
            SpreadOrExpression::Expression(ref expression) => write!(f, "{}", expression),
        }
    }
}

impl<'ast, NativeF: Field, F: Field + PrimeField> fmt::Display for Expression<NativeF, F> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            // Variables
            Expression::Identifier(ref variable) => write!(f, "{}", variable),

            // Values
            Expression::Integer(ref integer) => write!(f, "{}", integer),
            Expression::FieldElement(ref field) => write!(f, "{}", field),
            Expression::GroupElement(ref x, ref y) => write!(f, "({}, {})", x, y),
            Expression::Boolean(ref bool) => write!(f, "{}", bool.get_value().unwrap()),
            Expression::Implicit(ref value) => write!(f, "{}", value),

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
            Expression::Ge(ref lhs, ref rhs) => write!(f, "{} >= {}", lhs, rhs),
            Expression::Gt(ref lhs, ref rhs) => write!(f, "{} > {}", lhs, rhs),
            Expression::Le(ref lhs, ref rhs) => write!(f, "{} <= {}", lhs, rhs),
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

            // Circuits
            Expression::Circuit(ref var, ref members) => {
                write!(f, "{} {{", var)?;
                for (i, member) in members.iter().enumerate() {
                    write!(f, "{}: {}", member.identifier, member.expression)?;
                    if i < members.len() - 1 {
                        write!(f, ", ")?;
                    }
                }
                write!(f, "}}")
            }
            Expression::CircuitMemberAccess(ref circuit_name, ref member) => {
                write!(f, "{}.{}", circuit_name, member)
            }
            Expression::CircuitStaticFunctionAccess(ref circuit_name, ref member) => {
                write!(f, "{}::{}", circuit_name, member)
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

impl<NativeF: Field, F: Field + PrimeField> fmt::Display for Assignee<NativeF, F> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Assignee::Identifier(ref variable) => write!(f, "{}", variable),
            Assignee::Array(ref array, ref index) => write!(f, "{}[{}]", array, index),
            Assignee::CircuitField(ref circuit_variable, ref member) => {
                write!(f, "{}.{}", circuit_variable, member)
            }
        }
    }
}

impl<NativeF: Field, F: Field + PrimeField> fmt::Display for ConditionalNestedOrEnd<NativeF, F> {
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

impl<NativeF: Field, F: Field + PrimeField> fmt::Display for ConditionalStatement<NativeF, F> {
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

impl<NativeF: Field, F: Field + PrimeField> fmt::Display for Statement<NativeF, F> {
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
            Statement::Definition(ref variable, ref expression) => {
                write!(f, "let {} = {};", variable, expression)
            }
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

impl<NativeF: Field, F: Field + PrimeField> fmt::Display for Type<NativeF, F> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Type::IntegerType(ref integer_type) => write!(f, "{}", integer_type),
            Type::FieldElement => write!(f, "field"),
            Type::GroupElement => write!(f, "group"),
            Type::Boolean => write!(f, "bool"),
            Type::Circuit(ref variable) => write!(f, "{}", variable),
            Type::SelfType => write!(f, "Self"),
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

impl<NativeF: Field, F: Field + PrimeField> fmt::Display for CircuitMember<NativeF, F> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CircuitMember::CircuitField(ref identifier, ref _type) => {
                write!(f, "{}: {}", identifier, _type)
            }
            CircuitMember::CircuitFunction(ref _static, ref function) => {
                if *_static {
                    write!(f, "static ")?;
                }
                write!(f, "{}", function)
            }
        }
    }
}

impl<NativeF: Field, F: Field + PrimeField> Circuit<NativeF, F> {
    fn format(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "circuit {} {{ \n", self.identifier)?;
        for field in self.members.iter() {
            write!(f, "    {}\n", field)?;
        }
        write!(f, "}}")
    }
}

// impl<NativeF: Field, F: Field + PrimeField> fmt::Display for Circuit<NativeF, F> {// uncomment when we no longer print out Program
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         self.format(f)
//     }
// }

impl<NativeF: Field, F: Field + PrimeField> fmt::Debug for Circuit<NativeF, F> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.format(f)
    }
}

impl<NativeF: Field, F: Field + PrimeField> fmt::Display for InputModel<NativeF, F> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // mut var: private bool
        if self.mutable {
            write!(f, "mut ")?;
        }
        write!(f, "{}: ", self.identifier)?;
        if self.private {
            write!(f, "private ")?;
        } else {
            write!(f, "public ")?;
        }
        write!(f, "{}", self._type)
    }
}

impl<NativeF: Field, F: Field + PrimeField> fmt::Display for InputValue<NativeF, F> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            InputValue::Integer(ref integer) => write!(f, "{}", integer),
            InputValue::Field(ref field) => write!(f, "{}", field),
            InputValue::Group(ref x, ref y) => write!(f, "({}, {})", x, y),
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

impl<NativeF: Field, F: Field + PrimeField> Function<NativeF, F> {
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

impl<NativeF: Field, F: Field + PrimeField> fmt::Display for Function<NativeF, F> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.format(f)
    }
}

impl<NativeF: Field, F: Field + PrimeField> fmt::Debug for Function<NativeF, F> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.format(f)
    }
}
