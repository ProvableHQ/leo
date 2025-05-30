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

use crate::{
    BinaryOperation,
    FromStrRadix,
    IntegerType,
    Literal as LeoLiteral,
    LiteralVariant,
    Node as _,
    Type,
    UnaryOperation,
    halt,
    halt_no_span,
    tc_fail,
};

use super::{Address, Field, Group, LeoValue, Scalar, TryAsRef as _};

use leo_errors::{InterpreterHalt, LeoError, Result};
use leo_span::Span;

use snarkvm::prelude::{Double as _, Inverse as _, Pow as _, ProgramID, Square as _, SquareRoot as _};

use std::str::FromStr;

impl LeoValue {
    pub fn gte(&self, rhs: &Self) -> Result<bool> {
        rhs.gt(self).map(|v| !v)
    }

    pub fn lte(&self, rhs: &Self) -> Result<bool> {
        rhs.lt(self).map(|v| !v)
    }

    pub fn lt(&self, rhs: &Self) -> Result<bool> {
        let (Some(lit0), Some(lit1)) = (self.try_as_ref(), rhs.try_as_ref()) else {
            halt_no_span!("Type failure: {} < {}", self, rhs);
        };

        use snarkvm::prelude::Literal::*;
        Ok(match (lit0, lit1) {
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
        let (Some(lit0), Some(lit1)) = (self.try_as_ref(), rhs.try_as_ref()) else {
            halt_no_span!("Type failure: {} > {}", self, rhs);
        };

        use snarkvm::prelude::Literal::*;
        Ok(match (lit0, lit1) {
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
        use LeoValue::*;
        Ok(match (self, rhs) {
            (Unit, Unit) => true,
            (Value(v0), Value(v1)) => v0 == v1,
            (Tuple(t0), Tuple(t1)) => t0 == t1,
            (_, _) => tc_fail!(),
        })
    }

    pub fn inc_wrapping(&self) -> Self {
        let Some(lit) = self.try_as_ref() else {
            tc_fail!();
        };

        use snarkvm::prelude::Literal::*;
        match lit {
            U8(x) => x.wrapping_add(1).into(),
            U16(x) => x.wrapping_add(1).into(),
            U32(x) => x.wrapping_add(1).into(),
            U64(x) => x.wrapping_add(1).into(),
            U128(x) => x.wrapping_add(1).into(),
            I8(x) => x.wrapping_add(1).into(),
            I16(x) => x.wrapping_add(1).into(),
            I32(x) => x.wrapping_add(1).into(),
            I64(x) => x.wrapping_add(1).into(),
            I128(x) => x.wrapping_add(1).into(),
            _ => tc_fail!(),
        }
    }

    /// Return the group generator.
    pub fn generator() -> Self {
        Group::generator().into()
    }

    /// Doesn't correspond to Aleo's shl, because it
    /// does not fail when set bits are shifted out.
    pub fn simple_shl(&self, shift: u32) -> Self {
        let Some(lit) = self.try_as_ref() else {
            tc_fail!();
        };

        use snarkvm::prelude::Literal::*;
        match lit {
            U8(x) => (**x << shift).into(),
            U16(x) => (**x << shift).into(),
            U32(x) => (**x << shift).into(),
            U64(x) => (**x << shift).into(),
            U128(x) => (**x << shift).into(),
            I8(x) => (**x << shift).into(),
            I16(x) => (**x << shift).into(),
            I32(x) => (**x << shift).into(),
            I64(x) => (**x << shift).into(),
            I128(x) => (**x << shift).into(),
            _ => tc_fail!(),
        }
    }

    pub fn simple_shr(&self, shift: u32) -> Self {
        let Some(lit) = self.try_as_ref() else {
            tc_fail!();
        };

        use snarkvm::prelude::Literal::*;
        match lit {
            U8(x) => (**x >> shift).into(),
            U16(x) => (**x >> shift).into(),
            U32(x) => (**x >> shift).into(),
            U64(x) => (**x >> shift).into(),
            U128(x) => (**x >> shift).into(),
            I8(x) => (**x >> shift).into(),
            I16(x) => (**x >> shift).into(),
            I32(x) => (**x >> shift).into(),
            I64(x) => (**x >> shift).into(),
            I128(x) => (**x >> shift).into(),
            _ => tc_fail!(),
        }
    }
}

impl TryFrom<&LeoLiteral> for LeoValue {
    type Error = LeoError;

    fn try_from(value: &LeoLiteral) -> Result<Self, Self::Error> {
        literal_to_value(value, &None)
    }
}

pub fn literal_to_value(literal: &LeoLiteral, ty: &Option<Type>) -> Result<LeoValue> {
    // SnarkVM will not parse fields, groups, or scalars with
    // leading zeros, so we strip them out.
    fn parse_str<T: FromStr + Into<LeoValue>>(s: &str, suffix: &str) -> Result<LeoValue> {
        // If there's a `-`, separate it from the rest of the string.
        let (neg, rest) = s.strip_prefix("-").map(|rest| ("-", rest)).unwrap_or(("", s));
        // Remove leading zeros.
        let mut rest = rest.trim_start_matches('0');
        if rest.is_empty() {
            rest = "0";
        }
        let formatted = format!("{neg}{rest}{suffix}");
        let parsed: T = match formatted.parse() {
            Ok(p) => p,
            Err(_e) => panic!("Parsing guarantees this works"),
        };
        Ok(parsed.into())
    }

    fn int_value<T: FromStrRadix + Into<LeoValue>>(int: &str) -> LeoValue {
        let s = int.replace("_", "");
        T::from_str_by_radix(&s).expect("Parsing guarantees this works.").into()
    }

    use IntegerType::*;
    use LiteralVariant::{Integer, Unsuffixed};

    let value: LeoValue = match (&literal.variant, ty) {
        (Integer(U8, s, ..), _) | (Unsuffixed(s), Some(Type::Integer(U8))) => int_value::<u8>(s),
        (Integer(U16, s, ..), _) | (Unsuffixed(s), Some(Type::Integer(U16))) => int_value::<u16>(s),
        (Integer(U32, s, ..), _) | (Unsuffixed(s), Some(Type::Integer(U32))) => int_value::<u32>(s),
        (Integer(U64, s, ..), _) | (Unsuffixed(s), Some(Type::Integer(U64))) => int_value::<u64>(s),
        (Integer(U128, s, ..), _) | (Unsuffixed(s), Some(Type::Integer(U128))) => int_value::<u128>(s),
        (Integer(I8, s, ..), _) | (Unsuffixed(s), Some(Type::Integer(I8))) => int_value::<i8>(s),
        (Integer(I16, s, ..), _) | (Unsuffixed(s), Some(Type::Integer(I16))) => int_value::<i16>(s),
        (Integer(I32, s, ..), _) | (Unsuffixed(s), Some(Type::Integer(I32))) => int_value::<i32>(s),
        (Integer(I64, s, ..), _) | (Unsuffixed(s), Some(Type::Integer(I64))) => int_value::<i64>(s),
        (Integer(I128, s, ..), _) | (Unsuffixed(s), Some(Type::Integer(I128))) => int_value::<i128>(s),
        (LiteralVariant::Field(s), _) | (Unsuffixed(s), Some(Type::Field)) => parse_str::<Field>(s, "field")?,
        (LiteralVariant::Group(s), _) | (Unsuffixed(s), Some(Type::Group)) => parse_str::<Group>(s, "group")?,
        (LiteralVariant::Scalar(s), _) | (Unsuffixed(s), Some(Type::Scalar)) => parse_str::<Scalar>(s, "scalar")?,
        (LiteralVariant::Address(s), _) => {
            if s.ends_with(".aleo") {
                let program_id = ProgramID::from_str(s)?;
                let address: Address = program_id.to_address()?;
                address.into()
            } else {
                parse_str::<Address>(s, "")?.into()
            }
        }
        (LiteralVariant::Boolean(b), _) => (*b).into(),
        (LiteralVariant::String(..), _) => tc_fail!(),
        (Unsuffixed(_s), _) => halt!(literal.span(), "cannot infer type of unsuffixed literal"),
    };

    Ok(value)
}

/// Evaluate a unary operation.
pub fn evaluate_unary(span: Span, op: UnaryOperation, value: &LeoValue) -> Result<LeoValue> {
    let Some(lit) = value.try_as_ref() else {
        halt!(span, "Type error");
    };

    use snarkvm::prelude::Literal::*;

    macro_rules! abs {
        ($x: expr, $ty: ident) => {
            if **$x == $ty::MIN {
                halt!(span, "abs overflow");
            } else {
                $x.abs().into()
            }
        };
    }

    macro_rules! neg {
        ($x: expr) => {
            match $x.checked_neg() {
                Some(y) => y.into(),
                None => halt!(span, "negation overflow"),
            }
        };
    }

    let value: LeoValue = match (op, lit) {
        (UnaryOperation::Abs, I8(x)) => abs!(x, i8),
        (UnaryOperation::Abs, I16(x)) => abs!(x, i16),
        (UnaryOperation::Abs, I32(x)) => abs!(x, i32),
        (UnaryOperation::Abs, I64(x)) => abs!(x, i64),
        (UnaryOperation::Abs, I128(x)) => abs!(x, i128),
        (UnaryOperation::Abs, _) => halt!(span, "Type error"),
        (UnaryOperation::AbsWrapped, I8(x)) => (x.unsigned_abs() as i8).into(),
        (UnaryOperation::AbsWrapped, I16(x)) => (x.unsigned_abs() as i16).into(),
        (UnaryOperation::AbsWrapped, I32(x)) => (x.unsigned_abs() as i32).into(),
        (UnaryOperation::AbsWrapped, I64(x)) => (x.unsigned_abs() as i64).into(),
        (UnaryOperation::AbsWrapped, I128(x)) => (x.unsigned_abs() as i128).into(),
        (UnaryOperation::AbsWrapped, _) => halt!(span, "Type error"),
        (UnaryOperation::Double, Field(x)) => x.double().into(),
        (UnaryOperation::Double, Group(x)) => x.double().into(),
        (UnaryOperation::Double, _) => halt!(span, "Type error"),
        (UnaryOperation::Inverse, Field(x)) => match x.inverse() {
            Ok(y) => y.into(),
            Err(_) => halt!(span, "attempt to invert 0field"),
        },
        (UnaryOperation::Inverse, _) => halt!(span, "TypeError"),
        (UnaryOperation::Negate, I8(x)) => neg!(x),
        (UnaryOperation::Negate, I16(x)) => neg!(x),
        (UnaryOperation::Negate, I32(x)) => neg!(x),
        (UnaryOperation::Negate, I64(x)) => neg!(x),
        (UnaryOperation::Negate, I128(x)) => neg!(x),
        (UnaryOperation::Negate, Group(x)) => (-*x).into(),
        (UnaryOperation::Negate, Field(x)) => (-*x).into(),
        (UnaryOperation::Negate, _) => halt!(span, "Type error"),
        (UnaryOperation::Not, Boolean(x)) => (!**x).into(),
        (UnaryOperation::Not, U8(x)) => (!**x).into(),
        (UnaryOperation::Not, U16(x)) => (!**x).into(),
        (UnaryOperation::Not, U32(x)) => (!**x).into(),
        (UnaryOperation::Not, U64(x)) => (!**x).into(),
        (UnaryOperation::Not, U128(x)) => (!**x).into(),
        (UnaryOperation::Not, I8(x)) => (!**x).into(),
        (UnaryOperation::Not, I16(x)) => (!**x).into(),
        (UnaryOperation::Not, I32(x)) => (!**x).into(),
        (UnaryOperation::Not, I64(x)) => (!**x).into(),
        (UnaryOperation::Not, I128(x)) => (!**x).into(),
        (UnaryOperation::Not, _) => halt!(span, "Type error"),
        (UnaryOperation::Square, Field(x)) => x.square().into(),
        (UnaryOperation::Square, _) => halt!(span, "Can only square fields"),
        (UnaryOperation::SquareRoot, Field(x)) => match x.square_root() {
            Ok(y) => y.into(),
            Err(_) => halt!(span, "square root failure"),
        },
        (UnaryOperation::SquareRoot, _) => halt!(span, "Can only apply square_root to fields"),
        (UnaryOperation::ToXCoordinate, Group(x)) => x.to_x_coordinate().into(),
        (UnaryOperation::ToXCoordinate, _) => tc_fail!(),
        (UnaryOperation::ToYCoordinate, Group(x)) => x.to_y_coordinate().into(),
        (UnaryOperation::ToYCoordinate, _) => tc_fail!(),
    };

    Ok(value)
}

/// Evaluate a binary operation.
pub fn evaluate_binary(span: Span, op: BinaryOperation, lhs: &LeoValue, rhs: &LeoValue) -> Result<LeoValue> {
    let (Some(lit0), Some(lit1)) = (lhs.try_as_ref(), rhs.try_as_ref()) else {
        halt!(span, "Type error");
    };

    use snarkvm::prelude::Literal::*;

    macro_rules! checked_op {
        ($x: expr, $y: expr, $op: ident) => {
            match ($x).$op($y) {
                Some(z) => z.into(),
                None => halt!(span, "overflow"),
            }
        };
    }

    macro_rules! wrapping_op {
        ($x: expr, $y: expr, $op: ident) => {
            $x.$op($y).into()
        };
    }

    macro_rules! shl {
        ($bit_count: expr) => {{
            let Some(shift) = rhs.try_as_u32() else {
                tc_fail!();
            };
            if shift >= $bit_count {
                tc_fail!();
            }
            let shifted = lhs.simple_shl(shift);
            let reshifted = shifted.simple_shr(shift);
            if lhs.eq(&reshifted)? {
                shifted
            } else {
                halt!(span, "shl overflow");
            }
        }};
    }

    macro_rules! wrapping_shl {
        ($x: expr) => {{
            let Some(shift) = rhs.try_as_u32() else {
                tc_fail!();
            };
            $x.wrapping_shl(shift).into()
        }};
    }

    macro_rules! shr {
        ($bit_count: expr) => {{
            let Some(shift) = rhs.try_as_u32() else {
                tc_fail!();
            };
            if shift >= $bit_count {
                tc_fail!();
            }
            lhs.simple_shr(shift)
        }};
    }

    macro_rules! wrapping_shr {
        ($x: expr) => {{
            let Some(shift) = rhs.try_as_u32() else {
                tc_fail!();
            };
            $x.wrapping_shr(shift).into()
        }};
    }

    let value: LeoValue = match (op, lit0, lit1) {
        // Add operations
        (BinaryOperation::Add, U8(x), U8(y)) => checked_op!(x, **y, checked_add),
        (BinaryOperation::Add, U16(x), U16(y)) => checked_op!(x, **y, checked_add),
        (BinaryOperation::Add, U32(x), U32(y)) => checked_op!(x, **y, checked_add),
        (BinaryOperation::Add, U64(x), U64(y)) => checked_op!(x, **y, checked_add),
        (BinaryOperation::Add, U128(x), U128(y)) => checked_op!(x, **y, checked_add),
        (BinaryOperation::Add, I8(x), I8(y)) => checked_op!(x, **y, checked_add),
        (BinaryOperation::Add, I16(x), I16(y)) => checked_op!(x, **y, checked_add),
        (BinaryOperation::Add, I32(x), I32(y)) => checked_op!(x, **y, checked_add),
        (BinaryOperation::Add, I64(x), I64(y)) => checked_op!(x, **y, checked_add),
        (BinaryOperation::Add, I128(x), I128(y)) => checked_op!(x, **y, checked_add),
        (BinaryOperation::Add, Group(x), Group(y)) => (*x + *y).into(),
        (BinaryOperation::Add, Field(x), Field(y)) => (*x + *y).into(),
        (BinaryOperation::Add, Scalar(x), Scalar(y)) => (*x + *y).into(),

        // AddWrapped operations
        (BinaryOperation::AddWrapped, U8(x), U8(y)) => wrapping_op!(x, **y, wrapping_add),
        (BinaryOperation::AddWrapped, U16(x), U16(y)) => wrapping_op!(x, **y, wrapping_add),
        (BinaryOperation::AddWrapped, U32(x), U32(y)) => wrapping_op!(x, **y, wrapping_add),
        (BinaryOperation::AddWrapped, U64(x), U64(y)) => wrapping_op!(x, **y, wrapping_add),
        (BinaryOperation::AddWrapped, U128(x), U128(y)) => wrapping_op!(x, **y, wrapping_add),
        (BinaryOperation::AddWrapped, I8(x), I8(y)) => wrapping_op!(x, **y, wrapping_add),
        (BinaryOperation::AddWrapped, I16(x), I16(y)) => wrapping_op!(x, **y, wrapping_add),
        (BinaryOperation::AddWrapped, I32(x), I32(y)) => wrapping_op!(x, **y, wrapping_add),
        (BinaryOperation::AddWrapped, I64(x), I64(y)) => wrapping_op!(x, **y, wrapping_add),
        (BinaryOperation::AddWrapped, I128(x), I128(y)) => wrapping_op!(x, **y, wrapping_add),

        // And operations
        (BinaryOperation::And, Boolean(x), Boolean(y)) => (**x && **y).into(),
        (BinaryOperation::BitwiseAnd, Boolean(x), Boolean(y)) => (*x & *y).into(),
        (BinaryOperation::BitwiseAnd, U8(x), U8(y)) => (*x & *y).into(),
        (BinaryOperation::BitwiseAnd, U16(x), U16(y)) => (*x & *y).into(),
        (BinaryOperation::BitwiseAnd, U32(x), U32(y)) => (*x & *y).into(),
        (BinaryOperation::BitwiseAnd, U64(x), U64(y)) => (*x & *y).into(),
        (BinaryOperation::BitwiseAnd, U128(x), U128(y)) => (*x & *y).into(),
        (BinaryOperation::BitwiseAnd, I8(x), I8(y)) => (*x & *y).into(),
        (BinaryOperation::BitwiseAnd, I16(x), I16(y)) => (*x & *y).into(),
        (BinaryOperation::BitwiseAnd, I32(x), I32(y)) => (*x & *y).into(),
        (BinaryOperation::BitwiseAnd, I64(x), I64(y)) => (*x & *y).into(),
        (BinaryOperation::BitwiseAnd, I128(x), I128(y)) => (*x & *y).into(),

        // Div operations
        (BinaryOperation::Div, U8(x), U8(y)) => checked_op!(x, **y, checked_div),
        (BinaryOperation::Div, U16(x), U16(y)) => checked_op!(x, **y, checked_div),
        (BinaryOperation::Div, U32(x), U32(y)) => checked_op!(x, **y, checked_div),
        (BinaryOperation::Div, U64(x), U64(y)) => checked_op!(x, **y, checked_div),
        (BinaryOperation::Div, U128(x), U128(y)) => checked_op!(x, **y, checked_div),
        (BinaryOperation::Div, I8(x), I8(y)) => checked_op!(x, **y, checked_div),
        (BinaryOperation::Div, I16(x), I16(y)) => checked_op!(x, **y, checked_div),
        (BinaryOperation::Div, I32(x), I32(y)) => checked_op!(x, **y, checked_div),
        (BinaryOperation::Div, I64(x), I64(y)) => checked_op!(x, **y, checked_div),
        (BinaryOperation::Div, I128(x), I128(y)) => checked_op!(x, **y, checked_div),
        (BinaryOperation::Div, Field(x), Field(y)) => match y.inverse() {
            Ok(y) => (*x * y).into(),
            Err(_) => halt!(span, "attempt to divide by 0field"),
        },

        // DivWrapped operations
        (BinaryOperation::DivWrapped, U8(x), U8(y)) => wrapping_op!(x, **y, wrapping_div),
        (BinaryOperation::DivWrapped, U16(x), U16(y)) => wrapping_op!(x, **y, wrapping_div),
        (BinaryOperation::DivWrapped, U32(x), U32(y)) => wrapping_op!(x, **y, wrapping_div),
        (BinaryOperation::DivWrapped, U64(x), U64(y)) => wrapping_op!(x, **y, wrapping_div),
        (BinaryOperation::DivWrapped, U128(x), U128(y)) => wrapping_op!(x, **y, wrapping_div),
        (BinaryOperation::DivWrapped, I8(x), I8(y)) => wrapping_op!(x, **y, wrapping_div),
        (BinaryOperation::DivWrapped, I16(x), I16(y)) => wrapping_op!(x, **y, wrapping_div),
        (BinaryOperation::DivWrapped, I32(x), I32(y)) => wrapping_op!(x, **y, wrapping_div),
        (BinaryOperation::DivWrapped, I64(x), I64(y)) => wrapping_op!(x, **y, wrapping_div),
        (BinaryOperation::DivWrapped, I128(x), I128(y)) => wrapping_op!(x, **y, wrapping_div),

        // Comparison operations
        (BinaryOperation::Eq, _, _) => lhs.eq(rhs)?.into(),
        (BinaryOperation::Gte, _, _) => lhs.gte(rhs)?.into(),
        (BinaryOperation::Gt, _, _) => lhs.gt(rhs)?.into(),
        (BinaryOperation::Lte, _, _) => lhs.lte(rhs)?.into(),
        (BinaryOperation::Lt, _, _) => lhs.lt(rhs)?.into(),

        // Mod operations
        (BinaryOperation::Mod, U8(x), U8(y)) => checked_op!(x, **y, checked_rem),
        (BinaryOperation::Mod, U16(x), U16(y)) => checked_op!(x, **y, checked_rem),
        (BinaryOperation::Mod, U32(x), U32(y)) => checked_op!(x, **y, checked_rem),
        (BinaryOperation::Mod, U64(x), U64(y)) => checked_op!(x, **y, checked_rem),
        (BinaryOperation::Mod, U128(x), U128(y)) => checked_op!(x, **y, checked_rem),
        (BinaryOperation::Mod, I8(x), I8(y)) => checked_op!(x, **y, checked_rem),
        (BinaryOperation::Mod, I16(x), I16(y)) => checked_op!(x, **y, checked_rem),
        (BinaryOperation::Mod, I32(x), I32(y)) => checked_op!(x, **y, checked_rem),
        (BinaryOperation::Mod, I64(x), I64(y)) => checked_op!(x, **y, checked_rem),
        (BinaryOperation::Mod, I128(x), I128(y)) => checked_op!(x, **y, checked_rem),

        // Mul operations
        (BinaryOperation::Mul, U8(x), U8(y)) => checked_op!(x, **y, checked_mul),
        (BinaryOperation::Mul, U16(x), U16(y)) => checked_op!(x, **y, checked_mul),
        (BinaryOperation::Mul, U32(x), U32(y)) => checked_op!(x, **y, checked_mul),
        (BinaryOperation::Mul, U64(x), U64(y)) => checked_op!(x, **y, checked_mul),
        (BinaryOperation::Mul, U128(x), U128(y)) => checked_op!(x, **y, checked_mul),
        (BinaryOperation::Mul, I8(x), I8(y)) => checked_op!(x, **y, checked_mul),
        (BinaryOperation::Mul, I16(x), I16(y)) => checked_op!(x, **y, checked_mul),
        (BinaryOperation::Mul, I32(x), I32(y)) => checked_op!(x, **y, checked_mul),
        (BinaryOperation::Mul, I64(x), I64(y)) => checked_op!(x, **y, checked_mul),
        (BinaryOperation::Mul, I128(x), I128(y)) => checked_op!(x, **y, checked_mul),
        (BinaryOperation::Mul, Field(x), Field(y)) => (*x * *y).into(),
        (BinaryOperation::Mul, Group(x), Scalar(y)) => (*x * *y).into(),
        (BinaryOperation::Mul, Scalar(x), Group(y)) => (*x * *y).into(),

        // MulWrapped operations
        (BinaryOperation::MulWrapped, U8(x), U8(y)) => wrapping_op!(x, **y, wrapping_mul),
        (BinaryOperation::MulWrapped, U16(x), U16(y)) => wrapping_op!(x, **y, wrapping_mul),
        (BinaryOperation::MulWrapped, U32(x), U32(y)) => wrapping_op!(x, **y, wrapping_mul),
        (BinaryOperation::MulWrapped, U64(x), U64(y)) => wrapping_op!(x, **y, wrapping_mul),
        (BinaryOperation::MulWrapped, U128(x), U128(y)) => wrapping_op!(x, **y, wrapping_mul),
        (BinaryOperation::MulWrapped, I8(x), I8(y)) => wrapping_op!(x, **y, wrapping_mul),
        (BinaryOperation::MulWrapped, I16(x), I16(y)) => wrapping_op!(x, **y, wrapping_mul),
        (BinaryOperation::MulWrapped, I32(x), I32(y)) => wrapping_op!(x, **y, wrapping_mul),
        (BinaryOperation::MulWrapped, I64(x), I64(y)) => wrapping_op!(x, **y, wrapping_mul),
        (BinaryOperation::MulWrapped, I128(x), I128(y)) => wrapping_op!(x, **y, wrapping_mul),

        // Logical operations
        (BinaryOperation::Nand, Boolean(x), Boolean(y)) => (!(*x & *y)).into(),
        (BinaryOperation::Neq, _, _) => lhs.neq(rhs)?.into(),
        (BinaryOperation::Nor, Boolean(x), Boolean(y)) => (!(*x | *y)).into(),
        (BinaryOperation::Or, Boolean(x), Boolean(y)) => (*x | *y).into(),
        (BinaryOperation::BitwiseOr, Boolean(x), Boolean(y)) => (*x | *y).into(),
        (BinaryOperation::BitwiseOr, U8(x), U8(y)) => (*x | *y).into(),
        (BinaryOperation::BitwiseOr, U16(x), U16(y)) => (*x | *y).into(),
        (BinaryOperation::BitwiseOr, U32(x), U32(y)) => (*x | *y).into(),
        (BinaryOperation::BitwiseOr, U64(x), U64(y)) => (*x | *y).into(),
        (BinaryOperation::BitwiseOr, U128(x), U128(y)) => (*x | *y).into(),
        (BinaryOperation::BitwiseOr, I8(x), I8(y)) => (*x | *y).into(),
        (BinaryOperation::BitwiseOr, I16(x), I16(y)) => (*x | *y).into(),
        (BinaryOperation::BitwiseOr, I32(x), I32(y)) => (*x | *y).into(),
        (BinaryOperation::BitwiseOr, I64(x), I64(y)) => (*x | *y).into(),
        (BinaryOperation::BitwiseOr, I128(x), I128(y)) => (*x | *y).into(),

        // Pow operations
        (BinaryOperation::Pow, Field(x), Field(y)) => x.pow(y).into(),
        (BinaryOperation::Pow, U8(x), U8(y)) => checked_op!(x, **y as u32, checked_pow),
        (BinaryOperation::Pow, U8(x), U16(y)) => checked_op!(x, **y as u32, checked_pow),
        (BinaryOperation::Pow, U8(x), U32(y)) => checked_op!(x, **y, checked_pow),
        (BinaryOperation::Pow, U16(x), U8(y)) => checked_op!(x, **y as u32, checked_pow),
        (BinaryOperation::Pow, U16(x), U16(y)) => checked_op!(x, **y as u32, checked_pow),
        (BinaryOperation::Pow, U16(x), U32(y)) => checked_op!(x, **y, checked_pow),
        (BinaryOperation::Pow, U32(x), U8(y)) => checked_op!(x, **y as u32, checked_pow),
        (BinaryOperation::Pow, U32(x), U16(y)) => checked_op!(x, **y as u32, checked_pow),
        (BinaryOperation::Pow, U32(x), U32(y)) => checked_op!(x, **y, checked_pow),
        (BinaryOperation::Pow, U64(x), U8(y)) => checked_op!(x, **y as u32, checked_pow),
        (BinaryOperation::Pow, U64(x), U16(y)) => checked_op!(x, **y as u32, checked_pow),
        (BinaryOperation::Pow, U64(x), U32(y)) => checked_op!(x, **y, checked_pow),
        (BinaryOperation::Pow, U128(x), U8(y)) => checked_op!(x, **y as u32, checked_pow),
        (BinaryOperation::Pow, U128(x), U16(y)) => checked_op!(x, **y as u32, checked_pow),
        (BinaryOperation::Pow, U128(x), U32(y)) => checked_op!(x, **y, checked_pow),
        (BinaryOperation::Pow, I8(x), U8(y)) => checked_op!(x, **y as u32, checked_pow),
        (BinaryOperation::Pow, I8(x), U16(y)) => checked_op!(x, **y as u32, checked_pow),
        (BinaryOperation::Pow, I8(x), U32(y)) => checked_op!(x, **y, checked_pow),
        (BinaryOperation::Pow, I16(x), U8(y)) => checked_op!(x, **y as u32, checked_pow),
        (BinaryOperation::Pow, I16(x), U16(y)) => checked_op!(x, **y as u32, checked_pow),
        (BinaryOperation::Pow, I16(x), U32(y)) => checked_op!(x, **y, checked_pow),
        (BinaryOperation::Pow, I32(x), U8(y)) => checked_op!(x, **y as u32, checked_pow),
        (BinaryOperation::Pow, I32(x), U16(y)) => checked_op!(x, **y as u32, checked_pow),
        (BinaryOperation::Pow, I32(x), U32(y)) => checked_op!(x, **y, checked_pow),
        (BinaryOperation::Pow, I64(x), U8(y)) => checked_op!(x, **y as u32, checked_pow),
        (BinaryOperation::Pow, I64(x), U16(y)) => checked_op!(x, **y as u32, checked_pow),
        (BinaryOperation::Pow, I64(x), U32(y)) => checked_op!(x, **y, checked_pow),
        (BinaryOperation::Pow, I128(x), U8(y)) => checked_op!(x, **y as u32, checked_pow),
        (BinaryOperation::Pow, I128(x), U16(y)) => checked_op!(x, **y as u32, checked_pow),
        (BinaryOperation::Pow, I128(x), U32(y)) => checked_op!(x, **y, checked_pow),

        // PowWrapped operations
        (BinaryOperation::PowWrapped, U8(x), U8(y)) => wrapping_op!(x, **y as u32, wrapping_pow),
        (BinaryOperation::PowWrapped, U8(x), U16(y)) => wrapping_op!(x, **y as u32, wrapping_pow),
        (BinaryOperation::PowWrapped, U8(x), U32(y)) => wrapping_op!(x, **y, wrapping_pow),
        (BinaryOperation::PowWrapped, U16(x), U8(y)) => wrapping_op!(x, **y as u32, wrapping_pow),
        (BinaryOperation::PowWrapped, U16(x), U16(y)) => wrapping_op!(x, **y as u32, wrapping_pow),
        (BinaryOperation::PowWrapped, U16(x), U32(y)) => wrapping_op!(x, **y, wrapping_pow),
        (BinaryOperation::PowWrapped, U32(x), U8(y)) => wrapping_op!(x, **y as u32, wrapping_pow),
        (BinaryOperation::PowWrapped, U32(x), U16(y)) => wrapping_op!(x, **y as u32, wrapping_pow),
        (BinaryOperation::PowWrapped, U32(x), U32(y)) => wrapping_op!(x, **y, wrapping_pow),
        (BinaryOperation::PowWrapped, U64(x), U8(y)) => wrapping_op!(x, **y as u32, wrapping_pow),
        (BinaryOperation::PowWrapped, U64(x), U16(y)) => wrapping_op!(x, **y as u32, wrapping_pow),
        (BinaryOperation::PowWrapped, U64(x), U32(y)) => wrapping_op!(x, **y, wrapping_pow),
        (BinaryOperation::PowWrapped, U128(x), U8(y)) => wrapping_op!(x, **y as u32, wrapping_pow),
        (BinaryOperation::PowWrapped, U128(x), U16(y)) => wrapping_op!(x, **y as u32, wrapping_pow),
        (BinaryOperation::PowWrapped, U128(x), U32(y)) => wrapping_op!(x, **y, wrapping_pow),
        (BinaryOperation::PowWrapped, I8(x), U8(y)) => wrapping_op!(x, **y as u32, wrapping_pow),
        (BinaryOperation::PowWrapped, I8(x), U16(y)) => wrapping_op!(x, **y as u32, wrapping_pow),
        (BinaryOperation::PowWrapped, I8(x), U32(y)) => wrapping_op!(x, **y, wrapping_pow),
        (BinaryOperation::PowWrapped, I16(x), U8(y)) => wrapping_op!(x, **y as u32, wrapping_pow),
        (BinaryOperation::PowWrapped, I16(x), U16(y)) => wrapping_op!(x, **y as u32, wrapping_pow),
        (BinaryOperation::PowWrapped, I16(x), U32(y)) => wrapping_op!(x, **y, wrapping_pow),
        (BinaryOperation::PowWrapped, I32(x), U8(y)) => wrapping_op!(x, **y as u32, wrapping_pow),
        (BinaryOperation::PowWrapped, I32(x), U16(y)) => wrapping_op!(x, **y as u32, wrapping_pow),
        (BinaryOperation::PowWrapped, I32(x), U32(y)) => wrapping_op!(x, **y, wrapping_pow),
        (BinaryOperation::PowWrapped, I64(x), U8(y)) => wrapping_op!(x, **y as u32, wrapping_pow),
        (BinaryOperation::PowWrapped, I64(x), U16(y)) => wrapping_op!(x, **y as u32, wrapping_pow),
        (BinaryOperation::PowWrapped, I64(x), U32(y)) => wrapping_op!(x, **y, wrapping_pow),
        (BinaryOperation::PowWrapped, I128(x), U8(y)) => wrapping_op!(x, **y as u32, wrapping_pow),
        (BinaryOperation::PowWrapped, I128(x), U16(y)) => wrapping_op!(x, **y as u32, wrapping_pow),
        (BinaryOperation::PowWrapped, I128(x), U32(y)) => wrapping_op!(x, **y, wrapping_pow),

        // Rem operations
        (BinaryOperation::Rem, U8(x), U8(y)) => checked_op!(x, **y, checked_rem),
        (BinaryOperation::Rem, U16(x), U16(y)) => checked_op!(x, **y, checked_rem),
        (BinaryOperation::Rem, U32(x), U32(y)) => checked_op!(x, **y, checked_rem),
        (BinaryOperation::Rem, U64(x), U64(y)) => checked_op!(x, **y, checked_rem),
        (BinaryOperation::Rem, U128(x), U128(y)) => checked_op!(x, **y, checked_rem),
        (BinaryOperation::Rem, I8(x), I8(y)) => checked_op!(x, **y, checked_rem),
        (BinaryOperation::Rem, I16(x), I16(y)) => checked_op!(x, **y, checked_rem),
        (BinaryOperation::Rem, I32(x), I32(y)) => checked_op!(x, **y, checked_rem),
        (BinaryOperation::Rem, I64(x), I64(y)) => checked_op!(x, **y, checked_rem),
        (BinaryOperation::Rem, I128(x), I128(y)) => checked_op!(x, **y, checked_rem),

        // RemWrapped operations
        (BinaryOperation::RemWrapped, U8(x), U8(y)) => wrapping_op!(x, **y, wrapping_rem),
        (BinaryOperation::RemWrapped, U16(x), U16(y)) => wrapping_op!(x, **y, wrapping_rem),
        (BinaryOperation::RemWrapped, U32(x), U32(y)) => wrapping_op!(x, **y, wrapping_rem),
        (BinaryOperation::RemWrapped, U64(x), U64(y)) => wrapping_op!(x, **y, wrapping_rem),
        (BinaryOperation::RemWrapped, U128(x), U128(y)) => wrapping_op!(x, **y, wrapping_rem),
        (BinaryOperation::RemWrapped, I8(x), I8(y)) => wrapping_op!(x, **y, wrapping_rem),
        (BinaryOperation::RemWrapped, I16(x), I16(y)) => wrapping_op!(x, **y, wrapping_rem),
        (BinaryOperation::RemWrapped, I32(x), I32(y)) => wrapping_op!(x, **y, wrapping_rem),
        (BinaryOperation::RemWrapped, I64(x), I64(y)) => wrapping_op!(x, **y, wrapping_rem),
        (BinaryOperation::RemWrapped, I128(x), I128(y)) => wrapping_op!(x, **y, wrapping_rem),

        (BinaryOperation::Shl, U8(_), U8(_) | U16(_) | U32(_)) => shl!(8),
        (BinaryOperation::Shl, U16(_), U8(_) | U16(_) | U32(_)) => shl!(16),
        (BinaryOperation::Shl, U32(_), U8(_) | U16(_) | U32(_)) => shl!(32),
        (BinaryOperation::Shl, U64(_), U8(_) | U16(_) | U32(_)) => shl!(64),
        (BinaryOperation::Shl, U128(_), U8(_) | U16(_) | U32(_)) => shl!(128),
        (BinaryOperation::Shl, I8(_), U8(_) | U16(_) | U32(_)) => shl!(8),
        (BinaryOperation::Shl, I16(_), U8(_) | U16(_) | U32(_)) => shl!(16),
        (BinaryOperation::Shl, I32(_), U8(_) | U16(_) | U32(_)) => shl!(32),
        (BinaryOperation::Shl, I64(_), U8(_) | U16(_) | U32(_)) => shl!(64),
        (BinaryOperation::Shl, I128(_), U8(_) | U16(_) | U32(_)) => shl!(128),

        (BinaryOperation::ShlWrapped, U8(x), U8(_) | U16(_) | U32(_)) => wrapping_shl!(x),
        (BinaryOperation::ShlWrapped, U16(x), U8(_) | U16(_) | U32(_)) => wrapping_shl!(x),
        (BinaryOperation::ShlWrapped, U32(x), U8(_) | U16(_) | U32(_)) => wrapping_shl!(x),
        (BinaryOperation::ShlWrapped, U64(x), U8(_) | U16(_) | U32(_)) => wrapping_shl!(x),
        (BinaryOperation::ShlWrapped, U128(x), U8(_) | U16(_) | U32(_)) => wrapping_shl!(x),
        (BinaryOperation::ShlWrapped, I8(x), U8(_) | U16(_) | U32(_)) => wrapping_shl!(x),
        (BinaryOperation::ShlWrapped, I16(x), U8(_) | U16(_) | U32(_)) => wrapping_shl!(x),
        (BinaryOperation::ShlWrapped, I32(x), U8(_) | U16(_) | U32(_)) => wrapping_shl!(x),
        (BinaryOperation::ShlWrapped, I64(x), U8(_) | U16(_) | U32(_)) => wrapping_shl!(x),
        (BinaryOperation::ShlWrapped, I128(x), U8(_) | U16(_) | U32(_)) => wrapping_shl!(x),

        (BinaryOperation::Shr, U8(_), U8(_) | U16(_) | U32(_)) => shr!(8),
        (BinaryOperation::Shr, U16(_), U8(_) | U16(_) | U32(_)) => shr!(16),
        (BinaryOperation::Shr, U32(_), U8(_) | U16(_) | U32(_)) => shr!(32),
        (BinaryOperation::Shr, U64(_), U8(_) | U16(_) | U32(_)) => shr!(64),
        (BinaryOperation::Shr, U128(_), U8(_) | U16(_) | U32(_)) => shr!(128),
        (BinaryOperation::Shr, I8(_), U8(_) | U16(_) | U32(_)) => shr!(8),
        (BinaryOperation::Shr, I16(_), U8(_) | U16(_) | U32(_)) => shr!(16),
        (BinaryOperation::Shr, I32(_), U8(_) | U16(_) | U32(_)) => shr!(32),
        (BinaryOperation::Shr, I64(_), U8(_) | U16(_) | U32(_)) => shr!(64),
        (BinaryOperation::Shr, I128(_), U8(_) | U16(_) | U32(_)) => shr!(128),

        (BinaryOperation::ShrWrapped, U8(x), U8(_) | U16(_) | U32(_)) => wrapping_shr!(x),
        (BinaryOperation::ShrWrapped, U16(x), U8(_) | U16(_) | U32(_)) => wrapping_shr!(x),
        (BinaryOperation::ShrWrapped, U32(x), U8(_) | U16(_) | U32(_)) => wrapping_shr!(x),
        (BinaryOperation::ShrWrapped, U64(x), U8(_) | U16(_) | U32(_)) => wrapping_shr!(x),
        (BinaryOperation::ShrWrapped, U128(x), U8(_) | U16(_) | U32(_)) => wrapping_shr!(x),
        (BinaryOperation::ShrWrapped, I8(x), U8(_) | U16(_) | U32(_)) => wrapping_shr!(x),
        (BinaryOperation::ShrWrapped, I16(x), U8(_) | U16(_) | U32(_)) => wrapping_shr!(x),
        (BinaryOperation::ShrWrapped, I32(x), U8(_) | U16(_) | U32(_)) => wrapping_shr!(x),
        (BinaryOperation::ShrWrapped, I64(x), U8(_) | U16(_) | U32(_)) => wrapping_shr!(x),
        (BinaryOperation::ShrWrapped, I128(x), U8(_) | U16(_) | U32(_)) => wrapping_shr!(x),

        // Sub operations
        (BinaryOperation::Sub, U8(x), U8(y)) => checked_op!(x, **y, checked_sub),
        (BinaryOperation::Sub, U16(x), U16(y)) => checked_op!(x, **y, checked_sub),
        (BinaryOperation::Sub, U32(x), U32(y)) => checked_op!(x, **y, checked_sub),
        (BinaryOperation::Sub, U64(x), U64(y)) => checked_op!(x, **y, checked_sub),
        (BinaryOperation::Sub, U128(x), U128(y)) => checked_op!(x, **y, checked_sub),
        (BinaryOperation::Sub, I8(x), I8(y)) => checked_op!(x, **y, checked_sub),
        (BinaryOperation::Sub, I16(x), I16(y)) => checked_op!(x, **y, checked_sub),
        (BinaryOperation::Sub, I32(x), I32(y)) => checked_op!(x, **y, checked_sub),
        (BinaryOperation::Sub, I64(x), I64(y)) => checked_op!(x, **y, checked_sub),
        (BinaryOperation::Sub, I128(x), I128(y)) => checked_op!(x, **y, checked_sub),
        (BinaryOperation::Sub, Group(x), Group(y)) => (*x - *y).into(),
        (BinaryOperation::Sub, Field(x), Field(y)) => (*x - *y).into(),

        // SubWrapped operations
        (BinaryOperation::SubWrapped, U8(x), U8(y)) => wrapping_op!(x, **y, wrapping_sub),
        (BinaryOperation::SubWrapped, U16(x), U16(y)) => wrapping_op!(x, **y, wrapping_sub),
        (BinaryOperation::SubWrapped, U32(x), U32(y)) => wrapping_op!(x, **y, wrapping_sub),
        (BinaryOperation::SubWrapped, U64(x), U64(y)) => wrapping_op!(x, **y, wrapping_sub),
        (BinaryOperation::SubWrapped, U128(x), U128(y)) => wrapping_op!(x, **y, wrapping_sub),
        (BinaryOperation::SubWrapped, I8(x), I8(y)) => wrapping_op!(x, **y, wrapping_sub),
        (BinaryOperation::SubWrapped, I16(x), I16(y)) => wrapping_op!(x, **y, wrapping_sub),
        (BinaryOperation::SubWrapped, I32(x), I32(y)) => wrapping_op!(x, **y, wrapping_sub),
        (BinaryOperation::SubWrapped, I64(x), I64(y)) => wrapping_op!(x, **y, wrapping_sub),
        (BinaryOperation::SubWrapped, I128(x), I128(y)) => wrapping_op!(x, **y, wrapping_sub),

        // Xor operations
        (BinaryOperation::Xor, Boolean(x), Boolean(y)) => (*x ^ *y).into(),
        (BinaryOperation::Xor, U8(x), U8(y)) => (*x ^ *y).into(),
        (BinaryOperation::Xor, U16(x), U16(y)) => (*x ^ *y).into(),
        (BinaryOperation::Xor, U32(x), U32(y)) => (*x ^ *y).into(),
        (BinaryOperation::Xor, U64(x), U64(y)) => (*x ^ *y).into(),
        (BinaryOperation::Xor, U128(x), U128(y)) => (*x ^ *y).into(),
        (BinaryOperation::Xor, I8(x), I8(y)) => (*x ^ *y).into(),
        (BinaryOperation::Xor, I16(x), I16(y)) => (*x ^ *y).into(),
        (BinaryOperation::Xor, I32(x), I32(y)) => (*x ^ *y).into(),
        (BinaryOperation::Xor, I64(x), I64(y)) => (*x ^ *y).into(),
        (BinaryOperation::Xor, I128(x), I128(y)) => (*x ^ *y).into(),

        // Default case for unsupported operations
        _ => halt!(span, "Type error"),
    };

    Ok(value)
}
