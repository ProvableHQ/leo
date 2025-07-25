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
    FromStrRadix as _,
    IntegerType,
    Literal,
    LiteralVariant,
    Node as _,
    Type,
    UnaryOperation,
    halt,
    halt_no_span,
    interpreter_value::{
        StructContents,
        SvmAddress,
        SvmBoolean,
        SvmField,
        SvmGroup,
        SvmIdentifier,
        SvmInteger,
        SvmScalar,
        Value,
        util::ExpectTc,
    },
    tc_fail,
};
use leo_errors::{InterpreterHalt, Result};
use leo_span::Span;

use snarkvm::prelude::{
    Cast,
    Double as _,
    FromBits as _,
    Inverse as _,
    Literal as SvmLiteral,
    Plaintext,
    Pow as _,
    ProgramID,
    Square as _,
    SquareRoot as _,
    // Signature as SvmSignatureParam,
    TestnetV0,
    ToBits,
};
use std::str::FromStr as _;

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
            (Scalar(x), Scalar(y)) => x < y,
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
            (Scalar(x), Scalar(y)) => x > y,
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
            (Scalar(x), Scalar(y)) => x == y,
            (Address(x), Address(y)) => x == y,
            (Struct(x), Struct(y)) => {
                // They must have the same name
                if x.path != y.path {
                    return Ok(false);
                }

                // They must have the same number of fields
                if x.contents.len() != y.contents.len() {
                    return Ok(false);
                }

                // For each field in x, find the matching field in y and compare
                for (lhs_name, lhs_value) in x.contents.iter() {
                    match y.contents.get(lhs_name) {
                        Some(rhs_value) => match lhs_value.eq(rhs_value) {
                            Ok(true) => {}                 // Fields match, continue checking
                            Ok(false) => return Ok(false), // Values differ
                            Err(e) => return Err(e),       // Error in comparison
                        },
                        None => return Ok(false), // Field missing in y
                    }
                }

                true
            }
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

    /// Resolves an unsuffixed literal to a typed `Value` using the provided optional `Type`. If the value is unsuffixed
    /// and a type is provided, parses the string into the corresponding `Value` variant. Handles integers of various
    /// widths and special types like `Field`, `Group`, and `Scalar`. If no type is given or the value is already typed,
    /// returns the original value. Returns an error if type inference is not possible or parsing fails.
    pub fn resolve_if_unsuffixed(&self, ty: &Option<Type>, span: Span) -> Result<Value> {
        if let Value::Unsuffixed(s) = self {
            if let Some(ty) = ty {
                let value = match ty {
                    Type::Integer(IntegerType::U8) => {
                        let s = s.replace("_", "");
                        Value::U8(u8::from_str_by_radix(&s).expect("Parsing guarantees this works."))
                    }
                    Type::Integer(IntegerType::U16) => {
                        let s = s.replace("_", "");
                        Value::U16(u16::from_str_by_radix(&s).expect("Parsing guarantees this works."))
                    }
                    Type::Integer(IntegerType::U32) => {
                        let s = s.replace("_", "");
                        Value::U32(u32::from_str_by_radix(&s).expect("Parsing guarantees this works."))
                    }
                    Type::Integer(IntegerType::U64) => {
                        let s = s.replace("_", "");
                        Value::U64(u64::from_str_by_radix(&s).expect("Parsing guarantees this works."))
                    }
                    Type::Integer(IntegerType::U128) => {
                        let s = s.replace("_", "");
                        Value::U128(u128::from_str_by_radix(&s).expect("Parsing guarantees this works."))
                    }
                    Type::Integer(IntegerType::I8) => {
                        let s = s.replace("_", "");
                        Value::I8(i8::from_str_by_radix(&s).expect("Parsing guarantees this works."))
                    }
                    Type::Integer(IntegerType::I16) => {
                        let s = s.replace("_", "");
                        Value::I16(i16::from_str_by_radix(&s).expect("Parsing guarantees this works."))
                    }
                    Type::Integer(IntegerType::I32) => {
                        let s = s.replace("_", "");
                        Value::I32(i32::from_str_by_radix(&s).expect("Parsing guarantees this works."))
                    }
                    Type::Integer(IntegerType::I64) => {
                        let s = s.replace("_", "");
                        Value::I64(i64::from_str_by_radix(&s).expect("Parsing guarantees this works."))
                    }
                    Type::Integer(IntegerType::I128) => {
                        let s = s.replace("_", "");
                        Value::I128(i128::from_str_by_radix(&s).expect("Parsing guarantees this works."))
                    }
                    Type::Field => Value::Field(prepare_snarkvm_string(s, "field").parse().expect_tc(span)?),
                    Type::Group => Value::Group(prepare_snarkvm_string(s, "group").parse().expect_tc(span)?),
                    Type::Scalar => Value::Scalar(prepare_snarkvm_string(s, "scalar").parse().expect_tc(span)?),
                    _ => {
                        halt!(span, "cannot infer type of unsuffixed literal")
                    }
                };
                Ok(value)
            } else {
                Ok(self.clone())
            }
        } else {
            Ok(self.clone())
        }
    }
}

// SnarkVM will not parse fields, groups, or scalars with leading zeros, so we strip them out.
fn prepare_snarkvm_string(s: &str, suffix: &str) -> String {
    // If there's a `-`, separate it from the rest of the string.
    let (neg, rest) = s.strip_prefix("-").map(|rest| ("-", rest)).unwrap_or(("", s));
    // Remove leading zeros.
    let mut rest = rest.trim_start_matches('0');
    if rest.is_empty() {
        rest = "0";
    }
    format!("{neg}{rest}{suffix}")
}

