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

use leo_ast::{
    interpreter_value::{self, Value},
    *,
};
use leo_errors::StaticAnalyzerError;
use leo_span::{Symbol, sym};

use super::ConstPropagationVisitor;

const VALUE_ERROR: &str = "A non-future value should always be able to be converted into an expression";

impl AstReconstructor for ConstPropagationVisitor<'_> {
    type AdditionalInput = ();
    type AdditionalOutput = Option<Value>;

    /* Types */
    fn reconstruct_array_type(&mut self, input: leo_ast::ArrayType) -> (leo_ast::Type, Self::AdditionalOutput) {
        let (length, opt_value) = self.reconstruct_expression(*input.length, &());

        // If we can't evaluate this array length, keep track of it for error reporting later
        if opt_value.is_none() {
            self.array_length_not_evaluated = Some(length.span());
        }

        (
            leo_ast::Type::Array(leo_ast::ArrayType {
                element_type: Box::new(self.reconstruct_type(*input.element_type).0),
                length: Box::new(length),
            }),
            Default::default(),
        )
    }

    /* Expressions */
    fn reconstruct_expression(&mut self, input: Expression, _additional: &()) -> (Expression, Self::AdditionalOutput) {
        let opt_old_type = self.state.type_table.get(&input.id());
        let (new_expr, opt_value) = match input {
            Expression::Array(array) => self.reconstruct_array(array, &()),
            Expression::ArrayAccess(access) => self.reconstruct_array_access(*access, &()),
            Expression::AssociatedConstant(constant) => self.reconstruct_associated_constant(constant, &()),
            Expression::AssociatedFunction(function) => self.reconstruct_associated_function(function, &()),
            Expression::Async(async_) => self.reconstruct_async(async_, &()),
            Expression::Binary(binary) => self.reconstruct_binary(*binary, &()),
            Expression::Call(call) => self.reconstruct_call(*call, &()),
            Expression::Cast(cast) => self.reconstruct_cast(*cast, &()),
            Expression::Struct(struct_) => self.reconstruct_struct_init(struct_, &()),
            Expression::Err(err) => self.reconstruct_err(err, &()),
            Expression::Path(path) => self.reconstruct_path(path, &()),
            Expression::Literal(value) => self.reconstruct_literal(value, &()),
            Expression::Locator(locator) => self.reconstruct_locator(locator, &()),
            Expression::MemberAccess(access) => self.reconstruct_member_access(*access, &()),
            Expression::Repeat(repeat) => self.reconstruct_repeat(*repeat, &()),
            Expression::Ternary(ternary) => self.reconstruct_ternary(*ternary, &()),
            Expression::Tuple(tuple) => self.reconstruct_tuple(tuple, &()),
            Expression::TupleAccess(access) => self.reconstruct_tuple_access(*access, &()),
            Expression::Unary(unary) => self.reconstruct_unary(*unary, &()),
            Expression::Unit(unit) => self.reconstruct_unit(unit, &()),
        };

        // If the expression was in the type table before, make an entry for the new expression.
        if let Some(old_type) = opt_old_type {
            self.state.type_table.insert(new_expr.id(), old_type);
        }

        (new_expr, opt_value)
    }

    fn reconstruct_struct_init(
        &mut self,
        mut input: StructExpression,
        _additional: &(),
    ) -> (Expression, Self::AdditionalOutput) {
        let mut values = Vec::new();
        input.const_arguments.iter_mut().for_each(|arg| {
            *arg = self.reconstruct_expression(std::mem::take(arg), &()).0;
        });
        for member in input.members.iter_mut() {
            let expression = member.expression.take().unwrap_or_else(|| {
                Path::from(member.identifier).with_absolute_path(Some(vec![member.identifier.name])).into()
            });
            let (new_expr, value_opt) = self.reconstruct_expression(expression, &());
            member.expression = Some(new_expr);
            if let Some(value) = value_opt {
                values.push(value);
            }
        }

        if values.len() == input.members.len() && input.const_arguments.is_empty() {
            let value = Value::make_struct(
                input.members.iter().map(|mem| mem.identifier.name).zip(values),
                self.program,
                input.path.absolute_path(),
            );
            (input.into(), Some(value))
        } else {
            (input.into(), None)
        }
    }

    fn reconstruct_ternary(
        &mut self,
        input: TernaryExpression,
        _additional: &(),
    ) -> (Expression, Self::AdditionalOutput) {
        let (cond, cond_value) = self.reconstruct_expression(input.condition, &());

        match cond_value.and_then(|v| v.try_into().ok()) {
            Some(true) => self.reconstruct_expression(input.if_true, &()),
            Some(false) => self.reconstruct_expression(input.if_false, &()),
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

    fn reconstruct_array_access(
        &mut self,
        input: ArrayAccess,
        _additional: &(),
    ) -> (Expression, Self::AdditionalOutput) {
        let span = input.span();
        let id = input.id();
        let array_id = input.array.id();
        let (array, array_opt) = self.reconstruct_expression(input.array, &());
        let (index, index_opt) = self.reconstruct_expression(input.index, &());
        if let Some(index_value) = index_opt {
            // We can perform compile time bounds checking.

            let ty = self.state.type_table.get(&array_id);
            let Some(Type::Array(array_ty)) = ty else {
                panic!("Type checking guaranteed that this is an array.");
            };
            let len = array_ty.length.as_u32();

            if let Some(len) = len {
                let index_in_bounds = matches!(index_value.as_u32(), Some(index) if index < len);

                if !index_in_bounds {
                    // Only emit a bounds error if we have no other errors yet.
                    // This prevents a chain of redundant error messages when a loop is unrolled.
                    if !self.state.handler.had_errors() {
                        // Get the integer string with no suffix.
                        let integer_with_suffix = index_value.to_string();
                        let suffix_index = integer_with_suffix.find(['i', 'u']).unwrap_or(integer_with_suffix.len());
                        self.emit_err(StaticAnalyzerError::array_bounds(
                            &integer_with_suffix[..suffix_index],
                            len,
                            span,
                        ));
                    }
                } else if let Some(array_value) = array_opt {
                    // We're in bounds and we can evaluate the array at compile time, so just return the value.
                    let result_value = array_value
                        .array_index(index_value.as_u32().unwrap() as usize)
                        .expect("We already checked bounds.");
                    return (
                        self.value_to_expression(&result_value, span, id).expect(VALUE_ERROR),
                        Some(result_value.clone()),
                    );
                }
            }
        } else {
            self.array_index_not_evaluated = Some(index.span());
        }
        (ArrayAccess { array, index, ..input }.into(), None)
    }

    fn reconstruct_associated_constant(
        &mut self,
        input: leo_ast::AssociatedConstantExpression,
        _additional: &(),
    ) -> (Expression, Self::AdditionalOutput) {
        // Currently there is only one associated constant.
        let generator = Value::generator();
        let expr = self.value_to_expression_node(&generator, &input).expect(VALUE_ERROR);
        (expr, Some(generator))
    }

    fn reconstruct_associated_function(
        &mut self,
        mut input: leo_ast::AssociatedFunctionExpression,
        _additional: &(),
    ) -> (Expression, Self::AdditionalOutput) {
        let mut values = Vec::new();
        for argument in input.arguments.iter_mut() {
            let (new_argument, opt_value) = self.reconstruct_expression(std::mem::take(argument), &());
            *argument = new_argument;
            if let Some(value) = opt_value {
                values.push(value);
            }
        }

        if values.len() == input.arguments.len() && !matches!(input.variant.name, sym::CheatCode | sym::Mapping) {
            // We've evaluated every argument, and this function isn't a cheat code or mapping
            // operation, so maybe we can compute the result at compile time.
            let core_function = CoreFunction::try_from(&input).expect("Type checking guarantees this is valid.");

            match interpreter_value::evaluate_core_function(&mut values, core_function, &[], input.span()) {
                Ok(Some(value)) => {
                    // Successful evaluation.
                    let expr = self.value_to_expression_node(&value, &input).expect(VALUE_ERROR);
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

        (input.into(), Default::default())
    }

    fn reconstruct_member_access(
        &mut self,
        input: MemberAccess,
        _additional: &(),
    ) -> (Expression, Self::AdditionalOutput) {
        let span = input.span();
        let id = input.id();
        let (inner, value_opt) = self.reconstruct_expression(input.inner, &());
        let member_name = input.name.name;
        if let Some(struct_) = value_opt {
            let value_result = struct_.member_access(member_name).expect("Type checking guarantees the member exists.");

            (self.value_to_expression(&value_result, span, id).expect(VALUE_ERROR), Some(value_result.clone()))
        } else {
            (MemberAccess { inner, ..input }.into(), None)
        }
    }

    fn reconstruct_repeat(
        &mut self,
        input: leo_ast::RepeatExpression,
        _additional: &(),
    ) -> (Expression, Self::AdditionalOutput) {
        let (expr, expr_value) = self.reconstruct_expression(input.expr.clone(), &());
        let (count, count_value) = self.reconstruct_expression(input.count.clone(), &());

        if count_value.is_none() {
            self.repeat_count_not_evaluated = Some(count.span());
        }

        match (expr_value, count.as_u32()) {
            (Some(value), Some(count_u32)) => (
                RepeatExpression { expr, count, ..input }.into(),
                Some(Value::make_array(std::iter::repeat_n(value, count_u32 as usize))),
            ),
            _ => (RepeatExpression { expr, count, ..input }.into(), None),
        }
    }

    fn reconstruct_tuple_access(
        &mut self,
        input: TupleAccess,
        _additional: &(),
    ) -> (Expression, Self::AdditionalOutput) {
        let span = input.span();
        let id = input.id();
        let (tuple, value_opt) = self.reconstruct_expression(input.tuple, &());
        if let Some(tuple_value) = value_opt {
            let value_result = tuple_value.tuple_index(input.index.value()).expect("Type checking checked bounds.");
            (self.value_to_expression(&value_result, span, id).expect(VALUE_ERROR), Some(value_result.clone()))
        } else {
            (TupleAccess { tuple, ..input }.into(), None)
        }
    }

    fn reconstruct_array(
        &mut self,
        mut input: leo_ast::ArrayExpression,
        _additional: &(),
    ) -> (Expression, Self::AdditionalOutput) {
        let mut values = Vec::new();
        input.elements.iter_mut().for_each(|element| {
            let (new_element, value_opt) = self.reconstruct_expression(std::mem::take(element), &());
            if let Some(value) = value_opt {
                values.push(value);
            }
            *element = new_element;
        });
        if values.len() == input.elements.len() {
            (input.into(), Some(Value::make_array(values.into_iter())))
        } else {
            (input.into(), None)
        }
    }

    fn reconstruct_binary(
        &mut self,
        input: leo_ast::BinaryExpression,
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
                    let new_expr = self.value_to_expression(&new_value, span, input_id).expect(VALUE_ERROR);
                    return (new_expr, Some(new_value));
                }
                Err(err) => self
                    .emit_err(StaticAnalyzerError::compile_time_binary_op(lhs_value, rhs_value, input.op, err, span)),
            }
        }

        (BinaryExpression { left, right, ..input }.into(), None)
    }

    fn reconstruct_call(
        &mut self,
        mut input: leo_ast::CallExpression,
        _additional: &(),
    ) -> (Expression, Self::AdditionalOutput) {
        input.const_arguments.iter_mut().for_each(|arg| {
            *arg = self.reconstruct_expression(std::mem::take(arg), &()).0;
        });
        input.arguments.iter_mut().for_each(|arg| {
            *arg = self.reconstruct_expression(std::mem::take(arg), &()).0;
        });
        (input.into(), Default::default())
    }

    fn reconstruct_cast(
        &mut self,
        input: leo_ast::CastExpression,
        _additional: &(),
    ) -> (Expression, Self::AdditionalOutput) {
        let span = input.span();
        let id = input.id();

        let (expr, opt_value) = self.reconstruct_expression(input.expression, &());

        if let Some(value) = opt_value {
            if let Some(cast_value) = value.cast(&input.type_) {
                let expr = self.value_to_expression(&cast_value, span, id).expect(VALUE_ERROR);
                return (expr, Some(cast_value));
            } else {
                self.emit_err(StaticAnalyzerError::compile_time_cast(value, &input.type_, span));
            }
        }
        (CastExpression { expression: expr, ..input }.into(), None)
    }

    fn reconstruct_err(
        &mut self,
        _input: leo_ast::ErrExpression,
        _additional: &(),
    ) -> (Expression, Self::AdditionalOutput) {
        panic!("`ErrExpression`s should not be in the AST at this phase of compilation.")
    }

    fn reconstruct_path(&mut self, input: leo_ast::Path, _additional: &()) -> (Expression, Self::AdditionalOutput) {
        // Substitute the identifier with the constant value if it is a constant that's been evaluated.
        if let Some(expression) = self.state.symbol_table.lookup_const(self.program, &input.absolute_path()) {
            let (expression, opt_value) = self.reconstruct_expression(expression, &());
            if opt_value.is_some() {
                return (expression, opt_value);
            }
        }

        (input.into(), None)
    }

    fn reconstruct_literal(
        &mut self,
        mut input: leo_ast::Literal,
        _additional: &(),
    ) -> (Expression, Self::AdditionalOutput) {
        let type_info = self.state.type_table.get(&input.id());

        // If this is an optional, then unwrap it first.
        let type_info = type_info.as_ref().map(|ty| match ty {
            Type::Optional(opt) => *opt.inner.clone(),
            _ => ty.clone(),
        });

        if let Ok(value) = interpreter_value::literal_to_value(&input, &type_info) {
            // If we know the type of an unsuffixed literal, might as well change it to a suffixed literal. This way, we
            // do not have to infer the type again in later passes of type checking.
            if let LiteralVariant::Unsuffixed(s) = input.variant {
                match type_info.expect("Expected type information to be available") {
                    Type::Integer(ty) => input.variant = LiteralVariant::Integer(ty, s),
                    Type::Field => input.variant = LiteralVariant::Field(s),
                    Type::Group => input.variant = LiteralVariant::Group(s),
                    Type::Scalar => input.variant = LiteralVariant::Scalar(s),
                    _ => panic!("Type checking should have prevented this."),
                }
            }
            (input.into(), Some(value))
        } else {
            (input.into(), None)
        }
    }

    fn reconstruct_locator(
        &mut self,
        input: leo_ast::LocatorExpression,
        _additional: &(),
    ) -> (Expression, Self::AdditionalOutput) {
        (input.into(), Default::default())
    }

    fn reconstruct_tuple(
        &mut self,
        mut input: leo_ast::TupleExpression,
        _additional: &(),
    ) -> (Expression, Self::AdditionalOutput) {
        let mut values = Vec::with_capacity(input.elements.len());
        for expr in input.elements.iter_mut() {
            let (new_expr, opt_value) = self.reconstruct_expression(std::mem::take(expr), &());
            *expr = new_expr;
            if let Some(value) = opt_value {
                values.push(value);
            }
        }

        let opt_value = if values.len() == input.elements.len() { Some(Value::make_tuple(values)) } else { None };

        (input.into(), opt_value)
    }

    fn reconstruct_unary(&mut self, input: UnaryExpression, _additional: &()) -> (Expression, Self::AdditionalOutput) {
        let input_id = input.id();
        let span = input.span;
        let (receiver, opt_value) = self.reconstruct_expression(input.receiver, &());

        if let Some(value) = opt_value {
            // We were able to evaluate the operand, so we can evaluate the expression.
            match interpreter_value::evaluate_unary(span, input.op, &value, &self.state.type_table.get(&input_id)) {
                Ok(new_value) => {
                    let new_expr = self.value_to_expression(&new_value, span, input_id).expect(VALUE_ERROR);
                    return (new_expr, Some(new_value));
                }
                Err(err) => self.emit_err(StaticAnalyzerError::compile_time_unary_op(value, input.op, err, span)),
            }
        }
        (UnaryExpression { receiver, ..input }.into(), None)
    }

    fn reconstruct_unit(
        &mut self,
        input: leo_ast::UnitExpression,
        _additional: &(),
    ) -> (Expression, Self::AdditionalOutput) {
        (input.into(), None)
    }

    /* Statements */
    fn reconstruct_assert(&mut self, mut input: AssertStatement) -> (Statement, Self::AdditionalOutput) {
        // Catching asserts at compile time is not feasible here due to control flow, but could be done in
        // a later pass after loops are unrolled and conditionals are flattened.
        input.variant = match input.variant {
            AssertVariant::Assert(expr) => AssertVariant::Assert(self.reconstruct_expression(expr, &()).0),

            AssertVariant::AssertEq(lhs, rhs) => AssertVariant::AssertEq(
                self.reconstruct_expression(lhs, &()).0,
                self.reconstruct_expression(rhs, &()).0,
            ),

            AssertVariant::AssertNeq(lhs, rhs) => AssertVariant::AssertNeq(
                self.reconstruct_expression(lhs, &()).0,
                self.reconstruct_expression(rhs, &()).0,
            ),
        };

        (input.into(), None)
    }

    fn reconstruct_assign(&mut self, assign: AssignStatement) -> (Statement, Self::AdditionalOutput) {
        let value = self.reconstruct_expression(assign.value, &()).0;
        let place = self.reconstruct_expression(assign.place, &()).0;
        (AssignStatement { value, place, ..assign }.into(), None)
    }

    fn reconstruct_block(&mut self, mut block: Block) -> (Block, Self::AdditionalOutput) {
        self.in_scope(block.id(), |slf| {
            block.statements.retain_mut(|statement| {
                let bogus_statement = Statement::dummy();
                let this_statement = std::mem::replace(statement, bogus_statement);
                *statement = slf.reconstruct_statement(this_statement).0;
                !statement.is_empty()
            });
            (block, None)
        })
    }

    fn reconstruct_conditional(
        &mut self,
        mut conditional: ConditionalStatement,
    ) -> (Statement, Self::AdditionalOutput) {
        conditional.condition = self.reconstruct_expression(conditional.condition, &()).0;
        conditional.then = self.reconstruct_block(conditional.then).0;
        if let Some(mut otherwise) = conditional.otherwise {
            *otherwise = self.reconstruct_statement(*otherwise).0;
            conditional.otherwise = Some(otherwise);
        }

        (Statement::Conditional(conditional), None)
    }

    fn reconstruct_const(&mut self, mut input: ConstDeclaration) -> (Statement, Self::AdditionalOutput) {
        if matches!(input.type_, Type::Optional(_)) {
            return (input.into(), None);
        }

        let span = input.span();

        let type_ = self.reconstruct_type(input.type_).0;
        let (expr, opt_value) = self.reconstruct_expression(input.value, &());

        if opt_value.is_some() {
            let path: &[Symbol] = if self.state.symbol_table.global_scope() {
                // Then we need to insert the const with its full module-scoped path.
                &self.module.iter().copied().chain(std::iter::once(input.place.name)).collect::<Vec<_>>()
            } else {
                &[input.place.name]
            };
            if self.state.symbol_table.lookup_const(self.program, path).is_none() {
                // It wasn't already evaluated - insert it and record that we've made a change.
                self.state.symbol_table.insert_const(self.program, path, expr.clone());
                self.changed = true;
            }
        } else {
            self.const_not_evaluated = Some(span);
        }

        input.type_ = type_;
        input.value = expr;

        (Statement::Const(input), None)
    }

    fn reconstruct_definition(&mut self, definition: DefinitionStatement) -> (Statement, Self::AdditionalOutput) {
        (
            DefinitionStatement {
                type_: definition.type_.map(|ty| self.reconstruct_type(ty).0),
                value: self.reconstruct_expression(definition.value, &()).0,
                ..definition
            }
            .into(),
            None,
        )
    }

    fn reconstruct_expression_statement(
        &mut self,
        mut input: ExpressionStatement,
    ) -> (Statement, Self::AdditionalOutput) {
        input.expression = self.reconstruct_expression(input.expression, &()).0;

        if matches!(&input.expression, Expression::Unit(..) | Expression::Literal(..)) {
            // We were able to evaluate this at compile time, but we need to get rid of this statement as
            // we can't have expression statements that aren't calls.
            (Statement::dummy(), Default::default())
        } else {
            (input.into(), Default::default())
        }
    }

    fn reconstruct_iteration(&mut self, iteration: IterationStatement) -> (Statement, Self::AdditionalOutput) {
        let id = iteration.id();
        let type_ = iteration.type_.map(|ty| self.reconstruct_type(ty).0);
        let start = self.reconstruct_expression(iteration.start, &()).0;
        let stop = self.reconstruct_expression(iteration.stop, &()).0;
        self.in_scope(id, |slf| {
            (
                IterationStatement { type_, start, stop, block: slf.reconstruct_block(iteration.block).0, ..iteration }
                    .into(),
                None,
            )
        })
    }

    fn reconstruct_return(&mut self, input: ReturnStatement) -> (Statement, Self::AdditionalOutput) {
        (
            ReturnStatement { expression: self.reconstruct_expression(input.expression, &()).0, ..input }.into(),
            Default::default(),
        )
    }
}
