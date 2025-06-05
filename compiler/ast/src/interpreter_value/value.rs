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

use crate::Type;

use leo_span::{Symbol, sym};

use snarkvm::prelude::{
    Address as SvmAddressParam,
    Argument as SvmArgumentParam,
    Boolean as SvmBooleanParam,
    Field as SvmFieldParam,
    Future as SvmFutureParam,
    Group as SvmGroupParam,
    I8 as SvmI8Param,
    I16 as SvmI16Param,
    I32 as SvmI32Param,
    I64 as SvmI64Param,
    I128 as SvmI128Param,
    Identifier as SvmIdentifierParam,
    LiteralType,
    Plaintext as SvmPlaintextParam,
    Record as SvmRecordParam,
    Scalar as SvmScalarParam,
    Signature as SvmSignatureParam,
    TestnetV0,
    U8 as SvmU8Param,
    U16 as SvmU16Param,
    U32 as SvmU32Param,
    U64 as SvmU64Param,
    U128 as SvmU128Param,
    integers::Integer as SvmIntegerParam,
};
pub use snarkvm::prelude::{Literal as SvmLiteralParam, Value as SvmValueParam};

use itertools::Itertools as _;
use std::{fmt, fmt::Write as _, hash::Hash, str::FromStr};

pub type U8 = SvmU8Param<TestnetV0>;
pub type U16 = SvmU16Param<TestnetV0>;
pub type U32 = SvmU32Param<TestnetV0>;
pub type U64 = SvmU64Param<TestnetV0>;
pub type U128 = SvmU128Param<TestnetV0>;
pub type I8 = SvmI8Param<TestnetV0>;
pub type I16 = SvmI16Param<TestnetV0>;
pub type I32 = SvmI32Param<TestnetV0>;
pub type I64 = SvmI64Param<TestnetV0>;
pub type I128 = SvmI128Param<TestnetV0>;
pub type Literal = SvmLiteralParam<TestnetV0>;
pub type Plaintext = SvmPlaintextParam<TestnetV0>;
pub type Record = SvmRecordParam<TestnetV0, Plaintext>;
pub type Value = SvmValueParam<TestnetV0>;
pub type Address = SvmAddressParam<TestnetV0>;
pub type Boolean = SvmBooleanParam<TestnetV0>;
pub type Field = SvmFieldParam<TestnetV0>;
pub type Future = SvmFutureParam<TestnetV0>;
pub type Group = SvmGroupParam<TestnetV0>;
pub type SvmIdentifier = SvmIdentifierParam<TestnetV0>;
pub type Integer<I> = SvmIntegerParam<TestnetV0, I>;
pub type Scalar = SvmScalarParam<TestnetV0>;
pub type Signature = SvmSignatureParam<TestnetV0>;
pub type Argument = SvmArgumentParam<TestnetV0>;

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

/// A Leo value of any type.
///
/// Mappings and functions aren't considered values. The snarkvm `Value` is
/// almost what we need, but we also have tuples and Unit.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub enum LeoValue {
    #[default]
    Unit,
    Tuple(Vec<Value>),
    Value(Value),
}

