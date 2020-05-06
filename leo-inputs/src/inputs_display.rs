//! Display implementations for a Leo inputs file

use crate::inputs_ast::{
    ArrayInitializerExpression, ArrayInlineExpression, ArrayType, Assignment, BasicType, Boolean,
    BooleanType, Expression, Field, FieldType, File, Number, Parameter, Private, Section,
    StructInlineExpression, StructType, Type, U32Type, Value, Variable, Visibility, U32,
};

use std::fmt;

// Visibility

impl fmt::Display for Visibility {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Visibility::Private(_private) => write!(f, "private"),
            Visibility::Public(_public) => write!(f, "public"),
        }
    }
}

// Types

impl<'ast> fmt::Display for BooleanType<'ast> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "bool")
    }
}

impl<'ast> fmt::Display for FieldType<'ast> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "fe")
    }
}

impl<'ast> fmt::Display for U32Type<'ast> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "u32")
    }
}

impl<'ast> fmt::Display for BasicType<'ast> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BasicType::Boolean(bool) => write!(f, "{}", bool),
            BasicType::Field(field) => write!(f, "{}", field),
            BasicType::U32(u32) => write!(f, "{}", u32),
        }
    }
}

impl<'ast> fmt::Display for ArrayType<'ast> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}; {}]", self._type, self.count)
    }
}

impl<'ast> fmt::Display for StructType<'ast> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.variable)
    }
}

impl<'ast> fmt::Display for Type<'ast> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Type::Basic(ref _type) => write!(f, "{}", _type),
            Type::Array(ref _type) => write!(f, "{}", _type),
            Type::Struct(ref _type) => write!(f, "{}", _type),
        }
    }
}

// Values

impl<'ast> fmt::Display for Number<'ast> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl<'ast> fmt::Display for U32<'ast> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.number)
    }
}

impl<'ast> fmt::Display for Field<'ast> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.number)
    }
}

impl<'ast> fmt::Display for Boolean<'ast> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl<'ast> fmt::Display for Value<'ast> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Value::U32(ref value) => write!(f, "{}", value),
            Value::Field(ref value) => write!(f, "{}", value),
            Value::Boolean(ref value) => write!(f, "{}", value),
        }
    }
}
// Variables

impl<'ast> fmt::Display for Variable<'ast> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

// Expressions

impl<'ast> fmt::Display for StructInlineExpression<'ast> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {{", self.variable)?;
        for (i, member) in self.members.iter().enumerate() {
            write!(f, "{}: {}", member.variable, member.expression)?;
            if i < self.members.len() - 1 {
                write!(f, ", ")?;
            }
        }
        write!(f, "}}")
    }
}

impl<'ast> fmt::Display for ArrayInlineExpression<'ast> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[")?;
        for (i, expression) in self.expressions.iter().enumerate() {
            write!(f, "{}", expression)?;
            if i < self.expressions.len() - 1 {
                write!(f, ", ")?;
            }
        }
        write!(f, "]")
    }
}

impl<'ast> fmt::Display for ArrayInitializerExpression<'ast> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{} ; {}]", self.expression, self.count)
    }
}

impl<'ast> fmt::Display for Expression<'ast> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expression::StructInline(_struct) => write!(f, "{}", _struct),
            Expression::ArrayInline(array) => write!(f, "{}", array),
            Expression::ArrayInitializer(array) => write!(f, "{}", array),
            Expression::Value(value) => write!(f, "{}", value),
            Expression::Variable(variable) => write!(f, "{}", variable),
        }
    }
}

// Sections

impl<'ast> fmt::Display for Parameter<'ast> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}: {} {}",
            self.variable,
            self.visibility
                .as_ref()
                .unwrap_or(&Visibility::Private(Private {})), // private by default
            self._type
        )
    }
}

impl<'ast> fmt::Display for Assignment<'ast> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} = {}", self.parameter, self.expression)
    }
}

impl<'ast> fmt::Display for Section<'ast> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]\n", self.header.function_name.value)?;
        for assignment in self.assignments.iter() {
            write!(f, "\t{}\n", assignment)?;
        }
        write!(f, "")
    }
}

// File

impl<'ast> fmt::Display for File<'ast> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for value in self.sections.iter() {
            write!(f, "{}", value)?;
        }
        write!(f, "")
    }
}
