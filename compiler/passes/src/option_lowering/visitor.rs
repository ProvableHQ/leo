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

pub struct OptionLoweringVisitor<'a> {
    pub state: &'a mut CompilerState,
    // The name of the current program scope
    pub program: Symbol,
    // The path to the current module. This should be an empty vector for the root.
    pub module: Vec<Symbol>,
    // The name of the current function, if we're inside one.
    pub function: Option<Symbol>,
    // The newly created structs. Each struct correspond to a converted optional type. All these
    // structs are to be inserted in the program scope.
    pub new_structs: IndexMap<Symbol, Composite>,
    // The reconstructed structs. These are the new versions of the existing structs in the program.
    pub reconstructed_structs: IndexMap<Vec<Symbol>, Composite>,
}

impl OptionLoweringVisitor<'_> {
    /// Enter module scope with path `module`, execute `func`, and then return to the parent module.
    pub fn in_module_scope<T>(&mut self, module: &[Symbol], func: impl FnOnce(&mut Self) -> T) -> T {
        let parent_module = self.module.clone();
        self.module = module.to_vec();
        let result = func(self);
        self.module = parent_module;
        result
    }

    /// Wraps an expression of a given type in an `Optional<T>`-like struct representing `Some(value)`.
    ///
    /// This function creates a struct expression that encodes an optional value with `is_some = true`
    /// and the provided expression as the `val` field. It also ensures that the type is fully
    /// reconstructed, which guarantees that all necessary struct definitions are available and registered.
    ///
    /// # Parameters
    /// - `expr`: The expression to wrap as the value of the optional.
    /// - `ty`: The type of the expression.
    ///
    /// # Returns
    /// - An `Expression` representing the constructed `Optional<T>` struct instance.
    pub fn wrap_optional_value(&mut self, expr: Expression, ty: Type) -> Expression {
        let is_some_expr = Expression::Literal(Literal {
            span: Span::default(),
            id: self.state.node_builder.next_id(),
            variant: LiteralVariant::Boolean(true),
        });

        // Fully lower the type before proceeding. This also ensures that all required structs
        // are actually registered in `self.new_structs`.
        let lowered_inner_type = self.reconstruct_type(ty).0;

        let struct_name = crate::make_optional_struct_symbol(&lowered_inner_type);
        let struct_expr = StructExpression {
            path: Path::from(Identifier::new(struct_name, self.state.node_builder.next_id())).into_absolute(),
            const_arguments: vec![],
            members: vec![
                StructVariableInitializer {
                    identifier: Identifier::new(Symbol::intern("is_some"), self.state.node_builder.next_id()),
                    expression: Some(is_some_expr),
                    span: Span::default(),
                    id: self.state.node_builder.next_id(),
                },
                StructVariableInitializer {
                    identifier: Identifier::new(Symbol::intern("val"), self.state.node_builder.next_id()),
                    expression: Some(expr),
                    span: Span::default(),
                    id: self.state.node_builder.next_id(),
                },
            ],
            span: Span::default(),
            id: self.state.node_builder.next_id(),
        };

        struct_expr.into()
    }

    /// Constructs an `Optional<T>`-like struct representing `None` for a given inner type.
    ///
    /// The returned struct expression sets `is_some = false`, and provides a zero value for the `val`
    /// field, where "zero" is defined according to the type:
    /// numeric types use literal zero, structs are recursively zero-initialized, etc.
    ///
    /// This function assumes that all required struct types are already reconstructed and registered.
    ///
    /// # Parameters
    /// - `inner_ty`: The type `T` inside the `Optional<T>`.
    ///
    /// # Returns
    /// - An `Expression` representing the constructed `Optional<T>` struct instance with `None`.
    pub fn wrap_none(&mut self, inner_ty: &Type) -> Expression {
        let is_some_expr = Expression::Literal(Literal {
            span: Span::default(),
            id: self.state.node_builder.next_id(),
            variant: LiteralVariant::Boolean(false),
        });

        // Fully lower the type before proceeding. This also ensures that all required structs
        // are actually registered in `self.new_structs`.
        let lowered_inner_type = self.reconstruct_type(inner_ty.clone()).0;

        // Even though the `val` field of the struct will not be used as long as `is_some` is
        // `false`, we still have to set it to something. We choose "zero", whatever "zero" means
        // for each type.

        // Instead of relying on the symbol table (which does not get updated in this pass), we rely on the set of
        // reconstructed structs which is produced for all program scopes and all modules before doing anything else.
        let reconstructed_structs = &self.reconstructed_structs;
        let struct_lookup = |sym: &[Symbol]| {
            reconstructed_structs
                .get(sym) // check the new version of existing structs
                .or_else(|| self.new_structs.get(sym.last().unwrap())) // check the newly produced structs
                .expect("must exist by construction")
                .members
                .iter()
                .map(|mem| (mem.identifier.name, mem.type_.clone()))
                .collect()
        };

        let zero_val_expr =
            zero_value_expression(&lowered_inner_type, Span::default(), &self.state.node_builder, &struct_lookup)
                .expect("");

        let struct_name = crate::make_optional_struct_symbol(&lowered_inner_type);

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

        struct_expr.into()
    }
}

#[allow(clippy::type_complexity)]
fn zero_value_expression(
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
                const_arguments: composite_type.const_arguments.clone(),
                members: struct_members,
            }))
        }

        // Arrays
        Type::Array(array_type) => {
            let element_ty = &array_type.element_type;

            let element_expr = zero_value_expression(element_ty, span, node_builder, struct_lookup)?;

            Some(Expression::Repeat(
                RepeatExpression { span, id, expr: element_expr, count: *array_type.length.clone() }.into(),
            ))
        }

        // Other types are not expected or supported just yet
        _ => None,
    }
}