impl LeoValue {
    pub fn cast(&self, ty: &Type) -> Option<LeoValue> {
        let literal: &Literal = self.try_as_ref()?;
        let literal_type = match ty {
            Type::Address => LiteralType::Address,
            Type::Boolean => LiteralType::Boolean,
            Type::Field => LiteralType::Field,
            Type::Group => LiteralType::Group,
            Type::Scalar => LiteralType::Scalar,
            Type::Integer(crate::IntegerType::U8) => LiteralType::U8,
            Type::Integer(crate::IntegerType::U16) => LiteralType::U16,
            Type::Integer(crate::IntegerType::U32) => LiteralType::U32,
            Type::Integer(crate::IntegerType::U64) => LiteralType::U64,
            Type::Integer(crate::IntegerType::U128) => LiteralType::U128,
            Type::Integer(crate::IntegerType::I8) => LiteralType::I8,
            Type::Integer(crate::IntegerType::I16) => LiteralType::I16,
            Type::Integer(crate::IntegerType::I32) => LiteralType::I32,
            Type::Integer(crate::IntegerType::I64) => LiteralType::I64,
            Type::Integer(crate::IntegerType::I128) => LiteralType::I128,
            Type::Array(..)
            | Type::Composite(..)
            | Type::Future(..)
            | Type::Identifier(..)
            | Type::Mapping(..)
            | Type::Signature
            | Type::String
            | Type::Tuple(..)
            | Type::Numeric
            | Type::Unit
            | Type::Err => return None,
        };

        literal.cast(literal_type).ok().map(|lit_result| lit_result.into())
    }

    pub fn try_as_array(&self) -> Option<&[Plaintext]> {
        match self {
            LeoValue::Value(SvmValueParam::Plaintext(SvmPlaintextParam::Array(x, _))) => Some(x),
            _ => None,
        }
    }

    pub fn try_as_array_mut(&mut self) -> Option<&mut [Plaintext]> {
        match self {
            LeoValue::Value(SvmValueParam::Plaintext(SvmPlaintextParam::Array(x, _))) => Some(x),
            _ => None,
        }
    }

    pub fn record(nonce: Group, owner: Address, pairs: impl Iterator<Item = (Symbol, Plaintext)>) -> Option<Self> {
        let mut buffer = String::new();
        let Ok(indexmap): Result<_, ()> = pairs
            .map(|(symbol, plaintext)| {
                buffer.clear();
                write!(&mut buffer, "{symbol}").unwrap();
                let identifier: SvmIdentifier = buffer.parse().map_err(|_| ())?;
                Ok((identifier, snarkvm::prelude::Entry::Public(plaintext)))
            })
            .collect()
        else {
            return None;
        };

        Some(Record::from_plaintext(snarkvm::prelude::Owner::Public(owner), indexmap, nonce).ok()?.into())
    }

    pub fn struct_svm(pairs: impl Iterator<Item = (SvmIdentifier, Plaintext)>) -> Option<Self> {
        Some(Plaintext::Struct(pairs.collect(), Default::default()).into())
    }

    pub fn struct_(pairs: impl IntoIterator<Item = (Symbol, LeoValue)>) -> Option<Self> {
        let mut all_worked = true;
        let value =
            Self::struct_plaintext(pairs.into_iter().filter_map(|(sym, leo_value)| match leo_value.try_into() {
                Ok(plaintext) => Some((sym, plaintext)),
                Err(..) => {
                    all_worked = false;
                    None
                }
            }))?;
        all_worked.then_some(value)
    }

    pub fn struct_plaintext(pairs: impl Iterator<Item = (Symbol, Plaintext)>) -> Option<Self> {
        let mut buffer = String::new();
        let Ok(indexmap): Result<_, ()> = pairs
            .map(|(symbol, plaintext)| {
                buffer.clear();
                write!(&mut buffer, "{symbol}").unwrap();
                let identifier: SvmIdentifier = buffer.parse().map_err(|_| ())?;
                Ok((identifier, plaintext))
            })
            .collect()
        else {
            return None;
        };

        Some(Plaintext::Struct(indexmap, Default::default()).into())
    }

    pub fn struct_set(struct_: &mut Plaintext, key: Symbol, new_value: LeoValue) -> bool {
        let SvmPlaintextParam::Struct(struct_, _) = struct_ else {
            return false;
        };
        let Ok(key_id): Result<SvmIdentifier, _> = key.try_into() else {
            return false;
        };
        let Some(value) = struct_.get_mut(&key_id) else {
            return false;
        };
        let Ok(plaintext) = new_value.try_into() else {
            return false;
        };
        *value = plaintext;
        true
    }

