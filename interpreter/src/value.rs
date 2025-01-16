// Copyright (C) 2019-2025 Aleo Systems Inc.
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

use super::*;

use leo_ast::{IntegerType, Type};
use leo_span::Symbol;

use snarkvm::prelude::{
    Address as SvmAddressParam,
    Boolean as SvmBooleanParam,
    Cast,
    Field as SvmFieldParam,
    FromBits as _,
    Group as SvmGroupParam,
    Identifier as SvmIdentifierParam,
    Literal,
    Plaintext,
    Scalar as SvmScalarParam,
    // Signature as SvmSignatureParam,
    TestnetV0,
    ToBits,
    integers::Integer as SvmIntegerParam,
};

use indexmap::IndexMap;
use std::{
    fmt,
    hash::{Hash, Hasher},
    str::FromStr as _,
};

pub type SvmAddress = SvmAddressParam<TestnetV0>;
type SvmBoolean = SvmBooleanParam<TestnetV0>;
type SvmField = SvmFieldParam<TestnetV0>;
type SvmGroup = SvmGroupParam<TestnetV0>;
type SvmIdentifier = SvmIdentifierParam<TestnetV0>;
type SvmInteger<I> = SvmIntegerParam<TestnetV0, I>;
type SvmScalar = SvmScalarParam<TestnetV0>;
// type SvmSignature = SvmSignatureParam<TestnetV0>;

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
pub struct AsyncExecution {
    pub function: GlobalId,
    pub arguments: Vec<Value>,
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Hash)]
pub struct Future(pub Vec<AsyncExecution>);

impl fmt::Display for Future {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Future")?;
        if !self.0.is_empty() {
            write!(f, " with calls to ")?;
            let mut names = self.0.iter().map(|async_ex| async_ex.function).peekable();
            while let Some(name) = names.next() {
                write!(f, "{name}")?;
                if names.peek().is_some() {
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
    // Signature(Box<SvmSignature>),
    Tuple(Vec<Value>),
    Address(SvmAddress),
    Future(Future),
    Struct(StructContents),
    // String(()),
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
            // Signature(x) => write!(f, "{x}"),
            // String(_) => todo!(),
        }
    }
}

impl ToBits for Value {
    fn write_bits_le(&self, vec: &mut Vec<bool>) {
        use Value::*;

        let plaintext: Plaintext<TestnetV0> = match self {
            Bool(x) => Literal::Boolean(SvmBoolean::new(*x)).into(),
            U8(x) => Literal::U8(SvmInteger::new(*x)).into(),
            U16(x) => Literal::U16(SvmInteger::new(*x)).into(),
            U32(x) => Literal::U32(SvmInteger::new(*x)).into(),
            U64(x) => Literal::U64(SvmInteger::new(*x)).into(),
            U128(x) => Literal::U128(SvmInteger::new(*x)).into(),
            I8(x) => Literal::I8(SvmInteger::new(*x)).into(),
            I16(x) => Literal::I16(SvmInteger::new(*x)).into(),
            I32(x) => Literal::I32(SvmInteger::new(*x)).into(),
            I64(x) => Literal::I64(SvmInteger::new(*x)).into(),
            I128(x) => Literal::I128(SvmInteger::new(*x)).into(),
            Group(x) => Literal::Group(*x).into(),
            Field(x) => Literal::Field(*x).into(),
            Scalar(x) => Literal::Scalar(*x).into(),
            Address(x) => Literal::Address(*x).into(),
            Struct(StructContents { name: _, contents }) => {
                (contents.len() as u8).write_bits_le(vec);
                for (name, value) in contents.iter() {
                    let name_s = name.to_string();
                    let identifier = SvmIdentifier::from_str(&name_s).expect("identifier should parse");
                    identifier.size_in_bits().write_bits_le(vec);
                    identifier.write_bits_le(vec);
                    let value_bits = value.to_bits_le();
                    (value_bits.len() as u16).write_bits_le(vec);
                    vec.extend_from_slice(&value_bits);
                }
                return;
            }

            Array(array) => {
                for element in array.iter() {
                    let bits = element.to_bits_le();
                    (bits.len() as u16).write_bits_le(vec);
                    vec.extend_from_slice(&bits);
                }
                return;
            }
            _ => tc_fail!(),
        };

        plaintext.write_bits_le(vec);
    }

    fn write_bits_be(&self, _vec: &mut Vec<bool>) {
        todo!()
    }
}

impl Value {
    pub fn to_fields(&self) -> Vec<SvmField> {
        let mut bits = self.to_bits_le();
        bits.push(true);
        bits.chunks(SvmField::SIZE_IN_DATA_BITS)
            .map(|bits| SvmField::from_bits_le(bits).expect("conversion should work"))
            .collect()
    }

    pub fn gte(&self, rhs: &Self) -> Result<bool> {
        rhs.gt(self).map(|v| !v)
    }

