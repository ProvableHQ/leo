// Copyright (C) 2019-2026 Provable Inc.
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

use std::{
    collections::BTreeMap,
    fmt,
    hash::{Hash, Hasher},
    str::FromStr,
};

use itertools::Itertools as _;

use crate::Location;

use snarkvm::prelude::{
    Access,
    Address as SvmAddress,
    Argument,
    Boolean as SvmBoolean,
    Entry,
    Field as SvmField,
    Future as FutureParam,
    Group as SvmGroup,
    LiteralType,
    Owner,
    ProgramID as ProgramIDParam,
    Record,
    Scalar as SvmScalar,
};
pub(crate) use snarkvm::prelude::{
    Identifier as SvmIdentifierParam,
    Literal as SvmLiteralParam,
    Plaintext,
    Signature as SvmSignature,
    TestnetV0,
    Value as SvmValueParam,
};

use leo_errors::Result;
use leo_span::{Span, Symbol};

use crate::{Expression, IntegerType, NodeBuilder, Type};

pub(crate) type CurrentNetwork = TestnetV0;

pub(crate) type SvmValue = SvmValueParam<CurrentNetwork>;
pub(crate) type ProgramID = ProgramIDParam<CurrentNetwork>;
pub(crate) type SvmPlaintext = Plaintext<CurrentNetwork>;
pub(crate) type SvmLiteral = SvmLiteralParam<CurrentNetwork>;
pub(crate) type SvmIdentifier = SvmIdentifierParam<CurrentNetwork>;
pub(crate) type Group = SvmGroup<CurrentNetwork>;
pub(crate) type Field = SvmField<CurrentNetwork>;
pub(crate) type Scalar = SvmScalar<CurrentNetwork>;
pub(crate) type Address = SvmAddress<CurrentNetwork>;
pub(crate) type Boolean = SvmBoolean<CurrentNetwork>;
pub(crate) type Future = FutureParam<CurrentNetwork>;
pub(crate) type Signature = SvmSignature<CurrentNetwork>;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CompositeContents {
    pub path: Vec<Symbol>,
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Hash)]
pub struct Value {
    pub id: Option<Location>,
    pub(crate) contents: ValueVariants,
}

#[derive(Clone, Default, Debug, Eq, PartialEq)]
// SnarkVM's Value is large, but that's okay.
#[allow(clippy::large_enum_variant)]
pub(crate) enum ValueVariants {
    #[default]
    Unit,
    Svm(SvmValue),
    Tuple(Vec<Value>),
    Unsuffixed(String),
    Future(Vec<AsyncExecution>),
    String(String),
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum AsyncExecution {
    AsyncFunctionCall {
        function: Location,
        arguments: Vec<Value>,
    },
    AsyncBlock {
        containing_function: Location, // The function that contains the async block.
        block: crate::NodeID,
        names: BTreeMap<Vec<Symbol>, Value>, // Use a `BTreeMap` here because `HashMap` does not implement `Hash`.
    },
}

impl fmt::Display for AsyncExecution {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, " async call to ")?;

        match self {
            AsyncExecution::AsyncFunctionCall { function, .. } => {
                write!(f, "{function}")
            }
            AsyncExecution::AsyncBlock { containing_function, .. } => {
                write!(f, "{containing_function}/<async block>")
            }
        }
    }
}

fn hash_plaintext<H>(pt: &SvmPlaintext, state: &mut H)
where
    H: Hasher,
{
    match pt {
        Plaintext::Literal(literal, ..) => {
            6u8.hash(state);
            literal.hash(state);
        }
        Plaintext::Struct(index_map, ..) => {
            7u8.hash(state);
            index_map.len().hash(state);
            // The correctness of this hash depends on the members being
            // in the same order for each type.
            index_map.iter().for_each(|(key, value)| {
                key.hash(state);
                hash_plaintext(value, state);
            });
        }
        Plaintext::Array(vec, ..) => {
            8u8.hash(state);
            vec.len().hash(state);
            vec.iter().for_each(|pt0| hash_plaintext(pt0, state));
        }
    }
}

impl Hash for ValueVariants {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        use ValueVariants::*;