    pub fn record_set(record: &mut Value, key: Symbol, new_value: LeoValue) -> bool {
        let SvmValueParam::Record(record) = record else {
            return false;
        };
        let Ok(plaintext) = new_value.try_into() else {
            return false;
        };

        // Since snarkvm records are immutable, we have to rebuild a new record.
        // Consequently this is unfortunately linear in the number of entries.
        // Performance is unlikely to be a problem in practice.
        if key == sym::owner {
            // `owner` isn't a normal field.
            let SvmPlaintextParam::Literal(SvmLiteralParam::Address(address), _) = plaintext else {
                return false;
            };

            let owner = snarkvm::prelude::Owner::Public(address);
            let nonce = record.nonce().clone();
            let data = record.data().iter().map(|(id, entry)| (id.clone(), entry.clone())).collect();
            let Ok(new_record) = Record::from_plaintext(owner, data, nonce).into() else {
                return false;
            };
            *record = new_record;
            true
        } else {
            // We have to rebuild the whole record, updating one of the entries.
            let owner = record.owner().clone();
            let nonce = record.nonce().clone();
            let new_entry = snarkvm::prelude::Entry::Public(plaintext);
            let Ok(key_id): Result<SvmIdentifier, _> = key.try_into() else {
                return false;
            };
            let mut changed_entry = false;
            let data = record
                .data()
                .iter()
                .map(|(id, entry)| {
                    if *id == key_id {
                        changed_entry = true;
                        (key_id, new_entry.clone())
                    } else {
                        (id.clone(), entry.clone())
                    }
                })
                .collect();
            if !changed_entry {
                // We can't add a new entry - only update one.
                return false;
            }
            let Ok(new_record) = Record::from_plaintext(owner, data, nonce).into() else {
                return false;
            };
            *record = new_record;
            true
        }
    }

    /// Update an entry in the struct or record.
    ///
    /// Structs and records are handled rather differently than other sub-variants of `LeoValue`.
    /// We provide functions to access and update them, rather than expecting Leo compiler code
    /// to access the underlying `IndexMap` directly. This is largely because we don't want to
    /// guarantee that the version of `indexmap` used in snarkVM is the same as the version in
    /// Leo, and thus we don't want to force code to try to refer to the type `IndexMap`.
    pub fn member_set(&mut self, key: Symbol, value: LeoValue) -> bool {
        match self {
            LeoValue::Value(SvmValueParam::Plaintext(plaintext)) => Self::struct_set(plaintext, key, value),
            LeoValue::Value(record_value) => Self::record_set(record_value, key, value),
            LeoValue::Unit | LeoValue::Tuple(..) => false,
        }
    }

    pub fn member_get(&self, key: Symbol) -> Option<Plaintext> {
        let identifier = || -> Option<SvmIdentifier> { key.to_string().parse().ok() };

        match self {
            LeoValue::Value(SvmValueParam::Record(record)) => {
                if key == sym::owner {
                    let address: Address = **record.owner();
                    let literal: Literal = address.into();
                    Some(SvmPlaintextParam::Literal(literal, Default::default()))
                } else {
                    match record.data().get(&identifier()?)? {
                        snarkvm::prelude::Entry::Constant(x) | snarkvm::prelude::Entry::Public(x) => Some(x.clone()),
                        snarkvm::prelude::Entry::Private(..) => None,
                    }
                }
            }
            LeoValue::Value(SvmValueParam::Plaintext(SvmPlaintextParam::Struct(struct_, _))) => {
                struct_.get(&identifier()?).cloned()
            }
            _ => None,
        }
    }

    pub fn owner(&self) -> Option<Address> {
        let LeoValue::Value(SvmValueParam::Record(record)) = self else {
            return None;
        };

        Some(**record.owner())
    }

    pub fn nonce(&self) -> Option<Group> {
        let LeoValue::Value(SvmValueParam::Record(record)) = self else {
            return None;
        };

        Some(*record.nonce())
    }

