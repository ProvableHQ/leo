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

const ARITHMATIC_TYPES: &[Type] = &[
    Type::Field,
    Type::Group,
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

const FIELD_AND_INT_TYPES: &[Type] = &[
    Type::Field,
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

const INT_TYPES: &[Type] = &[
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

impl<'a> TypeChecker<'a> {
    pub fn new(symbol_table: &'a mut SymbolTable<'a>, handler: &'a Handler) -> Self {
        Self {
            symbol_table,
            handler,
            parent: None,
            negate: false,
        }
    }

    pub(crate) fn assert_type(&self, type_: Type, expected: Option<Type>, span: &Span) -> Type {
        if let Some(expected) = expected {
            if type_ != expected {
                self.handler
                    .emit_err(TypeCheckerError::type_should_be(type_.clone(), expected, span).into());
            }
        }

        type_
    }

    pub(crate) fn assert_one_of_types(&self, type_: Option<Type>, expected: &[Type], span: &Span) -> Option<Type> {
        if let Some(type_) = type_.clone() {
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

    pub(crate) fn assert_arith_type(&self, type_: Option<Type>, span: &Span) -> Option<Type> {
        self.assert_one_of_types(type_, ARITHMATIC_TYPES, span)
    }

    pub(crate) fn assert_field_or_int_type(&self, type_: Option<Type>, span: &Span) -> Option<Type> {
        self.assert_one_of_types(type_, FIELD_AND_INT_TYPES, span)
    }

    pub(crate) fn assert_int_type(&self, type_: Option<Type>, span: &Span) -> Option<Type> {
        self.assert_one_of_types(type_, INT_TYPES, span)
    }
}
