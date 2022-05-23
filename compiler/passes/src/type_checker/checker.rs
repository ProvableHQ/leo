// Copyright (C) 2019-2022 Aleo Systems Inc.
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

use leo_ast::{IntegerType, Type};
use leo_errors::{emitter::Handler, TypeCheckerError};
use leo_span::{Span, Symbol};

use crate::SymbolTable;

pub struct TypeChecker<'a> {
    pub(crate) symbol_table: &'a mut SymbolTable<'a>,
    pub(crate) handler: &'a Handler,
    pub(crate) parent: Option<Symbol>,
    pub(crate) negate: bool,
}

const INT_TYPES: [Type; 10] = [
    Type::IntegerType(IntegerType::I8),
    Type::IntegerType(IntegerType::I16),
    Type::IntegerType(IntegerType::I32),
    Type::IntegerType(IntegerType::I64),
    Type::IntegerType(IntegerType::I128),
    Type::IntegerType(IntegerType::U8),
    Type::IntegerType(IntegerType::U16),
    Type::IntegerType(IntegerType::U32),
    Type::IntegerType(IntegerType::U64),
    Type::IntegerType(IntegerType::U128),
];

const fn create_type_superset<const S: usize, const A: usize, const O: usize>(
    subset: [Type; S],
    additional: [Type; A],
) -> [Type; O] {
    let mut superset: [Type; O] = [Type::IntegerType(IntegerType::U8); O];
    let mut i = 0;
    while i < S {
        superset[i] = subset[i];
        i += 1;
    }
    let mut j = 0;
    while j < A {
        superset[i + j] = additional[j];
        j += 1;
    }
    superset
}

const FIELD_INT_TYPES: [Type; 11] = create_type_superset(INT_TYPES, [Type::Field]);

const FIELD_SCALAR_INT_TYPES: [Type; 12] = create_type_superset(FIELD_INT_TYPES, [Type::Scalar]);

const FIELD_GROUP_INT_TYPES: [Type; 12] = create_type_superset(FIELD_INT_TYPES, [Type::Group]);

const ALL_NUMERICAL_TYPES: [Type; 13] = create_type_superset(FIELD_GROUP_INT_TYPES, [Type::Scalar]);

impl<'a> TypeChecker<'a> {
    pub fn new(symbol_table: &'a mut SymbolTable<'a>, handler: &'a Handler) -> Self {
        Self {
            symbol_table,
            handler,
            parent: None,
            negate: false,
        }
    }

    pub(crate) fn assert_type(&self, type_: Type, expected: Option<Type>, span: Span) -> Type {
        if let Some(expected) = expected {
            if type_ != expected {
                self.handler
                    .emit_err(TypeCheckerError::type_should_be(type_, expected, span).into());
            }
        }

        type_
    }

    pub(crate) fn assert_one_of_types(&self, type_: Option<Type>, expected: &[Type], span: Span) -> Option<Type> {
        if let Some(type_) = type_ {
            for t in expected.iter() {
                if &type_ == t {
                    return Some(type_);
                }
            }

            self.handler.emit_err(
                TypeCheckerError::expected_one_type_of(
                    expected.iter().map(|t| t.to_string() + ",").collect::<String>(),
                    type_,
                    span,
                )
                .into(),
            );
        }

        type_
    }

    pub(crate) fn _assert_arith_type(&self, type_: Option<Type>, span: Span) -> Option<Type> {
        self.assert_one_of_types(type_, &FIELD_GROUP_INT_TYPES, span)
    }

    pub(crate) fn _assert_field_or_int_type(&self, type_: Option<Type>, span: Span) -> Option<Type> {
        self.assert_one_of_types(type_, &FIELD_INT_TYPES, span)
    }

    pub(crate) fn _assert_int_type(&self, type_: Option<Type>, span: Span) -> Option<Type> {
        self.assert_one_of_types(type_, &INT_TYPES, span)
    }

    pub(crate) fn assert_field_group_scalar_int_type(&self, type_: Option<Type>, span: Span) -> Option<Type> {
        self.assert_one_of_types(type_, &ALL_NUMERICAL_TYPES, span)
    }

    pub(crate) fn assert_field_group_int_type(&self, type_: Option<Type>, span: Span) -> Option<Type> {
        self.assert_one_of_types(type_, &FIELD_GROUP_INT_TYPES, span)
    }

    pub(crate) fn assert_field_scalar_int_type(&self, type_: Option<Type>, span: Span) -> Option<Type> {
        self.assert_one_of_types(type_, &FIELD_SCALAR_INT_TYPES, span)
    }

    pub(crate) fn assert_field_int_type(&self, type_: Option<Type>, span: Span) -> Option<Type> {
        self.assert_one_of_types(type_, &FIELD_INT_TYPES, span)
    }
}
