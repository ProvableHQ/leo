// Copyright (C) 2019-2025 Aleo Systems Inc.
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

use crate::{SymbolTable, TypeTable};

use leo_ast::{Expression, IntegerType, Literal, NodeBuilder, NodeID, TupleExpression};
use leo_errors::{StaticAnalyzerError, emitter::Handler};
use leo_interpreter::Value;
use leo_span::{Span, Symbol};

/// A pass to perform const propagation and folding.
///
/// This pass should be used in conjunction with the Unroller so that
/// loop bounds and consts in loop bodies can be evaluated.
///
/// Any of these expressions:
/// 1. unary operation,
/// 2. binary operation,
/// 3. core function other than cheat codes, mapping ops, or rand functions,
///
/// whose arguments are consts or literals will be subject to constant folding.
/// The ternary conditional operator will also be folded if its condition is
/// a constant or literal.
pub struct ConstPropagator<'a> {
    /// The symbol table associated with the program.
    pub(crate) symbol_table: &'a mut SymbolTable,
    /// A mapping between node IDs and their types.
    pub(crate) type_table: &'a TypeTable,
    /// A counter used to generate unique node IDs.
    pub(crate) node_builder: &'a NodeBuilder,
    /// The error handler.
    pub(crate) handler: &'a Handler,
    /// The program name.
    pub(crate) program: Symbol,
    /// Have we actually modified the progam at all?
    pub(crate) changed: bool,
    /// The RHS of a const declaration we were not able to evaluate.
    pub(crate) const_not_evaluated: Option<Span>,
    /// An array index which was not able to be evaluated.
    pub(crate) array_index_not_evaluated: Option<Span>,
}

impl<'a> ConstPropagator<'a> {
    pub(crate) fn new(
        handler: &'a Handler,
        symbol_table: &'a mut SymbolTable,
        type_table: &'a TypeTable,
        node_builder: &'a NodeBuilder,
    ) -> Self {
        Self {
            handler,
            symbol_table,
            type_table,
            node_builder,
            program: Symbol::intern(""),
            changed: false,
            const_not_evaluated: None,
            array_index_not_evaluated: None,
        }
    }

    /// Enter the symbol table's scope `id`, execute `func`, and then return to the parent scope.
    pub(crate) fn in_scope<T>(&mut self, id: NodeID, func: impl FnOnce(&mut Self) -> T) -> T {
        self.symbol_table.enter_scope(Some(id));
        let result = func(self);
        self.symbol_table.enter_parent();
        result
    }

    /// Emit a `StaticAnalyzerError`.
    pub(crate) fn emit_err(&self, err: StaticAnalyzerError) {
        self.handler.emit_err(err);
    }
}

pub(crate) fn value_to_expression(value: &Value, span: Span, node_builder: &NodeBuilder) -> Expression {
    use Value::*;
    let id = node_builder.next_id();

    match value {
        Unit => Expression::Unit(leo_ast::UnitExpression { span, id }),
        Bool(x) => Expression::Literal(Literal::Boolean(*x, span, id)),
        U8(x) => Expression::Literal(Literal::Integer(IntegerType::U8, format!("{x}"), span, id)),
        U16(x) => Expression::Literal(Literal::Integer(IntegerType::U16, format!("{x}"), span, id)),
        U32(x) => Expression::Literal(Literal::Integer(IntegerType::U32, format!("{x}"), span, id)),
        U64(x) => Expression::Literal(Literal::Integer(IntegerType::U64, format!("{x}"), span, id)),
        U128(x) => Expression::Literal(Literal::Integer(IntegerType::U128, format!("{x}"), span, id)),
        I8(x) => Expression::Literal(Literal::Integer(IntegerType::I8, format!("{x}"), span, id)),
        I16(x) => Expression::Literal(Literal::Integer(IntegerType::I16, format!("{x}"), span, id)),
        I32(x) => Expression::Literal(Literal::Integer(IntegerType::I32, format!("{x}"), span, id)),
        I64(x) => Expression::Literal(Literal::Integer(IntegerType::I64, format!("{x}"), span, id)),
        I128(x) => Expression::Literal(Literal::Integer(IntegerType::I128, format!("{x}"), span, id)),
        Group(x) => {
            let mut s = format!("{x}");
            // Strip off the `group` suffix.
            s.truncate(s.len() - 5);
            Expression::Literal(Literal::Group(s, span, id))
        }
        Field(x) => {
            let mut s = format!("{x}");
            // Strip off the `field` suffix.
            s.truncate(s.len() - 5);
            Expression::Literal(Literal::Field(s, span, id))
        }
        Scalar(x) => {
            let mut s = format!("{x}");
            // Strip off the `scalar` suffix.
            s.truncate(s.len() - 6);
            Expression::Literal(Literal::Scalar(s, span, id))
        }
        Tuple(x) => {
            let elements = x.iter().map(|val| value_to_expression(val, span, node_builder)).collect();
            Expression::Tuple(TupleExpression { elements, span, id })
        }
        _ => panic!("Can only evaluate literals and tuples."),
    }
}