        match self {
            Unit => 0u8.hash(state),
            Tuple(vec) => {
                1u8.hash(state);
                vec.len().hash(state);
            }
            Unsuffixed(s) => {
                2u8.hash(state);
                s.hash(state);
            }
            Future(async_executions) => {
                3u8.hash(state);
                async_executions.hash(state);
            }
            Svm(value) => match value {
                SvmValueParam::Record(record) => {
                    4u8.hash(state);
                    (**record.version()).hash(state);
                    record.nonce().hash(state);
                    // NOTE - we don't actually hash the data or owner. Thus this is
                    // a terrible hash function with many collisions.
                    // This shouldn't matter as the only place where we hash this is in
                    // keys for mappings, which cannot be records.
                }
                SvmValueParam::Future(future) => {
                    5u8.hash(state);
                    future.program_id().hash(state);
                    future.function_name().hash(state);
                    // Ditto - we don't hash the arguments.
                }
                SvmValueParam::Plaintext(plaintext) => {
                    hash_plaintext(plaintext, state);
                }
            },
            String(s) => {
                9u8.hash(state);
                s.hash(state);
            }
        }
    }
}

impl From<ValueVariants> for Value {
    fn from(contents: ValueVariants) -> Self {
        Value { id: None, contents }
    }
}

pub trait TryAsRef<T>
where
    T: ?Sized,
{
    fn try_as_ref(&self) -> Option<&T>;
}

macro_rules! impl_from_integer {
    ($($int_type: ident $variant: ident);* $(;)?) => {
        $(
            impl From<$int_type> for Value {
                fn from(value: $int_type) -> Self {
                    ValueVariants::Svm(
                        SvmValueParam::Plaintext(
                            Plaintext::Literal(
                                snarkvm::prelude::$variant::new(value).into(),
                                Default::default(),
                            )
                        )
                    ).into()
                }
            }

            impl From<snarkvm::prelude::$variant<CurrentNetwork>> for Value {
                fn from(x: snarkvm::prelude::$variant<CurrentNetwork>) -> Self {
                    ValueVariants::Svm(SvmValueParam::Plaintext(
                        Plaintext::Literal(
                            SvmLiteralParam::$variant(x),
                            Default::default()
                        )
                    )).into()
                }
            }

            impl TryFrom<Value> for $int_type {
                type Error = ();

                fn try_from(value: Value) -> Result<$int_type, Self::Error> {
                    match value.contents {
                        ValueVariants::Svm(SvmValueParam::Plaintext(Plaintext::Literal(SvmLiteralParam::$variant(x), _))) => Ok(*x),
                        _ => Err(()),
                    }
                }
            }

            impl TryAsRef<$int_type> for Value {
                fn try_as_ref(&self) -> Option<&$int_type> {
                    match &self.contents {
                        ValueVariants::Svm(SvmValueParam::Plaintext(Plaintext::Literal(SvmLiteralParam::$variant(x), _))) => Some(&*x),
                        _ => None,
                    }
                }
            }
        )*
    };
}

impl_from_integer! {
    u8 U8;
    u16 U16;
    u32 U32;
    u64 U64;
    u128 U128;
    i8 I8;
    i16 I16;
    i32 I32;
    i64 I64;
    i128 I128;
    bool Boolean;
}

macro_rules! impl_from_literal {
    ($($type_: ident);* $(;)?) => {
        $(
            impl From<snarkvm::prelude::$type_<CurrentNetwork>> for Value {
                fn from(x: snarkvm::prelude::$type_<CurrentNetwork>) -> Self {
                    let literal: SvmLiteral = x.into();
                    ValueVariants::Svm(SvmValueParam::Plaintext(Plaintext::Literal(literal, Default::default()))).into()
                }
            }

            impl TryFrom<Value> for snarkvm::prelude::$type_<CurrentNetwork> {
                type Error = ();

                fn try_from(x: Value) -> Result<Self, Self::Error> {
                    if let ValueVariants::Svm(SvmValueParam::Plaintext(Plaintext::Literal(SvmLiteralParam::$type_(val), ..))) = x.contents {
                        Ok(val)
                    } else {
                        Err(())
                    }
                }
            }
        )*
    };
}

impl_from_literal! {
    Field; Group; Scalar; Address;
}

impl TryFrom<Value> for snarkvm::prelude::Signature<CurrentNetwork> {
    type Error = ();

    fn try_from(x: Value) -> Result<Self, Self::Error> {
        if let ValueVariants::Svm(SvmValueParam::Plaintext(Plaintext::Literal(SvmLiteralParam::Signature(val), ..))) =
            x.contents
        {
            Ok(*val)
        } else {
            Err(())
        }
    }
}