    pub fn lte(&self, rhs: &Self) -> Result<bool> {
        rhs.lt(self).map(|v| !v)
    }

    pub fn lt(&self, rhs: &Self) -> Result<bool> {
        use Value::*;
        Ok(match (self, rhs) {
            (U8(x), U8(y)) => x < y,
            (U16(x), U16(y)) => x < y,
            (U32(x), U32(y)) => x < y,
            (U64(x), U64(y)) => x < y,
            (U128(x), U128(y)) => x < y,
            (I8(x), I8(y)) => x < y,
            (I16(x), I16(y)) => x < y,
            (I32(x), I32(y)) => x < y,
            (I64(x), I64(y)) => x < y,
            (I128(x), I128(y)) => x < y,
            (Field(x), Field(y)) => x < y,
            (a, b) => halt_no_span!("Type failure: {a} < {b}"),
        })
    }

    pub fn gt(&self, rhs: &Self) -> Result<bool> {
        use Value::*;
        Ok(match (self, rhs) {
            (U8(x), U8(y)) => x > y,
            (U16(x), U16(y)) => x > y,
            (U32(x), U32(y)) => x > y,
            (U64(x), U64(y)) => x > y,
            (U128(x), U128(y)) => x > y,
            (I8(x), I8(y)) => x > y,
            (I16(x), I16(y)) => x > y,
            (I32(x), I32(y)) => x > y,
            (I64(x), I64(y)) => x > y,
            (I128(x), I128(y)) => x > y,
            (Field(x), Field(y)) => x > y,
            (a, b) => halt_no_span!("Type failure: {a} > {b}"),
        })
    }

    pub fn neq(&self, rhs: &Self) -> Result<bool> {
        self.eq(rhs).map(|v| !v)
    }

    /// Are the values equal, according to SnarkVM?
    ///
    /// We use this rather than the Eq trait so we can
    /// fail when comparing values of different types,
    /// rather than just returning false.
    pub fn eq(&self, rhs: &Self) -> Result<bool> {
        use Value::*;
        Ok(match (self, rhs) {
            (Unit, Unit) => true,
            (Bool(x), Bool(y)) => x == y,
            (U8(x), U8(y)) => x == y,
            (U16(x), U16(y)) => x == y,
            (U32(x), U32(y)) => x == y,
            (U64(x), U64(y)) => x == y,
            (U128(x), U128(y)) => x == y,
            (I8(x), I8(y)) => x == y,
            (I16(x), I16(y)) => x == y,
            (I32(x), I32(y)) => x == y,
            (I64(x), I64(y)) => x == y,
            (I128(x), I128(y)) => x == y,
            (Field(x), Field(y)) => x == y,
            (Group(x), Group(y)) => x == y,
            (Array(x), Array(y)) => {
                if x.len() != y.len() {
                    return Ok(false);
                }
                for (lhs, rhs) in x.iter().zip(y.iter()) {
                    match lhs.eq(rhs) {
                        Ok(true) => {}
                        Ok(false) => return Ok(false),
                        Err(e) => return Err(e),
                    }
                }
                true
            }
            (a, b) => halt_no_span!("Type failure: {a} == {b}"),
        })
    }

    pub fn inc_wrapping(&self) -> Self {
        match self {
            Value::U8(x) => Value::U8(x.wrapping_add(1)),
            Value::U16(x) => Value::U16(x.wrapping_add(1)),
            Value::U32(x) => Value::U32(x.wrapping_add(1)),
            Value::U64(x) => Value::U64(x.wrapping_add(1)),
            Value::U128(x) => Value::U128(x.wrapping_add(1)),
            Value::I8(x) => Value::I8(x.wrapping_add(1)),
            Value::I16(x) => Value::I16(x.wrapping_add(1)),
            Value::I32(x) => Value::I32(x.wrapping_add(1)),
            Value::I64(x) => Value::I64(x.wrapping_add(1)),
            Value::I128(x) => Value::I128(x.wrapping_add(1)),
            _ => tc_fail!(),
        }
    }

    /// Return the group generator.
    pub fn generator() -> Self {
        Value::Group(SvmGroup::generator())
    }

    /// Doesn't correspond to Aleo's shl, because it
    /// does not fail when set bits are shifted out.
    pub fn simple_shl(&self, shift: u32) -> Self {
        match self {
            Value::U8(x) => Value::U8(x << shift),
            Value::U16(x) => Value::U16(x << shift),
            Value::U32(x) => Value::U32(x << shift),
            Value::U64(x) => Value::U64(x << shift),
            Value::U128(x) => Value::U128(x << shift),
            Value::I8(x) => Value::I8(x << shift),
            Value::I16(x) => Value::I16(x << shift),
            Value::I32(x) => Value::I32(x << shift),
            Value::I64(x) => Value::I64(x << shift),
            Value::I128(x) => Value::I128(x << shift),
            _ => tc_fail!(),
        }
    }

