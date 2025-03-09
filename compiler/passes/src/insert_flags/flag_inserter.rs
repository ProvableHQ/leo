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

use crate::{SymbolTable, TypeTable, VariableSymbol, VariableType};

use leo_ast::*;
use leo_errors::{StaticAnalyzerWarning, emitter::Handler};
use leo_span::{Span, Symbol, sym};

pub struct FlagInserter<'a> {
    /// The symbol table for the program.
    pub(crate) symbol_table: &'a mut SymbolTable,
    /// The type table for the program.
    pub(crate) type_table: &'a TypeTable,
    /// The error handler.
    pub(crate) handler: &'a Handler,
    /// The node builder.
    pub(crate) node_builder: &'a NodeBuilder,
    /// The name of the current program.
    pub(crate) program: Symbol,
    /// Depth of conditionals we're in.
    pub(crate) conditionals: u32,
}

impl<'a> FlagInserter<'a> {
    /// Returns a new static analyzer given a symbol table and error handler.
    pub fn new(
        symbol_table: &'a mut SymbolTable,
        type_table: &'a TypeTable,
        handler: &'a Handler,
        node_builder: &'a NodeBuilder,
    ) -> Self {
        Self { symbol_table, type_table, handler, node_builder, program: Symbol::intern(""), conditionals: 0u32 }
    }

    /// Emits a static analyzer warning.
    pub fn emit_warning(&self, warning: StaticAnalyzerWarning) {
        self.handler.emit_warning(warning.into());
    }

    pub(crate) fn in_scope<T>(&mut self, id: NodeID, func: impl FnOnce(&mut Self) -> T) -> T {
        self.symbol_table.enter_scope(Some(id));
        let result = func(self);
        self.symbol_table.enter_parent();
        result
    }

    pub(crate) fn insert_variable(&mut self, name: Symbol, type_: Type, span: Span, declaration: VariableType) {
        if let Err(e) =
            self.symbol_table.insert_variable(self.program, name, VariableSymbol { type_, span, declaration })
        {
            self.handler.emit_err(e);
        }
    }

    pub(crate) fn is_const(&self, expr: &Expression) -> bool {
        use Expression::*;

        let isc = |expr| self.is_const(expr);

        let var_is_const = |ident: &leo_ast::Identifier| -> bool {
            self.symbol_table
                .lookup_variable(self.program, ident.name)
                .map_or(false, |var_sym| var_sym.declaration == VariableType::Const)
        };

        match expr {
            Access(AccessExpression::Array(array)) => isc(&*array.array) && isc(&*array.index),
            Access(AccessExpression::AssociatedConstant(_)) => true,
            Access(AccessExpression::AssociatedFunction(assoc_function)) => {
                // TODO: Make this a function and put it somewhere sensible.
                // It can be const evaluated if it's not a cheat code, mapping, or rand.
                !matches!(assoc_function.variant.name, sym::CheatCode | sym::Mapping)
                    && !matches!(
                        assoc_function.name.name,
                        sym::rand_address
                            | sym::rand_bool
                            | sym::rand_field
                            | sym::rand_group
                            | sym::rand_i8
                            | sym::rand_i16
                            | sym::rand_i32
                            | sym::rand_i64
                            | sym::rand_i128
                            | sym::rand_scalar
                            | sym::rand_u8
                            | sym::rand_u16
                            | sym::rand_u32
                            | sym::rand_u64
                            | sym::rand_u128,
                    )
                    && assoc_function.arguments.iter().all(isc)
            }
            Access(AccessExpression::Member(_)) => todo!(),
            Access(AccessExpression::Tuple(_)) => todo!(),
            Array(array) => array.elements.iter().all(isc),
            Binary(bin) => isc(&bin.left) && isc(&bin.right),
            Call(..) => false,
            Cast(cast) => isc(&cast.expression),
            Err(..) => false,
            Identifier(identifier) => var_is_const(identifier),
            Struct(struct_) => struct_.members.iter().all(|initializer| {
                if let Some(expr) = initializer.expression.as_ref() {
                    isc(expr)
                } else {
                    var_is_const(&initializer.identifier)
                }
            }),
            Ternary(tern) => [&*tern.condition, &*tern.if_true, &*tern.if_false].into_iter().all(isc),
            Tuple(tuple) => tuple.elements.iter().all(isc),
            Unary(unary) => isc(&unary.receiver),
            Literal(_) | Locator(_) | Unit(_) => true,
        }
    }
}
