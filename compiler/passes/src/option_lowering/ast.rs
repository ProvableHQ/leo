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

use super::OptionLoweringVisitor;

use leo_ast::*;
use leo_span::{Span, Symbol};

use indexmap::IndexMap;

impl leo_ast::AstReconstructor for OptionLoweringVisitor<'_> {
    type AdditionalInput = Option<Type>;
    type AdditionalOutput = Vec<Statement>;

    /* Types */
    fn reconstruct_array_type(&mut self, input: ArrayType) -> (Type, Self::AdditionalOutput) {
        let (length, stmts) = self.reconstruct_expression(*input.length, &None);
        (
            Type::Array(ArrayType {
                element_type: Box::new(self.reconstruct_type(*input.element_type).0),
                length: Box::new(length),
            }),
            stmts,
        )
    }

    fn reconstruct_composite_type(&mut self, input: CompositeType) -> (Type, Self::AdditionalOutput) {
        let mut statements = Vec::new();

        let const_arguments = input
            .const_arguments
            .into_iter()
            .map(|arg| {
                let (expr, stmts) = self.reconstruct_expression(arg, &None);
                statements.extend(stmts);
                expr
            })
            .collect();

        (Type::Composite(CompositeType { const_arguments, ..input }), statements)
    }

    fn reconstruct_optional_type(&mut self, input: OptionalType) -> (Type, Self::AdditionalOutput) {
        let (inner_type, _) = self.reconstruct_type(*input.inner.clone());

        // Create or get an optional wrapper struct for `inner_type`
        let struct_name = self.insert_optional_wrapper_struct(&inner_type);

        (
            Type::Composite(CompositeType {
                path: Path::from(Identifier::new(struct_name, self.state.node_builder.next_id()))
                    .to_global(Location::new(self.program, vec![struct_name])),
                const_arguments: vec![], // this is not a generic struct
            }),
            Default::default(),
        )
    }

    /* Expressions */
    fn reconstruct_expression(
        &mut self,
        input: Expression,
        additional: &Option<Type>,
    ) -> (Expression, Self::AdditionalOutput) {
        // Handle `None` literal separately
        if let Expression::Literal(Literal { variant: LiteralVariant::None, .. }) = input {
            let Some(Type::Optional(OptionalType { inner })) = self.state.type_table.get(&input.id()) else {
                panic!("Type checking guarantees that `None` has an Optional type");
            };

            return (self.wrap_none(&inner), vec![]);
        }

        // Reconstruct the expression based on its variant
        let (expr, stmts) = match input {
            Expression::Intrinsic(e) => self.reconstruct_intrinsic(*e, additional),
            Expression::Async(e) => self.reconstruct_async(e, additional),
            Expression::Array(e) => self.reconstruct_array(e, additional),
            Expression::ArrayAccess(e) => self.reconstruct_array_access(*e, additional),
            Expression::Binary(e) => self.reconstruct_binary(*e, additional),
            Expression::Call(e) => self.reconstruct_call(*e, additional),
            Expression::Cast(e) => self.reconstruct_cast(*e, additional),
            Expression::Composite(e) => self.reconstruct_composite_init(e, additional),
            Expression::Err(e) => self.reconstruct_err(e, additional),
            Expression::Path(e) => self.reconstruct_path(e, additional),
            Expression::Literal(e) => self.reconstruct_literal(e, additional),
            Expression::Locator(e) => self.reconstruct_locator(e, additional),
            Expression::MemberAccess(e) => self.reconstruct_member_access(*e, additional),
            Expression::Repeat(e) => self.reconstruct_repeat(*e, additional),
            Expression::Ternary(e) => self.reconstruct_ternary(*e, additional),
            Expression::Tuple(e) => self.reconstruct_tuple(e, additional),
            Expression::TupleAccess(e) => self.reconstruct_tuple_access(*e, additional),
            Expression::Unary(e) => self.reconstruct_unary(*e, additional),
            Expression::Unit(e) => self.reconstruct_unit(e, additional),
        };

        // Optionally wrap in an optional if expected type is `Optional<T>`
        if let Some(Type::Optional(OptionalType { inner })) = additional {
            let actual_expr_type =
                self.state.type_table.get(&expr.id()).expect(
                    "Type table must contain type for this expression ID; IDs are not modified during lowering",
                );

            if actual_expr_type.can_coerce_to(inner) {
                return (self.wrap_optional_value(expr, *inner.clone()), stmts);
            }
        }

        (expr, stmts)
    }

    fn reconstruct_array_access(
        &mut self,
        mut input: ArrayAccess,
        _additional: &Self::AdditionalInput,
    ) -> (Expression, Self::AdditionalOutput) {
        let (array, mut stmts_array) = self.reconstruct_expression(input.array, &None);
        let (index, mut stmts_index) = self.reconstruct_expression(input.index, &None);

        input.array = array;
        input.index = index;

        // Merge side effects
        stmts_array.append(&mut stmts_index);

        (input.into(), stmts_array)
    }

    fn reconstruct_intrinsic(
        &mut self,
        mut input: IntrinsicExpression,
        _additional: &Option<Type>,
    ) -> (Expression, Self::AdditionalOutput) {
        match Intrinsic::from_symbol(input.name, &input.type_parameters) {
            Some(Intrinsic::OptionalUnwrap) => {
                let [optional_expr] = &input.arguments[..] else {
                    panic!("guaranteed by type checking");
                };

                let (reconstructed_optional_expr, mut stmts) =
                    self.reconstruct_expression(optional_expr.clone(), &None);

                // Access `.val` and `.is_some` from reconstructed expression
                let val_access = MemberAccess {
                    inner: reconstructed_optional_expr.clone(),
                    name: Identifier::new(Symbol::intern("val"), self.state.node_builder.next_id()),
                    span: Span::default(),
                    id: self.state.node_builder.next_id(),
                };
                let Some(Type::Optional(OptionalType { inner })) = self.state.type_table.get(&optional_expr.id())
                else {
                    panic!("guaranteed by type checking");
                };
                self.state.type_table.insert(val_access.id(), *inner);

                let is_some_access = MemberAccess {
                    inner: reconstructed_optional_expr.clone(),
                    name: Identifier::new(Symbol::intern("is_some"), self.state.node_builder.next_id()),
                    span: Span::default(),
                    id: self.state.node_builder.next_id(),
                };
                self.state.type_table.insert(is_some_access.id(), Type::Boolean);

                // Create assertion: ensure `is_some` is `true`.
                let assert_stmt = AssertStatement {
                    variant: AssertVariant::Assert(is_some_access.clone().into()),
                    span: Span::default(),
                    id: self.state.node_builder.next_id(),
                };

                // Combine all statements.
                stmts.push(assert_stmt.into());

                (val_access.into(), stmts)
            }
            Some(Intrinsic::OptionalUnwrapOr) => {
                let [optional_expr, default_expr] = &input.arguments[..] else {
                    panic!("unwrap_or must have 2 arguments: optional and default");
                };

                let (reconstructed_optional_expr, mut stmts1) =
                    self.reconstruct_expression(optional_expr.clone(), &None);

                // Extract the inner type from the expected type of the optional argument.
                let Some(Type::Optional(OptionalType { inner: expected_inner_type })) =
                    self.state.type_table.get(&optional_expr.id())
                else {
                    panic!("guaranteed by type checking")
                };

                let (reconstructed_fallback_expr, stmts2) =
                    self.reconstruct_expression(default_expr.clone(), &Some(*expected_inner_type.clone()));

                // Access `.val` and `.is_some` from reconstructed expression
                let val_access = MemberAccess {
                    inner: reconstructed_optional_expr.clone(),
                    name: Identifier::new(Symbol::intern("val"), self.state.node_builder.next_id()),
                    span: Span::default(),
                    id: self.state.node_builder.next_id(),
                };
                self.state.type_table.insert(val_access.id(), *expected_inner_type.clone());

                let is_some_access = MemberAccess {
                    inner: reconstructed_optional_expr,
                    name: Identifier::new(Symbol::intern("is_some"), self.state.node_builder.next_id()),
                    span: Span::default(),
                    id: self.state.node_builder.next_id(),
                };
                self.state.type_table.insert(is_some_access.id(), Type::Boolean);

                // s.is_some ? s.val : fallback
                let ternary_expr = TernaryExpression {
                    condition: is_some_access.into(),
                    if_true: val_access.into(),
                    if_false: reconstructed_fallback_expr,
                    span: Span::default(),
                    id: self.state.node_builder.next_id(),
                };
                self.state.type_table.insert(ternary_expr.id(), *expected_inner_type);

                stmts1.extend(stmts2);
                (ternary_expr.into(), stmts1)
            }
            _ => {
                let statements: Vec<_> = input
                    .arguments
                    .iter_mut()
                    .flat_map(|arg| {
                        let (expr, stmts) = self.reconstruct_expression(std::mem::take(arg), &None);
                        *arg = expr;
                        stmts
                    })
                    .collect();

                (input.into(), statements)
            }
        }
    }

    fn reconstruct_member_access(
        &mut self,
        mut input: MemberAccess,
        _additional: &Self::AdditionalInput,
    ) -> (Expression, Self::AdditionalOutput) {
        let (inner, stmts_inner) = self.reconstruct_expression(input.inner, &None);

        input.inner = inner;

        (input.into(), stmts_inner)
    }

    fn reconstruct_repeat(
        &mut self,
        mut input: RepeatExpression,
        additional: &Self::AdditionalInput,
    ) -> (Expression, Self::AdditionalOutput) {
        // Derive expected element type from the type of the whole expression
        let expected_element_type =
            additional.clone().or_else(|| self.state.type_table.get(&input.id)).and_then(|mut ty| {
                if let Type::Optional(inner) = ty {
                    ty = *inner.inner;
                }
                match ty {
                    Type::Array(array_ty) => Some(*array_ty.element_type),
                    _ => None,
                }
            });

        // Use expected type (if available) for `expr`
        let (expr, mut stmts_expr) = self.reconstruct_expression(input.expr, &expected_element_type);

        let (count, mut stmts_count) = self.reconstruct_expression(input.count, &None);

        input.expr = expr;
        input.count = count;

        stmts_expr.append(&mut stmts_count);

        (input.into(), stmts_expr)
    }

    fn reconstruct_tuple_access(
        &mut self,
        mut input: TupleAccess,
        _additional: &Self::AdditionalInput,
    ) -> (Expression, Self::AdditionalOutput) {
        let (tuple, stmts) = self.reconstruct_expression(input.tuple, &None);

        input.tuple = tuple;

        (input.into(), stmts)
    }

    fn reconstruct_array(
        &mut self,
        mut input: ArrayExpression,
        additional: &Option<Type>,
    ) -> (Expression, Self::AdditionalOutput) {
        let expected_element_type = additional
            .clone()
            .or_else(|| self.state.type_table.get(&input.id))
            .and_then(|mut ty| {
                // Unwrap Optional if any
                if let Type::Optional(inner) = ty {
                    ty = *inner.inner;
                }
                // Expect Array type
                match ty {
                    Type::Array(array_ty) => Some(*array_ty.element_type),
                    _ => None,
                }
            })
            .expect("guaranteed by type checking");

        let mut all_stmts = Vec::new();
        let mut new_elements = Vec::with_capacity(input.elements.len());

        for element in input.elements.into_iter() {
            let (expr, mut stmts) = self.reconstruct_expression(element, &Some(expected_element_type.clone()));
            all_stmts.append(&mut stmts);
            new_elements.push(expr);
        }

        input.elements = new_elements;

        (input.into(), all_stmts)
    }

    fn reconstruct_binary(
        &mut self,
        mut input: BinaryExpression,
        _additional: &Self::AdditionalInput,
    ) -> (Expression, Self::AdditionalOutput) {
        let (left, mut stmts_left) = self.reconstruct_expression(input.left, &None);
        let (right, mut stmts_right) = self.reconstruct_expression(input.right, &None);

        input.left = left;
        input.right = right;

        // Merge side effects
        stmts_left.append(&mut stmts_right);

        (input.into(), stmts_left)
    }

    fn reconstruct_call(
        &mut self,
        mut input: CallExpression,
        _additional: &Self::AdditionalInput,
    ) -> (Expression, Self::AdditionalOutput) {
        let callee_location = input.function.expect_global_location();
        let func_symbol = self
            .state
            .symbol_table
            .lookup_function(self.program, callee_location)
            .expect("The symbol table creator should already have visited all functions.")
            .clone();

        let mut all_stmts = Vec::new();

        // Reconstruct const arguments with expected types
        let mut const_arguments = Vec::with_capacity(input.const_arguments.len());
        for (arg, param) in input.const_arguments.into_iter().zip(func_symbol.function.const_parameters.iter()) {
            let expected_type = Some(param.type_.clone());
            let (expr, mut stmts) = self.reconstruct_expression(arg, &expected_type);
            all_stmts.append(&mut stmts);
            const_arguments.push(expr);
        }

        // Reconstruct normal arguments with expected types
        let mut arguments = Vec::with_capacity(input.arguments.len());
        for (arg, param) in input.arguments.into_iter().zip(func_symbol.function.input.iter()) {
            let expected_type = Some(param.type_.clone());
            let (expr, mut stmts) = self.reconstruct_expression(arg, &expected_type);
            all_stmts.append(&mut stmts);
            arguments.push(expr);
        }

        input.const_arguments = const_arguments;
        input.arguments = arguments;

        (input.into(), all_stmts)
    }

    fn reconstruct_cast(
        &mut self,
        mut input: CastExpression,
        _additional: &Self::AdditionalInput,
    ) -> (Expression, Self::AdditionalOutput) {
        let (expr, stmts) = self.reconstruct_expression(input.expression, &None);

        input.expression = expr;

        (input.into(), stmts)
    }

    fn reconstruct_composite_init(
        &mut self,
        mut input: CompositeExpression,
        additional: &Option<Type>,
    ) -> (Expression, Self::AdditionalOutput) {
        let (const_parameters, member_types): (Vec<Type>, IndexMap<Symbol, Type>) = {
            let mut ty = additional.clone().or_else(|| self.state.type_table.get(&input.id)).expect("type checked");

            if let Type::Optional(inner) = ty {
                ty = *inner.inner;
            }

            if let Type::Composite(composite) = ty {
                let composite_location = composite.path.expect_global_location();
                let composite_def = self
                    .state
                    .symbol_table
                    .lookup_record(self.program, composite_location)
                    .or_else(|| self.state.symbol_table.lookup_struct(self.program, composite_location))
                    .or_else(|| self.new_structs.get(composite_location))
                    .expect("guaranteed by type checking");

                let const_parameters = composite_def.const_parameters.iter().map(|param| param.type_.clone()).collect();
                let member_types =
                    composite_def.members.iter().map(|member| (member.identifier.name, member.type_.clone())).collect();

                (const_parameters, member_types)
            } else {
                panic!("expected Type::Composite")
            }
        };

        // Reconstruct const arguments with expected types
        let (const_arguments, mut const_arg_stmts): (Vec<_>, Vec<_>) = input
            .const_arguments
            .into_iter()
            .zip(const_parameters.iter())
            .map(|(arg, ty)| self.reconstruct_expression(arg, &Some(ty.clone())))
            .unzip();

        // Reconstruct members
        let (members, mut member_stmts): (Vec<_>, Vec<_>) = input
            .members
            .into_iter()
            .map(|member| {
                let expected_type =
                    member_types.get(&member.identifier.name).expect("guaranteed by type checking").clone();

                let expression = member.expression.unwrap_or_else(|| Path::from(member.identifier).to_local().into());

                let (new_expr, stmts) = self.reconstruct_expression(expression, &Some(expected_type));

                (
                    CompositeFieldInitializer {
                        identifier: member.identifier,
                        expression: Some(new_expr),
                        span: member.span,
                        id: member.id,
                    },
                    stmts,
                )
            })
            .unzip();

        input.const_arguments = const_arguments;
        input.members = members;

        // Merge all side effect statements
        const_arg_stmts.append(&mut member_stmts);
        let all_stmts = const_arg_stmts.into_iter().flatten().collect();

        (input.into(), all_stmts)
    }

    fn reconstruct_ternary(
        &mut self,
        mut input: TernaryExpression,
        additional: &Self::AdditionalInput,
    ) -> (Expression, Self::AdditionalOutput) {
        let type_ = self.state.type_table.get(&input.id());
        let (condition, mut stmts_condition) = self.reconstruct_expression(input.condition, &None);
        let additional = if let Some(expected) = additional { Some(expected.clone()) } else { type_ };

        let (if_true, mut stmts_if_true) = self.reconstruct_expression(input.if_true, &additional);
        let (if_false, mut stmts_if_false) = self.reconstruct_expression(input.if_false, &additional);

        input.condition = condition;
        input.if_true = if_true;
        input.if_false = if_false;

        // Merge all side effects
        stmts_condition.append(&mut stmts_if_true);
        stmts_condition.append(&mut stmts_if_false);

        (input.into(), stmts_condition)
    }

    fn reconstruct_tuple(
        &mut self,
        mut input: TupleExpression,
        additional: &Option<Type>,
    ) -> (Expression, Self::AdditionalOutput) {
        // Determine the expected tuple element types
        let expected_types = additional
            .clone()
            .or_else(|| self.state.type_table.get(&input.id))
            .and_then(|mut ty| {
                // Unwrap Optional if any
                if let Type::Optional(inner) = ty {
                    ty = *inner.inner;
                }

                // Expect Tuple type
                match ty {
                    Type::Tuple(tuple_ty) => Some(tuple_ty.elements.clone()),
                    _ => None,
                }
            })
            .expect("guaranteed by type checking");

        let mut all_stmts = Vec::new();
        let mut new_elements = Vec::with_capacity(input.elements.len());

        // Zip elements with expected types and reconstruct with expected type
        for (element, expected_ty) in input.elements.into_iter().zip(expected_types) {
            let (expr, mut stmts) = self.reconstruct_expression(element, &Some(expected_ty));
            all_stmts.append(&mut stmts);
            new_elements.push(expr);
        }

        input.elements = new_elements;

        (input.into(), all_stmts)
    }

    fn reconstruct_unary(
        &mut self,
        mut input: UnaryExpression,
        _additional: &Self::AdditionalInput,
    ) -> (Expression, Self::AdditionalOutput) {
        let (receiver, stmts) = self.reconstruct_expression(input.receiver, &None);

        input.receiver = receiver;

        (input.into(), stmts)
    }

    /* Statements */
    fn reconstruct_assert(&mut self, mut input: AssertStatement) -> (Statement, Self::AdditionalOutput) {
        let mut all_stmts = Vec::new();

        input.variant = match input.variant {
            AssertVariant::Assert(expr) => {
                let (expr, mut stmts) = self.reconstruct_expression(expr, &None);
                all_stmts.append(&mut stmts);
                AssertVariant::Assert(expr)
            }
            AssertVariant::AssertEq(left, right) => {
                let (left, mut stmts_left) = self.reconstruct_expression(left, &None);
                let (right, mut stmts_right) = self.reconstruct_expression(right, &None);
                all_stmts.append(&mut stmts_left);
                all_stmts.append(&mut stmts_right);
                AssertVariant::AssertEq(left, right)
            }
            AssertVariant::AssertNeq(left, right) => {
                let (left, mut stmts_left) = self.reconstruct_expression(left, &None);
                let (right, mut stmts_right) = self.reconstruct_expression(right, &None);
                all_stmts.append(&mut stmts_left);
                all_stmts.append(&mut stmts_right);
                AssertVariant::AssertNeq(left, right)
            }
        };

        (input.into(), all_stmts)
    }

    fn reconstruct_assign(&mut self, input: AssignStatement) -> (Statement, Self::AdditionalOutput) {
        let expected_ty = self.state.type_table.get(&input.place.id()).expect("type checked");

        let (new_place, place_stmts) = self.reconstruct_expression(input.place, &None);
        let (new_value, value_stmts) = self.reconstruct_expression(input.value, &Some(expected_ty));

        (AssignStatement { place: new_place, value: new_value, ..input }.into(), [place_stmts, value_stmts].concat())
    }

    fn reconstruct_conditional(&mut self, mut input: ConditionalStatement) -> (Statement, Self::AdditionalOutput) {
        let (condition, mut stmts_condition) = self.reconstruct_expression(input.condition, &None);
        let (then_block, mut stmts_then) = self.reconstruct_block(input.then);

        let otherwise = match input.otherwise {
            Some(otherwise_stmt) => {
                let (stmt, mut stmts_otherwise) = self.reconstruct_statement(*otherwise_stmt);
                stmts_condition.append(&mut stmts_then);
                stmts_condition.append(&mut stmts_otherwise);
                Some(Box::new(stmt))
            }
            None => {
                stmts_condition.append(&mut stmts_then);
                None
            }
        };

        input.condition = condition;
        input.then = then_block;
        input.otherwise = otherwise;

        (input.into(), stmts_condition)
    }

    fn reconstruct_const(&mut self, mut input: ConstDeclaration) -> (Statement, Self::AdditionalOutput) {
        let (type_, mut stmts_type) = self.reconstruct_type(input.type_.clone());
        let (value, mut stmts_value) = self.reconstruct_expression(input.value, &Some(input.type_));

        input.type_ = type_;
        input.value = value;

        stmts_type.append(&mut stmts_value);

        (input.into(), stmts_type)
    }

    fn reconstruct_block(&mut self, mut block: Block) -> (Block, Self::AdditionalOutput) {
        let mut statements = Vec::with_capacity(block.statements.len());

        for statement in block.statements {
            let (reconstructed_statement, mut additional_stmts) = self.reconstruct_statement(statement);
            statements.append(&mut additional_stmts);
            statements.push(reconstructed_statement);
        }

        block.statements = statements;

        (block, Default::default())
    }

    fn reconstruct_definition(&mut self, mut input: DefinitionStatement) -> (Statement, Self::AdditionalOutput) {
        // Use the explicitly provided type if available, otherwise fall back to the type table
        // Note that we have to consult the type annotation first to handle cases like `let x: u32? = 1`
        // where the type annotation is a `u32?` while the RHS is a `u32`.
        let expected_ty = input
            .type_
            .clone()
            .or_else(|| self.state.type_table.get(&input.value.id()))
            .expect("guaranteed by type checking");

        let (new_value, additional_stmts) = self.reconstruct_expression(input.value, &Some(expected_ty));

        input.type_ = input.type_.map(|ty| self.reconstruct_type(ty).0);
        input.value = new_value;

        (input.into(), additional_stmts)
    }

    fn reconstruct_expression_statement(
        &mut self,
        mut input: ExpressionStatement,
    ) -> (Statement, Self::AdditionalOutput) {
        let (expression, stmts) = self.reconstruct_expression(input.expression, &None);

        input.expression = expression;

        (input.into(), stmts)
    }

    fn reconstruct_iteration(&mut self, mut input: IterationStatement) -> (Statement, Self::AdditionalOutput) {
        let mut all_stmts = Vec::new();

        let type_ = match input.type_ {
            Some(ty) => {
                let (new_ty, mut stmts_ty) = self.reconstruct_type(ty);
                all_stmts.append(&mut stmts_ty);
                Some(new_ty)
            }
            None => None,
        };

        let (start, mut stmts_start) = self.reconstruct_expression(input.start, &None);
        let (stop, mut stmts_stop) = self.reconstruct_expression(input.stop, &None);
        let (block, mut stmts_block) = self.reconstruct_block(input.block);

        all_stmts.append(&mut stmts_start);
        all_stmts.append(&mut stmts_stop);
        all_stmts.append(&mut stmts_block);

        input.type_ = type_;
        input.start = start;
        input.stop = stop;
        input.block = block;

        (input.into(), all_stmts)
    }

    fn reconstruct_return(&mut self, mut input: ReturnStatement) -> (Statement, Self::AdditionalOutput) {
        let caller_name = self.function.expect("`self.function` is set every time a function is visited.");
        let caller_path = self.module.iter().cloned().chain(std::iter::once(caller_name)).collect::<Vec<Symbol>>();

        let func_symbol = self
            .state
            .symbol_table
            .lookup_function(self.program, &Location::new(self.program, caller_path))
            .expect("The symbol table creator should already have visited all functions.");

        let return_type = func_symbol.function.output_type.clone();

        let (expression, statements) = self.reconstruct_expression(input.expression, &Some(return_type));
        input.expression = expression;

        (input.into(), statements)
    }
}