    pub fn try_as_usize(&self) -> Option<usize> {
        let literal: &Literal = self.try_as_ref()?;
        match literal {
            SvmLiteralParam::I8(x) => (**x).try_into().ok(),
            SvmLiteralParam::I16(x) => (**x).try_into().ok(),
            SvmLiteralParam::I32(x) => (**x).try_into().ok(),
            SvmLiteralParam::I64(x) => (**x).try_into().ok(),
            SvmLiteralParam::I128(x) => (**x).try_into().ok(),
            SvmLiteralParam::U8(x) => Some(**x as usize),
            SvmLiteralParam::U16(x) => Some(**x as usize),
            SvmLiteralParam::U32(x) => (**x).try_into().ok(),
            SvmLiteralParam::U64(x) => (**x).try_into().ok(),
            SvmLiteralParam::U128(x) => (**x).try_into().ok(),
            SvmLiteralParam::Address(..)
            | SvmLiteralParam::Boolean(..)
            | SvmLiteralParam::Field(..)
            | SvmLiteralParam::Group(..)
            | SvmLiteralParam::Scalar(..)
            | SvmLiteralParam::Signature(..)
            | SvmLiteralParam::String(..) => None,
        }
    }

    pub fn try_as_u32(&self) -> Option<u32> {
        let literal: &Literal = self.try_as_ref()?;
        match literal {
            SvmLiteralParam::I8(x) => (**x).try_into().ok(),
            SvmLiteralParam::I16(x) => (**x).try_into().ok(),
            SvmLiteralParam::I32(x) => (**x).try_into().ok(),
            SvmLiteralParam::I64(x) => (**x).try_into().ok(),
            SvmLiteralParam::I128(x) => (**x).try_into().ok(),
            SvmLiteralParam::U8(x) => Some(**x as u32),
            SvmLiteralParam::U16(x) => Some(**x as u32),
            SvmLiteralParam::U32(x) => Some(**x),
            SvmLiteralParam::U64(x) => (**x).try_into().ok(),
            SvmLiteralParam::U128(x) => (**x).try_into().ok(),
            SvmLiteralParam::Address(..)
            | SvmLiteralParam::Boolean(..)
            | SvmLiteralParam::Field(..)
            | SvmLiteralParam::Group(..)
            | SvmLiteralParam::Scalar(..)
            | SvmLiteralParam::Signature(..)
            | SvmLiteralParam::String(..) => None,
        }
    }

    pub fn try_as_i128(&self) -> Option<i128> {
        let literal: &Literal = self.try_as_ref()?;
        match literal {
            SvmLiteralParam::I8(x) => Some(**x as i128),
            SvmLiteralParam::I16(x) => Some(**x as i128),
            SvmLiteralParam::I32(x) => Some(**x as i128),
            SvmLiteralParam::I64(x) => Some(**x as i128),
            SvmLiteralParam::I128(x) => Some(**x as i128),
            SvmLiteralParam::U8(x) => Some(**x as i128),
            SvmLiteralParam::U16(x) => Some(**x as i128),
            SvmLiteralParam::U32(x) => Some(**x as i128),
            SvmLiteralParam::U64(x) => Some(**x as i128),
            SvmLiteralParam::U128(x) => (**x).try_into().ok(),
            SvmLiteralParam::Address(..)
            | SvmLiteralParam::Boolean(..)
            | SvmLiteralParam::Field(..)
            | SvmLiteralParam::Group(..)
            | SvmLiteralParam::Scalar(..)
            | SvmLiteralParam::Signature(..)
            | SvmLiteralParam::String(..) => None,
        }
    }

    pub fn try_as_plaintext(&self) -> Option<&Plaintext> {
        match self {
            LeoValue::Value(Value::Plaintext(plaintext)) => Some(plaintext),
            _ => None,
        }
    }

