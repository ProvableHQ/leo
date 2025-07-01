// Copyright (C) 2019-2025 Provable Inc.
// This file is part of the Leo library.

// The Leo library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The Leo library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the Leo library. If not, see <https://www.gnu.org/licenses/>.

use leo_span::Symbol;
use snarkvm::prelude::{
    Address as SvmAddressParam,
    Boolean as SvmBooleanParam,
    Field as SvmFieldParam,
    Group as SvmGroupParam,
    Identifier as SvmIdentifierParam,
    Scalar as SvmScalarParam,
    // Signature as SvmSignatureParam,
    TestnetV0,
    integers::Integer as SvmIntegerParam,
};

use std::{
    collections::BTreeMap,
    fmt,
    hash::{Hash, Hasher},
};

use indexmap::IndexMap;

pub type SvmAddress = SvmAddressParam<TestnetV0>;
pub type SvmBoolean = SvmBooleanParam<TestnetV0>;
pub type SvmField = SvmFieldParam<TestnetV0>;
pub type SvmGroup = SvmGroupParam<TestnetV0>;
pub type SvmIdentifier = SvmIdentifierParam<TestnetV0>;
pub type SvmInteger<I> = SvmIntegerParam<TestnetV0, I>;
pub type SvmScalar = SvmScalarParam<TestnetV0>;

/// Global values - such as mappings, functions, etc -
/// are identified by program and name.
#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub struct GlobalId {
    pub program: Symbol,
    pub name: Symbol,
}

impl fmt::Display for GlobalId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.aleo/{}", self.program, self.name)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StructContents {
    pub name: Symbol,
    pub contents: IndexMap<Symbol, Value>,
}

impl Hash for StructContents {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
        for (_symbol, value) in self.contents.iter() {
            value.hash(state);
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum AsyncExecution {
    AsyncFunctionCall {
        function: GlobalId,
        arguments: Vec<Value>,
    },
    AsyncBlock {
        containing_function: GlobalId, // The function that contains the async block.
        block: crate::NodeID,
        names: BTreeMap<Symbol, Value>, // Use a `BTreeMap` here because `HashMap` does not implement `Hash`.
    },
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Hash)]
pub struct Future(pub Vec<AsyncExecution>);

impl fmt::Display for Future {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Future")?;
        if !self.0.is_empty() {
            write!(f, " with calls to ")?;

            let mut entries = self.0.iter().peekable();

            while let Some(async_ex) = entries.next() {
                match async_ex {
                    AsyncExecution::AsyncFunctionCall { function, .. } => {
                        write!(f, "{function}")?;
                    }
                    AsyncExecution::AsyncBlock { containing_function, .. } => {
                        write!(f, "{containing_function}/<async block>")?;
                    }
                }

                if entries.peek().is_some() {
                    write!(f, ", ")?;
                }
            }
        }
        Ok(())
    }
}

/// A Leo value of any type.
///
/// Mappings and functions aren't considered values.
#[derive(Clone, Debug, Default, Eq, PartialEq, Hash)]
pub enum Value {
    #[default]
    Unit,
    Bool(bool),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    U128(u128),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    I128(i128),
    Group(SvmGroup),
    Field(SvmField),
    Scalar(SvmScalar),
    Array(Vec<Value>),
    Repeat(Box<Value>, Box<Value>),
    // Signature(Box<SvmSignature>),
    Tuple(Vec<Value>),
    Address(SvmAddress),
    Future(Future),
    Struct(StructContents),
    Unsuffixed(String),
    // String(()),
}

impl Value {
    /// Gets the type of a `Value` but only if it is an integer, a field, a group, or a scalar.
    /// Return `None` otherwise. These are the only types that an unsuffixed literal can have.
    pub fn get_numeric_type(&self) -> Option<crate::Type> {
        use crate::{IntegerType::*, Type::*};
        match self {
            Value::U8(_) => Some(Integer(U8)),
            Value::U16(_) => Some(Integer(U16)),
            Value::U32(_) => Some(Integer(U32)),
            Value::U64(_) => Some(Integer(U64)),
            Value::U128(_) => Some(Integer(U128)),
            Value::I8(_) => Some(Integer(I8)),
            Value::I16(_) => Some(Integer(I16)),
            Value::I32(_) => Some(Integer(I32)),
            Value::I64(_) => Some(Integer(I64)),
            Value::I128(_) => Some(Integer(I128)),
            Value::Field(_) => Some(Field),
            Value::Group(_) => Some(Group),
            Value::Scalar(_) => Some(Scalar),
            _ => None,
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Value::*;
        match self {
            Unit => write!(f, "()"),

            Bool(x) => write!(f, "{x}"),
            U8(x) => write!(f, "{x}u8"),
            U16(x) => write!(f, "{x}u16"),
            U32(x) => write!(f, "{x}u32"),
            U64(x) => write!(f, "{x}u64"),
            U128(x) => write!(f, "{x}u128"),
            I8(x) => write!(f, "{x}i8"),
            I16(x) => write!(f, "{x}i16"),
            I32(x) => write!(f, "{x}i32"),
            I64(x) => write!(f, "{x}i64"),
            I128(x) => write!(f, "{x}i128"),
            Group(x) => write!(f, "{x}"),
            Field(x) => write!(f, "{x}"),
            Scalar(x) => write!(f, "{x}"),
            Array(x) => {
                write!(f, "[")?;
                let mut iter = x.iter().peekable();
                while let Some(value) = iter.next() {
                    write!(f, "{value}")?;
                    if iter.peek().is_some() {
                        write!(f, ", ")?;
                    }
                }
                write!(f, "]")
            }
            Repeat(expr, count) => write!(f, "[{expr}; {count}]"),
            Struct(StructContents { name, contents }) => {
                write!(f, "{name} {{")?;
                let mut iter = contents.iter().peekable();
                while let Some((member_name, value)) = iter.next() {
                    write!(f, "{member_name}: {value}")?;
                    if iter.peek().is_some() {
                        write!(f, ", ")?;
                    }
                }
                write!(f, "}}")
            }
            Tuple(x) => {
                write!(f, "(")?;
                let mut iter = x.iter().peekable();
                while let Some(value) = iter.next() {
                    write!(f, "{value}")?;
                    if iter.peek().is_some() {
                        write!(f, ", ")?;
                    }
                }
                write!(f, ")")
            }
            Address(x) => write!(f, "{x}"),
            Future(future) => write!(f, "{future}"),
            Unsuffixed(s) => write!(f, "{s}"),
            // Signature(x) => write!(f, "{x}"),
            // String(_) => todo!(),
        }
    }
}
