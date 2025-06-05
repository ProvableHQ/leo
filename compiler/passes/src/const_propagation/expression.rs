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

use super::ConstPropagationVisitor;

use leo_ast::{
    ArrayAccess,
    BinaryExpression,
    CastExpression,
    CoreFunction,
    Expression,
    ExpressionReconstructor,
    MemberAccess,
    Node,
    StructExpression,
    TernaryExpression,
    TupleAccess,
    Type,
    UnaryExpression,
    interpreter_value::{self, LeoValue},
};
use leo_errors::StaticAnalyzerError;
use leo_span::sym;

use snarkvm::prelude::Plaintext;

use std::{fmt::Write, str::FromStr as _};

const VALUE_ERROR: &str = "A non-future value should always be able to be converted into an expression";

#[derive(Clone, Default)]
pub struct ConstPropOutput {
    pub value: Option<LeoValue>,
    pub changed: bool,
}

impl ExpressionReconstructor for ConstPropagationVisitor<'_> {
    type AdditionalOutput = ConstPropOutput;

    fn reconstruct_expression(&mut self, input: Expression) -> (Expression, Self::AdditionalOutput) {
        let old_id = input.id();

        let (new_expr, output) = match input {
            Expression::Array(array) => self.reconstruct_array(array),
            Expression::ArrayAccess(access) => self.reconstruct_array_access(*access),
            Expression::AssociatedConstant(constant) => self.reconstruct_associated_constant(constant),
            Expression::AssociatedFunction(function) => self.reconstruct_associated_function(function),
            Expression::Binary(binary) => self.reconstruct_binary(*binary),
            Expression::Call(call) => self.reconstruct_call(*call),
            Expression::Cast(cast) => self.reconstruct_cast(*cast),
            Expression::Struct(struct_) => self.reconstruct_struct_init(struct_),
            Expression::Err(err) => self.reconstruct_err(err),
            Expression::Identifier(identifier) => self.reconstruct_identifier(identifier),
            Expression::Literal(value) => self.reconstruct_literal(value),
            Expression::Locator(locator) => self.reconstruct_locator(locator),
            Expression::MemberAccess(access) => self.reconstruct_member_access(*access),
            Expression::Ternary(ternary) => self.reconstruct_ternary(*ternary),
            Expression::Tuple(tuple) => self.reconstruct_tuple(tuple),
            Expression::TupleAccess(access) => self.reconstruct_tuple_access(*access),
            Expression::Unary(unary) => self.reconstruct_unary(*unary),
            Expression::Unit(unit) => self.reconstruct_unit(unit),
        };

        if output.changed {
            self.changed = true;
            let new_id = new_expr.id();
            if new_id != old_id {
                let old_type = self
                    .state
                    .type_table
                    .get(&old_id)
                    .expect("Type checking guarantees that all expressions have a type.");
                self.state.type_table.insert(new_id, old_type);
            }
        }

        (new_expr, output)
    }

    fn reconstruct_struct_init(&mut self, mut input: StructExpression) -> (Expression, Self::AdditionalOutput) {
        let id = input.id();
        let mut buffer = String::new();
        let mut member_changed = false;
        let members: Vec<_> = input
            .members
            .iter_mut()
            .filter_map(|member| {
                let expr = std::mem::take(&mut member.expression)?;
                let (new_expr, output) = self.reconstruct_expression(expr);
                member.expression = Some(new_expr);
                let value = output.value?;
                member_changed |= output.changed;
                let plaintext: Plaintext<_> = value.try_into().ok()?;
                buffer.clear();
                write!(&mut buffer, "{}", member.identifier).unwrap();
                Some((snarkvm::prelude::Identifier::from_str(&buffer).unwrap(), plaintext))
            })
            .collect();

        if members.len() == input.members.len() {
            let value = LeoValue::struct_svm(members.into_iter()).unwrap();
            let expr = if member_changed {
                let ty = self.state.type_table.get(&id).expect("Type should exist");
                self.value_to_expression(&value, input.span, &ty).expect(VALUE_ERROR)
            } else {
                input.into()
            };
            (expr, ConstPropOutput { value: Some(value), changed: member_changed })
        } else {
            (input.into(), ConstPropOutput { value: None, changed: member_changed })
        }
    }

    fn reconstruct_ternary(&mut self, input: TernaryExpression) -> (Expression, Self::AdditionalOutput) {
        let (condition, cond_output) = self.reconstruct_expression(input.condition);

        let (if_true, true_output) = self.reconstruct_expression(input.if_true);
        let (if_false, false_output) = self.reconstruct_expression(input.if_false);

        let changed = cond_output.changed | true_output.changed | false_output.changed;

        match cond_output.value.and_then(|val| val.try_into().ok()) {
            Some(true) if self.state.type_table.side_effect_free(&if_false) => {
                (if_true, ConstPropOutput { changed: true, value: true_output.value })
            }
            Some(false) if self.state.type_table.side_effect_free(&if_true) => {
                (if_false, ConstPropOutput { changed: true, value: false_output.value })
            }
            _ => {
                let expr = TernaryExpression { condition, if_true, if_false, ..input };
                (expr.into(), ConstPropOutput { changed, value: None })
            }
        }
    }

    fn reconstruct_array_access(&mut self, input: ArrayAccess) -> (Expression, Self::AdditionalOutput) {
        let id = input.id();
        let span = input.span();
        let array_id = input.array.id();
        let (array, array_output) = self.reconstruct_expression(input.array);
        let (index, index_output) = self.reconstruct_expression(input.index);
        if let Some(index_value) = index_output.value {
            // We can perform compile time bounds checking.

            let ty = self.state.type_table.get(&array_id);
            let Some(Type::Array(array_ty)) = ty else {
                panic!("Type checking guaranteed that this is an array.");
            };
            let len = array_ty.length();

            let index: usize = index_value.try_as_usize().unwrap_or(len);

            if index >= len {
                // Only emit a bounds error if we have no other errors yet.
                // This prevents a chain of redundant error messages when a loop is unrolled.
                if !self.state.handler.had_errors() {
                    let s = index_value.to_string();
                    // Remove the suffix
                    let suffix_index = s.rfind(|c: char| c == 'i' || c == 'u').unwrap_or(s.len());
                    self.emit_err(StaticAnalyzerError::array_bounds(&s[..suffix_index], len, span));
                }
            } else if let Some(slice) = array_output.value.as_ref().and_then(|value| value.try_as_array()) {
                // We're in bounds and we can evaluate the array at compile time, so just return the value.
                let result_plaintext = slice.get(index).expect("We already checked bounds.");
                let result_value: LeoValue = result_plaintext.clone().into();
                let ty = self.state.type_table.get(&id).expect("Type should exist");
                return (
                    self.value_to_expression(&result_value, input.span, &ty).expect(VALUE_ERROR),
                    ConstPropOutput { value: Some(result_value), changed: true },
                );
            }
        } else {
            self.array_index_not_evaluated = Some(index.span());
        }
        (ArrayAccess { array, index, ..input }.into(), ConstPropOutput {
            changed: array_output.changed | index_output.changed,
            value: None,
        })
    }

    fn reconstruct_associated_constant(
        &mut self,
        input: leo_ast::AssociatedConstantExpression,
    ) -> (Expression, Self::AdditionalOutput) {
        // Currently there is only one associated constant.
        let generator = LeoValue::generator();
        let expr = self.value_to_expression(&generator, input.span(), &Type::Group).expect(VALUE_ERROR);
        (expr, ConstPropOutput { changed: true, value: Some(generator) })
    }

    fn reconstruct_associated_function(
        &mut self,
        mut input: leo_ast::AssociatedFunctionExpression,
    ) -> (Expression, Self::AdditionalOutput) {
        let id = input.id();
        let mut values = Vec::new();
        let mut changed = false;
        for argument in input.arguments.iter_mut() {
            let (new_argument, argument_output) = self.reconstruct_expression(std::mem::take(argument));
            *argument = new_argument;
            if let Some(value) = argument_output.value {
                values.push(value);
            }
            changed |= argument_output.changed;
        }

        if values.len() == input.arguments.len() && !matches!(input.variant.name, sym::CheatCode | sym::Mapping) {
            // We've evaluated every argument, and this function isn't a cheat code or mapping
            // operation, so maybe we can compute the result at compile time.
            let core_function = CoreFunction::from_symbols(input.variant.name, input.name.name)
                .expect("Type checking guarantees this is valid.");

            match interpreter_value::evaluate_core_function(&mut values, core_function, &[], input.span()) {
                Ok(Some(value)) => {
                    // Successful evaluation.
                    let ty = self.state.type_table.get(&id).expect("Type should exist");
                    let expr = self.value_to_expression(&value, input.span(), &ty).expect(VALUE_ERROR);
                    return (expr, ConstPropOutput { value: Some(value), changed: true });
                }
                Ok(None) => {
                    // No errors, but we were unable to evaluate.
                }
                Err(err) => {
                    self.emit_err(StaticAnalyzerError::compile_core_function(err, input.span()));
                }
            }
        }

        (input.into(), ConstPropOutput { value: None, changed })
    }

    fn reconstruct_member_access(&mut self, input: MemberAccess) -> (Expression, Self::AdditionalOutput) {
        let id = input.id();
        let span = input.span();
        let (inner, output) = self.reconstruct_expression(input.inner);
        if let Some(value) = output.value {
            if let Some(member_plaintext) = value.member_get(input.name.name) {
                let value: LeoValue = member_plaintext.into();
                let ty = self.state.type_table.get(&id).expect("Type should exist");
                let expr = self.value_to_expression(&value, span, &ty).expect(VALUE_ERROR);
                return (expr, ConstPropOutput { value: Some(value), changed: true });
            }
        }
        (MemberAccess { inner, ..input }.into(), ConstPropOutput { value: None, changed: output.changed })
    }

    fn reconstruct_tuple_access(&mut self, input: TupleAccess) -> (Expression, Self::AdditionalOutput) {
        let span = input.span();
        let id = input.id();
        let (tuple, output) = self.reconstruct_expression(input.tuple);
        if let Some(LeoValue::Tuple(mut tuple)) = output.value {
            let ty = self.state.type_table.get(&id).expect("Type should exist");
            let result_plaintext = tuple.swap_remove(input.index.value());
            let value_result: LeoValue = result_plaintext.into();
            (self.value_to_expression(&value_result, span, &ty).expect(VALUE_ERROR), ConstPropOutput {
                value: Some(value_result),
                changed: true,
            })
        } else {
            (TupleAccess { tuple, ..input }.into(), ConstPropOutput { value: None, changed: output.changed })
        }
    }

    fn reconstruct_array(&mut self, mut input: leo_ast::ArrayExpression) -> (Expression, Self::AdditionalOutput) {
        let mut plaintexts = Vec::with_capacity(input.elements.len());
        let mut changed = false;
        input.elements.iter_mut().for_each(|element| {
            let (new_element, output) = self.reconstruct_expression(std::mem::take(element));
            if let Some(plaintext) = output.value.and_then(|val| val.try_into().ok()) {
                plaintexts.push(plaintext);
            }
            changed |= output.changed;
            *element = new_element;
        });

        let value = (plaintexts.len() == input.elements.len())
            .then_some(Plaintext::Array(plaintexts, Default::default()).into());

        (input.into(), ConstPropOutput { value, changed })
    }

    fn reconstruct_binary(&mut self, input: leo_ast::BinaryExpression) -> (Expression, Self::AdditionalOutput) {
        let span = input.span();
        let id = input.id();

        let (left, output_left) = self.reconstruct_expression(input.left);
        let (right, output_right) = self.reconstruct_expression(input.right);

        if let (Some(lhs_value), Some(rhs_value)) = (output_left.value, output_right.value) {
            // We were able to evaluate both operands, so we can evaluate this expression.
            match interpreter_value::evaluate_binary(span, input.op, &lhs_value, &rhs_value) {
                Ok(new_value) => {
                    let ty = self.state.type_table.get(&id).expect("Type should exist");
                    let new_expr = self.value_to_expression(&new_value, span, &ty).expect(VALUE_ERROR);
                    return (new_expr, ConstPropOutput { value: Some(new_value), changed: true });
                }
                Err(err) => self
                    .emit_err(StaticAnalyzerError::compile_time_binary_op(lhs_value, rhs_value, input.op, err, span)),
            }
        }

        (BinaryExpression { left, right, ..input }.into(), ConstPropOutput {
            value: None,
            changed: output_left.changed | output_right.changed,
        })
    }

    fn reconstruct_call(&mut self, mut input: leo_ast::CallExpression) -> (Expression, Self::AdditionalOutput) {
        let mut changed = false;
        input.arguments.iter_mut().for_each(|arg| {
            let (expr, output) = self.reconstruct_expression(std::mem::take(arg));
            changed |= output.changed;
            *arg = expr
        });
        (input.into(), ConstPropOutput { value: None, changed })
    }

    fn reconstruct_cast(&mut self, input: leo_ast::CastExpression) -> (Expression, Self::AdditionalOutput) {
        let span = input.span();
        let id = input.id();

        let (expr, output) = self.reconstruct_expression(input.expression);

        if let Some(value) = output.value {
            if let Some(cast_value) = value.cast(&input.type_) {
                let ty = self.state.type_table.get(&id).expect("Type should exist");
                let expr = self.value_to_expression(&cast_value, span, &ty).expect(VALUE_ERROR);
                return (expr, ConstPropOutput { value: Some(cast_value), changed: true });
            } else {
                self.emit_err(StaticAnalyzerError::compile_time_cast(value, &input.type_, span));
            }
        }
        (CastExpression { expression: expr, ..input }.into(), ConstPropOutput { value: None, changed: output.changed })
    }

    fn reconstruct_err(&mut self, _input: leo_ast::ErrExpression) -> (Expression, Self::AdditionalOutput) {
        panic!("`ErrExpression`s should not be in the AST at this phase of compilation.")
    }

    fn reconstruct_identifier(&mut self, input: leo_ast::Identifier) -> (Expression, Self::AdditionalOutput) {
        // Substitute the identifier with the constant value if it is a constant that's been evaluated.
        if let Some(expression) = self.state.symbol_table.lookup_const(self.program, input.name) {
            let (expression, output) = self.reconstruct_expression(expression);
            (expression, ConstPropOutput { value: output.value, changed: true })
        } else {
            (input.into(), Default::default())
        }
    }

    fn reconstruct_literal(&mut self, input: leo_ast::Literal) -> (Expression, Self::AdditionalOutput) {
        let value =
            interpreter_value::literal_to_value(&input, &self.state.type_table.get(&input.id())).expect("Should work");
        (input.into(), ConstPropOutput { value: Some(value), changed: false })
    }

    fn reconstruct_locator(&mut self, input: leo_ast::LocatorExpression) -> (Expression, Self::AdditionalOutput) {
        (input.into(), Default::default())
    }

    fn reconstruct_tuple(&mut self, mut input: leo_ast::TupleExpression) -> (Expression, Self::AdditionalOutput) {
        let mut changed = false;
        let mut plaintexts = Vec::with_capacity(input.elements.len());
        for expr in input.elements.iter_mut() {
            let (new_expr, output) = self.reconstruct_expression(std::mem::take(expr));
            *expr = new_expr;
            if let Some(plaintext) = output.value.and_then(|val| val.try_into().ok()) {
                plaintexts.push(plaintext);
            }
            changed |= output.changed;
        }

        let opt_value = (plaintexts.len() == input.elements.len()).then_some(LeoValue::Tuple(plaintexts));

        (input.into(), ConstPropOutput { value: opt_value, changed })
    }

    fn reconstruct_unary(&mut self, input: UnaryExpression) -> (Expression, Self::AdditionalOutput) {
        let ty = self.state.type_table.get(&input.id()).expect("Type should exist");
        let span = input.span;
        let (receiver, output) = self.reconstruct_expression(input.receiver);

        if let Some(value) = output.value {
            // We were able to evaluate the operand, so we can evaluate the expression.
            match interpreter_value::evaluate_unary(span, input.op, &value) {
                Ok(new_value) => {
                    let new_expr = self.value_to_expression(&new_value, span, &ty).expect(VALUE_ERROR);
                    return (new_expr, ConstPropOutput { value: Some(new_value), changed: true });
                }
                Err(err) => self.emit_err(StaticAnalyzerError::compile_time_unary_op(value, input.op, err, span)),
            }
        }
        (UnaryExpression { receiver, ..input }.into(), ConstPropOutput { value: None, changed: output.changed })
    }

    fn reconstruct_unit(&mut self, input: leo_ast::UnitExpression) -> (Expression, Self::AdditionalOutput) {
        (input.into(), Default::default())
    }
}
