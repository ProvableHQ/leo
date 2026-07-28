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

use snarkvm::prelude::{Double, Inverse as _, Pow as _, ProgramID, Square as _, SquareRoot as _};

use leo_errors::Formatted;
use leo_span::Span;

use crate::{BinaryOperation, FromStrRadix as _, IntegerType, Literal, LiteralVariant, TypeKind, UnaryOperation};

use super::{errors, *};

fn parse_tc<T, E: std::fmt::Display>(r: Result<T, E>) -> Result<T, String> {
    r.map_err(|e| format!("type failure: {e}"))
}

impl Value {
    /// Are the values equal, according to SnarkVM?
    ///
    /// We use this rather than the Eq trait so we can
    /// fail when comparing values of different types,
    /// rather than just returning false.
    pub fn eq(&self, rhs: &Self) -> Result<bool, String> {
        if self.id != rhs.id {
            return Ok(false);
        }
        use ValueVariants::*;
        Ok(match (&self.contents, &rhs.contents) {
            (Unsuffixed(..), _) | (_, Unsuffixed(..)) => return Err("equality on unsuffixed value".into()),
            (Unit, Unit) => true,
            (Tuple(x), Tuple(y)) => {
                if x.len() != y.len() {
                    return Ok(false);
                }
                for (x0, y0) in x.iter().zip(y) {
                    if !x0.eq(y0)? {
                        return Ok(false);
                    }
                }
                true
            }
            (Svm(x), Svm(y)) => x == y,
            (_, _) => return Err("Type failure".into()),
        })
    }

