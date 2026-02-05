// Copyright (C) 2019-2026 Provable Inc.
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
use leo_span::{Span, Symbol, sym};

use indexmap::IndexMap;

pub struct StorageLoweringVisitor<'a> {
    pub state: &'a mut CompilerState,
    // The name of the current program scope
    pub program: Symbol,
    pub new_mappings: IndexMap<Location, Mapping>,
}

impl StorageLoweringVisitor<'_> {
    /// Returns the two mapping expressions that back a vector: `<base>__` (values)
    /// and `<base>__len__` (length).
    ///
    /// Panics if `expr` is not a `Path`.
    pub fn generate_vector_mapping_exprs(&mut self, path: &Path) -> (Expression, Expression) {
        let base = path.identifier().name;
        let val = Symbol::intern(&format!("{base}__"));
        let len = Symbol::intern(&format!("{base}__len__"));

        let make_expr = |sym| {
            let ident = Identifier::new(sym, self.state.node_builder.next_id());
            let mut p = Path::from(ident);

            if let Some(program) = path.user_program().filter(|p| p.name != self.program) {
                p = p.with_user_program(*program);
            }

            p.to_global(Location::new(self.program, vec![sym])).into()
        };

        (make_expr(val), make_expr(len))
    }

    /// Standard literal expressions used frequently
    pub fn literal_false(&mut self) -> Expression {
        Literal::boolean(false, Span::default(), self.state.node_builder.next_id()).into()
    }

    pub fn literal_zero_u32(&mut self) -> Expression {
        Literal::integer(IntegerType::U32, "0".to_string(), Span::default(), self.state.node_builder.next_id()).into()
    }

    pub fn literal_one_u32(&mut self) -> Expression {
        Literal::integer(IntegerType::U32, "1".to_string(), Span::default(), self.state.node_builder.next_id()).into()
    }

    /// Generates `_mapping_get_or_use(len_path_expr, false, 0u32)`
    pub fn get_vector_len_expr(&mut self, len_path_expr: Expression, span: Span) -> Expression {
        IntrinsicExpression {
            name: sym::_mapping_get_or_use,
            type_parameters: vec![],
            arguments: vec![len_path_expr, self.literal_false(), self.literal_zero_u32()],
            span,
            id: self.state.node_builder.next_id(),
        }
        .into()
    }

    /// Generates `_mapping_set(path_expr, key_expr, value_expr)`
    pub fn set_mapping_expr(
        &mut self,
        path_expr: Expression,
        key_expr: Expression,
        value_expr: Expression,
        span: Span,
    ) -> Expression {
        IntrinsicExpression {
            name: sym::_mapping_set,
            type_parameters: vec![],
            arguments: vec![path_expr, key_expr, value_expr],
            span,
            id: self.state.node_builder.next_id(),
        }
        .into()
    }

    /// Generates `_mapping_get(path_expr, key_expr)`
    pub fn get_mapping_expr(&mut self, path_expr: Expression, key_expr: Expression, span: Span) -> Expression {
        IntrinsicExpression {
            name: sym::_mapping_get,
            type_parameters: vec![],
            arguments: vec![path_expr, key_expr],
            span,
            id: self.state.node_builder.next_id(),
        }
        .into()
    }

    /// Generates `_mapping_get_or_use(path_expr, key_expr, default_expr)`
    pub fn get_or_use_mapping_expr(
        &mut self,
        path_expr: Expression,
        key_expr: Expression,
        default_expr: Expression,
        span: Span,
    ) -> Expression {
        IntrinsicExpression {
            name: sym::_mapping_get_or_use,
            type_parameters: vec![],
            arguments: vec![path_expr, key_expr, default_expr],
            span,
            id: self.state.node_builder.next_id(),
        }
        .into()
    }

    /// Generates a ternary expression
    pub fn ternary_expr(
        &mut self,
        condition: Expression,
        if_true: Expression,
        if_false: Expression,
        span: Span,
    ) -> Expression {
        TernaryExpression { condition, if_true, if_false, span, id: self.state.node_builder.next_id() }.into()
    }

    /// Generates a binary expression
    pub fn binary_expr(&mut self, left: Expression, op: BinaryOperation, right: Expression) -> Expression {
        BinaryExpression { op, left, right, span: Span::default(), id: self.state.node_builder.next_id() }.into()
    }

    /// Produces a zero expression for `Type` `ty`.
    pub fn zero(&self, ty: &Type) -> Expression {
        // zero value for element type (used as default in get_or_use)
        let symbol_table = &self.state.symbol_table;
        let struct_lookup = |loc: &Location| {
            symbol_table
                .lookup_struct(self.program, loc)
                .unwrap()
                .members
                .iter()
                .map(|mem| (mem.identifier.name, mem.type_.clone()))
                .collect()
        };
        Expression::zero(ty, Span::default(), &self.state.node_builder, &struct_lookup)
            .expect("zero value generation failed")
    }

    pub fn reconstruct_path_or_locator(&self, input: Expression) -> Expression {
        let location = match input {
            Expression::Path(ref path) if path.is_local() => {
                // nothing to do for local paths.
                return input;
            }
            Expression::Path(ref path) => {
                // Otherwise, it should be a global path.
                path.expect_global_location().clone()
            }
            _ => panic!("unexpected expression type"),
        };

        // Check if this path corresponds to a global symbol.
        let Some(var) = self.state.symbol_table.lookup_global(self.program, &location) else {
            // Nothing to do
            return input;
        };

        match &var.type_ {
            Some(Type::Mapping(_)) => {
                // No transformation needed for mappings.
                input
            }

            Some(Type::Optional(OptionalType { inner })) => {
                // Input:
                //   storage x: field;
                //   ...
                //   let y = x;
                //
                // Lowered reconstruction:
                //  mapping x__: bool => field
                //  let y = x__.contains(false)
                //      ? x__.get_or_use(false, 0field)
                //      : None;

                let id = || self.state.node_builder.next_id();
                let var_name = location.path.last().unwrap();

                // Path to the mapping backing the optional variable: `<var_name>__`
                let mapping_symbol = Symbol::intern(&format!("{var_name}__"));
                let mapping_ident = Identifier::new(mapping_symbol, id());

                // === Build expressions ===
                let mapping_expr: Expression = {
                    let path = if let Expression::Path(path) = input {
                        path
                    } else {
                        panic!("unexpected expression type");
                    };

                    let mut base_path = Path::from(mapping_ident);

                    // Attach user program only if it's present and different from current
                    if let Some(user_program) = path.user_program()
                        && user_program.name != self.program
                    {
                        base_path = base_path.with_user_program(*user_program);
                    }

                    base_path.to_global(Location::new(self.program, vec![mapping_ident.name])).into()
                };

                let false_literal: Expression = Literal::boolean(false, Span::default(), id()).into();

                // `<var_name>__.contains(false)`
                let contains_expr: Expression = IntrinsicExpression {
                    name: sym::_mapping_contains,
                    type_parameters: vec![],
                    arguments: vec![mapping_expr.clone(), false_literal.clone()],
                    span: Span::default(),
                    id: id(),
                }
                .into();

                // zero value for element type
                let zero = self.zero(inner);

                // `<var_name>__.get_or_use(false, zero_value)`
                let get_or_use_expr: Expression = IntrinsicExpression {
                    name: sym::_mapping_get_or_use,
                    type_parameters: vec![],
                    arguments: vec![mapping_expr.clone(), false_literal, zero],
                    span: Span::default(),
                    id: id(),
                }
                .into();

                // `None`
                let none_expr =
                    Expression::Literal(Literal { variant: LiteralVariant::None, span: Span::default(), id: id() });

                // Combine into ternary:
                // `<var_name>__.contains(false) ? <var_name>__.get_or_use(false, zero_val) : None`
                let ternary_expr: Expression = TernaryExpression {
                    condition: contains_expr,
                    if_true: get_or_use_expr,
                    if_false: none_expr,
                    span: Span::default(),
                    id: id(),
                }
                .into();

                ternary_expr
            }

            _ => {
                panic!("Expected an optional or a mapping, found {:?}", var.type_);
            }
        }
    }
}
