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

use super::SsaConstPropagationVisitor;

use leo_ast::{
    interpreter_value::{self, Value},
    *,
};
use leo_errors::StaticAnalyzerError;

const VALUE_ERROR: &str = "A non-future value should always be able to be converted into an expression";

impl AstReconstructor for SsaConstPropagationVisitor<'_> {
    type AdditionalInput = ();
    type AdditionalOutput = Option<Value>;

    /// Reconstruct a path expression. If the path refers to a variable that has
    /// a constant value, replace it with that constant.
    fn reconstruct_path(&mut self, input: Path, _additional: &()) -> (Expression, Self::AdditionalOutput) {
        // In SSA form, paths should refer to local variables (or composite members).
        // Check if this variable has a constant value.
        let identifier_name = input.identifier().name;

        if let Some(constant_value) = self.constants.get(&identifier_name).cloned() {
            // Replace the path with the constant value.
            let span = input.span();
            let id = input.id();
            let (new_expr, _) = self.value_to_expression(&constant_value, span, id).expect(VALUE_ERROR);
            self.changed = true;
            (new_expr, Some(constant_value))
        } else {
            // No constant value for this variable, keep the path as is.
            (input.into(), None)
        }
    }

    /// Reconstruct a literal expression and convert it to a Value.
    fn reconstruct_literal(&mut self, mut input: Literal, _additional: &()) -> (Expression, Self::AdditionalOutput) {
        let type_info = self.state.type_table.get(&input.id());

        // If this is an optional, then unwrap it first.
        let type_info = type_info.as_ref().map(|ty| match ty {
            Type::Optional(opt) => *opt.inner.clone(),
            _ => ty.clone(),
        });

        if let Ok(value) = interpreter_value::literal_to_value(&input, &type_info) {
            match input.variant {
                LiteralVariant::Address(ref s) if s.ends_with("aleo") => {
                    // Do not fold program names as the VM needs to handle them directly
                    (input.into(), None)
                }

                // If we know the type of an unsuffixed literal, might as well change it to a suffixed literal.
                LiteralVariant::Unsuffixed(s) => {
                    match type_info.expect("Expected type information to be available") {
                        Type::Integer(ty) => input.variant = LiteralVariant::Integer(ty, s),
                        Type::Field => input.variant = LiteralVariant::Field(s),
                        Type::Group => input.variant = LiteralVariant::Group(s),
                        Type::Scalar => input.variant = LiteralVariant::Scalar(s),
                        _ => panic!("Type checking should have prevented this."),
                    }
                    (input.into(), Some(value))
                }
                _ => (input.into(), Some(value)),
            }
        } else {
            (input.into(), None)
        }
    }

    /// Reconstruct a binary expression and fold it if both operands are constants.
    fn reconstruct_binary(
        &mut self,
        input: BinaryExpression,
        _additional: &(),
    ) -> (Expression, Self::AdditionalOutput) {
        let span = input.span();
        let input_id = input.id();

        let (left, lhs_opt_value) = self.reconstruct_expression(input.left, &());
        let (right, rhs_opt_value) = self.reconstruct_expression(input.right, &());

        if let (Some(lhs_value), Some(rhs_value)) = (lhs_opt_value, rhs_opt_value) {
            // We were able to evaluate both operands, so we can evaluate this expression.
            match interpreter_value::evaluate_binary(
                span,
                input.op,
                &lhs_value,
                &rhs_value,
                &self.state.type_table.get(&input_id),
            ) {
                Ok(new_value) => {
                    let (new_expr, _) = self.value_to_expression(&new_value, span, input_id).expect(VALUE_ERROR);
                    self.changed = true;
                    return (new_expr, Some(new_value));
                }
                Err(err) => self
                    .emit_err(StaticAnalyzerError::compile_time_binary_op(lhs_value, rhs_value, input.op, err, span)),
            }
        }

        (BinaryExpression { left, right, ..input }.into(), None)
    }

    /// Reconstruct a unary expression and fold it if the operand is a constant.
    fn reconstruct_unary(&mut self, input: UnaryExpression, _additional: &()) -> (Expression, Self::AdditionalOutput) {
        let input_id = input.id();
        let span = input.span;
        let (receiver, opt_value) = self.reconstruct_expression(input.receiver, &());

        if let Some(value) = opt_value {
            // We were able to evaluate the operand, so we can evaluate the expression.
            match interpreter_value::evaluate_unary(span, input.op, &value, &self.state.type_table.get(&input_id)) {
                Ok(new_value) => {
                    let (new_expr, _) = self.value_to_expression(&new_value, span, input_id).expect(VALUE_ERROR);
                    self.changed = true;
                    return (new_expr, Some(new_value));
                }
                Err(err) => self.emit_err(StaticAnalyzerError::compile_time_unary_op(value, input.op, err, span)),
            }
        }
        (UnaryExpression { receiver, ..input }.into(), None)
    }

    /// Reconstruct a ternary expression and fold it if the condition is a constant.
    fn reconstruct_ternary(
        &mut self,
        input: TernaryExpression,
        _additional: &(),
    ) -> (Expression, Self::AdditionalOutput) {
        let (cond, cond_value) = self.reconstruct_expression(input.condition, &());

        match cond_value.and_then(|v| v.try_into().ok()) {
            Some(true) => {
                self.changed = true;
                self.reconstruct_expression(input.if_true, &())
            }
            Some(false) => {
                self.changed = true;
                self.reconstruct_expression(input.if_false, &())
            }
            _ => (
                TernaryExpression {
                    condition: cond,
                    if_true: self.reconstruct_expression(input.if_true, &()).0,
                    if_false: self.reconstruct_expression(input.if_false, &()).0,
                    ..input
                }
                .into(),
                None,
            ),
        }
    }

    /// Reconstruct an array access expression and fold it if array and index are constants.
    fn reconstruct_array_access(
        &mut self,
        input: ArrayAccess,
        _additional: &(),
    ) -> (Expression, Self::AdditionalOutput) {
        let span = input.span();
        let id = input.id();

        let (array, array_opt) = self.reconstruct_expression(input.array, &());
        let (index, index_opt) = self.reconstruct_expression(input.index, &());

        if let Some(index_value) = index_opt
            && let Some(array_value) = array_opt
        {
            let result_value =
                array_value.array_index(index_value.as_u32().unwrap() as usize).expect("We already checked bounds.");
            self.changed = true;
            let (new_expr, _) = self.value_to_expression(&result_value, span, id).expect(VALUE_ERROR);
            return (new_expr, Some(result_value.clone()));
        }

        (ArrayAccess { array, index, ..input }.into(), None)
    }

    /// Reconstruct an array expression and fold it if all elements are constants.
    fn reconstruct_array(
        &mut self,
        mut input: ArrayExpression,
        _additional: &(),
    ) -> (Expression, Self::AdditionalOutput) {
        let mut values = Vec::new();
        let mut elements_changed = false;
        input.elements.iter_mut().for_each(|element| {
            let old_element = element.clone();
            let (new_element, value_opt) = self.reconstruct_expression(std::mem::take(element), &());
            // Check if the element actually changed (not just its structure, but if it's a different expression)
            if old_element.id() != new_element.id() {
                elements_changed = true;
            }
            if let Some(value) = value_opt {
                values.push(value);
            }
            *element = new_element;
        });
        // Only set changed if elements actually changed. Don't set changed just because
        // we can evaluate the array - that would cause an infinite loop since the array
        // expression structure doesn't change.
        if elements_changed {
            self.changed = true;
        }

        if values.len() == input.elements.len() {
            (input.into(), Some(Value::make_array(values.into_iter())))
        } else {
            (input.into(), None)
        }
    }

    /// Reconstruct a tuple expression and fold it if all elements are constants.
    fn reconstruct_tuple(
        &mut self,
        mut input: TupleExpression,
        _additional: &(),
    ) -> (Expression, Self::AdditionalOutput) {
        let mut values = Vec::with_capacity(input.elements.len());
        let mut elements_changed = false;
        for expr in input.elements.iter_mut() {
            let old_expr = expr.clone();
            let (new_expr, opt_value) = self.reconstruct_expression(std::mem::take(expr), &());
            // Check if the element actually changed
            if old_expr.id() != new_expr.id() {
                elements_changed = true;
            }
            *expr = new_expr;
            if let Some(value) = opt_value {
                values.push(value);
            }
        }

        // Only set changed if elements actually changed. Don't set changed just because
        // we can evaluate the tuple - that would cause an infinite loop since the tuple
        // expression structure doesn't change.
        if elements_changed {
            self.changed = true;
        }

        let opt_value = if values.len() == input.elements.len() { Some(Value::make_tuple(values)) } else { None };

        (input.into(), opt_value)
    }

    /* Statements */
    /// Reconstruct a definition statement. If the RHS evaluates to a constant, track it
    /// in the constants map for propagation.
    fn reconstruct_definition(&mut self, mut input: DefinitionStatement) -> (Statement, Self::AdditionalOutput) {
        // Reconstruct the RHS expression first.
        let (new_value, opt_value) = self.reconstruct_expression(input.value, &());

        if let Some(value) = opt_value {
            match &input.place {
                DefinitionPlace::Single(identifier) => {
                    self.constants.insert(identifier.name, value);
                }
                DefinitionPlace::Multiple(identifiers) => {
                    for (i, id) in identifiers.iter().enumerate() {
                        if let Some(v) = value.tuple_index(i) {
                            self.constants.insert(id.name, v);
                        }
                    }
                }
            }
        }

        input.value = new_value;

        (input.into(), None)
    }

    fn reconstruct_assign(&mut self, _input: AssignStatement) -> (Statement, Self::AdditionalOutput) {
        panic!("there should be no assignments at this stage");
    }
}