    /// Resolves an unsuffixed literal to a typed `Value` using the provided optional `TypeKind`. If the value is unsuffixed
    /// and a type is provided, parses the string into the corresponding `Value` variant. Handles integers of various
    /// widths and special types like `Field`, `Group`, and `Scalar`. If no type is given or the value is already typed,
    /// returns the original value. Returns an error if type inference is not possible or parsing fails.
    pub fn resolve_if_unsuffixed(&self, ty: &Option<TypeKind>) -> Result<Value, String> {
        if let ValueVariants::Unsuffixed(s) = &self.contents {
            if let Some(ty) = ty {
                let value = match ty {
                    TypeKind::Integer(IntegerType::U8) => {
                        let s = s.replace("_", "");
                        u8::from_str_by_radix(&s).expect("Parsing guarantees this works.").into()
                    }
                    TypeKind::Integer(IntegerType::U16) => {
                        let s = s.replace("_", "");
                        u16::from_str_by_radix(&s).expect("Parsing guarantees this works.").into()
                    }
                    TypeKind::Integer(IntegerType::U32) => {
                        let s = s.replace("_", "");
                        u32::from_str_by_radix(&s).expect("Parsing guarantees this works.").into()
                    }
                    TypeKind::Integer(IntegerType::U64) => {
                        let s = s.replace("_", "");
                        u64::from_str_by_radix(&s).expect("Parsing guarantees this works.").into()
                    }
                    TypeKind::Integer(IntegerType::U128) => {
                        let s = s.replace("_", "");
                        u128::from_str_by_radix(&s).expect("Parsing guarantees this works.").into()
                    }
                    TypeKind::Integer(IntegerType::I8) => {
                        let s = s.replace("_", "");
                        i8::from_str_by_radix(&s).expect("Parsing guarantees this works.").into()
                    }
                    TypeKind::Integer(IntegerType::I16) => {
                        let s = s.replace("_", "");
                        i16::from_str_by_radix(&s).expect("Parsing guarantees this works.").into()
                    }
                    TypeKind::Integer(IntegerType::I32) => {
                        let s = s.replace("_", "");
                        i32::from_str_by_radix(&s).expect("Parsing guarantees this works.").into()
                    }
                    TypeKind::Integer(IntegerType::I64) => {
                        let s = s.replace("_", "");
                        i64::from_str_by_radix(&s).expect("Parsing guarantees this works.").into()
                    }
                    TypeKind::Integer(IntegerType::I128) => {
                        let s = s.replace("_", "");
                        i128::from_str_by_radix(&s).expect("Parsing guarantees this works.").into()
                    }
                    TypeKind::Field => {
                        SvmLiteralParam::Field(parse_tc(prepare_snarkvm_string(s, "field").parse())?).into()
                    }
                    TypeKind::Group => {
                        SvmLiteralParam::Group(parse_tc(prepare_snarkvm_string(s, "group").parse())?).into()
                    }
                    TypeKind::Scalar => {
                        SvmLiteralParam::Scalar(parse_tc(prepare_snarkvm_string(s, "scalar").parse())?).into()
                    }
                    _ => return Err("cannot infer type of unsuffixed literal".into()),
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

pub fn literal_to_value(literal: &Literal, expected_ty: &Option<TypeKind>) -> Result<Value, String> {
    Ok(match &literal.variant {
        LiteralVariant::Address(s) => {
            if s.ends_with(".aleo") {
                let program_id: ProgramID<CurrentNetwork> = parse_tc(s.parse())?;
                parse_tc(program_id.to_address())?.into()
            } else {
                let address: Address = parse_tc(s.parse())?;
                address.into()
            }
        }
        LiteralVariant::Boolean(b) => (*b).into(),
        LiteralVariant::Field(s) => {
            SvmLiteralParam::Field(parse_tc(prepare_snarkvm_string(s, "field").parse())?).into()
        }
        LiteralVariant::Group(s) => {
            SvmLiteralParam::Group(parse_tc(prepare_snarkvm_string(s, "group").parse())?).into()
        }

        LiteralVariant::Identifier(s) => {
            let identifier = parse_tc(SvmIdentifierLiteral::new(s))?;
            SvmLiteralParam::Identifier(Box::new(identifier)).into()
        }
        LiteralVariant::Integer(IntegerType::U8, s, ..) => {
            let s = s.replace("_", "");
            u8::from_str_by_radix(&s).expect("Parsing guarantees this works.").into()
        }
        LiteralVariant::Integer(IntegerType::U16, s, ..) => {
            let s = s.replace("_", "");
            u16::from_str_by_radix(&s).expect("Parsing guarantees this works.").into()
        }
        LiteralVariant::Integer(IntegerType::U32, s, ..) => {
            let s = s.replace("_", "");
            u32::from_str_by_radix(&s).expect("Parsing guarantees this works.").into()
        }
        LiteralVariant::Integer(IntegerType::U64, s, ..) => {
            let s = s.replace("_", "");
            u64::from_str_by_radix(&s).expect("Parsing guarantees this works.").into()
        }
        LiteralVariant::Integer(IntegerType::U128, s, ..) => {
            let s = s.replace("_", "");
            u128::from_str_by_radix(&s).expect("Parsing guarantees this works.").into()
        }
        LiteralVariant::Integer(IntegerType::I8, s, ..) => {
            let s = s.replace("_", "");
            i8::from_str_by_radix(&s).expect("Parsing guarantees this works.").into()
        }
        LiteralVariant::Integer(IntegerType::I16, s, ..) => {
            let s = s.replace("_", "");
            i16::from_str_by_radix(&s).expect("Parsing guarantees this works.").into()
        }
        LiteralVariant::Integer(IntegerType::I32, s, ..) => {
            let s = s.replace("_", "");
            i32::from_str_by_radix(&s).expect("Parsing guarantees this works.").into()
        }
        LiteralVariant::Integer(IntegerType::I64, s, ..) => {
            let s = s.replace("_", "");
            i64::from_str_by_radix(&s).expect("Parsing guarantees this works.").into()
        }
        LiteralVariant::Integer(IntegerType::I128, s, ..) => {
            let s = s.replace("_", "");
            i128::from_str_by_radix(&s).expect("Parsing guarantees this works.").into()
        }
        LiteralVariant::None => return Err("none literal has no value".into()),
        LiteralVariant::Scalar(s) => {
            SvmLiteralParam::Scalar(parse_tc(prepare_snarkvm_string(s, "scalar").parse())?).into()
        }
        LiteralVariant::Signature(s) => {
            let signature: Signature = parse_tc(s.parse())?;
            signature.into()
        }
        LiteralVariant::String(s) => Value::make_string(s.clone()),
        LiteralVariant::Unsuffixed(s) => {
            let unsuffixed = Value { id: None, contents: ValueVariants::Unsuffixed(s.clone()) };
            unsuffixed.resolve_if_unsuffixed(expected_ty)?
        }
    })
}

/// Resolves an unsuffixed operand for a unary operation by inferring its type based on the operation and an optional
/// expected type. Uses predefined types (`Field` or `Group`) for specific operations, otherwise defaults to the expected
/// type if available. Returns the resolved `Value` or an error if type resolution fails.
fn resolve_unsuffixed_unary_op_operand(
    val: &Value,
    op: &UnaryOperation,
    expected_ty: &Option<TypeKind>,
) -> Result<Value, String> {
    match op {
        UnaryOperation::Inverse | UnaryOperation::Square | UnaryOperation::SquareRoot => {
            // These ops only take a `field` and return a `field`
            val.resolve_if_unsuffixed(&Some(TypeKind::Field))
        }
        UnaryOperation::ToXCoordinate | UnaryOperation::ToYCoordinate => {
            // These ops only take a `Group`
            val.resolve_if_unsuffixed(&Some(TypeKind::Group))
        }
        _ => {
            // All other unary ops take the same type as the their return type
            val.resolve_if_unsuffixed(expected_ty)
        }
    }
}

/// Evaluate a unary operation.
pub fn evaluate_unary(
    span: Span,
    op: UnaryOperation,
    value: &Value,
    expected_ty: &Option<TypeKind>,
) -> Result<Value, Formatted> {
    evaluate_unary_inner(op, value, expected_ty).map_err(|reason| errors::unary_op_failure(value, op, reason, span))
}

fn evaluate_unary_inner(op: UnaryOperation, value: &Value, expected_ty: &Option<TypeKind>) -> Result<Value, String> {
    let value = resolve_unsuffixed_unary_op_operand(value, &op, expected_ty)?;
    let ValueVariants::Svm(SvmValueParam::Plaintext(Plaintext::Literal(literal, ..))) = &value.contents else {
        return Err("Type error".into());
    };
    let value_result: Value = match op {
        UnaryOperation::Abs => match literal {
            SvmLiteralParam::I8(x) => x.checked_abs().ok_or_else(|| "abs overflow".to_string())?.into(),
            SvmLiteralParam::I16(x) => x.checked_abs().ok_or_else(|| "abs overflow".to_string())?.into(),
            SvmLiteralParam::I32(x) => x.checked_abs().ok_or_else(|| "abs overflow".to_string())?.into(),
            SvmLiteralParam::I64(x) => x.checked_abs().ok_or_else(|| "abs overflow".to_string())?.into(),
            SvmLiteralParam::I128(x) => x.checked_abs().ok_or_else(|| "abs overflow".to_string())?.into(),
            _ => return Err("Type error".into()),
        },
        UnaryOperation::AbsWrapped => match literal {
            SvmLiteralParam::I8(x) => (x.unsigned_abs() as i8).into(),
            SvmLiteralParam::I16(x) => (x.unsigned_abs() as i16).into(),
            SvmLiteralParam::I32(x) => (x.unsigned_abs() as i32).into(),
            SvmLiteralParam::I64(x) => (x.unsigned_abs() as i64).into(),
            SvmLiteralParam::I128(x) => (x.unsigned_abs() as i128).into(),
            _ => return Err("Type error".into()),
        },
        UnaryOperation::Double => match literal {
            SvmLiteralParam::Field(x) => <Field as Double>::double(x).into(),
            SvmLiteralParam::Group(x) => <Group as Double>::double(x).into(),
            _ => return Err("Type error".into()),
        },
        UnaryOperation::Inverse => match literal {
            SvmLiteralParam::Field(x) => {
                let Ok(y) = x.inverse() else {
                    return Err("attempt to invert 0field".into());
                };
                y.into()
            }
            _ => return Err("Can only invert fields".into()),
        },
        UnaryOperation::Negate => match literal {
            SvmLiteralParam::I8(x) => x.checked_neg().ok_or_else(|| "negation overflow".to_string())?.into(),
            SvmLiteralParam::I16(x) => x.checked_neg().ok_or_else(|| "negation overflow".to_string())?.into(),
            SvmLiteralParam::I32(x) => x.checked_neg().ok_or_else(|| "negation overflow".to_string())?.into(),
            SvmLiteralParam::I64(x) => x.checked_neg().ok_or_else(|| "negation overflow".to_string())?.into(),
            SvmLiteralParam::I128(x) => x.checked_neg().ok_or_else(|| "negation overflow".to_string())?.into(),
            SvmLiteralParam::Group(x) => (-*x).into(),
            SvmLiteralParam::Field(x) => (-*x).into(),
            _ => return Err("Type error".into()),
        },
        UnaryOperation::Not => match literal {
            SvmLiteralParam::Boolean(x) => (!**x).into(),
            SvmLiteralParam::U8(x) => (!**x).into(),
            SvmLiteralParam::U16(x) => (!**x).into(),
            SvmLiteralParam::U32(x) => (!**x).into(),
            SvmLiteralParam::U64(x) => (!**x).into(),
            SvmLiteralParam::U128(x) => (!**x).into(),
            SvmLiteralParam::I8(x) => (!**x).into(),
            SvmLiteralParam::I16(x) => (!**x).into(),
            SvmLiteralParam::I32(x) => (!**x).into(),
            SvmLiteralParam::I64(x) => (!**x).into(),
            SvmLiteralParam::I128(x) => (!**x).into(),
            _ => return Err("Type error".into()),
        },
        UnaryOperation::Square => match literal {
            SvmLiteralParam::Field(x) => x.square().into(),
            _ => return Err("Can only square fields".into()),
        },
        UnaryOperation::SquareRoot => match literal {
            SvmLiteralParam::Field(x) => x.square_root().map_err(|e| format!("square root failure: {e}"))?.into(),
            _ => return Err("Can only apply square_root to fields".into()),
        },
        UnaryOperation::ToXCoordinate => match literal {
            SvmLiteral::Group(x) => x.to_x_coordinate().into(),
            _ => panic!("type checker failure"),
        },
        UnaryOperation::ToYCoordinate => match literal {
            SvmLiteral::Group(x) => x.to_y_coordinate().into(),
            _ => panic!("type checker failure"),
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
    expected_ty: &Option<TypeKind>,
) -> Result<(Value, Value), String> {
    use TypeKind::*;

    let lhs_ty = lhs.get_numeric_type();
    let rhs_ty = rhs.get_numeric_type();

    Ok(match op {
        BinaryOperation::Mul => {
            // For a `Mul`, if on operand is a Scalar, then the other must ba a `Group`. Otherwise, both ops must have
            // the same type as the return type of the multiplication.
            let lhs = match rhs_ty {
                Some(Group) => lhs.resolve_if_unsuffixed(&Some(Scalar))?,
                Some(Scalar) => lhs.resolve_if_unsuffixed(&Some(Group))?,
                _ => lhs.resolve_if_unsuffixed(&rhs_ty)?.resolve_if_unsuffixed(expected_ty)?,
            };

            let rhs = match lhs_ty {
                Some(Group) => rhs.resolve_if_unsuffixed(&Some(Scalar))?,
                Some(Scalar) => rhs.resolve_if_unsuffixed(&Some(Group))?,
                _ => rhs.resolve_if_unsuffixed(&lhs_ty)?.resolve_if_unsuffixed(expected_ty)?,
            };

            (lhs, rhs)
        }
        BinaryOperation::Pow => {
            // For a `Pow`, if one operand is a `Field`, then the other must also be a `Field.
            // Otherwise, only the `lhs` must match the return type.
            let lhs_resolved = lhs
                .resolve_if_unsuffixed(&rhs_ty.filter(|ty| matches!(ty, TypeKind::Field)))?
                .resolve_if_unsuffixed(expected_ty)?;

            let rhs_resolved = rhs.resolve_if_unsuffixed(&lhs_ty.filter(|ty| matches!(ty, TypeKind::Field)))?;

            (lhs_resolved, rhs_resolved)
        }
        _ => (
            lhs.resolve_if_unsuffixed(&rhs_ty)?.resolve_if_unsuffixed(expected_ty)?,
            rhs.resolve_if_unsuffixed(&lhs_ty)?.resolve_if_unsuffixed(expected_ty)?,
        ),
    })
}

/// Evaluate a binary operation.
pub fn evaluate_binary(
    span: Span,
    op: BinaryOperation,
    lhs: &Value,
    rhs: &Value,
    expected_ty: &Option<TypeKind>,
) -> Result<Value, Formatted> {
    evaluate_binary_inner(op, lhs, rhs, expected_ty)
        .map_err(|reason| errors::binary_op_failure(lhs, op, rhs, reason, span))
}

fn evaluate_binary_inner(
    op: BinaryOperation,
    lhs: &Value,
    rhs: &Value,
    expected_ty: &Option<TypeKind>,
) -> Result<Value, String> {
    let (lhs, rhs) = resolve_unsuffixed_binary_op_operands(lhs, rhs, &op, expected_ty)?;

    match op {
        BinaryOperation::Eq => return lhs.eq(&rhs).map(|x| x.into()),
        BinaryOperation::Neq => return lhs.eq(&rhs).map(|x| (!x).into()),
        _ => {}
    }

    let ValueVariants::Svm(SvmValueParam::Plaintext(Plaintext::Literal(lhs, ..))) = &lhs.contents else {
        return Err("Type error".into());
    };
    let ValueVariants::Svm(SvmValueParam::Plaintext(Plaintext::Literal(rhs, ..))) = &rhs.contents else {
        return Err("Type error".into());
    };
    let value = match op {
        BinaryOperation::Add => {
            let Some(value): Option<Value> = (match (lhs, rhs) {
                (SvmLiteralParam::U8(x), SvmLiteralParam::U8(y)) => (*x).checked_add(**y).map(|z| z.into()),
                (SvmLiteralParam::U16(x), SvmLiteralParam::U16(y)) => (*x).checked_add(**y).map(|z| z.into()),
                (SvmLiteralParam::U32(x), SvmLiteralParam::U32(y)) => (*x).checked_add(**y).map(|z| z.into()),
                (SvmLiteralParam::U64(x), SvmLiteralParam::U64(y)) => (*x).checked_add(**y).map(|z| z.into()),
                (SvmLiteralParam::U128(x), SvmLiteralParam::U128(y)) => (*x).checked_add(**y).map(|z| z.into()),
                (SvmLiteralParam::I8(x), SvmLiteralParam::I8(y)) => (*x).checked_add(**y).map(|z| z.into()),
                (SvmLiteralParam::I16(x), SvmLiteralParam::I16(y)) => (*x).checked_add(**y).map(|z| z.into()),
                (SvmLiteralParam::I32(x), SvmLiteralParam::I32(y)) => (*x).checked_add(**y).map(|z| z.into()),
                (SvmLiteralParam::I64(x), SvmLiteralParam::I64(y)) => (*x).checked_add(**y).map(|z| z.into()),
                (SvmLiteralParam::I128(x), SvmLiteralParam::I128(y)) => (*x).checked_add(**y).map(|z| z.into()),
                (SvmLiteralParam::Group(x), SvmLiteralParam::Group(y)) => Some((*x + y).into()),
                (SvmLiteralParam::Field(x), SvmLiteralParam::Field(y)) => Some((*x + y).into()),
                (SvmLiteralParam::Scalar(x), SvmLiteralParam::Scalar(y)) => Some((*x + y).into()),
                _ => return Err("Type error".into()),
            }) else {
                return Err("add overflow".into());
            };
            value
        }
        BinaryOperation::AddWrapped => match (lhs, rhs) {
            (SvmLiteralParam::U8(x), SvmLiteralParam::U8(y)) => (*x).wrapping_add(**y).into(),
            (SvmLiteralParam::U16(x), SvmLiteralParam::U16(y)) => (*x).wrapping_add(**y).into(),
            (SvmLiteralParam::U32(x), SvmLiteralParam::U32(y)) => (*x).wrapping_add(**y).into(),
            (SvmLiteralParam::U64(x), SvmLiteralParam::U64(y)) => (*x).wrapping_add(**y).into(),
            (SvmLiteralParam::U128(x), SvmLiteralParam::U128(y)) => (*x).wrapping_add(**y).into(),
            (SvmLiteralParam::I8(x), SvmLiteralParam::I8(y)) => (*x).wrapping_add(**y).into(),
            (SvmLiteralParam::I16(x), SvmLiteralParam::I16(y)) => (*x).wrapping_add(**y).into(),
            (SvmLiteralParam::I32(x), SvmLiteralParam::I32(y)) => (*x).wrapping_add(**y).into(),
            (SvmLiteralParam::I64(x), SvmLiteralParam::I64(y)) => (*x).wrapping_add(**y).into(),
            (SvmLiteralParam::I128(x), SvmLiteralParam::I128(y)) => (*x).wrapping_add(**y).into(),
            _ => return Err("Type error".into()),
        },
        BinaryOperation::And => match (lhs, rhs) {
            (SvmLiteralParam::Boolean(x), SvmLiteralParam::Boolean(y)) => (**x && **y).into(),
            _ => return Err("Type error".into()),
        },
        BinaryOperation::BitwiseAnd => match (lhs, rhs) {
            (SvmLiteralParam::Boolean(x), SvmLiteralParam::Boolean(y)) => (**x & **y).into(),
            (SvmLiteralParam::U8(x), SvmLiteralParam::U8(y)) => (**x & **y).into(),
            (SvmLiteralParam::U16(x), SvmLiteralParam::U16(y)) => (**x & **y).into(),
            (SvmLiteralParam::U32(x), SvmLiteralParam::U32(y)) => (**x & **y).into(),
            (SvmLiteralParam::U64(x), SvmLiteralParam::U64(y)) => (**x & **y).into(),
            (SvmLiteralParam::U128(x), SvmLiteralParam::U128(y)) => (**x & **y).into(),
            (SvmLiteralParam::I8(x), SvmLiteralParam::I8(y)) => (**x & **y).into(),
            (SvmLiteralParam::I16(x), SvmLiteralParam::I16(y)) => (**x & **y).into(),
            (SvmLiteralParam::I32(x), SvmLiteralParam::I32(y)) => (**x & **y).into(),
            (SvmLiteralParam::I64(x), SvmLiteralParam::I64(y)) => (**x & **y).into(),
            (SvmLiteralParam::I128(x), SvmLiteralParam::I128(y)) => (**x & **y).into(),
            _ => return Err("Type error".into()),
        },
        BinaryOperation::Div => {
            let Some(value) = (match (lhs, rhs) {
                (SvmLiteralParam::U8(x), SvmLiteralParam::U8(y)) => (*x).checked_div(**y).map(|z| z.into()),
                (SvmLiteralParam::U16(x), SvmLiteralParam::U16(y)) => (*x).checked_div(**y).map(|z| z.into()),
                (SvmLiteralParam::U32(x), SvmLiteralParam::U32(y)) => (*x).checked_div(**y).map(|z| z.into()),
                (SvmLiteralParam::U64(x), SvmLiteralParam::U64(y)) => (*x).checked_div(**y).map(|z| z.into()),
                (SvmLiteralParam::U128(x), SvmLiteralParam::U128(y)) => (*x).checked_div(**y).map(|z| z.into()),
                (SvmLiteralParam::I8(x), SvmLiteralParam::I8(y)) => (*x).checked_div(**y).map(|z| z.into()),
                (SvmLiteralParam::I16(x), SvmLiteralParam::I16(y)) => (*x).checked_div(**y).map(|z| z.into()),
                (SvmLiteralParam::I32(x), SvmLiteralParam::I32(y)) => (*x).checked_div(**y).map(|z| z.into()),
                (SvmLiteralParam::I64(x), SvmLiteralParam::I64(y)) => (*x).checked_div(**y).map(|z| z.into()),
                (SvmLiteralParam::I128(x), SvmLiteralParam::I128(y)) => (*x).checked_div(**y).map(|z| z.into()),
                (SvmLiteralParam::Field(x), SvmLiteralParam::Field(y)) => y.inverse().map(|y| (*x * y).into()).ok(),
                _ => return Err("Type error".into()),
            }) else {
                return Err("div overflow".into());
            };
            value
        }
        BinaryOperation::DivWrapped => match (lhs, rhs) {
            (SvmLiteralParam::U8(x), SvmLiteralParam::U8(y)) if **y != 0 => (*x).wrapping_div(**y).into(),
            (SvmLiteralParam::U16(x), SvmLiteralParam::U16(y)) if **y != 0 => (*x).wrapping_div(**y).into(),
            (SvmLiteralParam::U32(x), SvmLiteralParam::U32(y)) if **y != 0 => (*x).wrapping_div(**y).into(),
            (SvmLiteralParam::U64(x), SvmLiteralParam::U64(y)) if **y != 0 => (*x).wrapping_div(**y).into(),
            (SvmLiteralParam::U128(x), SvmLiteralParam::U128(y)) if **y != 0 => (*x).wrapping_div(**y).into(),
            (SvmLiteralParam::I8(x), SvmLiteralParam::I8(y)) if **y != 0 => (*x).wrapping_div(**y).into(),
            (SvmLiteralParam::I16(x), SvmLiteralParam::I16(y)) if **y != 0 => (*x).wrapping_div(**y).into(),
            (SvmLiteralParam::I32(x), SvmLiteralParam::I32(y)) if **y != 0 => (*x).wrapping_div(**y).into(),
            (SvmLiteralParam::I64(x), SvmLiteralParam::I64(y)) if **y != 0 => (*x).wrapping_div(**y).into(),
            (SvmLiteralParam::I128(x), SvmLiteralParam::I128(y)) if **y != 0 => (*x).wrapping_div(**y).into(),
            _ => return Err("Type error".into()),
        },
        BinaryOperation::Eq => unreachable!("This case was handled above"),
        BinaryOperation::Gte => match (lhs, rhs) {
            (SvmLiteralParam::U8(x), SvmLiteralParam::U8(y)) => (*x >= *y).into(),
            (SvmLiteralParam::U16(x), SvmLiteralParam::U16(y)) => (*x >= *y).into(),
            (SvmLiteralParam::U32(x), SvmLiteralParam::U32(y)) => (*x >= *y).into(),
            (SvmLiteralParam::U64(x), SvmLiteralParam::U64(y)) => (*x >= *y).into(),
            (SvmLiteralParam::U128(x), SvmLiteralParam::U128(y)) => (*x >= *y).into(),
            (SvmLiteralParam::I8(x), SvmLiteralParam::I8(y)) => (*x >= *y).into(),
            (SvmLiteralParam::I16(x), SvmLiteralParam::I16(y)) => (*x >= *y).into(),
            (SvmLiteralParam::I32(x), SvmLiteralParam::I32(y)) => (*x >= *y).into(),
            (SvmLiteralParam::I64(x), SvmLiteralParam::I64(y)) => (*x >= *y).into(),
            (SvmLiteralParam::I128(x), SvmLiteralParam::I128(y)) => (*x >= *y).into(),
            (SvmLiteralParam::Field(x), SvmLiteralParam::Field(y)) => (*x >= *y).into(),
            (SvmLiteralParam::Scalar(x), SvmLiteralParam::Scalar(y)) => (*x >= *y).into(),
            _ => return Err("Type error".into()),
        },
        BinaryOperation::Gt => match (lhs, rhs) {
            (SvmLiteralParam::U8(x), SvmLiteralParam::U8(y)) => (*x > *y).into(),
            (SvmLiteralParam::U16(x), SvmLiteralParam::U16(y)) => (*x > *y).into(),
            (SvmLiteralParam::U32(x), SvmLiteralParam::U32(y)) => (*x > *y).into(),
            (SvmLiteralParam::U64(x), SvmLiteralParam::U64(y)) => (*x > *y).into(),
            (SvmLiteralParam::U128(x), SvmLiteralParam::U128(y)) => (*x > *y).into(),
            (SvmLiteralParam::I8(x), SvmLiteralParam::I8(y)) => (*x > *y).into(),
            (SvmLiteralParam::I16(x), SvmLiteralParam::I16(y)) => (*x > *y).into(),
            (SvmLiteralParam::I32(x), SvmLiteralParam::I32(y)) => (*x > *y).into(),
            (SvmLiteralParam::I64(x), SvmLiteralParam::I64(y)) => (*x > *y).into(),
            (SvmLiteralParam::I128(x), SvmLiteralParam::I128(y)) => (*x > *y).into(),
            (SvmLiteralParam::Field(x), SvmLiteralParam::Field(y)) => (*x > *y).into(),
            (SvmLiteralParam::Scalar(x), SvmLiteralParam::Scalar(y)) => (*x > *y).into(),
            _ => return Err("Type error".into()),
        },
        BinaryOperation::Lte => match (lhs, rhs) {
            (SvmLiteralParam::U8(x), SvmLiteralParam::U8(y)) => (*x <= *y).into(),
            (SvmLiteralParam::U16(x), SvmLiteralParam::U16(y)) => (*x <= *y).into(),
            (SvmLiteralParam::U32(x), SvmLiteralParam::U32(y)) => (*x <= *y).into(),
            (SvmLiteralParam::U64(x), SvmLiteralParam::U64(y)) => (*x <= *y).into(),
            (SvmLiteralParam::U128(x), SvmLiteralParam::U128(y)) => (*x <= *y).into(),
            (SvmLiteralParam::I8(x), SvmLiteralParam::I8(y)) => (*x <= *y).into(),
            (SvmLiteralParam::I16(x), SvmLiteralParam::I16(y)) => (*x <= *y).into(),
            (SvmLiteralParam::I32(x), SvmLiteralParam::I32(y)) => (*x <= *y).into(),
            (SvmLiteralParam::I64(x), SvmLiteralParam::I64(y)) => (*x <= *y).into(),
            (SvmLiteralParam::I128(x), SvmLiteralParam::I128(y)) => (*x <= *y).into(),
            (SvmLiteralParam::Field(x), SvmLiteralParam::Field(y)) => (*x <= *y).into(),
            (SvmLiteralParam::Scalar(x), SvmLiteralParam::Scalar(y)) => (*x <= *y).into(),
            _ => return Err("Type error".into()),
        },
        BinaryOperation::Lt => match (lhs, rhs) {
            (SvmLiteralParam::U8(x), SvmLiteralParam::U8(y)) => (*x < *y).into(),
            (SvmLiteralParam::U16(x), SvmLiteralParam::U16(y)) => (*x < *y).into(),
            (SvmLiteralParam::U32(x), SvmLiteralParam::U32(y)) => (*x < *y).into(),
            (SvmLiteralParam::U64(x), SvmLiteralParam::U64(y)) => (*x < *y).into(),
            (SvmLiteralParam::U128(x), SvmLiteralParam::U128(y)) => (*x < *y).into(),
            (SvmLiteralParam::I8(x), SvmLiteralParam::I8(y)) => (*x < *y).into(),
            (SvmLiteralParam::I16(x), SvmLiteralParam::I16(y)) => (*x < *y).into(),
            (SvmLiteralParam::I32(x), SvmLiteralParam::I32(y)) => (*x < *y).into(),
            (SvmLiteralParam::I64(x), SvmLiteralParam::I64(y)) => (*x < *y).into(),
            (SvmLiteralParam::I128(x), SvmLiteralParam::I128(y)) => (*x < *y).into(),
            (SvmLiteralParam::Field(x), SvmLiteralParam::Field(y)) => (*x < *y).into(),
            (SvmLiteralParam::Scalar(x), SvmLiteralParam::Scalar(y)) => (*x < *y).into(),
            _ => return Err("Type error".into()),
        },
        BinaryOperation::Mod => {
            let Some(value) = (match (lhs, rhs) {
                (SvmLiteralParam::U8(x), SvmLiteralParam::U8(y)) => x.checked_rem(**y).map(|z| z.into()),
                (SvmLiteralParam::U16(x), SvmLiteralParam::U16(y)) => x.checked_rem(**y).map(|z| z.into()),
                (SvmLiteralParam::U32(x), SvmLiteralParam::U32(y)) => x.checked_rem(**y).map(|z| z.into()),
                (SvmLiteralParam::U64(x), SvmLiteralParam::U64(y)) => x.checked_rem(**y).map(|z| z.into()),
                (SvmLiteralParam::U128(x), SvmLiteralParam::U128(y)) => x.checked_rem(**y).map(|z| z.into()),
                _ => return Err("Type error".into()),
            }) else {
                return Err("mod overflow".into());
            };
            value
        }
        BinaryOperation::Mul => {
            let Some(value) = (match (lhs, rhs) {
                (SvmLiteralParam::U8(x), SvmLiteralParam::U8(y)) => x.checked_mul(**y).map(|z| z.into()),
                (SvmLiteralParam::U16(x), SvmLiteralParam::U16(y)) => x.checked_mul(**y).map(|z| z.into()),
                (SvmLiteralParam::U32(x), SvmLiteralParam::U32(y)) => x.checked_mul(**y).map(|z| z.into()),
                (SvmLiteralParam::U64(x), SvmLiteralParam::U64(y)) => x.checked_mul(**y).map(|z| z.into()),
                (SvmLiteralParam::U128(x), SvmLiteralParam::U128(y)) => x.checked_mul(**y).map(|z| z.into()),
                (SvmLiteralParam::I8(x), SvmLiteralParam::I8(y)) => x.checked_mul(**y).map(|z| z.into()),
                (SvmLiteralParam::I16(x), SvmLiteralParam::I16(y)) => x.checked_mul(**y).map(|z| z.into()),
                (SvmLiteralParam::I32(x), SvmLiteralParam::I32(y)) => x.checked_mul(**y).map(|z| z.into()),
                (SvmLiteralParam::I64(x), SvmLiteralParam::I64(y)) => x.checked_mul(**y).map(|z| z.into()),
                (SvmLiteralParam::I128(x), SvmLiteralParam::I128(y)) => x.checked_mul(**y).map(|z| z.into()),
                (SvmLiteralParam::Field(x), SvmLiteralParam::Field(y)) => Some((*x * y).into()),
                (SvmLiteralParam::Group(x), SvmLiteralParam::Scalar(y)) => Some((*x * y).into()),
                (SvmLiteralParam::Scalar(x), SvmLiteralParam::Group(y)) => Some((*x * y).into()),
                _ => return Err("Type error".into()),
            }) else {
                return Err("mul overflow".into());
            };
            value
        }
        BinaryOperation::MulWrapped => match (lhs, rhs) {
            (SvmLiteralParam::U8(x), SvmLiteralParam::U8(y)) => x.wrapping_mul(**y).into(),
            (SvmLiteralParam::U16(x), SvmLiteralParam::U16(y)) => x.wrapping_mul(**y).into(),
            (SvmLiteralParam::U32(x), SvmLiteralParam::U32(y)) => x.wrapping_mul(**y).into(),
            (SvmLiteralParam::U64(x), SvmLiteralParam::U64(y)) => x.wrapping_mul(**y).into(),
            (SvmLiteralParam::U128(x), SvmLiteralParam::U128(y)) => x.wrapping_mul(**y).into(),
            (SvmLiteralParam::I8(x), SvmLiteralParam::I8(y)) => x.wrapping_mul(**y).into(),
            (SvmLiteralParam::I16(x), SvmLiteralParam::I16(y)) => x.wrapping_mul(**y).into(),
            (SvmLiteralParam::I32(x), SvmLiteralParam::I32(y)) => x.wrapping_mul(**y).into(),
            (SvmLiteralParam::I64(x), SvmLiteralParam::I64(y)) => x.wrapping_mul(**y).into(),
            (SvmLiteralParam::I128(x), SvmLiteralParam::I128(y)) => x.wrapping_mul(**y).into(),
            _ => return Err("Type error".into()),
        },

        BinaryOperation::Nand => match (lhs, rhs) {
            (SvmLiteralParam::Boolean(x), SvmLiteralParam::Boolean(y)) => (!(**x & **y)).into(),
            _ => return Err("Type error".into()),
        },

        BinaryOperation::Neq => unreachable!("This case was handled above"),

        BinaryOperation::Nor => match (lhs, rhs) {
            (SvmLiteralParam::Boolean(x), SvmLiteralParam::Boolean(y)) => (!(**x | **y)).into(),
            _ => return Err("Type error".into()),
        },

        BinaryOperation::Or => match (lhs, rhs) {
            (SvmLiteralParam::Boolean(x), SvmLiteralParam::Boolean(y)) => (**x | **y).into(),
            _ => return Err("Type error".into()),
        },

        BinaryOperation::BitwiseOr => match (lhs, rhs) {
            (SvmLiteralParam::Boolean(x), SvmLiteralParam::Boolean(y)) => (**x | **y).into(),
            (SvmLiteralParam::U8(x), SvmLiteralParam::U8(y)) => (**x | **y).into(),
            (SvmLiteralParam::U16(x), SvmLiteralParam::U16(y)) => (**x | **y).into(),
            (SvmLiteralParam::U32(x), SvmLiteralParam::U32(y)) => (**x | **y).into(),
            (SvmLiteralParam::U64(x), SvmLiteralParam::U64(y)) => (**x | **y).into(),
            (SvmLiteralParam::U128(x), SvmLiteralParam::U128(y)) => (**x | **y).into(),
            (SvmLiteralParam::I8(x), SvmLiteralParam::I8(y)) => (**x | **y).into(),
            (SvmLiteralParam::I16(x), SvmLiteralParam::I16(y)) => (**x | **y).into(),
            (SvmLiteralParam::I32(x), SvmLiteralParam::I32(y)) => (**x | **y).into(),
            (SvmLiteralParam::I64(x), SvmLiteralParam::I64(y)) => (**x | **y).into(),
            (SvmLiteralParam::I128(x), SvmLiteralParam::I128(y)) => (**x | **y).into(),
            _ => return Err("Type error".into()),
        },

        BinaryOperation::Pow => {
            if let (SvmLiteralParam::Field(x), SvmLiteralParam::Field(y)) = (&lhs, &rhs) {
                x.pow(y).into()
            } else {
                let rhs: u32 = match rhs {
                    SvmLiteralParam::U8(y) => (**y).into(),
                    SvmLiteralParam::U16(y) => (**y).into(),
                    SvmLiteralParam::U32(y) => **y,
                    _ => panic!("type checker failure"),
                };

                let Some(value) = (match lhs {
                    SvmLiteralParam::U8(x) => x.checked_pow(rhs).map(|z| z.into()),
                    SvmLiteralParam::U16(x) => x.checked_pow(rhs).map(|z| z.into()),
                    SvmLiteralParam::U32(x) => x.checked_pow(rhs).map(|z| z.into()),
                    SvmLiteralParam::U64(x) => x.checked_pow(rhs).map(|z| z.into()),
                    SvmLiteralParam::U128(x) => x.checked_pow(rhs).map(|z| z.into()),
                    SvmLiteralParam::I8(x) => x.checked_pow(rhs).map(|z| z.into()),
                    SvmLiteralParam::I16(x) => x.checked_pow(rhs).map(|z| z.into()),
                    SvmLiteralParam::I32(x) => x.checked_pow(rhs).map(|z| z.into()),
                    SvmLiteralParam::I64(x) => x.checked_pow(rhs).map(|z| z.into()),
                    SvmLiteralParam::I128(x) => x.checked_pow(rhs).map(|z| z.into()),
                    _ => return Err("Type error".into()),
                }) else {
                    return Err("pow overflow".into());
                };
                value
            }
        }
        BinaryOperation::PowWrapped => {
            let rhs: u32 = match rhs {
                SvmLiteralParam::U8(y) => (**y).into(),
                SvmLiteralParam::U16(y) => (**y).into(),
                SvmLiteralParam::U32(y) => **y,
                _ => return Err("Type error".into()),
            };

            match lhs {
                SvmLiteralParam::U8(x) => x.wrapping_pow(rhs).into(),
                SvmLiteralParam::U16(x) => x.wrapping_pow(rhs).into(),
                SvmLiteralParam::U32(x) => x.wrapping_pow(rhs).into(),
                SvmLiteralParam::U64(x) => x.wrapping_pow(rhs).into(),
                SvmLiteralParam::U128(x) => x.wrapping_pow(rhs).into(),
                SvmLiteralParam::I8(x) => x.wrapping_pow(rhs).into(),
                SvmLiteralParam::I16(x) => x.wrapping_pow(rhs).into(),
                SvmLiteralParam::I32(x) => x.wrapping_pow(rhs).into(),
                SvmLiteralParam::I64(x) => x.wrapping_pow(rhs).into(),
                SvmLiteralParam::I128(x) => x.wrapping_pow(rhs).into(),
                _ => return Err("Type error".into()),
            }
        }

        BinaryOperation::Rem => {
            let Some(value) = (match (lhs, rhs) {
                (SvmLiteralParam::U8(x), SvmLiteralParam::U8(y)) => (**x).checked_rem(**y).map(|z| z.into()),
                (SvmLiteralParam::U16(x), SvmLiteralParam::U16(y)) => (**x).checked_rem(**y).map(|z| z.into()),
                (SvmLiteralParam::U32(x), SvmLiteralParam::U32(y)) => (**x).checked_rem(**y).map(|z| z.into()),
                (SvmLiteralParam::U64(x), SvmLiteralParam::U64(y)) => (**x).checked_rem(**y).map(|z| z.into()),
                (SvmLiteralParam::U128(x), SvmLiteralParam::U128(y)) => (**x).checked_rem(**y).map(|z| z.into()),
                (SvmLiteralParam::I8(x), SvmLiteralParam::I8(y)) => (**x).checked_rem(**y).map(|z| z.into()),
                (SvmLiteralParam::I16(x), SvmLiteralParam::I16(y)) => (**x).checked_rem(**y).map(|z| z.into()),
                (SvmLiteralParam::I32(x), SvmLiteralParam::I32(y)) => (**x).checked_rem(**y).map(|z| z.into()),
                (SvmLiteralParam::I64(x), SvmLiteralParam::I64(y)) => (**x).checked_rem(**y).map(|z| z.into()),
                (SvmLiteralParam::I128(x), SvmLiteralParam::I128(y)) => (**x).checked_rem(**y).map(|z| z.into()),
                _ => return Err("Type error".into()),
            }) else {
                return Err("rem error".into());
            };
            value
        }

        BinaryOperation::RemWrapped => match (lhs, rhs) {
            (SvmLiteralParam::U8(x), SvmLiteralParam::U8(y)) if **y != 0 => (*x).wrapping_rem(**y).into(),
            (SvmLiteralParam::U16(x), SvmLiteralParam::U16(y)) if **y != 0 => (*x).wrapping_rem(**y).into(),
            (SvmLiteralParam::U32(x), SvmLiteralParam::U32(y)) if **y != 0 => (*x).wrapping_rem(**y).into(),
            (SvmLiteralParam::U64(x), SvmLiteralParam::U64(y)) if **y != 0 => (*x).wrapping_rem(**y).into(),
            (SvmLiteralParam::U128(x), SvmLiteralParam::U128(y)) if **y != 0 => (*x).wrapping_rem(**y).into(),
            (SvmLiteralParam::I8(x), SvmLiteralParam::I8(y)) if **y != 0 => (*x).wrapping_rem(**y).into(),
            (SvmLiteralParam::I16(x), SvmLiteralParam::I16(y)) if **y != 0 => (*x).wrapping_rem(**y).into(),
            (SvmLiteralParam::I32(x), SvmLiteralParam::I32(y)) if **y != 0 => (*x).wrapping_rem(**y).into(),
            (SvmLiteralParam::I64(x), SvmLiteralParam::I64(y)) if **y != 0 => (*x).wrapping_rem(**y).into(),
            (SvmLiteralParam::I128(x), SvmLiteralParam::I128(y)) if **y != 0 => (*x).wrapping_rem(**y).into(),
            _ => return Err("Type error".into()),
        },

        BinaryOperation::Shl => {
            let rhs: u32 = match rhs {
                SvmLiteralParam::U8(y) => (**y).into(),
                SvmLiteralParam::U16(y) => (**y).into(),
                SvmLiteralParam::U32(y) => **y,
                _ => return Err("Type error".into()),
            };
            match lhs {
                SvmLiteralParam::U8(_) | SvmLiteralParam::I8(_) if rhs >= 8 => return Err("shl overflow".into()),
                SvmLiteralParam::U16(_) | SvmLiteralParam::I16(_) if rhs >= 16 => return Err("shl overflow".into()),
                SvmLiteralParam::U32(_) | SvmLiteralParam::I32(_) if rhs >= 32 => return Err("shl overflow".into()),
                SvmLiteralParam::U64(_) | SvmLiteralParam::I64(_) if rhs >= 64 => return Err("shl overflow".into()),
                SvmLiteralParam::U128(_) | SvmLiteralParam::I128(_) if rhs >= 128 => return Err("shl overflow".into()),
                SvmLiteralParam::U8(x) => {
                    let shifted = (**x) << rhs;
                    if (shifted >> rhs) != **x {
                        return Err("shl".into());
                    }
                    shifted.into()
                }
                SvmLiteralParam::U16(x) => {
                    let shifted = (**x) << rhs;
                    if (shifted >> rhs) != **x {
                        return Err("shl".into());
                    }
                    shifted.into()
                }
                SvmLiteralParam::U32(x) => {
                    let shifted = (**x) << rhs;
                    if (shifted >> rhs) != **x {
                        return Err("shl".into());
                    }
                    shifted.into()
                }
                SvmLiteralParam::U64(x) => {
                    let shifted = (**x) << rhs;
                    if (shifted >> rhs) != **x {
                        return Err("shl".into());
                    }
                    shifted.into()
                }
                SvmLiteralParam::U128(x) => {
                    let shifted = (**x) << rhs;
                    if (shifted >> rhs) != **x {
                        return Err("shl".into());
                    }
                    shifted.into()
                }
                SvmLiteralParam::I8(x) => {
                    let shifted = (**x) << rhs;
                    if (shifted >> rhs) != **x {
                        return Err("shl".into());
                    }
                    shifted.into()
                }
                SvmLiteralParam::I16(x) => {
                    let shifted = (**x) << rhs;
                    if (shifted >> rhs) != **x {
                        return Err("shl".into());
                    }
                    shifted.into()
                }
                SvmLiteralParam::I32(x) => {
                    let shifted = (**x) << rhs;
                    if (shifted >> rhs) != **x {
                        return Err("shl".into());
                    }
                    shifted.into()
                }
                SvmLiteralParam::I64(x) => {
                    let shifted = (**x) << rhs;
                    if (shifted >> rhs) != **x {
                        return Err("shl".into());
                    }
                    shifted.into()
                }
                SvmLiteralParam::I128(x) => {
                    let shifted = (**x) << rhs;
                    if (shifted >> rhs) != **x {
                        return Err("shl".into());
                    }
                    shifted.into()
                }
                _ => return Err("Type error".into()),
            }
        }

        BinaryOperation::ShlWrapped => {
            let rhs: u32 = match rhs {
                SvmLiteralParam::U8(y) => (**y).into(),
                SvmLiteralParam::U16(y) => (**y).into(),
                SvmLiteralParam::U32(y) => **y,
                _ => return Err("Type error".into()),
            };
            match lhs {
                SvmLiteralParam::U8(x) => x.wrapping_shl(rhs).into(),
                SvmLiteralParam::U16(x) => x.wrapping_shl(rhs).into(),
                SvmLiteralParam::U32(x) => x.wrapping_shl(rhs).into(),
                SvmLiteralParam::U64(x) => x.wrapping_shl(rhs).into(),
                SvmLiteralParam::U128(x) => x.wrapping_shl(rhs).into(),
                SvmLiteralParam::I8(x) => x.wrapping_shl(rhs).into(),
                SvmLiteralParam::I16(x) => x.wrapping_shl(rhs).into(),
                SvmLiteralParam::I32(x) => x.wrapping_shl(rhs).into(),
                SvmLiteralParam::I64(x) => x.wrapping_shl(rhs).into(),
                SvmLiteralParam::I128(x) => x.wrapping_shl(rhs).into(),
                _ => return Err("Type error".into()),
            }
        }

        BinaryOperation::Shr => {
            let rhs: u32 = match rhs {
                SvmLiteralParam::U8(y) => (**y).into(),
                SvmLiteralParam::U16(y) => (**y).into(),
                SvmLiteralParam::U32(y) => **y,
                _ => return Err("Type error".into()),
            };

            match lhs {
                SvmLiteralParam::U8(_) | SvmLiteralParam::I8(_) if rhs >= 8 => return Err("shr overflow".into()),
                SvmLiteralParam::U16(_) | SvmLiteralParam::I16(_) if rhs >= 16 => return Err("shr overflow".into()),
                SvmLiteralParam::U32(_) | SvmLiteralParam::I32(_) if rhs >= 32 => return Err("shr overflow".into()),
                SvmLiteralParam::U64(_) | SvmLiteralParam::I64(_) if rhs >= 64 => return Err("shr overflow".into()),
                SvmLiteralParam::U128(_) | SvmLiteralParam::I128(_) if rhs >= 128 => return Err("shr overflow".into()),
                SvmLiteralParam::U8(x) => (**x >> rhs).into(),
                SvmLiteralParam::U16(x) => (**x >> rhs).into(),
                SvmLiteralParam::U32(x) => (**x >> rhs).into(),
                SvmLiteralParam::U64(x) => (**x >> rhs).into(),
                SvmLiteralParam::U128(x) => (**x >> rhs).into(),
                SvmLiteralParam::I8(x) => (**x >> rhs).into(),
                SvmLiteralParam::I16(x) => (**x >> rhs).into(),
                SvmLiteralParam::I32(x) => (**x >> rhs).into(),
                SvmLiteralParam::I64(x) => (**x >> rhs).into(),
                SvmLiteralParam::I128(x) => (**x >> rhs).into(),
                _ => panic!("type checker failure"),
            }
        }

        BinaryOperation::ShrWrapped => {
            let rhs: u32 = match rhs {
                SvmLiteralParam::U8(y) => (**y).into(),
                SvmLiteralParam::U16(y) => (**y).into(),
                SvmLiteralParam::U32(y) => **y,
                _ => return Err("Type error".into()),
            };

            match lhs {
                SvmLiteralParam::U8(x) => (x.wrapping_shr(rhs)).into(),
                SvmLiteralParam::U16(x) => (x.wrapping_shr(rhs)).into(),
                SvmLiteralParam::U32(x) => (x.wrapping_shr(rhs)).into(),
                SvmLiteralParam::U64(x) => (x.wrapping_shr(rhs)).into(),
                SvmLiteralParam::U128(x) => (x.wrapping_shr(rhs)).into(),
                SvmLiteralParam::I8(x) => (x.wrapping_shr(rhs)).into(),
                SvmLiteralParam::I16(x) => (x.wrapping_shr(rhs)).into(),
                SvmLiteralParam::I32(x) => (x.wrapping_shr(rhs)).into(),
                SvmLiteralParam::I64(x) => (x.wrapping_shr(rhs)).into(),
                SvmLiteralParam::I128(x) => (x.wrapping_shr(rhs)).into(),
                _ => return Err("Type error".into()),
            }
        }

        BinaryOperation::Sub => {
            let Some(value) = (match (lhs, rhs) {
                (SvmLiteralParam::U8(x), SvmLiteralParam::U8(y)) => (**x).checked_sub(**y).map(|z| z.into()),
                (SvmLiteralParam::U16(x), SvmLiteralParam::U16(y)) => (**x).checked_sub(**y).map(|z| z.into()),
                (SvmLiteralParam::U32(x), SvmLiteralParam::U32(y)) => (**x).checked_sub(**y).map(|z| z.into()),
                (SvmLiteralParam::U64(x), SvmLiteralParam::U64(y)) => (**x).checked_sub(**y).map(|z| z.into()),
                (SvmLiteralParam::U128(x), SvmLiteralParam::U128(y)) => (**x).checked_sub(**y).map(|z| z.into()),
                (SvmLiteralParam::I8(x), SvmLiteralParam::I8(y)) => (**x).checked_sub(**y).map(|z| z.into()),
                (SvmLiteralParam::I16(x), SvmLiteralParam::I16(y)) => (**x).checked_sub(**y).map(|z| z.into()),
                (SvmLiteralParam::I32(x), SvmLiteralParam::I32(y)) => (**x).checked_sub(**y).map(|z| z.into()),
                (SvmLiteralParam::I64(x), SvmLiteralParam::I64(y)) => (**x).checked_sub(**y).map(|z| z.into()),
                (SvmLiteralParam::I128(x), SvmLiteralParam::I128(y)) => (**x).checked_sub(**y).map(|z| z.into()),
                (SvmLiteralParam::Group(x), SvmLiteralParam::Group(y)) => Some((*x - *y).into()),
                (SvmLiteralParam::Field(x), SvmLiteralParam::Field(y)) => Some((*x - *y).into()),
                _ => return Err("Type error".into()),
            }) else {
                return Err("sub overflow".into());
            };
            value
        }

        BinaryOperation::SubWrapped => match (lhs, rhs) {
            (SvmLiteralParam::U8(x), SvmLiteralParam::U8(y)) => (**x).wrapping_sub(**y).into(),
            (SvmLiteralParam::U16(x), SvmLiteralParam::U16(y)) => (**x).wrapping_sub(**y).into(),
            (SvmLiteralParam::U32(x), SvmLiteralParam::U32(y)) => (**x).wrapping_sub(**y).into(),
            (SvmLiteralParam::U64(x), SvmLiteralParam::U64(y)) => (**x).wrapping_sub(**y).into(),
            (SvmLiteralParam::U128(x), SvmLiteralParam::U128(y)) => (**x).wrapping_sub(**y).into(),
            (SvmLiteralParam::I8(x), SvmLiteralParam::I8(y)) => (**x).wrapping_sub(**y).into(),
            (SvmLiteralParam::I16(x), SvmLiteralParam::I16(y)) => (**x).wrapping_sub(**y).into(),
            (SvmLiteralParam::I32(x), SvmLiteralParam::I32(y)) => (**x).wrapping_sub(**y).into(),
            (SvmLiteralParam::I64(x), SvmLiteralParam::I64(y)) => (**x).wrapping_sub(**y).into(),
            (SvmLiteralParam::I128(x), SvmLiteralParam::I128(y)) => (**x).wrapping_sub(**y).into(),
            _ => return Err("Type error".into()),
        },
        BinaryOperation::Xor => match (lhs, rhs) {
            (SvmLiteralParam::Boolean(x), SvmLiteralParam::Boolean(y)) => (**x ^ **y).into(),
            (SvmLiteralParam::U8(x), SvmLiteralParam::U8(y)) => (**x ^ **y).into(),
            (SvmLiteralParam::U16(x), SvmLiteralParam::U16(y)) => (**x ^ **y).into(),
            (SvmLiteralParam::U32(x), SvmLiteralParam::U32(y)) => (**x ^ **y).into(),
            (SvmLiteralParam::U64(x), SvmLiteralParam::U64(y)) => (**x ^ **y).into(),
            (SvmLiteralParam::U128(x), SvmLiteralParam::U128(y)) => (**x ^ **y).into(),
            (SvmLiteralParam::I8(x), SvmLiteralParam::I8(y)) => (**x ^ **y).into(),
            (SvmLiteralParam::I16(x), SvmLiteralParam::I16(y)) => (**x ^ **y).into(),
            (SvmLiteralParam::I32(x), SvmLiteralParam::I32(y)) => (**x ^ **y).into(),
            (SvmLiteralParam::I64(x), SvmLiteralParam::I64(y)) => (**x ^ **y).into(),
            (SvmLiteralParam::I128(x), SvmLiteralParam::I128(y)) => (**x ^ **y).into(),
            _ => return Err("Type error".into()),
        },
    };

    Ok(value)
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