impl ToBits for Value {
    fn write_bits_le(&self, vec: &mut Vec<bool>) {
        use Value::*;

        let plaintext: Plaintext<TestnetV0> = match self {
            Bool(x) => SvmLiteral::Boolean(SvmBoolean::new(*x)).into(),
            U8(x) => SvmLiteral::U8(SvmInteger::new(*x)).into(),
            U16(x) => SvmLiteral::U16(SvmInteger::new(*x)).into(),
            U32(x) => SvmLiteral::U32(SvmInteger::new(*x)).into(),
            U64(x) => SvmLiteral::U64(SvmInteger::new(*x)).into(),
            U128(x) => SvmLiteral::U128(SvmInteger::new(*x)).into(),
            I8(x) => SvmLiteral::I8(SvmInteger::new(*x)).into(),
            I16(x) => SvmLiteral::I16(SvmInteger::new(*x)).into(),
            I32(x) => SvmLiteral::I32(SvmInteger::new(*x)).into(),
            I64(x) => SvmLiteral::I64(SvmInteger::new(*x)).into(),
            I128(x) => SvmLiteral::I128(SvmInteger::new(*x)).into(),
            Group(x) => SvmLiteral::Group(*x).into(),
            Field(x) => SvmLiteral::Field(*x).into(),
            Scalar(x) => SvmLiteral::Scalar(*x).into(),
            Address(x) => SvmLiteral::Address(*x).into(),
            Struct(StructContents { path: _, contents }) => {
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

pub fn literal_to_value(literal: &Literal, expected_ty: &Option<Type>) -> Result<Value> {
    Ok(match &literal.variant {
        LiteralVariant::Boolean(b) => Value::Bool(*b),
        LiteralVariant::Integer(IntegerType::U8, s, ..) => {
            let s = s.replace("_", "");
            Value::U8(u8::from_str_by_radix(&s).expect("Parsing guarantees this works."))
        }
        LiteralVariant::Integer(IntegerType::U16, s, ..) => {
            let s = s.replace("_", "");
            Value::U16(u16::from_str_by_radix(&s).expect("Parsing guarantees this works."))
        }
        LiteralVariant::Integer(IntegerType::U32, s, ..) => {
            let s = s.replace("_", "");
            Value::U32(u32::from_str_by_radix(&s).expect("Parsing guarantees this works."))
        }
        LiteralVariant::Integer(IntegerType::U64, s, ..) => {
            let s = s.replace("_", "");
            Value::U64(u64::from_str_by_radix(&s).expect("Parsing guarantees this works."))
        }
        LiteralVariant::Integer(IntegerType::U128, s, ..) => {
            let s = s.replace("_", "");
            Value::U128(u128::from_str_by_radix(&s).expect("Parsing guarantees this works."))
        }
        LiteralVariant::Integer(IntegerType::I8, s, ..) => {
            let s = s.replace("_", "");
            Value::I8(i8::from_str_by_radix(&s).expect("Parsing guarantees this works."))
        }
        LiteralVariant::Integer(IntegerType::I16, s, ..) => {
            let s = s.replace("_", "");
            Value::I16(i16::from_str_by_radix(&s).expect("Parsing guarantees this works."))
        }
        LiteralVariant::Integer(IntegerType::I32, s, ..) => {
            let s = s.replace("_", "");
            Value::I32(i32::from_str_by_radix(&s).expect("Parsing guarantees this works."))
        }
        LiteralVariant::Integer(IntegerType::I64, s, ..) => {
            let s = s.replace("_", "");
            Value::I64(i64::from_str_by_radix(&s).expect("Parsing guarantees this works."))
        }
        LiteralVariant::Integer(IntegerType::I128, s, ..) => {
            let s = s.replace("_", "");
            Value::I128(i128::from_str_by_radix(&s).expect("Parsing guarantees this works."))
        }
        LiteralVariant::Field(s) => Value::Field(prepare_snarkvm_string(s, "field").parse().expect_tc(literal.span())?),
        LiteralVariant::Group(s) => Value::Group(prepare_snarkvm_string(s, "group").parse().expect_tc(literal.span())?),
        LiteralVariant::Address(s) => {
            if s.ends_with(".aleo") {
                let program_id = ProgramID::from_str(s)?;
                Value::Address(program_id.to_address()?)
            } else {
                Value::Address(s.parse().expect_tc(literal.span())?)
            }
        }
        LiteralVariant::Scalar(s) => {
            Value::Scalar(prepare_snarkvm_string(s, "scalar").parse().expect_tc(literal.span())?)
        }
        LiteralVariant::Unsuffixed(s) => {
            Value::Unsuffixed(s.clone()).resolve_if_unsuffixed(expected_ty, literal.span())?
        }
        LiteralVariant::String(_) => tc_fail!(),
    })
}

/// Resolves an unsuffixed operand for a unary operation by inferring its type based on the operation and an optional
/// expected type. Uses predefined types (`Field` or `Group`) for specific operations, otherwise defaults to the expected
/// type if available. Returns the resolved `Value` or an error if type resolution fails.
fn resolve_unsuffixed_unary_op_operand(
    val: &Value,
    op: &UnaryOperation,
    expected_ty: &Option<Type>,
    span: &Span,
) -> Result<Value> {
    match op {
        UnaryOperation::Inverse | UnaryOperation::Square | UnaryOperation::SquareRoot => {
            // These ops only take a `field` and return a `field`
            val.resolve_if_unsuffixed(&Some(Type::Field), *span)
        }
        UnaryOperation::ToXCoordinate | UnaryOperation::ToYCoordinate => {
            // These ops only take a `Group`
            val.resolve_if_unsuffixed(&Some(Type::Group), *span)
        }
        _ => {
            // All other unary ops take the same type as the their return type
            val.resolve_if_unsuffixed(expected_ty, *span)
        }
    }
}

/// Evaluate a unary operation.
pub fn evaluate_unary(span: Span, op: UnaryOperation, value: &Value, expected_ty: &Option<Type>) -> Result<Value> {
    let value = resolve_unsuffixed_unary_op_operand(value, &op, expected_ty, &span)?;
    let value_result = match op {
        UnaryOperation::Abs => match &value {
            Value::I8(x) => {
                if *x == i8::MIN {
                    halt!(span, "abs overflow");
                } else {
                    Value::I8(x.abs())
                }
            }
            Value::I16(x) => {
                if *x == i16::MIN {
                    halt!(span, "abs overflow");
                } else {
                    Value::I16(x.abs())
                }
            }
            Value::I32(x) => {
                if *x == i32::MIN {
                    halt!(span, "abs overflow");
                } else {
                    Value::I32(x.abs())
                }
            }
            Value::I64(x) => {
                if *x == i64::MIN {
                    halt!(span, "abs overflow");
                } else {
                    Value::I64(x.abs())
                }
            }
            Value::I128(x) => {
                if *x == i128::MIN {
                    halt!(span, "abs overflow");
                } else {
                    Value::I128(x.abs())
                }
            }
            _ => halt!(span, "Type error"),
        },
        UnaryOperation::AbsWrapped => match value {
            Value::I8(x) => Value::I8(x.unsigned_abs() as i8),
            Value::I16(x) => Value::I16(x.unsigned_abs() as i16),
            Value::I32(x) => Value::I32(x.unsigned_abs() as i32),
            Value::I64(x) => Value::I64(x.unsigned_abs() as i64),
            Value::I128(x) => Value::I128(x.unsigned_abs() as i128),
            _ => halt!(span, "Type error"),
        },
        UnaryOperation::Double => match value {
            Value::Field(x) => Value::Field(x.double()),
            Value::Group(x) => Value::Group(x.double()),
            _ => halt!(span, "Type error"),
        },
        UnaryOperation::Inverse => match value {
            Value::Field(x) => {
                let Ok(y) = x.inverse() else {
                    halt!(span, "attempt to invert 0field");
                };
                Value::Field(y)
            }
            _ => halt!(span, "Can only invert fields"),
        },
        UnaryOperation::Negate => match value {
            Value::I8(x) => match x.checked_neg() {
                None => halt!(span, "negation overflow"),
                Some(y) => Value::I8(y),
            },
            Value::I16(x) => match x.checked_neg() {
                None => halt!(span, "negation overflow"),
                Some(y) => Value::I16(y),
            },
            Value::I32(x) => match x.checked_neg() {
                None => halt!(span, "negation overflow"),
                Some(y) => Value::I32(y),
            },
            Value::I64(x) => match x.checked_neg() {
                None => halt!(span, "negation overflow"),
                Some(y) => Value::I64(y),
            },
            Value::I128(x) => match x.checked_neg() {
                None => halt!(span, "negation overflow"),
                Some(y) => Value::I128(y),
            },
            Value::Group(x) => Value::Group(-x),
            Value::Field(x) => Value::Field(-x),
            _ => halt!(span, "Type error"),
        },
        UnaryOperation::Not => match value {
            Value::Bool(x) => Value::Bool(!x),
            Value::U8(x) => Value::U8(!x),
            Value::U16(x) => Value::U16(!x),
            Value::U32(x) => Value::U32(!x),
            Value::U64(x) => Value::U64(!x),
            Value::U128(x) => Value::U128(!x),
            Value::I8(x) => Value::I8(!x),
            Value::I16(x) => Value::I16(!x),
            Value::I32(x) => Value::I32(!x),
            Value::I64(x) => Value::I64(!x),
            Value::I128(x) => Value::I128(!x),
            _ => halt!(span, "Type error"),
        },
        UnaryOperation::Square => match value {
            Value::Field(x) => Value::Field(x.square()),
            _ => halt!(span, "Can only square fields"),
        },
        UnaryOperation::SquareRoot => match value {
            Value::Field(x) => {
                let Ok(y) = x.square_root() else {
                    halt!(span, "square root failure");
                };
                Value::Field(y)
            }
            _ => halt!(span, "Can only apply square_root to fields"),
        },
        UnaryOperation::ToXCoordinate => match value {
            Value::Group(x) => Value::Field(x.to_x_coordinate()),
            _ => tc_fail!(),
        },
        UnaryOperation::ToYCoordinate => match value {
            Value::Group(x) => Value::Field(x.to_y_coordinate()),
            _ => tc_fail!(),
        },
    };

    Ok(value_result)
}

/// Resolves unsuffixed numeric operands for binary operations by inferring types based on the other operand, the
/// operation type, and an optional expected type. Handles special cases for multiplication and exponentiation with
/// additional logic for `Group`, `Scalar`, and `Field` type inference. Ensures that both operands are resolved to
/// compatible types before evaluation. Returns a tuple of resolved `Value`s or an error if resolution fails.
fn resolve_unsuffixed_binary_op_operands(
    lhs: &Value,
    rhs: &Value,
    op: &BinaryOperation,
    expected_ty: &Option<Type>,
    span: &Span,
) -> Result<(Value, Value)> {
    use Type::*;

    let lhs_ty = lhs.get_numeric_type();
    let rhs_ty = rhs.get_numeric_type();

    Ok(match op {
        BinaryOperation::Mul => {
            // For a `Mul`, if on operand is a Scalar, then the other must ba a `Group`. Otherwise, both ops must have
            // the same type as the return type of the multiplication.
            let lhs = match rhs_ty {
                Some(Group) => lhs.resolve_if_unsuffixed(&Some(Scalar), *span)?,
                Some(Scalar) => lhs.resolve_if_unsuffixed(&Some(Group), *span)?,
                _ => lhs.resolve_if_unsuffixed(&rhs_ty, *span)?.resolve_if_unsuffixed(expected_ty, *span)?,
            };

            let rhs = match lhs_ty {
                Some(Group) => rhs.resolve_if_unsuffixed(&Some(Scalar), *span)?,
                Some(Scalar) => rhs.resolve_if_unsuffixed(&Some(Group), *span)?,
                _ => rhs.resolve_if_unsuffixed(&lhs_ty, *span)?.resolve_if_unsuffixed(expected_ty, *span)?,
            };

            (lhs, rhs)
        }
        BinaryOperation::Pow => {
            // For a `Pow`, if one operand is a `Field`, then the other must also be a `Field.
            // Otherwise, only the `lhs` must match the return type.
            let lhs_resolved = lhs
                .resolve_if_unsuffixed(&rhs_ty.filter(|ty| matches!(ty, Type::Field)), *span)?
                .resolve_if_unsuffixed(expected_ty, *span)?;

            let rhs_resolved = rhs.resolve_if_unsuffixed(&lhs_ty.filter(|ty| matches!(ty, Type::Field)), *span)?;

            (lhs_resolved, rhs_resolved)
        }
        _ => (
            lhs.resolve_if_unsuffixed(&rhs_ty, *span)?.resolve_if_unsuffixed(expected_ty, *span)?,
            rhs.resolve_if_unsuffixed(&lhs_ty, *span)?.resolve_if_unsuffixed(expected_ty, *span)?,
        ),
    })
}

/// Evaluate a binary operation.
pub fn evaluate_binary(
    span: Span,
    op: BinaryOperation,
    lhs: &Value,
    rhs: &Value,
    expected_ty: &Option<Type>,
) -> Result<Value> {
    let (lhs, rhs) = resolve_unsuffixed_binary_op_operands(lhs, rhs, &op, expected_ty, &span)?;
    let value = match op {
        BinaryOperation::Add => {
            let Some(value) = (match (lhs, rhs) {
                (Value::U8(x), Value::U8(y)) => x.checked_add(y).map(Value::U8),
                (Value::U16(x), Value::U16(y)) => x.checked_add(y).map(Value::U16),
                (Value::U32(x), Value::U32(y)) => x.checked_add(y).map(Value::U32),
                (Value::U64(x), Value::U64(y)) => x.checked_add(y).map(Value::U64),
                (Value::U128(x), Value::U128(y)) => x.checked_add(y).map(Value::U128),
                (Value::I8(x), Value::I8(y)) => x.checked_add(y).map(Value::I8),
                (Value::I16(x), Value::I16(y)) => x.checked_add(y).map(Value::I16),
                (Value::I32(x), Value::I32(y)) => x.checked_add(y).map(Value::I32),
                (Value::I64(x), Value::I64(y)) => x.checked_add(y).map(Value::I64),
                (Value::I128(x), Value::I128(y)) => x.checked_add(y).map(Value::I128),
                (Value::Group(x), Value::Group(y)) => Some(Value::Group(x + y)),
                (Value::Field(x), Value::Field(y)) => Some(Value::Field(x + y)),
                (Value::Scalar(x), Value::Scalar(y)) => Some(Value::Scalar(x + y)),
                _ => halt!(span, "Type error"),
            }) else {
                halt!(span, "add overflow");
            };
            value
        }
        BinaryOperation::AddWrapped => match (lhs, rhs) {
            (Value::U8(x), Value::U8(y)) => Value::U8(x.wrapping_add(y)),
            (Value::U16(x), Value::U16(y)) => Value::U16(x.wrapping_add(y)),
            (Value::U32(x), Value::U32(y)) => Value::U32(x.wrapping_add(y)),
            (Value::U64(x), Value::U64(y)) => Value::U64(x.wrapping_add(y)),
            (Value::U128(x), Value::U128(y)) => Value::U128(x.wrapping_add(y)),
            (Value::I8(x), Value::I8(y)) => Value::I8(x.wrapping_add(y)),
            (Value::I16(x), Value::I16(y)) => Value::I16(x.wrapping_add(y)),
            (Value::I32(x), Value::I32(y)) => Value::I32(x.wrapping_add(y)),
            (Value::I64(x), Value::I64(y)) => Value::I64(x.wrapping_add(y)),
            (Value::I128(x), Value::I128(y)) => Value::I128(x.wrapping_add(y)),
            _ => halt!(span, "Type error"),
        },
        BinaryOperation::And => match (lhs, rhs) {
            (Value::Bool(x), Value::Bool(y)) => Value::Bool(x && y),
            _ => halt!(span, "Type error"),
        },
        BinaryOperation::BitwiseAnd => match (lhs, rhs) {
            (Value::Bool(x), Value::Bool(y)) => Value::Bool(x & y),
            (Value::U8(x), Value::U8(y)) => Value::U8(x & y),
            (Value::U16(x), Value::U16(y)) => Value::U16(x & y),
            (Value::U32(x), Value::U32(y)) => Value::U32(x & y),
            (Value::U64(x), Value::U64(y)) => Value::U64(x & y),
            (Value::U128(x), Value::U128(y)) => Value::U128(x & y),
            (Value::I8(x), Value::I8(y)) => Value::I8(x & y),
            (Value::I16(x), Value::I16(y)) => Value::I16(x & y),
            (Value::I32(x), Value::I32(y)) => Value::I32(x & y),
            (Value::I64(x), Value::I64(y)) => Value::I64(x & y),
            (Value::I128(x), Value::I128(y)) => Value::I128(x & y),
            _ => halt!(span, "Type error"),
        },
        BinaryOperation::Div => {
            let Some(value) = (match (lhs, rhs) {
                (Value::U8(x), Value::U8(y)) => x.checked_div(y).map(Value::U8),
                (Value::U16(x), Value::U16(y)) => x.checked_div(y).map(Value::U16),
                (Value::U32(x), Value::U32(y)) => x.checked_div(y).map(Value::U32),
                (Value::U64(x), Value::U64(y)) => x.checked_div(y).map(Value::U64),
                (Value::U128(x), Value::U128(y)) => x.checked_div(y).map(Value::U128),
                (Value::I8(x), Value::I8(y)) => x.checked_div(y).map(Value::I8),
                (Value::I16(x), Value::I16(y)) => x.checked_div(y).map(Value::I16),
                (Value::I32(x), Value::I32(y)) => x.checked_div(y).map(Value::I32),
                (Value::I64(x), Value::I64(y)) => x.checked_div(y).map(Value::I64),
                (Value::I128(x), Value::I128(y)) => x.checked_div(y).map(Value::I128),
                (Value::Field(x), Value::Field(y)) => y.inverse().map(|y| Value::Field(x * y)).ok(),
                _ => halt!(span, "Type error"),
            }) else {
                halt!(span, "div overflow");
            };
            value
        }
        BinaryOperation::DivWrapped => match (lhs, rhs) {
            (Value::U8(_), Value::U8(0))
            | (Value::U16(_), Value::U16(0))
            | (Value::U32(_), Value::U32(0))
            | (Value::U64(_), Value::U64(0))
            | (Value::U128(_), Value::U128(0))
            | (Value::I8(_), Value::I8(0))
            | (Value::I16(_), Value::I16(0))
            | (Value::I32(_), Value::I32(0))
            | (Value::I64(_), Value::I64(0))
            | (Value::I128(_), Value::I128(0)) => halt!(span, "divide by 0"),
            (Value::U8(x), Value::U8(y)) => Value::U8(x.wrapping_div(y)),
            (Value::U16(x), Value::U16(y)) => Value::U16(x.wrapping_div(y)),
            (Value::U32(x), Value::U32(y)) => Value::U32(x.wrapping_div(y)),
            (Value::U64(x), Value::U64(y)) => Value::U64(x.wrapping_div(y)),
            (Value::U128(x), Value::U128(y)) => Value::U128(x.wrapping_div(y)),
            (Value::I8(x), Value::I8(y)) => Value::I8(x.wrapping_div(y)),
            (Value::I16(x), Value::I16(y)) => Value::I16(x.wrapping_div(y)),
            (Value::I32(x), Value::I32(y)) => Value::I32(x.wrapping_div(y)),
            (Value::I64(x), Value::I64(y)) => Value::I64(x.wrapping_div(y)),
            (Value::I128(x), Value::I128(y)) => Value::I128(x.wrapping_div(y)),
            _ => halt!(span, "Type error"),
        },
        BinaryOperation::Eq => Value::Bool(lhs.eq(&rhs)?),
        BinaryOperation::Gte => Value::Bool(lhs.gte(&rhs)?),
        BinaryOperation::Gt => Value::Bool(lhs.gt(&rhs)?),
        BinaryOperation::Lte => Value::Bool(lhs.lte(&rhs)?),
        BinaryOperation::Lt => Value::Bool(lhs.lt(&rhs)?),
        BinaryOperation::Mod => {
            let Some(value) = (match (lhs, rhs) {
                (Value::U8(x), Value::U8(y)) => x.checked_rem(y).map(Value::U8),
                (Value::U16(x), Value::U16(y)) => x.checked_rem(y).map(Value::U16),
                (Value::U32(x), Value::U32(y)) => x.checked_rem(y).map(Value::U32),
                (Value::U64(x), Value::U64(y)) => x.checked_rem(y).map(Value::U64),
                (Value::U128(x), Value::U128(y)) => x.checked_rem(y).map(Value::U128),
                _ => halt!(span, "Type error"),
            }) else {
                halt!(span, "mod overflow");
            };
            value
        }
        BinaryOperation::Mul => {
            let Some(value) = (match (lhs, rhs) {
                (Value::U8(x), Value::U8(y)) => x.checked_mul(y).map(Value::U8),
                (Value::U16(x), Value::U16(y)) => x.checked_mul(y).map(Value::U16),
                (Value::U32(x), Value::U32(y)) => x.checked_mul(y).map(Value::U32),
                (Value::U64(x), Value::U64(y)) => x.checked_mul(y).map(Value::U64),
                (Value::U128(x), Value::U128(y)) => x.checked_mul(y).map(Value::U128),
                (Value::I8(x), Value::I8(y)) => x.checked_mul(y).map(Value::I8),
                (Value::I16(x), Value::I16(y)) => x.checked_mul(y).map(Value::I16),
                (Value::I32(x), Value::I32(y)) => x.checked_mul(y).map(Value::I32),
                (Value::I64(x), Value::I64(y)) => x.checked_mul(y).map(Value::I64),
                (Value::I128(x), Value::I128(y)) => x.checked_mul(y).map(Value::I128),
                (Value::Field(x), Value::Field(y)) => Some(Value::Field(x * y)),
                (Value::Group(x), Value::Scalar(y)) => Some(Value::Group(x * y)),
                (Value::Scalar(x), Value::Group(y)) => Some(Value::Group(x * y)),
                _ => halt!(span, "Type error"),
            }) else {
                halt!(span, "mul overflow");
            };
            value
        }
        BinaryOperation::MulWrapped => match (lhs, rhs) {
            (Value::U8(x), Value::U8(y)) => Value::U8(x.wrapping_mul(y)),
            (Value::U16(x), Value::U16(y)) => Value::U16(x.wrapping_mul(y)),
            (Value::U32(x), Value::U32(y)) => Value::U32(x.wrapping_mul(y)),
            (Value::U64(x), Value::U64(y)) => Value::U64(x.wrapping_mul(y)),
            (Value::U128(x), Value::U128(y)) => Value::U128(x.wrapping_mul(y)),
            (Value::I8(x), Value::I8(y)) => Value::I8(x.wrapping_mul(y)),
            (Value::I16(x), Value::I16(y)) => Value::I16(x.wrapping_mul(y)),
            (Value::I32(x), Value::I32(y)) => Value::I32(x.wrapping_mul(y)),
            (Value::I64(x), Value::I64(y)) => Value::I64(x.wrapping_mul(y)),
            (Value::I128(x), Value::I128(y)) => Value::I128(x.wrapping_mul(y)),
            _ => halt!(span, "Type error"),
        },

        BinaryOperation::Nand => match (lhs, rhs) {
            (Value::Bool(x), Value::Bool(y)) => Value::Bool(!(x & y)),
            _ => halt!(span, "Type error"),
        },
        BinaryOperation::Neq => Value::Bool(lhs.neq(&rhs)?),
        BinaryOperation::Nor => match (lhs, rhs) {
            (Value::Bool(x), Value::Bool(y)) => Value::Bool(!(x | y)),
            _ => halt!(span, "Type error"),
        },
        BinaryOperation::Or => match (lhs, rhs) {
            (Value::Bool(x), Value::Bool(y)) => Value::Bool(x | y),
            _ => halt!(span, "Type error"),
        },
        BinaryOperation::BitwiseOr => match (lhs, rhs) {
            (Value::Bool(x), Value::Bool(y)) => Value::Bool(x | y),
            (Value::U8(x), Value::U8(y)) => Value::U8(x | y),
            (Value::U16(x), Value::U16(y)) => Value::U16(x | y),
            (Value::U32(x), Value::U32(y)) => Value::U32(x | y),
            (Value::U64(x), Value::U64(y)) => Value::U64(x | y),
            (Value::U128(x), Value::U128(y)) => Value::U128(x | y),
            (Value::I8(x), Value::I8(y)) => Value::I8(x | y),
            (Value::I16(x), Value::I16(y)) => Value::I16(x | y),
            (Value::I32(x), Value::I32(y)) => Value::I32(x | y),
            (Value::I64(x), Value::I64(y)) => Value::I64(x | y),
            (Value::I128(x), Value::I128(y)) => Value::I128(x | y),
            _ => halt!(span, "Type error"),
        },
        BinaryOperation::Pow => {
            if let (Value::Field(x), Value::Field(y)) = (&lhs, &rhs) {
                Value::Field(x.pow(y))
            } else {
                let rhs: u32 = match rhs {
                    Value::U8(y) => (y).into(),
                    Value::U16(y) => (y).into(),
                    Value::U32(y) => y,
                    _ => tc_fail!(),
                };

                let Some(value) = (match lhs {
                    Value::U8(x) => x.checked_pow(rhs).map(Value::U8),
                    Value::U16(x) => x.checked_pow(rhs).map(Value::U16),
                    Value::U32(x) => x.checked_pow(rhs).map(Value::U32),
                    Value::U64(x) => x.checked_pow(rhs).map(Value::U64),
                    Value::U128(x) => x.checked_pow(rhs).map(Value::U128),
                    Value::I8(x) => x.checked_pow(rhs).map(Value::I8),
                    Value::I16(x) => x.checked_pow(rhs).map(Value::I16),
                    Value::I32(x) => x.checked_pow(rhs).map(Value::I32),
                    Value::I64(x) => x.checked_pow(rhs).map(Value::I64),
                    Value::I128(x) => x.checked_pow(rhs).map(Value::I128),
                    _ => halt!(span, "Type error"),
                }) else {
                    halt!(span, "pow overflow");
                };
                value
            }
        }
        BinaryOperation::PowWrapped => {
            let rhs: u32 = match rhs {
                Value::U8(y) => (y).into(),
                Value::U16(y) => (y).into(),
                Value::U32(y) => y,
                _ => halt!(span, "Type error"),
            };

            match lhs {
                Value::U8(x) => Value::U8(x.wrapping_pow(rhs)),
                Value::U16(x) => Value::U16(x.wrapping_pow(rhs)),
                Value::U32(x) => Value::U32(x.wrapping_pow(rhs)),
                Value::U64(x) => Value::U64(x.wrapping_pow(rhs)),
                Value::U128(x) => Value::U128(x.wrapping_pow(rhs)),
                Value::I8(x) => Value::I8(x.wrapping_pow(rhs)),
                Value::I16(x) => Value::I16(x.wrapping_pow(rhs)),
                Value::I32(x) => Value::I32(x.wrapping_pow(rhs)),
                Value::I64(x) => Value::I64(x.wrapping_pow(rhs)),
                Value::I128(x) => Value::I128(x.wrapping_pow(rhs)),
                _ => halt!(span, "Type error"),
            }
        }
        BinaryOperation::Rem => {
            let Some(value) = (match (lhs, rhs) {
                (Value::U8(x), Value::U8(y)) => x.checked_rem(y).map(Value::U8),
                (Value::U16(x), Value::U16(y)) => x.checked_rem(y).map(Value::U16),
                (Value::U32(x), Value::U32(y)) => x.checked_rem(y).map(Value::U32),
                (Value::U64(x), Value::U64(y)) => x.checked_rem(y).map(Value::U64),
                (Value::U128(x), Value::U128(y)) => x.checked_rem(y).map(Value::U128),
                (Value::I8(x), Value::I8(y)) => x.checked_rem(y).map(Value::I8),
                (Value::I16(x), Value::I16(y)) => x.checked_rem(y).map(Value::I16),
                (Value::I32(x), Value::I32(y)) => x.checked_rem(y).map(Value::I32),
                (Value::I64(x), Value::I64(y)) => x.checked_rem(y).map(Value::I64),
                (Value::I128(x), Value::I128(y)) => x.checked_rem(y).map(Value::I128),
                _ => halt!(span, "Type error"),
            }) else {
                halt!(span, "rem error");
            };
            value
        }
        BinaryOperation::RemWrapped => match (lhs, rhs) {
            (Value::U8(_), Value::U8(0))
            | (Value::U16(_), Value::U16(0))
            | (Value::U32(_), Value::U32(0))
            | (Value::U64(_), Value::U64(0))
            | (Value::U128(_), Value::U128(0))
            | (Value::I8(_), Value::I8(0))
            | (Value::I16(_), Value::I16(0))
            | (Value::I32(_), Value::I32(0))
            | (Value::I64(_), Value::I64(0))
            | (Value::I128(_), Value::I128(0)) => halt!(span, "rem by 0"),
            (Value::U8(x), Value::U8(y)) => Value::U8(x.wrapping_rem(y)),
            (Value::U16(x), Value::U16(y)) => Value::U16(x.wrapping_rem(y)),
            (Value::U32(x), Value::U32(y)) => Value::U32(x.wrapping_rem(y)),
            (Value::U64(x), Value::U64(y)) => Value::U64(x.wrapping_rem(y)),
            (Value::U128(x), Value::U128(y)) => Value::U128(x.wrapping_rem(y)),
            (Value::I8(x), Value::I8(y)) => Value::I8(x.wrapping_rem(y)),
            (Value::I16(x), Value::I16(y)) => Value::I16(x.wrapping_rem(y)),
            (Value::I32(x), Value::I32(y)) => Value::I32(x.wrapping_rem(y)),
            (Value::I64(x), Value::I64(y)) => Value::I64(x.wrapping_rem(y)),
            (Value::I128(x), Value::I128(y)) => Value::I128(x.wrapping_rem(y)),
            _ => halt!(span, "Type error"),
        },
        BinaryOperation::Shl => {
            let rhs: u32 = match rhs {
                Value::U8(y) => (y).into(),
                Value::U16(y) => (y).into(),
                Value::U32(y) => y,
                _ => halt!(span, "Type error"),
            };
            match lhs {
                Value::U8(_) | Value::I8(_) if rhs >= 8 => halt!(span, "shl overflow"),
                Value::U16(_) | Value::I16(_) if rhs >= 16 => halt!(span, "shl overflow"),
                Value::U32(_) | Value::I32(_) if rhs >= 32 => halt!(span, "shl overflow"),
                Value::U64(_) | Value::I64(_) if rhs >= 64 => halt!(span, "shl overflow"),
                Value::U128(_) | Value::I128(_) if rhs >= 128 => halt!(span, "shl overflow"),
                _ => {}
            }

            // Aleo's shl halts if set bits are shifted out.
            let shifted = lhs.simple_shl(rhs);
            let reshifted = shifted.simple_shr(rhs);
            if lhs.eq(&reshifted)? {
                shifted
            } else {
                halt!(span, "shl overflow");
            }
        }

        BinaryOperation::ShlWrapped => {
            let rhs: u32 = match rhs {
                Value::U8(y) => (y).into(),
                Value::U16(y) => (y).into(),
                Value::U32(y) => y,
                _ => halt!(span, "Type error"),
            };
            match lhs {
                Value::U8(x) => Value::U8(x.wrapping_shl(rhs)),
                Value::U16(x) => Value::U16(x.wrapping_shl(rhs)),
                Value::U32(x) => Value::U32(x.wrapping_shl(rhs)),
                Value::U64(x) => Value::U64(x.wrapping_shl(rhs)),
                Value::U128(x) => Value::U128(x.wrapping_shl(rhs)),
                Value::I8(x) => Value::I8(x.wrapping_shl(rhs)),
                Value::I16(x) => Value::I16(x.wrapping_shl(rhs)),
                Value::I32(x) => Value::I32(x.wrapping_shl(rhs)),
                Value::I64(x) => Value::I64(x.wrapping_shl(rhs)),
                Value::I128(x) => Value::I128(x.wrapping_shl(rhs)),
                _ => halt!(span, "Type error"),
            }
        }

        BinaryOperation::Shr => {
            let rhs: u32 = match rhs {
                Value::U8(y) => (y).into(),
                Value::U16(y) => (y).into(),
                Value::U32(y) => y,
                _ => halt!(span, "Type error"),
            };

            match lhs {
                Value::U8(_) | Value::I8(_) if rhs >= 8 => halt!(span, "shr overflow"),
                Value::U16(_) | Value::I16(_) if rhs >= 16 => halt!(span, "shr overflow"),
                Value::U32(_) | Value::I32(_) if rhs >= 32 => halt!(span, "shr overflow"),
                Value::U64(_) | Value::I64(_) if rhs >= 64 => halt!(span, "shr overflow"),
                Value::U128(_) | Value::I128(_) if rhs >= 128 => halt!(span, "shr overflow"),
                _ => {}
            }

            lhs.simple_shr(rhs)
        }

        BinaryOperation::ShrWrapped => {
            let rhs: u32 = match rhs {
                Value::U8(y) => (y).into(),
                Value::U16(y) => (y).into(),
                Value::U32(y) => y,
                _ => halt!(span, "Type error"),
            };

            match lhs {
                Value::U8(x) => Value::U8(x.wrapping_shr(rhs)),
                Value::U16(x) => Value::U16(x.wrapping_shr(rhs)),
                Value::U32(x) => Value::U32(x.wrapping_shr(rhs)),
                Value::U64(x) => Value::U64(x.wrapping_shr(rhs)),
                Value::U128(x) => Value::U128(x.wrapping_shr(rhs)),
                Value::I8(x) => Value::I8(x.wrapping_shr(rhs)),
                Value::I16(x) => Value::I16(x.wrapping_shr(rhs)),
                Value::I32(x) => Value::I32(x.wrapping_shr(rhs)),
                Value::I64(x) => Value::I64(x.wrapping_shr(rhs)),
                Value::I128(x) => Value::I128(x.wrapping_shr(rhs)),
                _ => halt!(span, "Type error"),
            }
        }

        BinaryOperation::Sub => {
            let Some(value) = (match (lhs, rhs) {
                (Value::U8(x), Value::U8(y)) => x.checked_sub(y).map(Value::U8),
                (Value::U16(x), Value::U16(y)) => x.checked_sub(y).map(Value::U16),
                (Value::U32(x), Value::U32(y)) => x.checked_sub(y).map(Value::U32),
                (Value::U64(x), Value::U64(y)) => x.checked_sub(y).map(Value::U64),
                (Value::U128(x), Value::U128(y)) => x.checked_sub(y).map(Value::U128),
                (Value::I8(x), Value::I8(y)) => x.checked_sub(y).map(Value::I8),
                (Value::I16(x), Value::I16(y)) => x.checked_sub(y).map(Value::I16),
                (Value::I32(x), Value::I32(y)) => x.checked_sub(y).map(Value::I32),
                (Value::I64(x), Value::I64(y)) => x.checked_sub(y).map(Value::I64),
                (Value::I128(x), Value::I128(y)) => x.checked_sub(y).map(Value::I128),
                (Value::Group(x), Value::Group(y)) => Some(Value::Group(x - y)),
                (Value::Field(x), Value::Field(y)) => Some(Value::Field(x - y)),
                _ => halt!(span, "Type error"),
            }) else {
                halt!(span, "sub overflow");
            };
            value
        }

        BinaryOperation::SubWrapped => match (lhs, rhs) {
            (Value::U8(x), Value::U8(y)) => Value::U8(x.wrapping_sub(y)),
            (Value::U16(x), Value::U16(y)) => Value::U16(x.wrapping_sub(y)),
            (Value::U32(x), Value::U32(y)) => Value::U32(x.wrapping_sub(y)),
            (Value::U64(x), Value::U64(y)) => Value::U64(x.wrapping_sub(y)),
            (Value::U128(x), Value::U128(y)) => Value::U128(x.wrapping_sub(y)),
            (Value::I8(x), Value::I8(y)) => Value::I8(x.wrapping_sub(y)),
            (Value::I16(x), Value::I16(y)) => Value::I16(x.wrapping_sub(y)),
            (Value::I32(x), Value::I32(y)) => Value::I32(x.wrapping_sub(y)),
            (Value::I64(x), Value::I64(y)) => Value::I64(x.wrapping_sub(y)),
            (Value::I128(x), Value::I128(y)) => Value::I128(x.wrapping_sub(y)),
            _ => halt!(span, "Type error"),
        },

        BinaryOperation::Xor => match (lhs, rhs) {
            (Value::Bool(x), Value::Bool(y)) => Value::Bool(x ^ y),
            (Value::U8(x), Value::U8(y)) => Value::U8(x ^ y),
            (Value::U16(x), Value::U16(y)) => Value::U16(x ^ y),
            (Value::U32(x), Value::U32(y)) => Value::U32(x ^ y),
            (Value::U64(x), Value::U64(y)) => Value::U64(x ^ y),
            (Value::U128(x), Value::U128(y)) => Value::U128(x ^ y),
            (Value::I8(x), Value::I8(y)) => Value::I8(x ^ y),
            (Value::I16(x), Value::I16(y)) => Value::I16(x ^ y),
            (Value::I32(x), Value::I32(y)) => Value::I32(x ^ y),
            (Value::I64(x), Value::I64(y)) => Value::I64(x ^ y),
            (Value::I128(x), Value::I128(y)) => Value::I128(x ^ y),
            _ => halt!(span, "Type error"),
        },
    };
    Ok(value)
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
