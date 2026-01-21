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

use super::StorageLoweringVisitor;

use leo_ast::*;
use leo_span::{Span, Symbol, sym};

impl leo_ast::AstReconstructor for StorageLoweringVisitor<'_> {
    type AdditionalInput = ();
    type AdditionalOutput = Vec<Statement>;

    /* Types */
    fn reconstruct_array_type(&mut self, input: ArrayType) -> (Type, Self::AdditionalOutput) {
        let (length, stmts) = self.reconstruct_expression(*input.length, &());
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
                let (expr, stmts) = self.reconstruct_expression(arg, &Default::default());
                statements.extend(stmts);
                expr
            })
            .collect();

        (Type::Composite(CompositeType { const_arguments, ..input }), statements)
    }

    /* Expressions */
    fn reconstruct_array_access(
        &mut self,
        mut input: ArrayAccess,
        _additional: &(),
    ) -> (Expression, Self::AdditionalOutput) {
        let (array, mut stmts_array) = self.reconstruct_expression(input.array, &());
        let (index, mut stmts_index) = self.reconstruct_expression(input.index, &());

        input.array = array;
        input.index = index;

        // Merge side effects
        stmts_array.append(&mut stmts_index);

        (input.into(), stmts_array)
    }

    fn reconstruct_intrinsic(
        &mut self,
        mut input: IntrinsicExpression,
        _additional: &Self::AdditionalInput,
    ) -> (Expression, Self::AdditionalOutput) {
        match Intrinsic::from_symbol(input.name, &input.type_parameters) {
            Some(Intrinsic::VectorPush) => {
                // Input:
                //   Vector::push(v, 42u32)
                //
                // Lowered reconstruction:
                //   let $len_var = Mapping::get_or_use(len_map, false, 0u32);
                //   Mapping::set(vec_map, $len_var, 42u32);
                //   Mapping::set(len_map, false, $len_var + 1);

                // Unpack arguments
                let [vector_expr, value_expr] = &mut input.arguments[..] else {
                    panic!("Vector::push should have 2 arguments");
                };

                // Validate vector type
                assert!(matches!(self.state.type_table.get(&vector_expr.id()), Some(Type::Vector(_))));

                // Reconstruct value
                let (value, stmts) = self.reconstruct_expression(value_expr.clone(), &());

                let (vec_values_mapping_name, vec_length_mapping_name) =
                    self.generate_mapping_names_for_vector(vector_expr);
                let vec_path_expr = self.symbol_to_path_expr(vec_values_mapping_name);
                let len_path_expr = self.symbol_to_path_expr(vec_length_mapping_name);

                // let $len_var = Mapping::get_or_use(len_map, false, 0u32)
                let len_var_sym = self.state.assigner.unique_symbol("$len_var", "$");
                let len_var_ident =
                    Identifier { name: len_var_sym, span: Default::default(), id: self.state.node_builder.next_id() };
                let get_len_expr = self.get_vector_len_expr(len_path_expr.clone(), input.span);
                let len_stmt = self.state.assigner.simple_definition(
                    len_var_ident,
                    get_len_expr,
                    self.state.node_builder.next_id(),
                );
                let len_var_expr: Expression = len_var_ident.into();

                // index + 1
                let literal_one = self.literal_one_u32();
                let increment_expr = self.binary_expr(len_var_expr.clone(), BinaryOperation::Add, literal_one);

                // Mapping::set(vec__, $len_var, value)
                let set_vec_stmt_expr = self.set_mapping_expr(vec_path_expr, len_var_expr.clone(), value, input.span);

                // Mapping::set(len_map, false, $len_var + 1)
                let literal_false = self.literal_false();
                let set_len_stmt = Statement::Expression(ExpressionStatement {
                    expression: self.set_mapping_expr(len_path_expr, literal_false, increment_expr, input.span),
                    span: input.span,
                    id: self.state.node_builder.next_id(),
                });

                (set_vec_stmt_expr, [stmts, vec![len_stmt, set_len_stmt]].concat())
            }

            Some(Intrinsic::VectorLen) => {
                // Input:
                //   Vector::len(v)
                //
                // Lowered reconstruction:
                //   Mapping::get_or_use(len_map, false, 0u32)

                //  Unpack arguments
                let [vector_expr] = &mut input.arguments[..] else {
                    panic!("Vector::len should have 1 argument");
                };

                // Validate vector type
                assert!(matches!(self.state.type_table.get(&vector_expr.id()), Some(Type::Vector(_))));

                let (_vec_values_mapping_name, vec_length_mapping_name) =
                    self.generate_mapping_names_for_vector(vector_expr);
                let len_path_expr = self.symbol_to_path_expr(vec_length_mapping_name);

                let get_len_expr = self.get_vector_len_expr(len_path_expr, input.span);
                (get_len_expr, vec![])
            }

            Some(Intrinsic::VectorPop) => {
                // Input:
                //   Vector::pop(v)
                //
                // Lowered reconstruction:
                //   let $len_var = Mapping::get_or_use(len_map, false, 0u32);
                //   Mapping::set(len_map, false, $len_var > 0 ? $len_var - 1 : $len_var);
                //   $len_var > 0 ? Mapping::get_or_use(vec_map, $len_var - 1, zero_value) : None

                // Unpack argument
                let [vector_expr] = &mut input.arguments[..] else {
                    panic!("Vector::pop should have 1 argument");
                };

                // Validate vector type
                let Some(Type::Vector(VectorType { element_type })) = self.state.type_table.get(&vector_expr.id())
                else {
                    panic!("argument to Vector::pop should be of type `Vector`.");
                };

                let (vec_values_mapping_name, vec_length_mapping_name) =
                    self.generate_mapping_names_for_vector(vector_expr);
                let vec_path_expr = self.symbol_to_path_expr(vec_values_mapping_name);
                let len_path_expr = self.symbol_to_path_expr(vec_length_mapping_name);

                // let $len_var = Mapping::get_or_use(len_map, false, 0u32)
                let len_var_sym = self.state.assigner.unique_symbol("$len_var", "$");
                let len_var_ident =
                    Identifier { name: len_var_sym, span: Default::default(), id: self.state.node_builder.next_id() };
                let get_len_expr = self.get_vector_len_expr(len_path_expr.clone(), input.span);
                let len_stmt = self.state.assigner.simple_definition(
                    len_var_ident,
                    get_len_expr,
                    self.state.node_builder.next_id(),
                );
                let len_var_expr: Expression = len_var_ident.into();

                // $len_var > 0
                let literal_zero = self.literal_zero_u32();
                let len_gt_zero_expr = self.binary_expr(len_var_expr.clone(), BinaryOperation::Gt, literal_zero);

                // $len_var - 1
                let literal_one = self.literal_one_u32();
                let len_minus_one_expr =
                    self.binary_expr(len_var_expr.clone(), BinaryOperation::SubWrapped, literal_one);

                // ternary for new length: ($len_var > 0 ? $len_var - 1 : $len_var)
                let new_len_expr = self.ternary_expr(
                    len_gt_zero_expr.clone(),
                    len_minus_one_expr.clone(),
                    len_var_expr.clone(),
                    input.span,
                );

                // Mapping::set(len_map, false, new_len)
                let literal_false = self.literal_false();
                let set_len_stmt = Statement::Expression(ExpressionStatement {
                    expression: self.set_mapping_expr(len_path_expr.clone(), literal_false, new_len_expr, input.span),
                    span: input.span,
                    id: self.state.node_builder.next_id(),
                });

                // zero value for element type (used as default in get_or_use)
                let zero = self.zero(&element_type);

                // Mapping::get_or_use(vec_map, $len_var - 1, zero)
                let get_or_use_expr =
                    self.get_or_use_mapping_expr(vec_path_expr, len_minus_one_expr.clone(), zero, input.span);

                // ternary: $len_var > 0 ? get(vec, len-1) : None
                let none_expr: Expression = Literal::none(Span::default(), self.state.node_builder.next_id()).into();
                let ternary_expr = self.ternary_expr(len_gt_zero_expr, get_or_use_expr, none_expr, input.span);

                (ternary_expr, vec![len_stmt, set_len_stmt])
            }

            Some(Intrinsic::VectorGet) => {
                // Unpack arguments (container, index/key)
                let [container_expr, key_expr] = &mut input.arguments[..] else {
                    panic!("Get should have 2 arguments");
                };

                // Reconstruct key/index)
                let (reconstructed_key_expr, key_stmts) =
                    self.reconstruct_expression(key_expr.clone(), &Default::default());

                if let Some(Type::Vector(VectorType { element_type })) = self.state.type_table.get(&container_expr.id())
                {
                    // Input:
                    //   Get(v, index)
                    //
                    // Lowered reconstruction:
                    //   let $len_var = Mapping::get_or_use(len_map, false, 0u32);
                    //   index < $len_var
                    //       ? Mapping::get_or_use(vec_map, index, zero_value)
                    //       : None

                    let (vec_values_mapping_name, vec_length_mapping_name) =
                        self.generate_mapping_names_for_vector(container_expr);
                    let vec_path_expr = self.symbol_to_path_expr(vec_values_mapping_name);
                    let len_path_expr = self.symbol_to_path_expr(vec_length_mapping_name);

                    // let $len_var = Mapping::get_or_use(len_map, false, 0u32)
                    let len_var_sym = self.state.assigner.unique_symbol("$len_var", "$");
                    let len_var_ident = Identifier {
                        name: len_var_sym,
                        span: Default::default(),
                        id: self.state.node_builder.next_id(),
                    };
                    let get_len_expr = self.get_vector_len_expr(len_path_expr.clone(), input.span);
                    let len_stmt = self.state.assigner.simple_definition(
                        len_var_ident,
                        get_len_expr,
                        self.state.node_builder.next_id(),
                    );
                    let len_var_expr: Expression = len_var_ident.into();

                    // index < len
                    let index_lt_len_expr =
                        self.binary_expr(reconstructed_key_expr.clone(), BinaryOperation::Lt, len_var_expr.clone());

                    // zero value for element type (used as default in get_or_use)
                    let zero = self.zero(&element_type);

                    // Mapping::get(vec_map, index)
                    let get_or_use_expr =
                        self.get_or_use_mapping_expr(vec_path_expr, reconstructed_key_expr.clone(), zero, input.span);

                    // ternary: index < len ? get(vec, index) : None
                    let none_expr: Expression =
                        Literal::none(Span::default(), self.state.node_builder.next_id()).into();
                    let ternary_expr = self.ternary_expr(index_lt_len_expr, get_or_use_expr, none_expr, input.span);

                    (ternary_expr, [key_stmts, vec![len_stmt]].concat())
                } else {
                    panic!("type checking should guarantee that no other type is expected here.")
                }
            }

            Some(Intrinsic::VectorSet) => {
                // Unpack arguments (container, index/key, value)
                let [container_expr, index_expr, value_expr] = &mut input.arguments[..] else {
                    panic!("Set should have 3 arguments");
                };

                // Reconstruct key/index and value
                let (reconstructed_key_expr, key_stmts) =
                    self.reconstruct_expression(index_expr.clone(), &Default::default());
                let (reconstructed_value_expr, value_stmts) =
                    self.reconstruct_expression(value_expr.clone(), &Default::default());

                // Input:
                //   Set(v, index, value)
                //
                // Lowered reconstruction (conceptually):
                //   let $len_var = Mapping::get_or_use(len_map, false, 0u32);
                //   assert(index < $len_var);
                //   Mapping::set(vec_map, index, value);

                let (vec_values_mapping_name, vec_length_mapping_name) =
                    self.generate_mapping_names_for_vector(container_expr);
                let vec_path_expr = self.symbol_to_path_expr(vec_values_mapping_name);
                let len_path_expr = self.symbol_to_path_expr(vec_length_mapping_name);

                // let $len_var = Mapping::get_or_use(len_map, false, 0u32)
                let len_var_sym = self.state.assigner.unique_symbol("$len_var", "$");
                let len_var_ident =
                    Identifier { name: len_var_sym, span: Default::default(), id: self.state.node_builder.next_id() };
                let get_len_expr = self.get_vector_len_expr(len_path_expr.clone(), input.span);
                let len_stmt = self.state.assigner.simple_definition(
                    len_var_ident,
                    get_len_expr,
                    self.state.node_builder.next_id(),
                );
                let len_var_expr: Expression = len_var_ident.into();

                // index < $len_var
                let index_lt_len_expr =
                    self.binary_expr(reconstructed_key_expr.clone(), BinaryOperation::Lt, len_var_expr.clone());

                // Mapping::set(vec_map, index, value)
                let set_stmt_expr = self.set_mapping_expr(
                    vec_path_expr.clone(),
                    reconstructed_key_expr.clone(),
                    reconstructed_value_expr.clone(),
                    input.span,
                );

                // assert(index < len)
                let assert_stmt = Statement::Assert(AssertStatement {
                    variant: AssertVariant::Assert(index_lt_len_expr.clone()),
                    span: Span::default(),
                    id: self.state.node_builder.next_id(),
                });

                // Emit assert then set
                (set_stmt_expr, [key_stmts, value_stmts, vec![len_stmt, assert_stmt]].concat())
            }

            Some(Intrinsic::VectorClear) => {
                // Input:
                //   Vector::clear(v)
                //
                // Lowered reconstruction (conceptually):
                //   Mapping::set(len_map, false, 0u32);
                //
                // Note: `VectorClear` does not actually remove any elements from the mapping of
                // vector values.

                // Unpack arguments
                let [vector_expr] = &mut input.arguments[..] else {
                    panic!("Vector::clear should have 1 argument");
                };

                // Validate vector type
                assert!(matches!(self.state.type_table.get(&vector_expr.id()), Some(Type::Vector(_))));

                let (_vec_values_mapping_name, vec_length_mapping_name) =
                    self.generate_mapping_names_for_vector(vector_expr);
                let len_path_expr = self.symbol_to_path_expr(vec_length_mapping_name);

                // Mapping::set(len_map, false, 0u32)
                let literal_false = self.literal_false();
                let literal_zero = self.literal_zero_u32();
                let set_len_stmt_expr = self.set_mapping_expr(len_path_expr, literal_false, literal_zero, input.span);

                (set_len_stmt_expr, vec![])
            }

            Some(Intrinsic::VectorSwapRemove) => {
                // Input:
                //   Vector::swap_remove(v, index)
                //
                // Lowered reconstruction (conceptually):
                //   let $len_var = Mapping::get_or_use(len_map, false, 0u32);
                //   assert(index < $len_var);
                //   let $removed = Mapping::get(vec_map, index);
                //   Mapping::set(vec_map, index, Mapping::get(vec_map, $len_var - 1));
                //   Mapping::set(len_map, false, $len_var - 1);
                //   $removed

                let [vector_expr, index_expr] = &mut input.arguments[..] else {
                    panic!("Vector::swap_remove should have 2 arguments");
                };

                // Validate vector type
                assert!(matches!(self.state.type_table.get(&vector_expr.id()), Some(Type::Vector(_))));

                // Reconstruct index
                let (reconstructed_index_expr, index_stmts) =
                    self.reconstruct_expression(index_expr.clone(), &Default::default());

                let (vec_values_mapping_name, vec_length_mapping_name) =
                    self.generate_mapping_names_for_vector(vector_expr);
                let vec_path_expr = self.symbol_to_path_expr(vec_values_mapping_name);
                let len_path_expr = self.symbol_to_path_expr(vec_length_mapping_name);

                // let $len_var = Mapping::get_or_use(len_map, false, 0u32)
                let len_var_sym = self.state.assigner.unique_symbol("$len_var", "$");
                let len_var_ident =
                    Identifier { name: len_var_sym, span: Default::default(), id: self.state.node_builder.next_id() };
                let get_len_expr = self.get_vector_len_expr(len_path_expr.clone(), input.span);
                let len_stmt = self.state.assigner.simple_definition(
                    len_var_ident,
                    get_len_expr,
                    self.state.node_builder.next_id(),
                );
                let len_var_expr: Expression = len_var_ident.into();

                // assert(index < $len_var);
                let index_lt_len_expr =
                    self.binary_expr(reconstructed_index_expr.clone(), BinaryOperation::Lt, len_var_expr.clone());
                let assert_stmt = Statement::Assert(AssertStatement {
                    variant: AssertVariant::Assert(index_lt_len_expr.clone()),
                    span: input.span,
                    id: self.state.node_builder.next_id(),
                });

                // let $removed = Mapping::get(vec_map, index); // the element to return
                let get_elem_expr =
                    self.get_mapping_expr(vec_path_expr.clone(), reconstructed_index_expr.clone(), input.span);
                let removed_sym = self.state.assigner.unique_symbol("$removed", "$");
                let removed_ident =
                    Identifier { name: removed_sym, span: Default::default(), id: self.state.node_builder.next_id() };
                let removed_stmt = Statement::Definition(DefinitionStatement {
                    place: DefinitionPlace::Single(removed_ident),
                    type_: None,
                    value: get_elem_expr,
                    span: input.span,
                    id: self.state.node_builder.next_id(),
                });

                // len - 1
                let literal_one = self.literal_one_u32();
                let len_minus_one_expr = self.binary_expr(len_var_expr.clone(), BinaryOperation::Sub, literal_one);

                // Mapping::set(vec_map, index, Mapping::get(vec_map, len - 1));
                let get_last_expr =
                    self.get_mapping_expr(vec_path_expr.clone(), len_minus_one_expr.clone(), input.span);
                let set_swap_stmt = Statement::Expression(ExpressionStatement {
                    expression: self.set_mapping_expr(
                        vec_path_expr.clone(),
                        reconstructed_index_expr.clone(),
                        get_last_expr,
                        input.span,
                    ),
                    span: input.span,
                    id: self.state.node_builder.next_id(),
                });

                // Mapping::set(len_map, false, len - 1);
                let literal_false = self.literal_false();
                let set_len_stmt = Statement::Expression(ExpressionStatement {
                    expression: self.set_mapping_expr(
                        len_path_expr.clone(),
                        literal_false,
                        len_minus_one_expr,
                        input.span,
                    ),
                    span: input.span,
                    id: self.state.node_builder.next_id(),
                });

                // Return `$removed` as the resulting expression
                (
                    removed_ident.into(),
                    [index_stmts, vec![len_stmt, assert_stmt, removed_stmt, set_swap_stmt, set_len_stmt]].concat(),
                )
            }

            _ => {
                // Default: reconstruct all arguments recursively and return the (possibly updated) original call
                let statements: Vec<_> = input
                    .arguments
                    .iter_mut()
                    .flat_map(|arg| {
                        let (expr, stmts) = self.reconstruct_expression(std::mem::take(arg), &());
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
        _additional: &(),
    ) -> (Expression, Self::AdditionalOutput) {
        let (inner, stmts_inner) = self.reconstruct_expression(input.inner, &());

        input.inner = inner;

        (input.into(), stmts_inner)
    }

    fn reconstruct_repeat(
        &mut self,
        mut input: RepeatExpression,
        _additional: &(),
    ) -> (Expression, Self::AdditionalOutput) {
        // Use expected type (if available) for `expr`
        let (expr, mut stmts_expr) = self.reconstruct_expression(input.expr, &());
        let (count, mut stmts_count) = self.reconstruct_expression(input.count, &());

        input.expr = expr;
        input.count = count;

        stmts_expr.append(&mut stmts_count);

        (input.into(), stmts_expr)
    }

    fn reconstruct_tuple_access(
        &mut self,
        mut input: TupleAccess,
        _additional: &(),
    ) -> (Expression, Self::AdditionalOutput) {
        let (tuple, stmts) = self.reconstruct_expression(input.tuple, &());

        input.tuple = tuple;

        (input.into(), stmts)
    }

    fn reconstruct_array(
        &mut self,
        mut input: ArrayExpression,
        _additional: &(),
    ) -> (Expression, Self::AdditionalOutput) {
        let mut all_stmts = Vec::new();
        let mut new_elements = Vec::with_capacity(input.elements.len());

        for element in input.elements.into_iter() {
            let (expr, mut stmts) = self.reconstruct_expression(element, &());
            all_stmts.append(&mut stmts);
            new_elements.push(expr);
        }

        input.elements = new_elements;

        (input.into(), all_stmts)
    }

    fn reconstruct_binary(
        &mut self,
        mut input: BinaryExpression,
        _additional: &(),
    ) -> (Expression, Self::AdditionalOutput) {
        let (left, mut stmts_left) = self.reconstruct_expression(input.left, &());
        let (right, mut stmts_right) = self.reconstruct_expression(input.right, &());

        input.left = left;
        input.right = right;

        // Merge side effects
        stmts_left.append(&mut stmts_right);

        (input.into(), stmts_left)
    }

    fn reconstruct_call(&mut self, mut input: CallExpression, _addiional: &()) -> (Expression, Self::AdditionalOutput) {
        let mut statements = Vec::new();
        for arg in input.arguments.iter_mut() {
            let (expr, statements2) = self.reconstruct_expression(std::mem::take(arg), &());
            statements.extend(statements2);
            *arg = expr;
        }
        (input.into(), statements)
    }

    fn reconstruct_cast(&mut self, input: CastExpression, _addiional: &()) -> (Expression, Self::AdditionalOutput) {
        let (expression, statements) = self.reconstruct_expression(input.expression, &());
        (CastExpression { expression, ..input }.into(), statements)
    }

    fn reconstruct_composite_init(
        &mut self,
        mut input: CompositeExpression,
        _additional: &(),
    ) -> (Expression, Self::AdditionalOutput) {
        let mut statements = Vec::new();

        // Reconstruct const_arguments and extract statements
        for const_arg in input.const_arguments.iter_mut() {
            let (expr, statements2) = self.reconstruct_expression(const_arg.clone(), &());
            statements.extend(statements2);
            *const_arg = expr;
        }

        // Reconstruct members and extract statements
        for member in input.members.iter_mut() {
            assert!(member.expression.is_some());
            let (expr, statements2) = self.reconstruct_expression(member.expression.take().unwrap(), &());
            statements.extend(statements2);
            member.expression = Some(expr);
        }

        (input.into(), statements)
    }

    fn reconstruct_path(&mut self, input: Path, _additional: &()) -> (Expression, Self::AdditionalOutput) {
        // Check if this path corresponds to a global symbol.
        let Some(global_location) = input.try_global_location() else {
            return (input.into(), vec![]);
        };
        let var = self.state.symbol_table.lookup_global(global_location).expect("A global path must point to a global");

        match var.type_.as_ref().expect("must be known by now") {
            Type::Mapping(_) => {
                // No transformation needed for mappings.
                (input.into(), vec![])
            }

            Type::Optional(OptionalType { inner }) => {
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
                let var_name = input.identifier().name;

                // Path to the mapping backing the optional variable: `<var_name>__`
                let mapping_symbol = Symbol::intern(&format!("{var_name}__"));
                let mapping_ident = Identifier::new(mapping_symbol, id());

                // === Build expressions ===
                let mapping_expr: Expression =
                    Path::from(mapping_ident).to_global(Location::new(self.program, vec![mapping_symbol])).into();
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

                (ternary_expr, vec![])
            }

            _ => {
                panic!("Expected a non-vector type in reconstruct_path, found {:?}", var.type_);
            }
        }
    }

    fn reconstruct_ternary(
        &mut self,
        input: TernaryExpression,
        _addiional: &(),
    ) -> (Expression, Self::AdditionalOutput) {
        let (condition, mut statements) = self.reconstruct_expression(input.condition, &());
        let (if_true, statements2) = self.reconstruct_expression(input.if_true, &());
        let (if_false, statements3) = self.reconstruct_expression(input.if_false, &());
        statements.extend(statements2);
        statements.extend(statements3);
        (TernaryExpression { condition, if_true, if_false, ..input }.into(), statements)
    }

    fn reconstruct_tuple(
        &mut self,
        input: leo_ast::TupleExpression,
        _addiional: &(),
    ) -> (Expression, Self::AdditionalOutput) {
        // This should ony appear in a return statement.
        let mut statements = Vec::new();
        let elements = input
            .elements
            .into_iter()
            .map(|element| {
                let (expr, statements2) = self.reconstruct_expression(element, &());
                statements.extend(statements2);
                expr
            })
            .collect();
        (TupleExpression { elements, ..input }.into(), statements)
    }

    fn reconstruct_unary(
        &mut self,
        input: leo_ast::UnaryExpression,
        _addiional: &(),
    ) -> (Expression, Self::AdditionalOutput) {
        let (receiver, statements) = self.reconstruct_expression(input.receiver, &());
        (UnaryExpression { receiver, ..input }.into(), statements)
    }

    /* Statements */
    fn reconstruct_assert(&mut self, input: leo_ast::AssertStatement) -> (Statement, Self::AdditionalOutput) {
        let mut statements = Vec::new();
        let stmt = AssertStatement {
            variant: match input.variant {
                AssertVariant::Assert(expr) => {
                    let (expr, statements2) = self.reconstruct_expression(expr, &());
                    statements.extend(statements2);
                    AssertVariant::Assert(expr)
                }
                AssertVariant::AssertEq(left, right) => {
                    let (left, statements2) = self.reconstruct_expression(left, &());
                    statements.extend(statements2);
                    let (right, statements3) = self.reconstruct_expression(right, &());
                    statements.extend(statements3);
                    AssertVariant::AssertEq(left, right)
                }
                AssertVariant::AssertNeq(left, right) => {
                    let (left, statements2) = self.reconstruct_expression(left, &());
                    statements.extend(statements2);
                    let (right, statements3) = self.reconstruct_expression(right, &());
                    statements.extend(statements3);
                    AssertVariant::AssertNeq(left, right)
                }
            },
            ..input
        }
        .into();
        (stmt, statements)
    }

    fn reconstruct_assign(&mut self, input: AssignStatement) -> (Statement, Self::AdditionalOutput) {
        let AssignStatement { place, value, span, .. } = input;
        let mut statements = vec![];

        // Check if `place` is a path
        if let Expression::Path(path) = &place {
            // Check if the path corresponds to a global storage variable
            if let Some(global_location) = path.try_global_location() {
                let var = self
                    .state
                    .symbol_table
                    .lookup_global(global_location)
                    .expect("A global path must point to a global");

                // Storage variables that are not optional nor mappings are implicitly wrapped in an optional.
                assert!(
                    var.type_.as_ref().expect("must be known by now").is_optional(),
                    "Only storage variables that are not vectors or mappings are expected here."
                );

                // Reconstruct the RHS
                let (new_value, mut value_stmts) = self.reconstruct_expression(value, &());
                statements.append(&mut value_stmts);

                let id = || self.state.node_builder.next_id();
                let var_name = path.identifier().name;

                // Path to the mapping backing the storage variable: `<var_name>__`
                let mapping_symbol = Symbol::intern(&format!("{var_name}__"));
                let mapping_ident = Identifier::new(mapping_symbol, id());
                let mapping_expr: Expression =
                    Path::from(mapping_ident).to_global(Location::new(self.program, vec![mapping_symbol])).into();
                let false_literal: Expression = Literal::boolean(false, Span::default(), id()).into();

                let stmt = if matches!(new_value, Expression::Literal(Literal { variant: LiteralVariant::None, .. })) {
                    // Input:
                    //   storage x: field;
                    //   ...
                    //   x = none;
                    //
                    // Lowered reconstruction:
                    //   mapping x__: bool => field;
                    //   ...
                    //   _mapping_remove(x__, false);
                    let remove_expr: Expression = IntrinsicExpression {
                        name: sym::_mapping_remove,
                        type_parameters: vec![],
                        arguments: vec![mapping_expr, false_literal],
                        span,
                        id: id(),
                    }
                    .into();
                    Statement::Expression(ExpressionStatement { expression: remove_expr, span, id: id() })
                } else {
                    // Input:
                    //   storage x: field;
                    //   ...
                    //   x = 5field;
                    //
                    // Lowered reconstruction:
                    //   mapping x__: bool => field;
                    //   ...
                    //   _mapping_set(x__, false, 5field);
                    let set_expr: Expression = IntrinsicExpression {
                        name: sym::_mapping_set,
                        type_parameters: vec![],
                        arguments: vec![mapping_expr, false_literal, new_value],
                        span,
                        id: id(),
                    }
                    .into();
                    Statement::Expression(ExpressionStatement { expression: set_expr, span, id: id() })
                };
                return (stmt, statements);
            }
        }

        // In all other cases, nothing special to do.
        let (new_place, mut place_stmts) = self.reconstruct_expression(place, &());
        let (new_value, mut value_stmts) = self.reconstruct_expression(value, &());
        statements.append(&mut place_stmts);
        statements.append(&mut value_stmts);

        let stmt =
            AssignStatement { place: new_place, value: new_value, span, id: self.state.node_builder.next_id() }.into();
        (stmt, statements)
    }

    fn reconstruct_block(&mut self, block: Block) -> (Block, Self::AdditionalOutput) {
        let mut statements = Vec::with_capacity(block.statements.len());

        // Flatten each statement, accumulating any new statements produced.
        for statement in block.statements {
            let (reconstructed_statement, additional_statements) = self.reconstruct_statement(statement);
            statements.extend(additional_statements);
            statements.push(reconstructed_statement);
        }

        (Block { span: block.span, statements, id: self.state.node_builder.next_id() }, Default::default())
    }

    fn reconstruct_conditional(&mut self, input: leo_ast::ConditionalStatement) -> (Statement, Self::AdditionalOutput) {
        let (condition, mut statements) = self.reconstruct_expression(input.condition, &());
        let (then, statements2) = self.reconstruct_block(input.then);
        statements.extend(statements2);
        let otherwise = input.otherwise.map(|oth| {
            let (expr, statements3) = self.reconstruct_statement(*oth);
            statements.extend(statements3);
            Box::new(expr)
        });
        (ConditionalStatement { condition, then, otherwise, ..input }.into(), statements)
    }

    fn reconstruct_const(&mut self, input: ConstDeclaration) -> (Statement, Self::AdditionalOutput) {
        let (type_expr, type_statements) = self.reconstruct_type(input.type_);
        let (value_expr, value_statements) = self.reconstruct_expression(input.value, &Default::default());

        let mut statements = Vec::new();
        statements.extend(type_statements);
        statements.extend(value_statements);

        (ConstDeclaration { type_: type_expr, value: value_expr, ..input }.into(), statements)
    }

    fn reconstruct_definition(&mut self, mut input: DefinitionStatement) -> (Statement, Self::AdditionalOutput) {
        let (new_value, additional_stmts) = self.reconstruct_expression(input.value, &());

        input.type_ = input.type_.map(|ty| self.reconstruct_type(ty).0);
        input.value = new_value;

        (input.into(), additional_stmts)
    }

    fn reconstruct_expression_statement(&mut self, input: ExpressionStatement) -> (Statement, Self::AdditionalOutput) {
        let (reconstructed_expression, statements) = self.reconstruct_expression(input.expression, &Default::default());
        if !matches!(reconstructed_expression, Expression::Call(_) | Expression::Intrinsic(_)) {
            (
                ExpressionStatement {
                    expression: Expression::Unit(UnitExpression {
                        span: Span::default(),
                        id: self.state.node_builder.next_id(),
                    }),
                    ..input
                }
                .into(),
                statements,
            )
        } else {
            (ExpressionStatement { expression: reconstructed_expression, ..input }.into(), statements)
        }
    }

    fn reconstruct_iteration(&mut self, _input: IterationStatement) -> (Statement, Self::AdditionalOutput) {
        panic!("`IterationStatement`s should not be in the AST at this point.");
    }

    fn reconstruct_return(&mut self, input: ReturnStatement) -> (Statement, Self::AdditionalOutput) {
        let (expression, statements) = self.reconstruct_expression(input.expression, &());
        (ReturnStatement { expression, ..input }.into(), statements)
    }
}
