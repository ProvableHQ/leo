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

use super::{StorageLoweringVisitor, zero_value_expression};

use leo_ast::*;
use leo_span::{Span, Symbol, sym};

impl StorageLoweringVisitor<'_> {
    /// Extracts the base symbol, vec_map symbol and len_map symbol from a vector expression
    fn extract_vector_symbols(&self, expr: &Expression) -> (Symbol, Symbol, Symbol) {
        let path = match expr {
            Expression::Path(path) => path,
            _ => panic!("Expected path expression for vector"),
        };
        let base_sym = path.identifier().name;
        let vec_map_sym = Symbol::intern(&format!("{base_sym}__"));
        let len_map_sym = Symbol::intern(&format!("{base_sym}__len__"));
        (base_sym, vec_map_sym, len_map_sym)
    }

    /// Creates a path expression from a symbol
    fn path_expr(&mut self, sym: Symbol) -> Expression {
        Expression::Path(Path::from(Identifier::new(sym, self.state.node_builder.next_id())).into_absolute())
    }

    /// Standard literal expressions used frequently
    fn literal_false(&mut self) -> Expression {
        Literal::boolean(false, Span::default(), self.state.node_builder.next_id()).into()
    }

    fn literal_zero_u32(&mut self) -> Expression {
        Literal::integer(IntegerType::U32, "0".to_string(), Span::default(), self.state.node_builder.next_id()).into()
    }

    fn literal_one_u32(&mut self) -> Expression {
        Literal::integer(IntegerType::U32, "1".to_string(), Span::default(), self.state.node_builder.next_id()).into()
    }

    /// Generates Mapping::get_or_use(len_path_expr, false, 0u32)
    fn get_vector_len_expr(&mut self, len_path_expr: Expression, span: Span) -> Expression {
        Expression::AssociatedFunction(AssociatedFunctionExpression {
            variant: Identifier::new(sym::Mapping, self.state.node_builder.next_id()),
            name: Identifier::new(Symbol::intern("get_or_use"), self.state.node_builder.next_id()),
            arguments: vec![len_path_expr, self.literal_false(), self.literal_zero_u32()],
            span,
            id: self.state.node_builder.next_id(),
        })
    }

    /// Generates Mapping::set(path_expr, key_expr, value_expr)
    fn set_mapping_expr(
        &mut self,
        path_expr: Expression,
        key_expr: Expression,
        value_expr: Expression,
        span: Span,
    ) -> Expression {
        Expression::AssociatedFunction(AssociatedFunctionExpression {
            variant: Identifier::new(sym::Mapping, self.state.node_builder.next_id()),
            name: Identifier::new(Symbol::intern("set"), self.state.node_builder.next_id()),
            arguments: vec![path_expr, key_expr, value_expr],
            span,
            id: self.state.node_builder.next_id(),
        })
    }

    /// Generates Mapping::get(path_expr, key_expr)
    fn get_mapping_expr(&mut self, path_expr: Expression, key_expr: Expression, span: Span) -> Expression {
        Expression::AssociatedFunction(AssociatedFunctionExpression {
            variant: Identifier::new(sym::Mapping, self.state.node_builder.next_id()),
            name: Identifier::new(Symbol::intern("get"), self.state.node_builder.next_id()),
            arguments: vec![path_expr, key_expr],
            span,
            id: self.state.node_builder.next_id(),
        })
    }

    /// Generates Mapping::get_or_use(path_expr, key_expr, default_expr)
    fn get_or_use_mapping_expr(
        &mut self,
        path_expr: Expression,
        key_expr: Expression,
        default_expr: Expression,
        span: Span,
    ) -> Expression {
        Expression::AssociatedFunction(AssociatedFunctionExpression {
            variant: Identifier::new(sym::Mapping, self.state.node_builder.next_id()),
            name: Identifier::new(Symbol::intern("get_or_use"), self.state.node_builder.next_id()),
            arguments: vec![path_expr, key_expr, default_expr],
            span,
            id: self.state.node_builder.next_id(),
        })
    }

    /// Generates a ternary expression
    fn ternary_expr(
        &mut self,
        condition: Expression,
        if_true: Expression,
        if_false: Expression,
        span: Span,
    ) -> Expression {
        Expression::Ternary(Box::new(TernaryExpression {
            condition,
            if_true,
            if_false,
            span,
            id: self.state.node_builder.next_id(),
        }))
    }

    /// Generates a binary expression
    fn binary_expr(&mut self, left: Expression, op: BinaryOperation, right: Expression) -> Expression {
        Expression::Binary(Box::new(BinaryExpression {
            op,
            left,
            right,
            span: Span::default(),
            id: self.state.node_builder.next_id(),
        }))
    }

    fn zero_val_expr(&self, ty: &Type) -> Expression {
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
        zero_value_expression(ty, Span::default(), &self.state.node_builder, &struct_lookup)
            .expect("zero value generation failed")
    }
}

