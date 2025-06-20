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
    RepeatExpression,
    StructExpression,
    StructVariableInitializer,
    TupleExpression,
    interpreter_value::Value,
};
use leo_errors::StaticAnalyzerError;
use leo_span::{Span, Symbol};

pub struct ConstPropagationVisitor<'a> {
    pub state: &'a mut CompilerState,
    /// The program name.
    pub program: Symbol,
    /// Have we actually modified the program at all?
    pub changed: bool,
    /// The RHS of a const declaration we were not able to evaluate.
    pub const_not_evaluated: Option<Span>,
    /// An array index which was not able to be evaluated.
    pub array_index_not_evaluated: Option<Span>,
    /// An array length which was not able to be evaluated.
    pub array_length_not_evaluated: Option<Span>,
    /// A repeat expression count which was not able to be evaluated.
    pub repeat_count_not_evaluated: Option<Span>,
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
        Unit => leo_ast::UnitExpression { span, id }.into(),
        Bool(x) => Literal::boolean(*x, span, id).into(),
        U8(x) => Literal::integer(IntegerType::U8, format!("{x}"), span, id).into(),
        U16(x) => Literal::integer(IntegerType::U16, format!("{x}"), span, id).into(),
        U32(x) => Literal::integer(IntegerType::U32, format!("{x}"), span, id).into(),
        U64(x) => Literal::integer(IntegerType::U64, format!("{x}"), span, id).into(),
        U128(x) => Literal::integer(IntegerType::U128, format!("{x}"), span, id).into(),
        I8(x) => Literal::integer(IntegerType::I8, format!("{x}"), span, id).into(),
        I16(x) => Literal::integer(IntegerType::I16, format!("{x}"), span, id).into(),
        I32(x) => Literal::integer(IntegerType::I32, format!("{x}"), span, id).into(),
        I64(x) => Literal::integer(IntegerType::I64, format!("{x}"), span, id).into(),
        I128(x) => Literal::integer(IntegerType::I128, format!("{x}"), span, id).into(),
        Address(x) => Literal::address(format!("{x}"), span, id).into(),
        Group(x) => {
            let mut s = format!("{x}");
            // Strip off the `group` suffix.
            s.truncate(s.len() - 5);
            Literal::group(s, span, id).into()
        }
        Field(x) => {
            let mut s = format!("{x}");
            // Strip off the `field` suffix.
            s.truncate(s.len() - 5);
            Literal::field(s, span, id).into()
        }
        Scalar(x) => {
            let mut s = format!("{x}");
            // Strip off the `scalar` suffix.
            s.truncate(s.len() - 6);
            Literal::scalar(s, span, id).into()
        }
        Tuple(x) => {
            let mut elements = Vec::with_capacity(x.len());
            for value in x.iter() {
                elements.push(value_to_expression(value, span, node_builder)?);
            }
            TupleExpression { elements, span, id }.into()
        }
        Array(x) => {
            let mut elements = Vec::with_capacity(x.len());
            for value in x.iter() {
                elements.push(value_to_expression(value, span, node_builder)?);
            }
            ArrayExpression { elements, span, id }.into()
        }
        Repeat(expr, count) => RepeatExpression {
            expr: value_to_expression(expr, span, node_builder)?,
            count: value_to_expression(count, span, node_builder)?,
            span,
            id,
        }
        .into(),
        Struct(x) => StructExpression {
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
        }
        .into(),
        Future(..) => return None,
    };

    Some(result)
}