impl From<Future> for Value {
    fn from(value: Future) -> Self {
        SvmValueParam::Future(value).into()
    }
}

impl TryAsRef<SvmLiteral> for Value {
    fn try_as_ref(&self) -> Option<&SvmLiteral> {
        match &self.contents {
            ValueVariants::Svm(SvmValueParam::Plaintext(Plaintext::Literal(literal, ..))) => Some(literal),
            _ => None,
        }
    }
}

impl From<SvmLiteral> for Value {
    fn from(x: SvmLiteral) -> Self {
        ValueVariants::Svm(SvmValueParam::Plaintext(Plaintext::Literal(x, Default::default()))).into()
    }
}

impl TryFrom<Value> for SvmValue {
    type Error = ();

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        if let ValueVariants::Svm(x) = value.contents { Ok(x) } else { Err(()) }
    }
}

impl TryFrom<Value> for SvmPlaintext {
    type Error = ();

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        if let ValueVariants::Svm(SvmValueParam::Plaintext(x)) = value.contents { Ok(x) } else { Err(()) }
    }
}

impl TryAsRef<SvmPlaintext> for Value {
    fn try_as_ref(&self) -> Option<&SvmPlaintext> {
        if let ValueVariants::Svm(SvmValueParam::Plaintext(x)) = &self.contents { Some(x) } else { None }
    }
}

impl From<SvmPlaintext> for Value {
    fn from(x: SvmPlaintext) -> Self {
        ValueVariants::Svm(SvmValueParam::Plaintext(x)).into()
    }
}

impl From<SvmValue> for Value {
    fn from(x: SvmValue) -> Self {
        ValueVariants::Svm(x).into()
    }
}

impl From<Vec<AsyncExecution>> for Value {
    fn from(x: Vec<AsyncExecution>) -> Self {
        ValueVariants::Future(x).into()
    }
}

impl TryFrom<Value> for String {
    type Error = ();

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        if let ValueVariants::String(s) = value.contents { Ok(s) } else { Err(()) }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.contents {
            ValueVariants::Unit => "()".fmt(f),
            ValueVariants::Svm(SvmValueParam::Plaintext(Plaintext::Struct(struct_, ..))) => {
                if let Some(id) = &self.id {
                    write!(f, "{id} ")?
                }
                let mut debug_map = f.debug_map();
                for (key, value) in struct_ {
                    debug_map.entry(key, value);
                }
                debug_map.finish()
            }
            ValueVariants::Svm(value) => value.fmt(f),
            ValueVariants::Tuple(vec) => write!(f, "({})", vec.iter().format(", ")),
            ValueVariants::Unsuffixed(s) => s.fmt(f),
            ValueVariants::Future(_async_executions) => "Future".fmt(f),
            ValueVariants::String(s) => write!(f, "\"{s}\""),
        }
    }
}

impl Value {
    pub fn lt(&self, rhs: &Self) -> Option<bool> {
        use SvmLiteralParam::*;

        let literal0: &SvmLiteral = self.try_as_ref()?;
        let literal1: &SvmLiteral = rhs.try_as_ref()?;

        let value = match (literal0, literal1) {
            (I8(val0), I8(val1)) => val0 < val1,
            (I16(val0), I16(val1)) => val0 < val1,
            (I32(val0), I32(val1)) => val0 < val1,
            (I64(val0), I64(val1)) => val0 < val1,
            (I128(val0), I128(val1)) => val0 < val1,
            (U8(val0), U8(val1)) => val0 < val1,
            (U16(val0), U16(val1)) => val0 < val1,
            (U32(val0), U32(val1)) => val0 < val1,
            (U64(val0), U64(val1)) => val0 < val1,
            (U128(val0), U128(val1)) => val0 < val1,
            (Field(val0), Field(val1)) => val0 < val1,
            _ => return None,
        };

        Some(value)
    }