    pub fn try_as_plaintext_mut(&mut self) -> Option<&mut Plaintext> {
        match self {
            LeoValue::Value(Value::Plaintext(plaintext)) => Some(plaintext),
            _ => None,
        }
    }

    pub fn try_make_array(values: impl IntoIterator<Item = LeoValue>) -> Option<Self> {
        let plaintext_vec = values.into_iter().map(LeoValue::try_into).collect::<Result<Vec<Plaintext>, _>>().ok()?;

        Some(SvmPlaintextParam::Array(plaintext_vec, Default::default()).into())
    }

    pub fn try_make_tuple(values: impl IntoIterator<Item = LeoValue>) -> Option<Self> {
        let value_vec = values.into_iter().map(LeoValue::try_into).collect::<Result<Vec<Value>, _>>().ok()?;

        Some(LeoValue::Tuple(value_vec))
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlaintextHash(pub Plaintext);

fn pt_hash<H: std::hash::Hasher>(pt: &Plaintext, state: &mut H) {
    use SvmPlaintextParam::*;
    match pt {
        Literal(literal, _) => {
            0u8.hash(state);
            literal.hash(state);
        }
        Struct(index_map, _) => {
            1u8.hash(state);
            index_map.iter().for_each(|(key, val)| {
                key.hash(state);
                pt_hash(val, state);
            });
        }
        Array(vec, _) => {
            2u8.hash(state);
            vec.iter().for_each(|val| pt_hash(val, state));
        }
    }
}

impl From<Plaintext> for PlaintextHash {
    fn from(value: Plaintext) -> Self {
        PlaintextHash(value)
    }
}

impl Hash for PlaintextHash {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        pt_hash(&self.0, state);
    }
}

impl fmt::Display for LeoValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use LeoValue::*;
        match self {
            Unit => write!(f, "()"),
            Tuple(x) => {
                write!(f, "({})", x.iter().format(", "))
            }
            Value(v) => write!(f, "{v}"),
        }
    }
}

impl From<Value> for LeoValue {
    fn from(value: Value) -> Self {
        LeoValue::Value(value)
    }
}

impl From<Plaintext> for LeoValue {
    fn from(value: Plaintext) -> Self {
        LeoValue::Value(SvmValueParam::Plaintext(value))
    }
}

impl From<Literal> for LeoValue {
    fn from(value: Literal) -> Self {
        LeoValue::Value(SvmValueParam::Plaintext(SvmPlaintextParam::Literal(value, Default::default())))
    }
}

impl From<Future> for LeoValue {
    fn from(value: Future) -> Self {
        LeoValue::Value(SvmValueParam::Future(value))
    }
}

impl From<Record> for LeoValue {
    fn from(value: Record) -> Self {
        LeoValue::Value(SvmValueParam::Record(value))
    }
}

impl TryFrom<LeoValue> for Future {
    type Error = ();

    fn try_from(value: LeoValue) -> Result<Self, Self::Error> {
        match value {
            LeoValue::Value(SvmValueParam::Future(x)) => Ok(x),
            _ => Err(()),
        }
    }
}

impl TryFrom<LeoValue> for Value {
    type Error = ();

    fn try_from(value: LeoValue) -> Result<Self, Self::Error> {
        match value {
            LeoValue::Value(value) => Ok(value),
            _ => Err(()),
        }
    }
}

impl TryFrom<LeoValue> for Plaintext {
    type Error = ();

    fn try_from(value: LeoValue) -> Result<Self, Self::Error> {
        match value {
            LeoValue::Value(SvmValueParam::Plaintext(plaintext)) => Ok(plaintext),
            _ => Err(()),
        }
    }
}

