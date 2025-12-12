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
use leo_span::{Span, Symbol, sym};

use indexmap::IndexMap;

pub struct StorageLoweringVisitor<'a> {
    pub state: &'a mut CompilerState,
    // The name of the current program scope
    pub program: Symbol,
    pub new_mappings: IndexMap<Symbol, Mapping>,
}

impl StorageLoweringVisitor<'_> {
    /// Generate mapping names for a vector expression. Each vector is represented using two
    /// mappings: a mapping for values and a mapping for the length.
    pub fn generate_mapping_names_for_vector(&self, expr: &Expression) -> (Symbol, Symbol) {
        let path = match expr {
            Expression::Path(path) => path,
            _ => panic!("Expected path expression for vector"),
        };
        let base_sym = path.identifier().name;
        let vec_values_mapping_name = Symbol::intern(&format!("{base_sym}__"));
        let vec_length_mapping_name = Symbol::intern(&format!("{base_sym}__len__"));
        (vec_values_mapping_name, vec_length_mapping_name)
    }

    /// Creates a path expression from a symbol
    pub fn symbol_to_path_expr(&mut self, sym: Symbol) -> Expression {
        Expression::Path(Path::from(Identifier::new(sym, self.state.node_builder.next_id())).into_absolute())
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
        let struct_lookup = |sym: &[Symbol]| {
            symbol_table
                .lookup_struct(sym)
                .unwrap()
                .members
                .iter()
                .map(|mem| (mem.identifier.name, mem.type_.clone()))
                .collect()
        };
        Expression::zero(ty, Span::default(), &self.state.node_builder, &struct_lookup)
            .expect("zero value generation failed")
    }
}
