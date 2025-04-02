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

use crate::CompilerState;

use leo_ast::{
    ArrayExpression,
    Expression,
    Identifier,
    IntegerType,
    Literal,
    NodeBuilder,
    NodeID,
    StructExpression,
    StructVariableInitializer,
    TupleExpression,
};
use leo_errors::StaticAnalyzerError;
use leo_interpreter::Value;
use leo_span::{Span, Symbol};

pub struct ConstPropagationVisitor<'a> {
    pub state: &'a mut CompilerState,
    /// The program name.
    pub program: Symbol,
    /// Have we actually modified the progam at all?
    pub changed: bool,
    /// The RHS of a const declaration we were not able to evaluate.
    pub const_not_evaluated: Option<Span>,
    /// An array index which was not able to be evaluated.
    pub array_index_not_evaluated: Option<Span>,
}

impl ConstPropagationVisitor<'_> {
    /// Enter the symbol table's scope `id`, execute `func`, and then return to the parent scope.
    pub fn in_scope<T>(&mut self, id: NodeID, func: impl FnOnce(&mut Self) -> T) -> T {
        self.state.symbol_table.enter_scope(Some(id));
        let result = func(self);
        self.state.symbol_table.enter_parent();
        result
    }

    /// Emit a `StaticAnalyzerError`.
    pub fn emit_err(&self, err: StaticAnalyzerError) {
        self.state.handler.emit_err(err);
    }
}

pub fn value_to_expression(value: &Value, span: Span, node_builder: &NodeBuilder) -> Option<Expression> {
    use Value::*;
    let id = node_builder.next_id();

    let result = match value {
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
        Address(x) => Expression::Literal(Literal::Address(format!("{x}"), span, id)),
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
            let mut elements = Vec::with_capacity(x.len());
            for value in x.iter() {
                elements.push(value_to_expression(value, span, node_builder)?);
            }
            Expression::Tuple(TupleExpression { elements, span, id })
        }
        Array(x) => {
            let mut elements = Vec::with_capacity(x.len());
            for value in x.iter() {
                elements.push(value_to_expression(value, span, node_builder)?);
            }
            Expression::Array(ArrayExpression { elements, span, id })
        }
        Struct(x) => Expression::Struct(StructExpression {
            name: Identifier { name: x.name, id: node_builder.next_id(), span },
            members: {
                let mut members = Vec::with_capacity(x.contents.len());
                for (name, val) in x.contents.iter() {
                    let initializer = StructVariableInitializer {
                        identifier: Identifier { name: *name, id: node_builder.next_id(), span },
                        expression: Some(value_to_expression(val, span, node_builder)?),
                        span,
                        id: node_builder.next_id(),
                    };
                    members.push(initializer)
                }
                members
            },
            span,
            id,
        }),
        Future(..) => return None,
    };

    Some(result)
}
