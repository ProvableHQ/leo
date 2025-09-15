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

use leo_ast::*;
use leo_span::{Span, Symbol};

use indexmap::IndexMap;

pub struct StorageLoweringVisitor<'a> {
    pub state: &'a mut CompilerState,
    // The name of the current program scope
    pub program: Symbol,
    pub new_mappings: IndexMap<Symbol, Mapping>,
}

#[allow(clippy::type_complexity)]
pub fn zero_value_expression(
    ty: &Type,
    span: Span,
    node_builder: &NodeBuilder,
    struct_lookup: &dyn Fn(&[Symbol]) -> Vec<(Symbol, Type)>,
) -> Option<Expression> {
    let id = node_builder.next_id();

    match ty {
        // Numeric types
        Type::Integer(IntegerType::I8) => Some(Literal::integer(IntegerType::I8, "0".to_string(), span, id).into()),
        Type::Integer(IntegerType::I16) => Some(Literal::integer(IntegerType::I16, "0".to_string(), span, id).into()),
        Type::Integer(IntegerType::I32) => Some(Literal::integer(IntegerType::I32, "0".to_string(), span, id).into()),
        Type::Integer(IntegerType::I64) => Some(Literal::integer(IntegerType::I64, "0".to_string(), span, id).into()),
        Type::Integer(IntegerType::I128) => Some(Literal::integer(IntegerType::I128, "0".to_string(), span, id).into()),
        Type::Integer(IntegerType::U8) => Some(Literal::integer(IntegerType::U8, "0".to_string(), span, id).into()),
        Type::Integer(IntegerType::U16) => Some(Literal::integer(IntegerType::U16, "0".to_string(), span, id).into()),
        Type::Integer(IntegerType::U32) => Some(Literal::integer(IntegerType::U32, "0".to_string(), span, id).into()),
        Type::Integer(IntegerType::U64) => Some(Literal::integer(IntegerType::U64, "0".to_string(), span, id).into()),
        Type::Integer(IntegerType::U128) => Some(Literal::integer(IntegerType::U128, "0".to_string(), span, id).into()),

        // Boolean
        Type::Boolean => Some(Literal::boolean(false, span, id).into()),

        // Field, Group, Scalar
        Type::Field => Some(Literal::field("0".to_string(), span, id).into()),
        Type::Group => Some(Literal::group("0".to_string(), span, id).into()),
        Type::Scalar => Some(Literal::scalar("0".to_string(), span, id).into()),
        Type::Address => {
            // TODO: is the arbitrary address here safe to use?
            Some(
                Literal::address(
                    "aleo1fj982yqchhy973kz7e9jk6er7t6qd6jm9anplnlprem507w6lv9spwvfxx".to_string(),
                    span,
                    id,
                )
                .into(),
            )
        }

        // Structs (composite types)
        Type::Composite(composite_type) => {
            let path = &composite_type.path;
            let members = struct_lookup(&path.absolute_path());

            let struct_members = members
                .into_iter()
                .map(|(symbol, member_type)| {
                    let member_id = node_builder.next_id();
                    let zero_expr = zero_value_expression(&member_type, span, node_builder, struct_lookup)?;

                    Some(StructVariableInitializer {
                        span,
                        id: member_id,
                        identifier: Identifier::new(symbol, node_builder.next_id()),
                        expression: Some(zero_expr),
                    })
                })
                .collect::<Option<Vec<_>>>()?;

            Some(Expression::Struct(StructExpression {
                span,
                id,
                path: path.clone(),
                const_arguments: Vec::new(),
                members: struct_members,
            }))
        }

        // Arrays
        Type::Array(array_type) => {
            let element_ty = &array_type.element_type;
            let num_elements = array_type.length.as_u32().expect("should have been const evaluated by now.");

            let element_expr = zero_value_expression(element_ty, span, node_builder, struct_lookup)?;
            let elements = vec![element_expr; num_elements as usize];

            Some(Expression::Array(ArrayExpression { span, id, elements }))
        }

        // Other types are not expected
        _ => None,
    }
}
