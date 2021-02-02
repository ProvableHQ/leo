// Copyright (C) 2019-2020 Aleo Systems Inc.
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

use std::{
    fmt,
    sync::{Arc, Weak},
};

/// A type in an asg.
#[derive(Clone, PartialEq)]
pub enum Type {
    // Data types
    Address,
    Boolean,
    Field,
    Group,
    Integer(IntegerType),

    // Data type wrappers
    Array(Box<Type>, usize),
    Tuple(Vec<Type>),
    Circuit(Arc<Circuit>),
}

#[derive(Clone)]
pub enum WeakType {
    Type(Type), // circuit not allowed
    Circuit(Weak<Circuit>),
}

#[derive(Clone, PartialEq)]
pub enum PartialType {
    Type(Type),                                        // non-array or tuple
    Integer(Option<IntegerType>, Option<IntegerType>), // specific, context-specific
    Array(Option<Box<PartialType>>, Option<usize>),
    Tuple(Vec<Option<PartialType>>),
}

impl Into<Type> for WeakType {
    fn into(self) -> Type {
        match self {
            WeakType::Type(t) => t,
            WeakType::Circuit(circuit) => Type::Circuit(circuit.upgrade().unwrap()),
        }
    }
}

impl WeakType {
    pub fn strong(self) -> Type {
        self.into()
    }

    pub fn is_unit(&self) -> bool {
        matches!(self, WeakType::Type(Type::Tuple(t)) if t.is_empty())
    }
}

impl Into<WeakType> for Type {
    fn into(self) -> WeakType {
        match self {
            Type::Circuit(circuit) => WeakType::Circuit(Arc::downgrade(&circuit)),
            t => WeakType::Type(t),
        }
    }
}

impl Into<Option<Type>> for PartialType {
    fn into(self) -> Option<Type> {
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

impl PartialType {
    pub fn full(self) -> Option<Type> {
        self.into()
    }

    pub fn matches(&self, other: &Type) -> bool {
        match (self, other) {
            (PartialType::Type(t), other) => t.is_assignable_from(other),
            (PartialType::Integer(self_sub_type, _), Type::Integer(sub_type)) => {
                self_sub_type.as_ref().map(|x| x == sub_type).unwrap_or(true)
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

impl Into<PartialType> for Type {
    fn into(self) -> PartialType {
        match self {
            Type::Integer(sub_type) => PartialType::Integer(Some(sub_type), None),
            Type::Array(element, len) => PartialType::Array(Some(Box::new((*element).into())), Some(len)),
            Type::Tuple(sub_types) => PartialType::Tuple(sub_types.into_iter().map(Into::into).map(Some).collect()),
            x => PartialType::Type(x),
        }
    }
}

impl Type {
    pub fn is_assignable_from(&self, from: &Type) -> bool {
        self == from
    }

    pub fn partial(self) -> PartialType {
        self.into()
    }

    pub fn is_unit(&self) -> bool {
        matches!(self, Type::Tuple(t) if t.is_empty())
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Type::Address => write!(f, "address"),
            Type::Boolean => write!(f, "bool"),
            Type::Field => write!(f, "field"),
            Type::Group => write!(f, "group"),
            Type::Integer(sub_type) => sub_type.fmt(f),
            Type::Array(sub_type, len) => write!(f, "[{}; {}]", sub_type, len),
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

impl fmt::Display for PartialType {
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

impl Into<leo_ast::Type> for &Type {
    fn into(self) -> leo_ast::Type {
        use Type::*;
        match self {
            Address => leo_ast::Type::Address,
            Boolean => leo_ast::Type::Boolean,
            Field => leo_ast::Type::Field,
            Group => leo_ast::Type::Group,
            Integer(int_type) => leo_ast::Type::IntegerType(int_type.clone()),
            Array(type_, len) => leo_ast::Type::Array(
                Box::new(type_.as_ref().into()),
                leo_ast::ArrayDimensions(vec![leo_ast::PositiveNumber { value: len.to_string() }]),
            ),
            Tuple(subtypes) => leo_ast::Type::Tuple(subtypes.iter().map(Into::into).collect()),
            Circuit(circuit) => leo_ast::Type::Circuit(circuit.name.borrow().clone()),
        }
    }
}