macro_rules! impl_try_from {
    ($($ty: ident)*) => {
        $(
            impl TryFrom<LeoValue> for $ty {
                type Error = ();

                fn try_from(value: LeoValue) -> Result<Self, Self::Error> {
                    match value {
                        LeoValue::Value(SvmValueParam::Plaintext(SvmPlaintextParam::Literal(SvmLiteralParam::$ty(x), _))) => {
                            Ok(x)
                        }
                        _ => Err(()),
                    }
                }
            }
        )*
    };
}

impl_try_from! {
    Scalar Group Field Address
}

pub trait TryAsRef<T>
where
    T: ?Sized,
{
    fn try_as_ref(&self) -> Option<&T>;
}

impl TryAsRef<Literal> for LeoValue {
    fn try_as_ref(&self) -> Option<&Literal> {
        match self {
            LeoValue::Value(Value::Plaintext(Plaintext::Literal(lit, _))) => Some(lit),
            _ => None,
        }
    }
}

impl TryAsRef<Plaintext> for LeoValue {
    fn try_as_ref(&self) -> Option<&Plaintext> {
        match self {
            LeoValue::Value(Value::Plaintext(plaintext)) => Some(plaintext),
            _ => None,
        }
    }
}

macro_rules! impl_from_integer {
    ($($int_type: ident $variant: ident);* $(;)?) => {
        $(
            impl From<$int_type> for LeoValue {
                fn from(value: $int_type) -> Self {
                    LeoValue::Value(
                        SvmValueParam::Plaintext(
                            SvmPlaintextParam::Literal(
                                snarkvm::prelude::$variant::new(value).into(),
                                Default::default(),
                            )
                        )
                    )
                }
            }

            impl TryFrom<LeoValue> for $int_type {
                type Error = ();

                fn try_from(value: LeoValue) -> Result<$int_type, Self::Error> {
                    match value {
                        LeoValue::Value(SvmValueParam::Plaintext(SvmPlaintextParam::Literal(SvmLiteralParam::$variant(x), _))) => Ok(*x),
                        _ => Err(()),
                    }
                }
            }

            impl TryAsRef<$int_type> for LeoValue {
                fn try_as_ref(&self) -> Option<&$int_type> {
                    match self {
                        LeoValue::Value(SvmValueParam::Plaintext(SvmPlaintextParam::Literal(SvmLiteralParam::$variant(x), _))) => Some(&**x),
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

macro_rules! impl_from_type {
    ($($type_: ident);* $(;)?) => {
        $(
            impl From<$type_> for LeoValue {
                fn from(value: $type_) -> Self {
                    LeoValue::Value(
                        SvmValueParam::Plaintext(
                            SvmPlaintextParam::Literal(
                                value.into(),
                                Default::default(),
                            )
                        )
                    )
                }
            }
        )*
    };
}

impl_from_type! {
    Group; Field; Scalar; Address; Signature;
    U8; U16; U32; U64; U128;
    I8; I16; I32; I64; I128;
    Boolean;
}

impl TryFrom<LeoValue> for Argument {
    type Error = ();

    fn try_from(value: LeoValue) -> Result<Self, Self::Error> {
        let LeoValue::Value(value) = value else {
            return Err(());
        };

        match value {
            SvmValueParam::Plaintext(plaintext) => Ok(SvmArgumentParam::Plaintext(plaintext)),
            SvmValueParam::Future(future) => Ok(SvmArgumentParam::Future(future)),
            SvmValueParam::Record(..) => Err(()),
        }
    }
}

impl FromStr for LeoValue {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "()" {
            return Ok(LeoValue::Unit);
        }

        if let Some(s) = s.strip_prefix("(") {
            if let Some(s) = s.strip_suffix(")") {
                let mut results = Vec::new();
                for item in s.split(',') {
                    let item = item.trim();
                    let value: Value = item.parse().map_err(|_| ())?;
                    results.push(value);
                }

                return Ok(LeoValue::Tuple(results));
            }
        }

        let value: Value = s.parse().map_err(|_| ())?;
        Ok(value.into())
    }
}
