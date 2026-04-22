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

            if let Some(program) = path.user_program().filter(|p| p.as_symbol() != self.program) {
                p = p.with_user_program(*program);
            }

            p.to_global(Location::new(self.program, vec![sym])).into()
        };

        (make_expr(val), make_expr(len))
    }

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
            input_types: vec![],
            return_types: vec![],
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
            input_types: vec![],
            return_types: vec![],
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
            input_types: vec![],
            return_types: vec![],
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
            input_types: vec![],
            return_types: vec![],
            arguments: vec![path_expr, key_expr, default_expr],
            span,
            id: self.state.node_builder.next_id(),
        }
        .into()
    }

    pub fn ternary_expr(
        &mut self,
        condition: Expression,
        if_true: Expression,
        if_false: Expression,
        span: Span,
    ) -> Expression {
        TernaryExpression { condition, if_true, if_false, span, id: self.state.node_builder.next_id() }.into()
    }

    /// Emits an identifier literal expression (e.g. `'x__'`).
    pub fn literal_identifier(&mut self, name: Symbol) -> Expression {
        Literal::identifier(name.to_string(), Span::default(), self.state.node_builder.next_id()).into()
    }

    /// Emits the default network literal `'aleo'`.
    pub fn literal_default_network(&mut self) -> Expression {
        Literal::identifier("aleo".to_string(), Span::default(), self.state.node_builder.next_id()).into()
    }

    /// Emits `_dynamic_contains(prog, net, mapping, key)`.
    pub fn dynamic_contains_expr(
        &mut self,
        prog: Expression,
        net: Expression,
        mapping: Expression,
        key: Expression,
        span: Span,
    ) -> Expression {
        IntrinsicExpression {
            name: sym::_dynamic_contains,
            type_parameters: vec![],
            input_types: vec![],
            return_types: vec![],
            arguments: vec![prog, net, mapping, key],
            span,
            id: self.state.node_builder.next_id(),
        }
        .into()
    }

    /// Emits `_dynamic_get_or_use::<value_ty>(prog, net, mapping, key, default)`.
    #[allow(clippy::too_many_arguments)]
    pub fn dynamic_get_or_use_expr(
        &mut self,
        prog: Expression,
        net: Expression,
        mapping: Expression,
        key: Expression,
        default: Expression,
        value_ty: Type,
        span: Span,
    ) -> Expression {
        IntrinsicExpression {
            name: sym::_dynamic_get_or_use,
            type_parameters: vec![(value_ty, span)],
            input_types: vec![],
            return_types: vec![],
            arguments: vec![prog, net, mapping, key, default],
            span,
            id: self.state.node_builder.next_id(),
        }
        .into()
    }

    /// Looks up the interface referenced by an interface type expression.
    pub fn lookup_interface_from_type(&self, interface_ty: &Type) -> Interface {
        let Type::Composite(CompositeType { path, .. }) = interface_ty else {
            panic!("Dynamic access requires a composite interface type, got `{interface_ty}`");
        };
        let location = path.try_global_location().expect("interface path must resolve to a global location");
        self.state
            .symbol_table
            .lookup_interface(self.program, location)
            .expect("type checking guarantees the interface exists")
            .clone()
    }

    pub fn binary_expr(&mut self, left: Expression, op: BinaryOperation, right: Expression) -> Expression {
        BinaryExpression { op, left, right, span: Span::default(), id: self.state.node_builder.next_id() }.into()
    }

    /// Lowers `Interface@(target)::storage` (singleton bare read) to a ternary
    /// `contains.dynamic ? get_or_use.dynamic(..) : None` producing `Option<T>`.
    pub fn lower_dynamic_read(
        &mut self,
        interface_ty: Type,
        target_program: Expression,
        network: Option<Expression>,
        storage: Identifier,
        span: Span,
    ) -> (Expression, Vec<Statement>) {
        let interface = self.lookup_interface_from_type(&interface_ty);
        let storage_proto = interface
            .storages
            .iter()
            .find(|s| s.identifier.name == storage.name)
            .cloned()
            .expect("type checking guarantees storage exists in interface");

        let inner_type = match storage_proto.type_ {
            Type::Vector(_) => panic!("vector storage cannot be read as a singleton"),
            t => t,
        };

        let (prog_expr, prog_stmts) = self.reconstruct_expression(target_program, &());
        let (net_expr, net_stmts) = match network {
            Some(n) => self.reconstruct_expression(n, &()),
            None => (self.literal_default_network(), vec![]),
        };

        let mapping_sym = Symbol::intern(&format!("{}__", storage.name));
        let mapping_lit_a = self.literal_identifier(mapping_sym);
        let mapping_lit_b = self.literal_identifier(mapping_sym);
        let false_lit_a = self.literal_false();
        let false_lit_b = self.literal_false();

        let contains_expr =
            self.dynamic_contains_expr(prog_expr.clone(), net_expr.clone(), mapping_lit_a, false_lit_a, span);

        let zero = self.zero(&inner_type);
        let get_or_use_expr =
            self.dynamic_get_or_use_expr(prog_expr, net_expr, mapping_lit_b, false_lit_b, zero, inner_type, span);

        let none_expr: Expression = Literal::none(Span::default(), self.state.node_builder.next_id()).into();
        let ternary = self.ternary_expr(contains_expr, get_or_use_expr, none_expr, span);

        let mut stmts = prog_stmts;
        stmts.extend(net_stmts);
        (ternary, stmts)
    }

    /// Lowers `Interface@(target)::vec.get(i)` to a ternary checking `i < len` and
    /// reading `<base>__[i]` from the backing mapping, producing `Option<element>`.
    pub fn lower_dynamic_vector_get(
        &mut self,
        target_program: Expression,
        network: Option<Expression>,
        member: Identifier,
        element_type: Type,
        arguments: Vec<Expression>,
        span: Span,
    ) -> (Expression, Vec<Statement>) {
        let (prog_expr, prog_stmts) = self.reconstruct_expression(target_program, &());
        let (net_expr, net_stmts) = match network {
            Some(n) => self.reconstruct_expression(n, &()),
            None => (self.literal_default_network(), vec![]),
        };
        let (index_expr, index_stmts) = self
            .reconstruct_expression(arguments.into_iter().next().expect("type checking guarantees one argument"), &());

        let base_name = member.name.to_string();
        let val_mapping_sym = Symbol::intern(&format!("{base_name}__"));
        let len_mapping_sym = Symbol::intern(&format!("{base_name}__len__"));

        let len_mapping_lit = self.literal_identifier(len_mapping_sym);
        let false_lit = self.literal_false();
        let zero_u32 = self.literal_zero_u32();
        let get_len_expr = self.dynamic_get_or_use_expr(
            prog_expr.clone(),
            net_expr.clone(),
            len_mapping_lit,
            false_lit,
            zero_u32,
            Type::Integer(IntegerType::U32),
            span,
        );
        let len_var_sym = self.state.assigner.unique_symbol("$len_var", "$");
        let len_var_ident =
            Identifier { name: len_var_sym, span: Default::default(), id: self.state.node_builder.next_id() };
        let len_stmt =
            self.state.assigner.simple_definition(len_var_ident, get_len_expr, self.state.node_builder.next_id());
        let len_var_expr: Expression = len_var_ident.into();

        let index_lt_len_expr = self.binary_expr(index_expr.clone(), BinaryOperation::Lt, len_var_expr);

        let val_mapping_lit = self.literal_identifier(val_mapping_sym);
        let zero = self.zero(&element_type);
        let get_or_use_expr =
            self.dynamic_get_or_use_expr(prog_expr, net_expr, val_mapping_lit, index_expr, zero, element_type, span);

        let none_expr: Expression = Literal::none(Span::default(), self.state.node_builder.next_id()).into();
        let ternary = self.ternary_expr(index_lt_len_expr, get_or_use_expr, none_expr, span);

        let mut stmts = prog_stmts;
        stmts.extend(net_stmts);
        stmts.extend(index_stmts);
        stmts.push(len_stmt);

        (ternary, stmts)
    }

    /// Lowers `Interface@(target)::vec.len()` to a `_dynamic_get_or_use::<u32>` read of the
    /// backing `<base>__len__` mapping, defaulting to `0u32` when the length has not been set.
    pub fn lower_dynamic_vector_len(
        &mut self,
        target_program: Expression,
        network: Option<Expression>,
        member: Identifier,
        span: Span,
    ) -> (Expression, Vec<Statement>) {
        let (prog_expr, prog_stmts) = self.reconstruct_expression(target_program, &());
        let (net_expr, net_stmts) = match network {
            Some(n) => self.reconstruct_expression(n, &()),
            None => (self.literal_default_network(), vec![]),
        };

        let len_mapping_sym = Symbol::intern(&format!("{}__len__", member.name));
        let len_mapping_lit = self.literal_identifier(len_mapping_sym);
        let false_lit = self.literal_false();
        let zero_u32 = self.literal_zero_u32();
        let expr = self.dynamic_get_or_use_expr(
            prog_expr,
            net_expr,
            len_mapping_lit,
            false_lit,
            zero_u32,
            Type::Integer(IntegerType::U32),
            span,
        );

        let mut stmts = prog_stmts;
        stmts.extend(net_stmts);
        (expr, stmts)
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
                        && user_program.as_symbol() != self.program
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
                    input_types: vec![],
                    return_types: vec![],
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
                    input_types: vec![],
                    return_types: vec![],
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