    pub fn lte(&self, rhs: &Self) -> Option<bool> {
        use SvmLiteralParam::*;

        let literal0: &SvmLiteral = self.try_as_ref()?;
        let literal1: &SvmLiteral = rhs.try_as_ref()?;

        let value = match (literal0, literal1) {
            (I8(val0), I8(val1)) => val0 <= val1,
            (I16(val0), I16(val1)) => val0 <= val1,
            (I32(val0), I32(val1)) => val0 <= val1,
            (I64(val0), I64(val1)) => val0 <= val1,
            (I128(val0), I128(val1)) => val0 <= val1,
            (U8(val0), U8(val1)) => val0 <= val1,
            (U16(val0), U16(val1)) => val0 <= val1,
            (U32(val0), U32(val1)) => val0 <= val1,
            (U64(val0), U64(val1)) => val0 <= val1,
            (U128(val0), U128(val1)) => val0 <= val1,
            (Field(val0), Field(val1)) => val0 <= val1,
            _ => return None,
        };

        Some(value)
    }

    pub fn gt(&self, rhs: &Self) -> Option<bool> {
        rhs.lt(self)
    }

    pub fn gte(&self, rhs: &Self) -> Option<bool> {
        rhs.lte(self)
    }

    pub fn inc_wrapping(&self) -> Option<Self> {
        let literal: &SvmLiteral = self.try_as_ref()?;

        let value = match literal {
            SvmLiteralParam::U8(x) => x.wrapping_add(1).into(),
            SvmLiteralParam::U16(x) => x.wrapping_add(1).into(),
            SvmLiteralParam::U32(x) => x.wrapping_add(1).into(),
            SvmLiteralParam::U64(x) => x.wrapping_add(1).into(),
            SvmLiteralParam::U128(x) => x.wrapping_add(1).into(),
            SvmLiteralParam::I8(x) => x.wrapping_add(1).into(),
            SvmLiteralParam::I16(x) => x.wrapping_add(1).into(),
            SvmLiteralParam::I32(x) => x.wrapping_add(1).into(),
            SvmLiteralParam::I64(x) => x.wrapping_add(1).into(),
            SvmLiteralParam::I128(x) => x.wrapping_add(1).into(),
            _ => return None,
        };
        Some(value)
    }

    pub fn add(&self, i: usize) -> Option<Self> {
        let literal: &SvmLiteral = self.try_as_ref()?;

        macro_rules! sum {
            ($ty: ident, $int: ident) => {{
                let rhs: $ty = i.try_into().ok()?;
                (**$int + rhs).into()
            }};
        }

        let value = match literal {
            SvmLiteralParam::I8(int) => sum!(i8, int),
            SvmLiteralParam::I16(int) => sum!(i16, int),
            SvmLiteralParam::I32(int) => sum!(i32, int),
            SvmLiteralParam::I64(int) => sum!(i64, int),
            SvmLiteralParam::I128(int) => sum!(i128, int),
            SvmLiteralParam::U8(int) => sum!(u8, int),
            SvmLiteralParam::U16(int) => sum!(u16, int),
            SvmLiteralParam::U32(int) => sum!(u32, int),
            SvmLiteralParam::U64(int) => sum!(u64, int),
            SvmLiteralParam::U128(int) => sum!(u128, int),
            SvmLiteralParam::Address(..)
            | SvmLiteralParam::Boolean(..)
            | SvmLiteralParam::Field(..)
            | SvmLiteralParam::Group(..)
            | SvmLiteralParam::Scalar(..)
            | SvmLiteralParam::Signature(..)
            | SvmLiteralParam::String(..) => return None,
        };
        Some(value)
    }

    pub fn cast(&self, ty: &Type) -> Option<Self> {
        let literal_ty = match ty {
            Type::Address => LiteralType::Address,
            Type::Boolean => LiteralType::Boolean,
            Type::Field => LiteralType::Field,
            Type::Group => LiteralType::Group,
            Type::Integer(IntegerType::U8) => LiteralType::U8,
            Type::Integer(IntegerType::U16) => LiteralType::U16,
            Type::Integer(IntegerType::U32) => LiteralType::U32,
            Type::Integer(IntegerType::U64) => LiteralType::U64,
            Type::Integer(IntegerType::U128) => LiteralType::U128,
            Type::Integer(IntegerType::I8) => LiteralType::I8,
            Type::Integer(IntegerType::I16) => LiteralType::I16,
            Type::Integer(IntegerType::I32) => LiteralType::I32,
            Type::Integer(IntegerType::I64) => LiteralType::I64,
            Type::Integer(IntegerType::I128) => LiteralType::I128,
            Type::Scalar => LiteralType::Scalar,
            Type::Signature => LiteralType::Signature,
            Type::String => LiteralType::String,
            _ => return None,
        };

        let literal: &SvmLiteral = self.try_as_ref()?;

        Some(literal.cast(literal_ty).ok()?.into())
    }