    pub fn simple_shr(&self, shift: u32) -> Self {
        match self {
            Value::U8(x) => Value::U8(x >> shift),
            Value::U16(x) => Value::U16(x >> shift),
            Value::U32(x) => Value::U32(x >> shift),
            Value::U64(x) => Value::U64(x >> shift),
            Value::U128(x) => Value::U128(x >> shift),
            Value::I8(x) => Value::I8(x >> shift),
            Value::I16(x) => Value::I16(x >> shift),
            Value::I32(x) => Value::I32(x >> shift),
            Value::I64(x) => Value::I64(x >> shift),
            Value::I128(x) => Value::I128(x >> shift),
            _ => tc_fail!(),
        }
    }

    /// Convert to the given type if possible under Aleo casting rules.
    pub fn cast(&self, cast_type: &Type) -> Option<Value> {
        match self {
            Value::Bool(b) => really_cast(SvmBoolean::new(*b), cast_type),
            Value::U8(x) => really_cast(SvmInteger::new(*x), cast_type),
            Value::U16(x) => really_cast(SvmInteger::new(*x), cast_type),
            Value::U32(x) => really_cast(SvmInteger::new(*x), cast_type),
            Value::U64(x) => really_cast(SvmInteger::new(*x), cast_type),
            Value::U128(x) => really_cast(SvmInteger::new(*x), cast_type),
            Value::I8(x) => really_cast(SvmInteger::new(*x), cast_type),
            Value::I16(x) => really_cast(SvmInteger::new(*x), cast_type),
            Value::I32(x) => really_cast(SvmInteger::new(*x), cast_type),
            Value::I64(x) => really_cast(SvmInteger::new(*x), cast_type),
            Value::I128(x) => really_cast(SvmInteger::new(*x), cast_type),
            Value::Group(g) => really_cast(g.to_x_coordinate(), cast_type),
            Value::Field(f) => really_cast(*f, cast_type),
            Value::Scalar(s) => really_cast(*s, cast_type),
            Value::Address(a) => really_cast(a.to_group().to_x_coordinate(), cast_type),
            _ => None,
        }
    }
}

fn really_cast<C>(c: C, cast_type: &Type) -> Option<Value>
where
    C: Cast<SvmAddress>
        + Cast<SvmField>
        + Cast<SvmAddress>
        + Cast<SvmGroup>
        + Cast<SvmBoolean>
        + Cast<SvmScalar>
        + Cast<SvmInteger<u8>>
        + Cast<SvmInteger<u16>>
        + Cast<SvmInteger<u32>>
        + Cast<SvmInteger<u64>>
        + Cast<SvmInteger<u128>>
        + Cast<SvmInteger<i8>>
        + Cast<SvmInteger<i16>>
        + Cast<SvmInteger<i32>>
        + Cast<SvmInteger<i64>>
        + Cast<SvmInteger<i128>>,
{
    use Type::*;

    let value = match cast_type {
        Address => Value::Address(c.cast().ok()?),
        Boolean => Value::Bool({
            let b: SvmBoolean = c.cast().ok()?;
            *b
        }),
        Field => Value::Field(c.cast().ok()?),
        Group => Value::Group(c.cast().ok()?),
        Integer(IntegerType::U8) => Value::U8({
            let i: SvmInteger<u8> = c.cast().ok()?;
            *i
        }),
        Integer(IntegerType::U16) => Value::U16({
            let i: SvmInteger<u16> = c.cast().ok()?;
            *i
        }),
        Integer(IntegerType::U32) => Value::U32({
            let i: SvmInteger<u32> = c.cast().ok()?;
            *i
        }),
        Integer(IntegerType::U64) => Value::U64({
            let i: SvmInteger<u64> = c.cast().ok()?;
            *i
        }),
        Integer(IntegerType::U128) => Value::U128({
            let i: SvmInteger<u128> = c.cast().ok()?;
            *i
        }),
        Integer(IntegerType::I8) => Value::I8({
            let i: SvmInteger<i8> = c.cast().ok()?;
            *i
        }),
        Integer(IntegerType::I16) => Value::I16({
            let i: SvmInteger<i16> = c.cast().ok()?;
            *i
        }),
        Integer(IntegerType::I32) => Value::I32({
            let i: SvmInteger<i32> = c.cast().ok()?;
            *i
        }),
        Integer(IntegerType::I64) => Value::I64({
            let i: SvmInteger<i64> = c.cast().ok()?;
            *i
        }),
        Integer(IntegerType::I128) => Value::I128({
            let i: SvmInteger<i128> = c.cast().ok()?;
            *i
        }),
        Scalar => Value::Scalar(c.cast().ok()?),

        _ => tc_fail!(),
    };
    Some(value)
}
