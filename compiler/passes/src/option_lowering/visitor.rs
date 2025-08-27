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

use leo_ast::{interpreter_value::Value, *};
use leo_span::{Span, Symbol};

use indexmap::IndexMap;

pub struct OptionLoweringVisitor<'a> {
    pub state: &'a mut CompilerState,
    pub program: Symbol,
    pub module: Vec<Symbol>,
    pub function: Option<Symbol>,
    pub new_structs: IndexMap<Symbol, Composite>,
}

impl OptionLoweringVisitor<'_> {
    pub fn wrap_optional_value(&mut self, expr: Expression, ty: Type) -> (Expression, Vec<Statement>) {
        let is_some_expr = Expression::Literal(Literal {
            span: Span::default(),
            id: self.state.node_builder.next_id(),
            variant: LiteralVariant::Boolean(true),
        });

        // Fully lower the type before proceeding. This also ensures that all required structs
        // are actually registered in `self.new_structs`.
        let lowered_inner_type = self.reconstruct_type(ty).0;

        let struct_name = Symbol::intern(&crate::sanitize_name(&format!("Op__{lowered_inner_type}")));
        let struct_expr = StructExpression {
            path: Path::from(Identifier::new(struct_name, self.state.node_builder.next_id())).into_absolute(),
            const_arguments: vec![],
            members: vec![
                StructVariableInitializer {
                    identifier: Identifier::new(Symbol::intern("is_some"), self.state.node_builder.next_id()),
                    expression: Some(is_some_expr.clone()),
                    span: Span::default(),
                    id: self.state.node_builder.next_id(),
                },
                StructVariableInitializer {
                    identifier: Identifier::new(Symbol::intern("val"), self.state.node_builder.next_id()),
                    expression: Some(expr.clone()),
                    span: Span::default(),
                    id: self.state.node_builder.next_id(),
                },
            ],
            span: Span::default(),
            id: self.state.node_builder.next_id(),
        };

        (struct_expr.into(), vec![])
    }

    pub fn wrap_none(&mut self, inner_ty: &Type) -> (Expression, Vec<Statement>) {
        let is_some_expr = Expression::Literal(Literal {
            span: Span::default(),
            id: self.state.node_builder.next_id(),
            variant: LiteralVariant::Boolean(false),
        });

        // Fully lower the type before proceeding. This also ensures that all required structs
        // are actually registered in `self.new_structs`.
        let lowered_inner_type = self.reconstruct_type(inner_ty.clone()).0;

        let zero_val = self.zero_value_for_type(inner_ty);
        let zero_val_expr = self
            .value_to_expression(&zero_val, inner_ty, Span::default(), self.state.node_builder.next_id())
            .expect("must be valid");

        let struct_name = Symbol::intern(&crate::sanitize_name(&format!("Op__{lowered_inner_type}")));

        let struct_expr = StructExpression {
            path: Path::from(Identifier::new(struct_name, self.state.node_builder.next_id())).into_absolute(),
            const_arguments: vec![],
            members: vec![
                StructVariableInitializer {
                    identifier: Identifier::new(Symbol::intern("is_some"), self.state.node_builder.next_id()),
                    expression: Some(is_some_expr.clone()),
                    span: Span::default(),
                    id: self.state.node_builder.next_id(),
                },
                StructVariableInitializer {
                    identifier: Identifier::new(Symbol::intern("val"), self.state.node_builder.next_id()),
                    expression: Some(zero_val_expr.clone()),
                    span: Span::default(),
                    id: self.state.node_builder.next_id(),
                },
            ],
            span: Span::default(),
            id: self.state.node_builder.next_id(),
        };

        (struct_expr.into(), vec![])
    }

    pub fn zero_value_for_type(&mut self, ty: &Type) -> Value {
        let program = self.program.clone();
        match ty {
            Type::Unit => Value::make_unit(),

            Type::Boolean => Value::from(false),

            Type::Integer(int_type) => match int_type {
                IntegerType::U8 => Value::from(0u8),
                IntegerType::U16 => Value::from(0u16),
                IntegerType::U32 => Value::from(0u32),
                IntegerType::U64 => Value::from(0u64),
                IntegerType::U128 => Value::from(0u128),
                IntegerType::I8 => Value::from(0i8),
                IntegerType::I16 => Value::from(0i16),
                IntegerType::I32 => Value::from(0i32),
                IntegerType::I64 => Value::from(0i64),
                IntegerType::I128 => Value::from(0i128),
            },

            Type::Array(array) => {
                let len = array.length.as_u32().unwrap_or(0);
                let element = self.zero_value_for_type(&array.element_type);
                Value::make_array((0..len).map(|_| element.clone()))
            }

            Type::Tuple(tuple) => {
                let elements = tuple.elements.iter().map(|ty| self.zero_value_for_type(ty));
                Value::make_tuple(elements)
            }

            Type::Optional(opt) => {
                let lowered_inner_type = self.reconstruct_type((*opt.inner).clone()).0;
                let inner_zero = self.zero_value_for_type(&opt.inner);

                let mut struct_fields = IndexMap::new();
                struct_fields.insert(Symbol::intern("is_some"), Value::from(false));
                struct_fields.insert(Symbol::intern("val"), inner_zero);

                let opt_struct_name = crate::sanitize_name(&format!("Op__{lowered_inner_type}"));
                Value::make_struct(
                    struct_fields.into_iter(),
                    Symbol::intern("self"), // or whatever program name applies
                    vec![Symbol::intern(&opt_struct_name)],
                )
            }

            Type::Composite(composite) => {
                // Step 1: Immutable borrow ends early
                let members = {
                    let struct_def = self
                        .state
                        .symbol_table
                        .lookup_struct(&composite.path.absolute_path())
                        .expect("must be in symbol table");

                    struct_def.members.clone()
                };

                // Step 2: Mutably borrow self and compute values
                let contents = members
                    .into_iter()
                    .map(|member| {
                        let value = self.zero_value_for_type(&member.type_);
                        (member.identifier.name, value)
                    })
                    .collect::<Vec<_>>();

                Value::make_struct(contents.into_iter(), program, composite.path.absolute_path())
            }
            // Catch-all fallback for unhandled types
            _ => Value::make_unit(),
        }
    }

    pub fn value_to_expression(&self, value: &Value, ty: &Type, span: Span, id: NodeID) -> Option<Expression> {
        dbg!(&value);
        let symbol_table = &self.state.symbol_table;
        let struct_lookup = |sym: &[Symbol]| {
            symbol_table
                .lookup_struct(sym)
                .unwrap()
                .members
                .iter()
                .map(|mem| (mem.identifier.name, mem.type_.clone()))
                .collect()
        };
        value.to_expression(span, &self.state.node_builder, &ty, &struct_lookup)
    }
}