    pub fn cast_lossy(&self, ty: &LiteralType) -> Option<Self> {
        let ValueVariants::Svm(SvmValueParam::Plaintext(Plaintext::Literal(literal, ..))) = &self.contents else {
            return None;
        };
        literal.cast_lossy(*ty).ok().map(|lit| lit.into())
    }

    /// Return the group generator.
    pub fn generator() -> Self {
        Group::generator().into()
    }

    pub fn as_u32(&self) -> Option<u32> {
        let literal = self.try_as_ref()?;
        let value = match literal {
            SvmLiteralParam::U8(x) => **x as u32,
            SvmLiteralParam::U16(x) => **x as u32,
            SvmLiteralParam::U32(x) => **x,
            SvmLiteralParam::U64(x) => (**x).try_into().ok()?,
            SvmLiteralParam::U128(x) => (**x).try_into().ok()?,
            SvmLiteralParam::I8(x) => (**x).try_into().ok()?,
            SvmLiteralParam::I16(x) => (**x).try_into().ok()?,
            SvmLiteralParam::I32(x) => (**x).try_into().ok()?,
            SvmLiteralParam::I64(x) => (**x).try_into().ok()?,
            SvmLiteralParam::I128(x) => (**x).try_into().ok()?,
            _ => return None,
        };
        Some(value)
    }

    pub fn as_i128(&self) -> Option<i128> {
        let literal = self.try_as_ref()?;
        let value = match literal {
            SvmLiteralParam::U8(x) => **x as i128,
            SvmLiteralParam::U16(x) => **x as i128,
            SvmLiteralParam::U32(x) => **x as i128,
            SvmLiteralParam::U64(x) => **x as i128,
            SvmLiteralParam::U128(x) => (**x).try_into().ok()?,
            SvmLiteralParam::I8(x) => **x as i128,
            SvmLiteralParam::I16(x) => **x as i128,
            SvmLiteralParam::I32(x) => **x as i128,
            SvmLiteralParam::I64(x) => **x as i128,
            SvmLiteralParam::I128(x) => **x,
            _ => return None,
        };
        Some(value)
    }

    pub fn array_index(&self, i: usize) -> Option<Self> {
        let plaintext: &SvmPlaintext = self.try_as_ref()?;
        let Plaintext::Array(array, ..) = plaintext else {
            return None;
        };

        Some(array[i].clone().into())
    }

    pub fn array_index_set(&mut self, i: usize, value: Self) -> Option<()> {
        let plaintext_rhs: SvmPlaintext = value.try_into().ok()?;

        let ValueVariants::Svm(SvmValue::Plaintext(Plaintext::Array(arr, once_cell))) = &mut self.contents else {
            return None;
        };
        *once_cell = Default::default();

        *arr.get_mut(i)? = plaintext_rhs;

        Some(())
    }

    pub fn tuple_len(&self) -> Option<usize> {
        let ValueVariants::Tuple(tuple) = &self.contents else {
            return None;
        };

        Some(tuple.len())
    }

    pub fn tuple_index(&self, i: usize) -> Option<Self> {
        let ValueVariants::Tuple(tuple) = &self.contents else {
            return None;
        };

        Some(tuple[i].clone())
    }

    pub fn tuple_index_set(&mut self, i: usize, value: Self) -> Option<()> {
        let ValueVariants::Tuple(tuple) = &mut self.contents else {
            return None;
        };

        *tuple.get_mut(i)? = value;

        Some(())
    }

    pub fn as_future(&self) -> Option<&[AsyncExecution]> {
        if let ValueVariants::Future(asyncs) = &self.contents { Some(asyncs) } else { None }
    }

    pub fn accesses(&self, accesses: impl IntoIterator<Item = Access<CurrentNetwork>>) -> Option<Value> {
        self.accesses_impl(&mut accesses.into_iter())
    }

    fn accesses_impl(&self, accesses: &mut dyn Iterator<Item = Access<CurrentNetwork>>) -> Option<Value> {
        let ValueVariants::Svm(SvmValueParam::Plaintext(current)) = &self.contents else {
            return None;
        };

        let mut current = current;

        for access in accesses {
            current = match (access, current) {
                (Access::Member(identifier), Plaintext::Struct(struct_, ..)) => struct_.get(&identifier)?,
                (Access::Index(integer), Plaintext::Array(array, ..)) => array.get(*integer as usize)?,
                _ => return None,
            };
        }

        Some(current.clone().into())
    }

