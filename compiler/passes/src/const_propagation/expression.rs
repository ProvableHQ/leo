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

use leo_ast::{CoreFunction, Expression, ExpressionReconstructor, Node, StructExpression, TernaryExpression, Type};
use leo_errors::StaticAnalyzerError;
use leo_interpreter::{StructContents, Value};
use leo_span::sym;

use super::{ConstPropagationVisitor, value_to_expression};

const VALUE_ERROR: &str = "A non-future value should always be able to be converted into an expression";

impl ExpressionReconstructor for ConstPropagationVisitor<'_> {
    type AdditionalOutput = Option<Value>;

    fn reconstruct_expression(&mut self, input: Expression) -> (Expression, Self::AdditionalOutput) {
        let old_id = input.id();
        let (new_expr, opt_value) = match input {
            Expression::Access(access) => self.reconstruct_access(access),
            Expression::Array(array) => self.reconstruct_array(array),
            Expression::AssociatedConstant(associated_constant) => {
                self.reconstruct_associated_constant(associated_constant)
            }
            Expression::AssociatedFunction(associated_function) => {
                self.reconstruct_associated_function(associated_function)
            }
            Expression::Binary(binary) => self.reconstruct_binary(binary),
            Expression::Call(call) => self.reconstruct_call(call),
            Expression::Cast(cast) => self.reconstruct_cast(cast),
            Expression::Struct(struct_) => self.reconstruct_struct_init(struct_),
            Expression::Err(err) => self.reconstruct_err(err),
            Expression::Identifier(identifier) => self.reconstruct_identifier(identifier),
            Expression::Literal(value) => self.reconstruct_literal(value),
            Expression::Locator(locator) => self.reconstruct_locator(locator),
            Expression::Ternary(ternary) => self.reconstruct_ternary(ternary),
            Expression::Tuple(tuple) => self.reconstruct_tuple(tuple),
            Expression::Unary(unary) => self.reconstruct_unary(unary),
            Expression::Unit(unit) => self.reconstruct_unit(unit),
        };

        if old_id != new_expr.id() {
            self.changed = true;
            let old_type =
                self.state.type_table.get(&old_id).expect("Type checking guarantees that all expressions have a type.");
            self.state.type_table.insert(new_expr.id(), old_type);
        }

        (new_expr, opt_value)
    }

    fn reconstruct_struct_init(&mut self, mut input: StructExpression) -> (Expression, Self::AdditionalOutput) {
        let mut values = Vec::new();
        for member in input.members.iter_mut() {
            if let Some(expr) = std::mem::take(&mut member.expression) {
                let (new_expr, value_opt) = self.reconstruct_expression(expr);
                member.expression = Some(new_expr);
                if let Some(value) = value_opt {
                    values.push(value);
                }
            }
        }
        if values.len() == input.members.len() {
            let value = Value::Struct(StructContents {
                name: input.name.name,
                contents: input.members.iter().map(|mem| mem.identifier.name).zip(values).collect(),
            });
            (Expression::Struct(input), Some(value))
        } else {
            (Expression::Struct(input), None)
        }
    }

    fn reconstruct_ternary(&mut self, mut input: TernaryExpression) -> (Expression, Self::AdditionalOutput) {
        let (cond, cond_value) = self.reconstruct_expression(*input.condition);

        match cond_value {
            Some(Value::Bool(true)) => self.reconstruct_expression(*input.if_true),
            Some(Value::Bool(false)) => self.reconstruct_expression(*input.if_false),
            _ => {
                *input.condition = cond;
                *input.if_true = self.reconstruct_expression(*input.if_true).0;
                *input.if_false = self.reconstruct_expression(*input.if_false).0;
                (Expression::Ternary(input), None)
            }
        }
    }

    fn reconstruct_access(&mut self, input: leo_ast::AccessExpression) -> (Expression, Self::AdditionalOutput) {
        match input {
            leo_ast::AccessExpression::Array(array) => self.reconstruct_array_access(array),
            leo_ast::AccessExpression::Member(member) => self.reconstruct_member_access(member),
            leo_ast::AccessExpression::Tuple(tuple) => self.reconstruct_tuple_access(tuple),
        }
    }

    fn reconstruct_array_access(&mut self, mut input: leo_ast::ArrayAccess) -> (Expression, Self::AdditionalOutput) {
        let span = input.span();
        let (array_expr, value_opt) = self.reconstruct_expression(*input.array);
        *input.array = array_expr;
        let (index_expr, opt_value) = self.reconstruct_expression(*input.index);
        *input.index = index_expr;
        if let Some(value) = opt_value {
            // We can perform compile time bounds checking.

            let ty = self.state.type_table.get(&input.array.id());
            let Some(Type::Array(array_ty)) = ty else {
                panic!("Type checking guaranteed that this is an array.");
            };
            let len = array_ty.length();

            let index: usize = match value {
                Value::U8(x) => x as usize,
                Value::U16(x) => x as usize,
                Value::U32(x) => x.try_into().unwrap_or(len),
                Value::U64(x) => x.try_into().unwrap_or(len),
                Value::U128(x) => x.try_into().unwrap_or(len),
                Value::I8(x) => x.try_into().unwrap_or(len),
                Value::I16(x) => x.try_into().unwrap_or(len),
                Value::I32(x) => x.try_into().unwrap_or(len),
                Value::I64(x) => x.try_into().unwrap_or(len),
                Value::I128(x) => x.try_into().unwrap_or(len),
                _ => panic!("Type checking guarantees this is an intger"),
            };

            if index >= len {
                // Only emit a bounds error if we have no other errors yet.
                // This prevents a chain of redundant error messages when a loop is unrolled.
                if !self.state.handler.had_errors() {
                    // Get the integer string with no suffix.
                    let str_index = match value {
                        Value::U8(x) => format!("{x}"),
                        Value::U16(x) => format!("{x}"),
                        Value::U32(x) => format!("{x}"),
                        Value::U64(x) => format!("{x}"),
                        Value::U128(x) => format!("{x}"),
                        Value::I8(x) => format!("{x}"),
                        Value::I16(x) => format!("{x}"),
                        Value::I32(x) => format!("{x}"),
                        Value::I64(x) => format!("{x}"),
                        Value::I128(x) => format!("{x}"),
                        _ => unreachable!("We would have panicked above"),
                    };
                    self.emit_err(StaticAnalyzerError::array_bounds(str_index, len, span));
                }
            } else if let Some(Value::Array(value)) = value_opt {
                // We're in bounds and we can evaluate the array at compile time, so just return the value.
                let result_value = value.get(index).expect("We already checked bounds.");
                return (
                    value_to_expression(result_value, input.span, &self.state.node_builder).expect(VALUE_ERROR),
                    Some(result_value.clone()),
                );
            }
        } else {
            self.array_index_not_evaluated = Some(input.index.span());
        }
        (Expression::Access(leo_ast::AccessExpression::Array(input)), None)
    }

    fn reconstruct_associated_constant(
        &mut self,
        input: leo_ast::AssociatedConstantExpression,
    ) -> (Expression, Self::AdditionalOutput) {
        // Currently there is only one associated constant.
        let generator = Value::generator();
        let expr = value_to_expression(&generator, input.span(), &self.state.node_builder).expect(VALUE_ERROR);
        (expr, Some(generator))
    }

    fn reconstruct_associated_function(
        &mut self,
        mut input: leo_ast::AssociatedFunctionExpression,
    ) -> (Expression, Self::AdditionalOutput) {
        let mut values = Vec::new();
        for argument in input.arguments.iter_mut() {
            let (new_argument, opt_value) = self.reconstruct_expression(std::mem::take(argument));
            *argument = new_argument;
            if let Some(value) = opt_value {
                values.push(value);
            }
        }

        if values.len() == input.arguments.len() && !matches!(input.variant.name, sym::CheatCode | sym::Mapping) {
            // We've evaluated every argument, and this function isn't a cheat code or mapping
            // operation, so maybe we can compute the result at compile time.
            let core_function = CoreFunction::from_symbols(input.variant.name, input.name.name)
                .expect("Type checking guarantees this is valid.");

            match leo_interpreter::evaluate_core_function(&mut values, core_function, &[], input.span()) {
                Ok(Some(value)) => {
                    // Successful evaluation.
                    let expr = value_to_expression(&value, input.span(), &self.state.node_builder).expect(VALUE_ERROR);
                    return (expr, Some(value));
                }
                Ok(None) =>
                    // No errors, but we were unable to evaluate.
                    {}
                Err(err) => {
                    self.emit_err(StaticAnalyzerError::compile_core_function(err, input.span()));
                }
            }
        }

        (Expression::AssociatedFunction(input), Default::default())
    }

    fn reconstruct_member_access(&mut self, mut input: leo_ast::MemberAccess) -> (Expression, Self::AdditionalOutput) {
        let span = input.span();
        let (expr, value_opt) = self.reconstruct_expression(*input.inner);
        *input.inner = expr;
        let member_name = input.name.name;
        let expr_result = Expression::Access(leo_ast::AccessExpression::Member(input));
        if let Some(Value::Struct(contents)) = value_opt {
            let value_result =
                contents.contents.get(&member_name).expect("Type checking guarantees the member exists.");

            (
                value_to_expression(value_result, span, &self.state.node_builder).expect(VALUE_ERROR),
                Some(value_result.clone()),
            )
        } else {
            (expr_result, None)
        }
    }

    fn reconstruct_tuple_access(&mut self, mut input: leo_ast::TupleAccess) -> (Expression, Self::AdditionalOutput) {
        let span = input.span();
        let (expr, value_opt) = self.reconstruct_expression(*input.tuple);
        *input.tuple = expr;
        if let Some(Value::Tuple(tuple)) = value_opt {
            let value_result = tuple.get(input.index.value()).expect("Type checking checked bounds.");
            (
                value_to_expression(value_result, span, &self.state.node_builder).expect(VALUE_ERROR),
                Some(value_result.clone()),
            )
        } else {
            (Expression::Access(leo_ast::AccessExpression::Tuple(input)), None)
        }
    }

    fn reconstruct_array(&mut self, mut input: leo_ast::ArrayExpression) -> (Expression, Self::AdditionalOutput) {
        let mut values = Vec::new();
        input.elements.iter_mut().for_each(|element| {
            let (new_element, value_opt) = self.reconstruct_expression(std::mem::take(element));
            if let Some(value) = value_opt {
                values.push(value);
            }
            *element = new_element;
        });
        if values.len() == input.elements.len() {
            (Expression::Array(input), Some(Value::Array(values)))
        } else {
            (Expression::Array(input), None)
        }
    }

    fn reconstruct_binary(&mut self, mut input: leo_ast::BinaryExpression) -> (Expression, Self::AdditionalOutput) {
        let span = input.span();

        let (lhs_expr, lhs_opt_value) = self.reconstruct_expression(*input.left);
        let (rhs_expr, rhs_opt_value) = self.reconstruct_expression(*input.right);

        if let (Some(lhs_value), Some(rhs_value)) = (lhs_opt_value, rhs_opt_value) {
            // We were able to evaluate both operands, so we can evaluate this expression.
            match leo_interpreter::evaluate_binary(span, input.op, &lhs_value, &rhs_value) {
                Ok(new_value) => {
                    let new_expr = value_to_expression(&new_value, span, &self.state.node_builder).expect(VALUE_ERROR);
                    return (new_expr, Some(new_value));
                }
                Err(err) => self
                    .emit_err(StaticAnalyzerError::compile_time_binary_op(lhs_value, rhs_value, input.op, err, span)),
            }
        }

        *input.left = lhs_expr;
        *input.right = rhs_expr;

        (Expression::Binary(input), None)
    }

    fn reconstruct_call(&mut self, mut input: leo_ast::CallExpression) -> (Expression, Self::AdditionalOutput) {
        input.arguments.iter_mut().for_each(|arg| {
            *arg = self.reconstruct_expression(std::mem::take(arg)).0;
        });
        (Expression::Call(input), Default::default())
    }

    fn reconstruct_cast(&mut self, mut input: leo_ast::CastExpression) -> (Expression, Self::AdditionalOutput) {
        let span = input.span();

        let (expr, opt_value) = self.reconstruct_expression(*input.expression);

        if let Some(value) = opt_value {
            if let Some(cast_value) = value.cast(&input.type_) {
                let expr = value_to_expression(&cast_value, span, &self.state.node_builder).expect(VALUE_ERROR);
                return (expr, Some(cast_value));
            } else {
                self.emit_err(StaticAnalyzerError::compile_time_cast(value, &input.type_, span));
            }
        }
        *input.expression = expr;
        (Expression::Cast(input), None)
    }

    fn reconstruct_err(&mut self, _input: leo_ast::ErrExpression) -> (Expression, Self::AdditionalOutput) {
        panic!("`ErrExpression`s should not be in the AST at this phase of compilation.")
    }

    fn reconstruct_identifier(&mut self, input: leo_ast::Identifier) -> (Expression, Self::AdditionalOutput) {
        // Substitute the identifier with the constant value if it is a constant that's been evaluated.
        if let Some(expression) = self.state.symbol_table.lookup_const(self.program, input.name) {
            let (expression, opt_value) = self.reconstruct_expression(expression);
            if opt_value.is_some() {
                return (expression, opt_value);
            }
        }

        (Expression::Identifier(input), None)
    }

    fn reconstruct_literal(&mut self, input: leo_ast::Literal) -> (Expression, Self::AdditionalOutput) {
        let value = leo_interpreter::literal_to_value(&input).expect("Should work");
        (Expression::Literal(input), Some(value))
    }

    fn reconstruct_locator(&mut self, input: leo_ast::LocatorExpression) -> (Expression, Self::AdditionalOutput) {
        (Expression::Locator(input), Default::default())
    }

    fn reconstruct_tuple(&mut self, mut input: leo_ast::TupleExpression) -> (Expression, Self::AdditionalOutput) {
        let mut values = Vec::with_capacity(input.elements.len());
        for expr in input.elements.iter_mut() {
            let (new_expr, opt_value) = self.reconstruct_expression(std::mem::take(expr));
            *expr = new_expr;
            if let Some(value) = opt_value {
                values.push(value);
            }
        }

        let opt_value = if values.len() == input.elements.len() { Some(Value::Tuple(values)) } else { None };

        (Expression::Tuple(input), opt_value)
    }

    fn reconstruct_unary(&mut self, mut input: leo_ast::UnaryExpression) -> (Expression, Self::AdditionalOutput) {
        let (expr, opt_value) = self.reconstruct_expression(*input.receiver);
        let span = input.span;

        if let Some(value) = opt_value {
            // We were able to evaluate the operand, so we can evaluate the expression.
            match leo_interpreter::evaluate_unary(span, input.op, &value) {
                Ok(new_value) => {
                    let new_expr = value_to_expression(&new_value, span, &self.state.node_builder).expect(VALUE_ERROR);
                    return (new_expr, Some(new_value));
                }
                Err(err) => self.emit_err(StaticAnalyzerError::compile_time_unary_op(value, input.op, err, span)),
            }
        }
        *input.receiver = expr;
        (Expression::Unary(input), None)
    }

    fn reconstruct_unit(&mut self, input: leo_ast::UnitExpression) -> (Expression, Self::AdditionalOutput) {
        (Expression::Unit(input), None)
    }
}
