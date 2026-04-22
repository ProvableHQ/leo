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
use leo_span::{Span, Symbol};

use indexmap::IndexMap;

pub struct OptionLoweringVisitor<'a> {
    pub state: &'a mut CompilerState,
    // The name of the current program scope
    pub program: Symbol,
    // The path to the current module. This should be an empty vector for the root.
    pub module: Vec<Symbol>,
    // The name of the current function, if we're inside one.
    pub function: Option<Symbol>,
    // All composites visible to this pass, keyed by owning unit and path. Holds user-defined
    // composites (registered during phase 1) and generated Optional wrapper structs (inserted on
    // demand by `insert_optional_wrapper_struct`, always at path length 1).
    pub composites: IndexMap<Location, Composite>,
}

impl OptionLoweringVisitor<'_> {
    /// Enter module scope with path `module`, execute `func`, and then return to the parent module.
    pub fn in_module_scope<T>(&mut self, module: &[Symbol], func: impl FnOnce(&mut Self) -> T) -> T {
        let parent_module = self.module.clone();
        self.module = module.to_vec();
        let result = func(self);
        self.module = parent_module;
        result
    }

    /// Wraps an expression of a given type in an `Optional<T>`-like struct representing `Some(value)`.
    ///
    /// This function creates a struct expression that encodes an optional value with `is_some = true`
    /// and the provided expression as the `val` field. It also ensures that the type is fully
    /// reconstructed, which guarantees that all necessary struct definitions are available and registered.
    ///
    /// # Parameters
    /// - `expr`: The expression to wrap as the value of the optional.
    /// - `ty`: The type of the expression.
    ///
    /// # Returns
    /// - An `Expression` representing the constructed `Optional<T>` struct instance.
    pub fn wrap_optional_value(&mut self, expr: Expression, ty: Type) -> Expression {
        let is_some_expr = Expression::Literal(Literal {
            span: Span::default(),
            id: self.state.node_builder.next_id(),
            variant: LiteralVariant::Boolean(true),
        });

        // Fully lower the type before proceeding. This also ensures that all required structs
        // are actually registered.
        let lowered_inner_type = self.reconstruct_type(ty).0;

        // Create or get an optional wrapper struct for `lowered_inner_type`
        let struct_name = self.insert_optional_wrapper_struct(&lowered_inner_type);

        let struct_expr = CompositeExpression {
            path: Path::from(Identifier::new(struct_name, self.state.node_builder.next_id()))
                .to_global(Location::new(self.program, vec![struct_name])),
            const_arguments: vec![],
            members: vec![
                CompositeFieldInitializer {
                    identifier: Identifier::new(Symbol::intern("is_some"), self.state.node_builder.next_id()),
                    expression: Some(is_some_expr),
                    span: Span::default(),
                    id: self.state.node_builder.next_id(),
                },
                CompositeFieldInitializer {
                    identifier: Identifier::new(Symbol::intern("val"), self.state.node_builder.next_id()),
                    expression: Some(expr),
                    span: Span::default(),
                    id: self.state.node_builder.next_id(),
                },
            ],
            span: Span::default(),
            id: self.state.node_builder.next_id(),
        };

        struct_expr.into()
    }

    /// Constructs an `Optional<T>`-like struct representing `None` for a given inner type.
    ///
    /// The returned struct expression sets `is_some = false`, and provides a zero value for the `val`
    /// field, where "zero" is defined according to the type:
    /// numeric types use literal zero, structs are recursively zero-initialized, etc.
    ///
    /// This function assumes that all required struct types are already reconstructed and registered.
    ///
    /// # Parameters
    /// - `inner_ty`: The type `T` inside the `Optional<T>`.
    ///
    /// # Returns
    /// - An `Expression` representing the constructed `Optional<T>` struct instance with `None`.
    pub fn wrap_none(&mut self, inner_ty: &Type) -> Expression {
        let is_some_expr = Expression::Literal(Literal {
            span: Span::default(),
            id: self.state.node_builder.next_id(),
            variant: LiteralVariant::Boolean(false),
        });

        // Fully lower the type before proceeding. This also ensures that all required composites
        // are actually registered.
        let lowered_inner_type = self.reconstruct_type(inner_ty.clone()).0;

        // Even though the `val` field of the struct will not be used as long as `is_some` is
        // `false`, we still have to set it to something. We choose "zero", whatever "zero" means
        // for each type.

        // The symbol table is not updated during this pass, so resolve composite members through
        // the pass-local registry populated by phase 1.
        let composites = &self.composites;
        let struct_lookup = |loc: &Location| {
            composites
                .get(loc)
                .unwrap_or_else(|| {
                    panic!(
                        "no composite registered for {loc:?}; phase-1 collect_composites_* must register every referenced composite"
                    )
                })
                .members
                .iter()
                .map(|mem| (mem.identifier.name, mem.type_.clone()))
                .collect()
        };

        let zero_val_expr =
            Expression::zero(&lowered_inner_type, Span::default(), &self.state.node_builder, &struct_lookup)
                .expect("this must work if type checking was successful");

        // Create or get an optional wrapper struct for `lowered_inner_type`
        let struct_name = self.insert_optional_wrapper_struct(&lowered_inner_type);

        let struct_expr = CompositeExpression {
            path: Path::from(Identifier::new(struct_name, self.state.node_builder.next_id()))
                .to_global(Location::new(self.program, vec![struct_name])),
            const_arguments: vec![],
            members: vec![
                CompositeFieldInitializer {
                    identifier: Identifier::new(Symbol::intern("is_some"), self.state.node_builder.next_id()),
                    expression: Some(is_some_expr.clone()),
                    span: Span::default(),
                    id: self.state.node_builder.next_id(),
                },
                CompositeFieldInitializer {
                    identifier: Identifier::new(Symbol::intern("val"), self.state.node_builder.next_id()),
                    expression: Some(zero_val_expr.clone()),
                    span: Span::default(),
                    id: self.state.node_builder.next_id(),
                },
            ],
            span: Span::default(),
            id: self.state.node_builder.next_id(),
        };

        struct_expr.into()
    }

    /// Binds `expr` to a fresh temporary and returns both the `let` statement and a
    /// `Path` expression that references the temporary.
    pub fn bind_optional_receiver(&mut self, expr: Expression) -> (Statement, Expression) {
        let name = self.state.assigner.unique_symbol("$opt", "$");
        let ident = Identifier::new(name, self.state.node_builder.next_id());
        let stmt = self.state.assigner.simple_definition(ident, expr, self.state.node_builder.next_id());
        (stmt, Path::from(ident).to_local().into())
    }

    /// Inserts (or reuses) a compiler-generated struct representing `Optional<T>`.
    ///
    /// The struct has two fields:
    /// - `is_some: bool` — indicates whether the value is present.
    /// - `val: T` — the wrapped value.
    ///
    /// If the struct for this type already exists, it’s reused; otherwise, a new one is created.
    /// Returns the `Symbol` for the struct name.
    pub fn insert_optional_wrapper_struct(&mut self, ty: &Type) -> Symbol {
        let struct_name = crate::make_optional_struct_symbol(ty);

        self.composites.entry(Location::new(self.program, vec![struct_name])).or_insert_with(|| Composite {
            identifier: Identifier::new(struct_name, self.state.node_builder.next_id()),
            const_parameters: vec![], // this is not a generic struct
            members: vec![
                Member {
                    mode: Mode::None,
                    identifier: Identifier::new(Symbol::intern("is_some"), self.state.node_builder.next_id()),
                    type_: Type::Boolean,
                    span: Span::default(),
                    id: self.state.node_builder.next_id(),
                },
                Member {
                    mode: Mode::None,
                    identifier: Identifier::new(Symbol::intern("val"), self.state.node_builder.next_id()),
                    type_: ty.clone(),
                    span: Span::default(),
                    id: self.state.node_builder.next_id(),
                },
            ],
            is_record: false,
            span: Span::default(),
            id: self.state.node_builder.next_id(),
        });

        struct_name
    }
}
