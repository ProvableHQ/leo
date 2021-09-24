// Copyright (C) 2019-2021 Aleo Systems Inc.
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

use crate::Circuit;
pub use leo_ast::IntegerType;

use std::fmt;

/// A type in an asg.
#[derive(Clone, PartialEq)]
pub enum Type<'a> {
    // Data types
    Address,
    Boolean,
    Char,
    Field,
    Group,
    Integer(IntegerType),

    // Data type wrappers
    Array(Box<Type<'a>>, usize),
    ArrayWithoutSize(Box<Type<'a>>),
    Tuple(Vec<Type<'a>>),
    Circuit(&'a Circuit<'a>),
}

#[derive(Clone, PartialEq)]
pub enum PartialType<'a> {
    Type(Type<'a>),                                    // non-array or tuple
    Integer(Option<IntegerType>, Option<IntegerType>), // specific, context-specific
    Array(Option<Box<PartialType<'a>>>, Option<usize>),
    Tuple(Vec<Option<PartialType<'a>>>),
}

impl<'a> Into<Option<Type<'a>>> for PartialType<'a> {
    fn into(self) -> Option<Type<'a>> {
        match self {
            PartialType::Type(t) => Some(t),
            PartialType::Integer(sub_type, contextual_type) => Some(Type::Integer(sub_type.or(contextual_type)?)),
            PartialType::Array(element, len) => Some(Type::Array(Box::new((*element?).full()?), len?)),
            PartialType::Tuple(sub_types) => Some(Type::Tuple(
                sub_types
                    .into_iter()
                    .map(|x| x.map(|x| x.full()).flatten())
                    .collect::<Option<Vec<Type>>>()?,
            )),
        }
    }
}

impl<'a> PartialType<'a> {
    pub fn full(self) -> Option<Type<'a>> {
        self.into()
    }

    pub fn matches(&self, other: &Type<'a>) -> bool {
        match (self, other) {
            (PartialType::Type(t), other) => t.is_assignable_from(other),
            (PartialType::Integer(self_sub_type, _), Type::Integer(sub_type)) => {
                self_sub_type.as_ref().map(|x| x == sub_type).unwrap_or(true)
            }
            (PartialType::Array(element, _len), Type::ArrayWithoutSize(other_element)) => {
                if let Some(element) = element {
                    if !element.matches(&*other_element) {
                        return false;
                    }
                }
                true
            }
            (PartialType::Array(element, len), Type::Array(other_element, other_len)) => {
                if let Some(element) = element {
                    if !element.matches(&*other_element) {
                        return false;
                    }
                }
                if let Some(len) = len {
                    return len == other_len;
                }
                true
            }
            (PartialType::Tuple(sub_types), Type::Tuple(other_sub_types)) => {
                // we dont enforce exact length for tuples here (relying on prior type checking) to allow for full-context-free tuple indexing
                if sub_types.len() > other_sub_types.len() {
                    return false;
                }
                for (sub_type, other_sub_type) in sub_types.iter().zip(other_sub_types.iter()) {
                    if let Some(sub_type) = sub_type {
                        if !sub_type.matches(other_sub_type) {
                            return false;
                        }
                    }
                }
                true
            }
            _ => false,
        }
    }
}

impl<'a> Into<PartialType<'a>> for Type<'a> {
    fn into(self) -> PartialType<'a> {
        match self {
            Type::Integer(sub_type) => PartialType::Integer(Some(sub_type), None),
            Type::Array(element, len) => PartialType::Array(Some(Box::new((*element).into())), Some(len)),
            Type::Tuple(sub_types) => PartialType::Tuple(sub_types.into_iter().map(Into::into).map(Some).collect()),
            x => PartialType::Type(x),
        }
    }
}

impl<'a> Type<'a> {
    pub fn is_assignable_from(&self, from: &Type<'a>) -> bool {
        match (self, from) {
            (Type::Array(_, _), Type::ArrayWithoutSize(_)) => true,
            (Type::ArrayWithoutSize(_), Type::Array(_, _)) => true,
            _ => self == from,
        }
    }

    pub fn partial(self) -> PartialType<'a> {
        self.into()
    }

    pub fn is_unit(&self) -> bool {
        matches!(self, Type::Tuple(t) if t.is_empty())
    }

    pub fn can_cast_to(&self, to: &Type<'a>) -> bool {
        matches!(self, Type::Integer(_)) && matches!(to, Type::Integer(_))
    }
}

impl<'a> fmt::Display for Type<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Type::Address => write!(f, "address"),
            Type::Boolean => write!(f, "bool"),
            Type::Char => write!(f, "char"),
            Type::Field => write!(f, "field"),
            Type::Group => write!(f, "group"),
            Type::Integer(sub_type) => sub_type.fmt(f),
            Type::Array(sub_type, len) => write!(f, "[{}; {}]", sub_type, len),
            Type::ArrayWithoutSize(sub_type) => write!(f, "[{}; _]", sub_type),
            Type::Tuple(sub_types) => {
                write!(f, "(")?;
                for (i, sub_type) in sub_types.iter().enumerate() {
                    write!(f, "{}", sub_type)?;
                    if i < sub_types.len() - 1 {
                        write!(f, ", ")?;
                    }
                }
                write!(f, ")")
            }
            Type::Circuit(circuit) => write!(f, "{}", &circuit.name.borrow().name),
        }
    }
}

impl<'a> fmt::Display for PartialType<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PartialType::Type(t) => t.fmt(f),
            PartialType::Integer(Some(sub_type), _) => write!(f, "{}", sub_type),
            PartialType::Integer(_, Some(sub_type)) => write!(f, "<{}>", sub_type),
            PartialType::Integer(_, _) => write!(f, "integer"),
            PartialType::Array(sub_type, len) => {
                write!(f, "[")?;
                if let Some(sub_type) = sub_type {
                    write!(f, "{}", *sub_type)?;
                } else {
                    write!(f, "?")?;
                }
                write!(f, "; ")?;
                if let Some(len) = len {
                    write!(f, "{}", len)?;
                } else {
                    write!(f, "?")?;
                }
                write!(f, "]")
            }
            PartialType::Tuple(sub_types) => {
                write!(f, "(")?;
                for (i, sub_type) in sub_types.iter().enumerate() {
                    if let Some(sub_type) = sub_type {
                        write!(f, "{}", *sub_type)?;
                    } else {
                        write!(f, "?")?;
                    }
                    if i < sub_types.len() - 1 {
                        write!(f, ", ")?;
                    }
                }
                write!(f, ")")
            }
        }
    }
}

impl<'a> Into<leo_ast::Type> for &Type<'a> {
    fn into(self) -> leo_ast::Type {
        use Type::*;
        match self {
            Address => leo_ast::Type::Address,
            Boolean => leo_ast::Type::Boolean,
            Char => leo_ast::Type::Char,
            Field => leo_ast::Type::Field,
            Group => leo_ast::Type::Group,
            Integer(int_type) => leo_ast::Type::IntegerType(int_type.clone()),
            Array(type_, len) => leo_ast::Type::Array(
                Box::new(type_.as_ref().into()),
                Some(leo_ast::ArrayDimensions(vec![leo_ast::PositiveNumber {
                    value: len.to_string().into(),
                }])),
            ),
            ArrayWithoutSize(type_) => leo_ast::Type::Array(Box::new(type_.as_ref().into()), None),
            Tuple(subtypes) => leo_ast::Type::Tuple(subtypes.iter().map(Into::into).collect()),
            Circuit(circuit) => leo_ast::Type::Identifier(circuit.name.borrow().clone()),
        }
    }
}