    pub fn member_access(&self, member: Symbol) -> Option<Self> {
        match &self.contents {
            ValueVariants::Svm(SvmValueParam::Plaintext(Plaintext::Struct(map, ..))) => {
                let identifier: SvmIdentifier =
                    member.to_string().parse().expect("Member name should be valid identifier");
                Some(map.get(&identifier)?.clone().into())
            }
            _ => None,
        }
    }

    pub fn member_set(&mut self, member: Symbol, value: Value) -> Option<()> {
        let plaintext: SvmPlaintext = value.try_into().ok()?;
        match &mut self.contents {
            ValueVariants::Svm(SvmValueParam::Plaintext(Plaintext::Struct(map, once_cell))) => {
                *once_cell = Default::default();
                let identifier: SvmIdentifier = member.to_string().parse().ok()?;
                *map.get_mut(&identifier)? = plaintext;
                Some(())
            }

            _ => None,
        }
    }

    pub fn make_future(
        program_name: Symbol,
        function: Symbol,
        arguments: impl IntoIterator<Item = Value>,
    ) -> Option<Value> {
        Self::make_future_impl(program_name, function, &mut arguments.into_iter())
    }

    fn make_future_impl(
        program_name: Symbol,
        function: Symbol,
        arguments: &mut dyn Iterator<Item = Value>,
    ) -> Option<Value> {
        let program = ProgramID::try_from(format!("{program_name}.aleo")).ok()?;

        let function_identifier: SvmIdentifier = function.to_string().parse().ok()?;

        let arguments_vec = arguments
            .map(|value| match value.contents {
                ValueVariants::Svm(SvmValueParam::Plaintext(plaintext)) => Some(Argument::Plaintext(plaintext)),
                ValueVariants::Svm(SvmValueParam::Future(future)) => Some(Argument::Future(future)),
                _ => None,
            })
            .collect::<Option<Vec<_>>>()?;

        Some(Future::new(program, function_identifier, arguments_vec).into())
    }

    pub fn make_unit() -> Self {
        Value { id: None, contents: ValueVariants::Unit }
    }

    pub fn make_struct(contents: impl Iterator<Item = (Symbol, Value)>, location: Location) -> Self {
        let id = Some(location);

        let contents = Plaintext::Struct(
            contents
                .map(|(symbol, value)| {
                    let identifier =
                        symbol.to_string().parse().expect("Invalid identifiers shouldn't have been allowed.");
                    let plaintext = value.try_into().expect("Invalid struct members shouldn't have been allowed.");
                    (identifier, plaintext)
                })
                .collect(),
            Default::default(),
        );

        Value { id, contents: ValueVariants::Svm(contents.into()) }
    }

    pub fn make_record(contents: impl Iterator<Item = (Symbol, Value)>, program: Symbol, path: Vec<Symbol>) -> Self {
        let id = Some(Location { program, path });

        // Find the owner, storing the other contents for later.
        let mut non_owners = Vec::new();
        // let mut non_owners: Vec<(SvmIdentifier, SvmPlaintext)> = Vec::new();
        let symbol_owner = Symbol::intern("owner");
        let mut opt_owner_value = None;
        for (symbol, value) in contents {
            if symbol == symbol_owner {
                let owner: Address = value.try_into().expect("Owner should be an address.");
                opt_owner_value = Some(owner);
            } else {
                let identifier: SvmIdentifier = symbol.to_string().parse().expect("Can't parse identifier.");
                let plaintext: SvmPlaintext = value.try_into().expect("Record member not plaintext.");
                non_owners.push((identifier, Entry::Public(plaintext)));
            }
        }

        let Some(owner_value) = opt_owner_value else {
            panic!("No owner in record.");
        };

        let contents = SvmValueParam::Record(
            Record::<CurrentNetwork, SvmPlaintext>::from_plaintext(
                Owner::Public(owner_value),
                non_owners.into_iter().collect(),
                Group::generator(), // Just an arbitrary nonce.
                snarkvm::prelude::U8::new(1u8),
            )
            .expect("Failed to make record."),
        );

        Value { id, contents: ValueVariants::Svm(contents) }
    }