impl leo_ast::AstReconstructor for StorageLoweringVisitor<'_> {
    type AdditionalInput = ();
    type AdditionalOutput = Vec<Statement>;

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

    fn reconstruct_associated_function(
        &mut self,
        mut input: AssociatedFunctionExpression,
        _additional: &(),
    ) -> (Expression, Self::AdditionalOutput) {
        match CoreFunction::from_symbols(input.variant.name, input.name.name) {
            Some(CoreFunction::VectorPush) => {
                // Step 1. Unpack arguments
                // Step 2. Reconstruct value expression
                // Step 3. Extract vector symbols (base, vec_map, len_map)
                // Step 4. Build path expressions for vec__ and vec__len__
                // Step 5. Create $len_var = Mapping::get_or_use(len_map, false, 0u32)
                // Step 6. set vec__[ $len_var ] = value
                // Step 7. set vec__len__[false] = $len_var + 1
                let [vector_expr, value_expr] = &mut input.arguments[..] else {
                    panic!("Vector::push should have 2 arguments");
                };

                // Reconstruct value
                let (value, _) = self.reconstruct_expression(value_expr.clone(), &());

                // Symbols and paths
                let (_base_sym, vec_map_sym, len_map_sym) = self.extract_vector_symbols(vector_expr);
                let vec_path_expr = self.path_expr(vec_map_sym);
                let len_path_expr = self.path_expr(len_map_sym);

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

                (set_vec_stmt_expr, vec![len_stmt, set_len_stmt])
            }

            Some(CoreFunction::VectorLen) => {
                // Step 1. Unpack arguments
                // Step 2. Extract len_map symbol and create path
                // Step 3. Return Mapping::get_or_use(len_map, false, 0u32)
                let [vector_expr] = &mut input.arguments[..] else {
                    panic!("Vector::len should have 1 argument");
                };

                let (_base_sym, _vec_map_sym, len_map_sym) = self.extract_vector_symbols(vector_expr);
                let len_path_expr = self.path_expr(len_map_sym);

                let get_len_expr = self.get_vector_len_expr(len_path_expr, input.span);
                (get_len_expr, vec![])
            }

            Some(CoreFunction::VectorPop) => {
                // Step 1. Unpack argument
                // Step 2. Ensure vector type and extract symbols
                // Step 3. let $len_var = Mapping::get_or_use(len_map, false, 0u32)
                // Step 4. condition: $len_var > 0
                // Step 5. decrement len: set(len_map, false, $len_var - 1)
                // Step 6. result: $len_var > 0 ? Some(Mapping::get(vec_map, $len_var - 1)) : None
                let [vector_expr] = &mut input.arguments[..] else {
                    panic!("Vector::pop should have 1 argument");
                };

                // validate vector type
                let Some(Type::Vector(VectorType { element_type })) = self.state.type_table.get(&vector_expr.id())
                else {
                    panic!("expecting a vector type here");
                };

                let (_base_sym, vec_map_sym, len_map_sym) = self.extract_vector_symbols(vector_expr);
                let vec_path_expr = self.path_expr(vec_map_sym);
                let len_path_expr = self.path_expr(len_map_sym);

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

                // condition: $len_var > 0
                let literal_zero = self.literal_zero_u32();
                let len_gt_zero_expr = self.binary_expr(len_var_expr.clone(), BinaryOperation::Gt, literal_zero);

                // $len_var - 1
                let literal_one = self.literal_one_u32();
                let len_minus_one_expr = self.binary_expr(len_var_expr.clone(), BinaryOperation::Sub, literal_one);

                // Mapping::set(len_map, false, $len_var - 1) (inside if)
                let literal_false = self.literal_false();
                let set_len_stmt = Statement::Expression(ExpressionStatement {
                    expression: self.set_mapping_expr(
                        len_path_expr.clone(),
                        literal_false,
                        len_minus_one_expr.clone(),
                        input.span,
                    ),
                    span: input.span,
                    id: self.state.node_builder.next_id(),
                });

                // if ($len_var > 0) { set(len_map, false, $len_var - 1) }
                let if_stmt = Statement::Conditional(ConditionalStatement {
                    condition: len_gt_zero_expr.clone(),
                    then: Block {
                        statements: vec![set_len_stmt],
                        span: Span::default(),
                        id: self.state.node_builder.next_id(),
                    },
                    otherwise: None,
                    span: Span::default(),
                    id: self.state.node_builder.next_id(),
                });

                // zero value for element type (used as default in get_or_use)
                let zero_val_expr = self.zero_val_expr(&element_type);

                // Mapping::get(vec_map, $len_var - 1) with fallback zero_val_expr
                let get_or_use_expr =
                    self.get_or_use_mapping_expr(vec_path_expr, len_minus_one_expr.clone(), zero_val_expr, input.span);

                // ternary: $len_var > 0 ? get(vec, len-1) : None
                let none_expr: Expression = Literal::none(Span::default(), self.state.node_builder.next_id()).into();
                let ternary_expr = self.ternary_expr(len_gt_zero_expr, get_or_use_expr, none_expr, input.span);

                (ternary_expr, vec![len_stmt, if_stmt])
            }

            Some(CoreFunction::Get) => {
                // Step 1. Unpack arguments (vector, index)
                // Step 2. If vector type unknown => reconstruct arguments recursively and return original call + statements
                // Step 3. Reconstruct index expression
                // Step 4. let $len_var = Mapping::get_or_use(len_map, false, 0u32)
                // Step 5. cond: index < $len_var
                // Step 6. result: index < len ? Mapping::get(vec_map, index) : None
                let [vector_expr, index_expr] = &mut input.arguments[..] else {
                    panic!("Vector::get should have 2 arguments");
                };

                let Some(Type::Vector(VectorType { element_type })) = self.state.type_table.get(&vector_expr.id())
                else {
                    // reconstruct sub-expressions and return the input as-is with statements collected
                    let statements: Vec<_> = input
                        .arguments
                        .iter_mut()
                        .flat_map(|arg| {
                            let (expr, stmts) = self.reconstruct_expression(std::mem::take(arg), &());
                            *arg = expr;
                            stmts
                        })
                        .collect();

                    return (input.into(), statements);
                };

                // Reconstruct index (so macros/expressions inside index are flattened)
                let (reconstructed_index_expr, _) =
                    self.reconstruct_expression(index_expr.clone(), &Default::default());

                let (_base_sym, vec_map_sym, len_map_sym) = self.extract_vector_symbols(vector_expr);
                let vec_path_expr = self.path_expr(vec_map_sym);
                let len_path_expr = self.path_expr(len_map_sym);

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

                // index < len
                let index_lt_len_expr =
                    self.binary_expr(reconstructed_index_expr.clone(), BinaryOperation::Lt, len_var_expr.clone());

                // zero value for element type (used as default in get_or_use)
                let zero_val_expr = self.zero_val_expr(&element_type);

                // Mapping::get(vec_map, index)
                let get_or_use_expr = self.get_or_use_mapping_expr(
                    vec_path_expr,
                    reconstructed_index_expr.clone(),
                    zero_val_expr,
                    input.span,
                );

                // ternary: index < len ? get(vec, index) : None
                let none_expr: Expression = Literal::none(Span::default(), self.state.node_builder.next_id()).into();
                let ternary_expr = self.ternary_expr(index_lt_len_expr, get_or_use_expr, none_expr, input.span);

                (ternary_expr, vec![len_stmt])
            }

            Some(CoreFunction::Set) => {
                // Step 1. Unpack arguments (vector, index, value)
                // Step 2. If vector not a vector type => reconstruct args and return original call + statements
                // Step 3. Reconstruct index & value expressions
                // Step 4. let $len_var = Mapping::get_or_use(len_map, false, 0u32)
                // Step 5. assert(index < $len_var)
                // Step 6. Mapping::set(vec_map, index, value)
                let [vector_expr, index_expr, value_expr] = &mut input.arguments[..] else {
                    panic!("Vector::set should have 3 arguments");
                };

                if !matches!(self.state.type_table.get(&vector_expr.id()), Some(Type::Vector(_))) {
                    let statements: Vec<_> = input
                        .arguments
                        .iter_mut()
                        .flat_map(|arg| {
                            let (expr, stmts) = self.reconstruct_expression(std::mem::take(arg), &());
                            *arg = expr;
                            stmts
                        })
                        .collect();

                    return (input.into(), statements);
                };

                // Reconstruct index and value
                let (reconstructed_index_expr, _) =
                    self.reconstruct_expression(index_expr.clone(), &Default::default());
                let (reconstructed_value_expr, _) =
                    self.reconstruct_expression(value_expr.clone(), &Default::default());

                let (_base_sym, vec_map_sym, len_map_sym) = self.extract_vector_symbols(vector_expr);
                let vec_path_expr = self.path_expr(vec_map_sym);
                let len_path_expr = self.path_expr(len_map_sym);

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
                    self.binary_expr(reconstructed_index_expr.clone(), BinaryOperation::Lt, len_var_expr.clone());

                // Mapping::set(vec_map, index, value)
                let set_stmt_expr = self.set_mapping_expr(
                    vec_path_expr.clone(),
                    reconstructed_index_expr.clone(),
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
                (set_stmt_expr, vec![len_stmt, assert_stmt])
            }

            Some(CoreFunction::VectorClear) => {
                // Step 1. Unpack arg
                // Step 2. Extract len_map symbol and path
                // Step 3. Mapping::set(len_map, false, 0u32)
                let [vector_expr] = &mut input.arguments[..] else {
                    panic!("Vector::clear should have 1 argument");
                };

                let (_base_sym, _vec_map_sym, len_map_sym) = self.extract_vector_symbols(vector_expr);
                let len_path_expr = self.path_expr(len_map_sym);

                // Mapping::set(len_map, false, 0u32)
                let literal_false = self.literal_false();
                let literal_zero = self.literal_zero_u32();
                let set_len_stmt_expr = self.set_mapping_expr(len_path_expr, literal_false, literal_zero, input.span);

                (set_len_stmt_expr, vec![])
            }

            Some(CoreFunction::VectorSwapRemove) => {
                // Step 1. Unpack args (vector, index)
                // Step 2. Ensure vector type
                // Step 3. Reconstruct index
                // Step 4. let $len_var = Mapping::get_or_use(len_map, false, 0u32)
                // Step 5. cond: index < $len_var
                // Step 6. compute len - 1, get last element, set vec[index] = last_element
                // Step 7. set len_map to len - 1
                // Step 8. return index < len ? get(vec, index) : None
                let [vector_expr, index_expr] = &mut input.arguments[..] else {
                    panic!("Vector::swap_remove should have 2 arguments");
                };

                if !matches!(self.state.type_table.get(&vector_expr.id()), Some(Type::Vector(_))) {
                    panic!("expecting a vector type here");
                };

                // Reconstruct index
                let (reconstructed_index_expr, _) =
                    self.reconstruct_expression(index_expr.clone(), &Default::default());

                let (_base_sym, vec_map_sym, len_map_sym) = self.extract_vector_symbols(vector_expr);
                let vec_path_expr = self.path_expr(vec_map_sym);
                let len_path_expr = self.path_expr(len_map_sym);

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

                // condition: index < len
                let index_lt_len_expr =
                    self.binary_expr(reconstructed_index_expr.clone(), BinaryOperation::Lt, len_var_expr.clone());

                // len - 1
                let literal_one = self.literal_one_u32();
                let len_minus_one_expr = self.binary_expr(len_var_expr.clone(), BinaryOperation::Sub, literal_one);

                // Mapping::set(len_map, false, len - 1)
                let literal_false = self.literal_false();
                let set_len_stmt = Statement::Expression(ExpressionStatement {
                    expression: self.set_mapping_expr(
                        len_path_expr.clone(),
                        literal_false,
                        len_minus_one_expr.clone(),
                        input.span,
                    ),
                    span: input.span,
                    id: self.state.node_builder.next_id(),
                });

                // Mapping::get(vec_map, index)  // the element to return (old element)
                let get_elem_expr =
                    self.get_mapping_expr(vec_path_expr.clone(), reconstructed_index_expr.clone(), input.span);

                // Mapping::get(vec_map, len - 1)  // last_elem
                let get_last_expr =
                    self.get_mapping_expr(vec_path_expr.clone(), len_minus_one_expr.clone(), input.span);

                // Mapping::set(vec_map, index, last_elem)
                let set_swap_stmt = Statement::Expression(ExpressionStatement {
                    expression: self.set_mapping_expr(
                        vec_path_expr.clone(),
                        reconstructed_index_expr.clone(),
                        get_last_expr.clone(),
                        input.span,
                    ),
                    span: input.span,
                    id: self.state.node_builder.next_id(),
                });

                // assert(index < len)
                let assert_stmt = Statement::Assert(AssertStatement {
                    variant: AssertVariant::Assert(index_lt_len_expr.clone()),
                    span: Span::default(),
                    id: self.state.node_builder.next_id(),
                });

                // ternary: index < len ? get(vec, index) : None
                let none_expr: Expression = Literal::none(Span::default(), self.state.node_builder.next_id()).into();
                let ternary_expr = self.ternary_expr(index_lt_len_expr, get_elem_expr, none_expr, input.span);

                (ternary_expr, vec![len_stmt, assert_stmt, set_len_stmt, set_swap_stmt])
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

    fn reconstruct_struct_init(
        &mut self,
        mut input: StructExpression,
        _addiional: &(),
    ) -> (Expression, Self::AdditionalOutput) {
        let mut statements = Vec::new();
        for member in input.members.iter_mut() {
            assert!(member.expression.is_some());
            let (expr, statements2) = self.reconstruct_expression(member.expression.take().unwrap(), &());
            statements.extend(statements2);
            member.expression = Some(expr);
        }

        (input.into(), statements)
    }

    fn reconstruct_path(&mut self, input: Path, __additional: &()) -> (Expression, Self::AdditionalOutput) {
        if let Some(var) = self.state.symbol_table.lookup_global(&Location::new(self.program, input.absolute_path())) {
            if let Type::Optional(OptionalType { ref inner }) = var.type_ {
                let node_id = self.state.node_builder.next_id();
                let x = input.identifier().name;
                let x_ = Symbol::intern(&format!("{x}__"));

                let id = || self.state.node_builder.next_id();

                let mapping_path = Path::from(Identifier::new(x_, id())).into_absolute();

                // Build `x_.contains(false)`
                let contains_call = Expression::AssociatedFunction(AssociatedFunctionExpression {
                    variant: Identifier::new(sym::Mapping, id()),
                    name: Identifier::new(Symbol::intern("contains"), id()),
                    arguments: vec![
                        Expression::Path(mapping_path.clone()),
                        Literal::boolean(false, Span::default(), self.state.node_builder.next_id()).into(),
                    ],
                    span: Span::default(),
                    id: id(),
                });

                // zero value for element type (used as default in get_or_use)
                let zero_val_expr = self.zero_val_expr(inner);

                // Build `x_.get(0)`
                let get_or_use_call = Expression::AssociatedFunction(AssociatedFunctionExpression {
                    variant: Identifier::new(sym::Mapping, id()),
                    name: Identifier::new(Symbol::intern("get_or_use"), id()),
                    arguments: vec![
                        Expression::Path(mapping_path.clone()),
                        Literal::boolean(false, Span::default(), self.state.node_builder.next_id()).into(),
                        zero_val_expr,
                    ],
                    span: Span::default(),
                    id: id(),
                });

                // Build `None`
                let none_expr =
                    Expression::Literal(Literal { variant: LiteralVariant::None, span: Span::default(), id: id() });

                // Combine into ternary: `x_.contains(false) ? x_.get(0) : None`
                let ternary = Expression::Ternary(Box::new(TernaryExpression {
                    condition: contains_call,
                    if_true: get_or_use_call,
                    if_false: none_expr,
                    span: Span::default(),
                    id: node_id,
                }));

                return (ternary, vec![]);
            }
        }

        (input.into(), Default::default())
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

    fn reconstruct_assign(&mut self, input: AssignStatement) -> (Statement, Self::AdditionalOutput) {
        let AssignStatement { place, value, span, id } = input;

        // Only transform if the place is a path to a global storage variable that is NOT already a mapping
        if let Expression::Path(path) = &place {
            if let Some(var) = self.state.symbol_table.lookup_global(&Location::new(self.program, path.absolute_path()))
            {
                if !matches!(var.type_, Type::Mapping(_)) {
                    // Generate `x__` from `x`
                    let x = path.identifier().name;
                    let x__ = Symbol::intern(&format!("{x}__"));

                    // Create Path to `x__`
                    let mapping_path = Expression::Path(
                        Path::from(Identifier::new(x__, self.state.node_builder.next_id())).into_absolute(),
                    );

                    // Create `false` as the hardcoded key
                    let key_expr = Expression::Literal(Literal {
                        variant: LiteralVariant::Boolean(false),
                        span: Span::default(),
                        id: self.state.node_builder.next_id(),
                    });

                    // Reconstruct the value expression
                    let (new_value, _) = self.reconstruct_expression(value, &());

                    // Create Mapping::set(x__, false, value)
                    let call_expr = Expression::AssociatedFunction(AssociatedFunctionExpression {
                        variant: Identifier::new(sym::Mapping, self.state.node_builder.next_id()),
                        name: Identifier::new(Symbol::intern("set"), self.state.node_builder.next_id()),
                        arguments: vec![mapping_path, key_expr, new_value],
                        span,
                        id: self.state.node_builder.next_id(),
                    });

                    // Wrap as expression statement
                    let stmt = Statement::Expression(ExpressionStatement { expression: call_expr, span, id });

                    return (stmt, vec![]);
                }
            }
        }

        let (new_place, _) = self.reconstruct_expression(place, &());
        let (new_value, _) = self.reconstruct_expression(value, &());

        (AssignStatement { place: new_place, value: new_value, span, id }.into(), vec![])
    }

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
        (stmt, Default::default())
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

    fn reconstruct_definition(&mut self, mut input: DefinitionStatement) -> (Statement, Self::AdditionalOutput) {
        let (new_value, additional_stmts) = self.reconstruct_expression(input.value, &());

        input.type_ = input.type_.map(|ty| self.reconstruct_type(ty).0);
        input.value = new_value;

        (input.into(), additional_stmts)
    }

    fn reconstruct_expression_statement(&mut self, input: ExpressionStatement) -> (Statement, Self::AdditionalOutput) {
        let (reconstructed_expression, statements) = self.reconstruct_expression(input.expression, &Default::default());
        if !matches!(reconstructed_expression, Expression::Call(_) | Expression::AssociatedFunction(_)) {
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
}
