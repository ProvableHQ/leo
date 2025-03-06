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

use crate::FlagInserter;

use leo_ast::*;
use leo_span::{Span, Symbol};

use std::mem;

#[derive(Clone, Debug)]
pub struct ToDefine {
    pub(crate) name: Symbol,
    pub(crate) flag: Symbol,
    pub(crate) expr: Expression,
    pub(crate) first_type: Type,
    pub(crate) span: Span,
}

impl ExpressionReconstructor for FlagInserter<'_> {
    type AdditionalOutput = Vec<ToDefine>;

    fn reconstruct_access(&mut self, input: AccessExpression) -> (Expression, Self::AdditionalOutput) {
        match input {
            AccessExpression::Array(array) => self.reconstruct_array_access(array),
            AccessExpression::AssociatedConstant(constant) => self.reconstruct_associated_constant(constant),
            AccessExpression::AssociatedFunction(function) => self.reconstruct_associated_function(function),
            AccessExpression::Member(member) => self.reconstruct_member_access(member),
            AccessExpression::Tuple(tuple) => self.reconstruct_tuple_access(tuple),
        }
    }

    fn reconstruct_array_access(&mut self, mut input: ArrayAccess) -> (Expression, Self::AdditionalOutput) {
        let (array, mut definitions) = self.reconstruct_expression(mem::take(&mut *input.array));
        let (index, definitions2) = self.reconstruct_expression(mem::take(&mut *input.index));
        *input.array = array;
        *input.index = index;
        definitions.extend(definitions2);
        (Expression::Access(AccessExpression::Array(input)), definitions)
    }

    fn reconstruct_associated_function(
        &mut self,
        mut input: AssociatedFunction,
    ) -> (Expression, Self::AdditionalOutput) {
        let mut definitions = Vec::new();
        input.arguments = input
            .arguments
            .into_iter()
            .map(|arg| {
                let (expr, new_definitions) = self.reconstruct_expression(arg);
                definitions.extend(new_definitions);
                expr
            })
            .collect();
        (Expression::Access(AccessExpression::AssociatedFunction(input)), definitions)
    }

    fn reconstruct_member_access(&mut self, mut input: MemberAccess) -> (Expression, Self::AdditionalOutput) {
        let (inner, definitions) = self.reconstruct_expression(mem::take(&mut *input.inner));
        *input.inner = inner;
        (Expression::Access(AccessExpression::Member(input)), definitions)
    }

    fn reconstruct_tuple_access(&mut self, mut input: TupleAccess) -> (Expression, Self::AdditionalOutput) {
        let (tuple, definitions) = self.reconstruct_expression(mem::take(&mut *input.tuple));
        *input.tuple = tuple;
        (Expression::Access(AccessExpression::Tuple(input)), definitions)
    }

    fn reconstruct_array(&mut self, mut input: ArrayExpression) -> (Expression, Self::AdditionalOutput) {
        let mut definitions = Vec::new();
        input.elements = input
            .elements
            .into_iter()
            .map(|arg| {
                let (expr, new_definitions) = self.reconstruct_expression(arg);
                definitions.extend(new_definitions);
                expr
            })
            .collect();
        (Expression::Array(input), definitions)
    }

    fn reconstruct_binary(&mut self, mut input: BinaryExpression) -> (Expression, Self::AdditionalOutput) {
        let span = input.span();
        let (left, mut definitions) = self.reconstruct_expression(mem::take(&mut *input.left));
        let (right, definitions2) = self.reconstruct_expression(mem::take(&mut *input.right));
        *input.left = left;
        *input.right = right;
        definitions.extend(definitions2);
        if let BinaryOperation::Div = input.op {
            // We need to make a new ToDefine and replace the current expression with its new symbol name.
            let name = self.symbol_table.gensym("div_result");
            let name_identifier = Expression::Identifier(Identifier { name, span, id: self.node_builder.next_id() });
            let flag_name = self.symbol_table.gensym("div_flag");
            let ty = self.type_table.get(&input.id()).expect("Types have been assigned.");
            self.type_table.insert(name_identifier.id(), ty.clone());
            let expr = Expression::Binary(BinaryExpression {
                left: input.left,
                right: input.right,
                op: BinaryOperation::DivFlagged,
                span,
                id: self.node_builder.next_id(),
            });
            definitions.push(ToDefine { first_type: ty, name, flag: flag_name, expr, span });

            (name_identifier, definitions)
        } else {
            (Expression::Binary(input), definitions)
        }
    }

    fn reconstruct_call(&mut self, mut input: CallExpression) -> (Expression, Self::AdditionalOutput) {
        let mut definitions = Vec::new();
        input.arguments = input
            .arguments
            .into_iter()
            .map(|arg| {
                let (expr, new_definitions) = self.reconstruct_expression(arg);
                definitions.extend(new_definitions);
                expr
            })
            .collect();
        (Expression::Call(input), definitions)
    }

    fn reconstruct_cast(&mut self, mut input: CastExpression) -> (Expression, Self::AdditionalOutput) {
        let (expr, definitions) = self.reconstruct_expression(mem::take(&mut *input.expression));
        *input.expression = expr;
        (Expression::Cast(input), definitions)
    }

    fn reconstruct_struct_init(&mut self, mut input: StructExpression) -> (Expression, Self::AdditionalOutput) {
        let mut definitions = Vec::new();
        input.members = input
            .members
            .into_iter()
            .map(|mut member| {
                member.expression = member.expression.map(|expr| {
                    let (expr, new_definitions) = self.reconstruct_expression(expr);
                    definitions.extend(new_definitions);
                    expr
                });
                member
            })
            .collect();
        (Expression::Struct(input), definitions)
    }

    fn reconstruct_err(&mut self, _input: ErrExpression) -> (Expression, Self::AdditionalOutput) {
        panic!("`ErrExpression`s should not be in the AST at this phase of compilation.")
    }

    fn reconstruct_ternary(&mut self, mut input: TernaryExpression) -> (Expression, Self::AdditionalOutput) {
        let (condition, mut definitions) = self.reconstruct_expression(mem::take(&mut *input.condition));
        let (if_true, definitions2) = self.reconstruct_expression(mem::take(&mut *input.if_true));
        let (if_false, definitions3) = self.reconstruct_expression(mem::take(&mut *input.if_false));
        *input.condition = condition;
        *input.if_true = if_true;
        *input.if_false = if_false;
        definitions.extend(definitions2);
        definitions.extend(definitions3);
        (Expression::Ternary(input), definitions)
    }

    fn reconstruct_tuple(&mut self, mut input: TupleExpression) -> (Expression, Self::AdditionalOutput) {
        let mut definitions = Vec::new();
        input.elements = input
            .elements
            .into_iter()
            .map(|arg| {
                let (expr, new_definitions) = self.reconstruct_expression(arg);
                definitions.extend(new_definitions);
                expr
            })
            .collect();
        (Expression::Tuple(input), definitions)
    }

    fn reconstruct_unary(&mut self, mut input: UnaryExpression) -> (Expression, Self::AdditionalOutput) {
        let span = input.span();
        let (receiver, mut definitions) = self.reconstruct_expression(mem::take(&mut *input.receiver));
        *input.receiver = receiver;

        // TODO: Redundancy here and in `reconstruct_binary`.
        if let UnaryOperation::Inverse = input.op {
            let name = self.symbol_table.gensym("inv_result");
            let name_identifier = Expression::Identifier(Identifier { name, span, id: self.node_builder.next_id() });
            let flag_name = self.symbol_table.gensym("div_flag");
            let ty = self.type_table.get(&input.id()).expect("Types should have been assigned.");
            self.type_table.insert(name_identifier.id(), ty.clone());
            let expr = Expression::Unary(UnaryExpression {
                receiver: input.receiver,
                op: UnaryOperation::InvFlagged,
                span,
                id: self.node_builder.next_id(),
            });
            definitions.push(ToDefine { first_type: ty, name, flag: flag_name, expr, span });

            (name_identifier, definitions)
        } else {
            (Expression::Unary(input), definitions)
        }
    }
}