    pub fn make_array(contents: impl Iterator<Item = Value>) -> Self {
        let vec = contents
            .map(|value| {
                let plaintext: SvmPlaintext =
                    value.try_into().expect("Invalid array members shouldn't have been allowed.");
                plaintext
            })
            .collect();
        SvmPlaintext::Array(vec, Default::default()).into()
    }

    pub fn make_tuple(contents: impl IntoIterator<Item = Value>) -> Self {
        ValueVariants::Tuple(contents.into_iter().collect()).into()
    }

    pub fn make_string(s: String) -> Self {
        ValueVariants::String(s).into()
    }

    /// Gets the type of a `Value` but only if it is an integer, a field, a group, or a scalar.
    /// Return `None` otherwise. These are the only types that an unsuffixed literal can have.
    pub fn get_numeric_type(&self) -> Option<Type> {
        use IntegerType::*;
        use Type::*;
        let ValueVariants::Svm(SvmValueParam::Plaintext(Plaintext::Literal(literal, ..))) = &self.contents else {
            return None;
        };
        match literal {
            SvmLiteralParam::U8(_) => Some(Integer(U8)),
            SvmLiteralParam::U16(_) => Some(Integer(U16)),
            SvmLiteralParam::U32(_) => Some(Integer(U32)),
            SvmLiteralParam::U64(_) => Some(Integer(U64)),
            SvmLiteralParam::U128(_) => Some(Integer(U128)),
            SvmLiteralParam::I8(_) => Some(Integer(I8)),
            SvmLiteralParam::I16(_) => Some(Integer(I16)),
            SvmLiteralParam::I32(_) => Some(Integer(I32)),
            SvmLiteralParam::I64(_) => Some(Integer(I64)),
            SvmLiteralParam::I128(_) => Some(Integer(I128)),
            SvmLiteralParam::Field(_) => Some(Field),
            SvmLiteralParam::Group(_) => Some(Group),
            SvmLiteralParam::Scalar(_) => Some(Scalar),
            _ => None,
        }
    }

    #[allow(clippy::type_complexity)]
    pub fn to_expression(
        &self,
        span: Span,
        node_builder: &NodeBuilder,
        ty: &Type,
        struct_lookup: &dyn Fn(&[Symbol]) -> Vec<(Symbol, Type)>,
    ) -> Option<Expression> {
        use crate::{Literal, TupleExpression, UnitExpression};

        let id = node_builder.next_id();
        let expression = match &self.contents {
            ValueVariants::Unit => UnitExpression { span, id }.into(),
            ValueVariants::Tuple(vec) => {
                let Type::Tuple(tuple_type) = ty else {
                    return None;
                };

                if vec.len() != tuple_type.elements().len() {
                    return None;
                }

                TupleExpression {
                    span,
                    id,
                    elements: vec
                        .iter()
                        .zip(tuple_type.elements())
                        .map(|(val, ty)| val.to_expression(span, node_builder, ty, struct_lookup))
                        .collect::<Option<Vec<_>>>()?,
                }
                .into()
            }
            ValueVariants::Unsuffixed(s) => Literal::unsuffixed(s.clone(), span, id).into(),
            ValueVariants::Svm(value) => match value {
                SvmValueParam::Plaintext(plaintext) => {
                    plaintext_to_expression(plaintext, span, node_builder, ty, &struct_lookup)?
                }
                SvmValueParam::Record(..) => return None,
                SvmValueParam::Future(..) => return None,
            },
            ValueVariants::Future(..) => return None,
            ValueVariants::String(value) => Literal::string(value.clone(), span, id).into(),
        };

        Some(expression)
    }
}

#[allow(clippy::type_complexity)]
fn plaintext_to_expression(
    plaintext: &SvmPlaintext,
    span: Span,
    node_builder: &NodeBuilder,
    ty: &Type,
    struct_lookup: &dyn Fn(&[Symbol]) -> Vec<(Symbol, Type)>,
) -> Option<Expression> {
    use crate::{ArrayExpression, CompositeExpression, CompositeFieldInitializer, Identifier, IntegerType, Literal};

    let id = node_builder.next_id();

    let expression = match plaintext {
        Plaintext::Literal(literal, ..) => match literal {
            SvmLiteralParam::Address(address) => {
                Literal::address(address.to_string(), span, node_builder.next_id()).into()
            }
            SvmLiteralParam::Boolean(boolean) => Literal::boolean(**boolean, span, node_builder.next_id()).into(),
            SvmLiteralParam::Field(field) => {
                let mut s = field.to_string();
                // Strip off the `field` suffix.
                s.truncate(s.len() - 5);
                Literal::field(s, span, id).into()
            }
            SvmLiteralParam::Group(group) => {
                let mut s = group.to_string();
                // Strip off the `group` suffix.
                s.truncate(s.len() - 5);
                Literal::group(s, span, id).into()
            }
            SvmLiteralParam::Scalar(scalar) => {
                let mut s = scalar.to_string();
                // Strip off the `scalar` suffix.
                s.truncate(s.len() - 6);
                Literal::scalar(s, span, id).into()
            }
            SvmLiteralParam::I8(int) => Literal::integer(IntegerType::I8, (**int).to_string(), span, id).into(),
            SvmLiteralParam::I16(int) => Literal::integer(IntegerType::I16, (**int).to_string(), span, id).into(),
            SvmLiteralParam::I32(int) => Literal::integer(IntegerType::I32, (**int).to_string(), span, id).into(),
            SvmLiteralParam::I64(int) => Literal::integer(IntegerType::I64, (**int).to_string(), span, id).into(),
            SvmLiteralParam::I128(int) => Literal::integer(IntegerType::I128, (**int).to_string(), span, id).into(),
            SvmLiteralParam::U8(int) => Literal::integer(IntegerType::U8, (**int).to_string(), span, id).into(),
            SvmLiteralParam::U16(int) => Literal::integer(IntegerType::U16, (**int).to_string(), span, id).into(),
            SvmLiteralParam::U32(int) => Literal::integer(IntegerType::U32, (**int).to_string(), span, id).into(),
            SvmLiteralParam::U64(int) => Literal::integer(IntegerType::U64, (**int).to_string(), span, id).into(),
            SvmLiteralParam::U128(int) => Literal::integer(IntegerType::U128, (**int).to_string(), span, id).into(),
            SvmLiteralParam::Signature(..) => todo!(),
            SvmLiteralParam::String(..) => return None,
        },
        Plaintext::Struct(index_map, ..) => {
            let Type::Composite(composite_type) = ty else {
                return None;
            };
            let symbols = &composite_type.path.expect_global_location().path;
            let iter_members = struct_lookup(symbols);
            CompositeExpression {
                span,
                id,
                path: composite_type.path.clone(),
                // If we were able to construct a Value, the const arguments must have already been resolved
                // and inserted appropriately.
                const_arguments: Vec::new(),
                members: iter_members
                    .into_iter()
                    .map(|(sym, ty)| {
                        let svm_identifier: snarkvm::prelude::Identifier<CurrentNetwork> =
                            sym.to_string().parse().ok()?;
                        Some(CompositeFieldInitializer {
                            span,
                            id: node_builder.next_id(),
                            identifier: Identifier::new(sym, node_builder.next_id()),
                            expression: Some(plaintext_to_expression(
                                index_map.get(&svm_identifier)?,
                                span,
                                node_builder,
                                &ty,
                                &struct_lookup,
                            )?),
                        })
                    })
                    .collect::<Option<Vec<_>>>()?,
            }
            .into()
        }
        Plaintext::Array(vec, ..) => {
            let Type::Array(array_ty) = ty else {
                return None;
            };
            ArrayExpression {
                span,
                id,
                elements: vec
                    .iter()
                    .map(|pt| plaintext_to_expression(pt, span, node_builder, &array_ty.element_type, &struct_lookup))
                    .collect::<Option<Vec<_>>>()?,
            }
            .into()
        }
    };

    Some(expression)
}

impl FromStr for Value {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Either it's a unit.
        if s == "()" {
            return Ok(Value::make_unit());
        }

        // Or it's a tuple.
        if let Some(s) = s.strip_prefix("(")
            && let Some(s) = s.strip_suffix(")")
        {
            let mut results = Vec::new();
            for item in s.split(',') {
                let item = item.trim();
                let value: Value = item.parse().map_err(|_| ())?;
                results.push(value);
            }

            return Ok(Value::make_tuple(results));
        }

        // Or it's an unsuffixed numeric literal.
        if s.chars().all(|c| c == '_' || c.is_ascii_digit()) {
            return Ok(ValueVariants::Unsuffixed(s.to_string()).into());
        }

        // Or it's a snarkvm value.
        let value: SvmValue = s.parse().map_err(|_| ())?;
        Ok(value.into())
    }
}
