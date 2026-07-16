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

use super::composite_sra::{AtomFields, atom_fields_from_alias, atom_fields_from_composite, atom_for_member, is_atom};

use crate::{CompilerState, option_lowering::is_optional_struct_symbol};

use leo_ast::*;
use leo_errors::Result;
use leo_span::Symbol;

use indexmap::{IndexMap, IndexSet};

pub(super) fn run(state: &mut CompilerState) -> Result<()> {
    for _ in 0..1024 {
        let ast = std::mem::take(&mut state.ast);
        let mut visitor = LateOptionalUnwrapVisitor {
            state,
            program: Symbol::intern(""),
            optional_fields: Default::default(),
            aliases: Default::default(),
            optional_selections: Default::default(),
            atom_ternaries: Default::default(),
            composites: Default::default(),
            changed: false,
        };

        let ast = ast.map(
            |program| visitor.reconstruct_program(program),
            |library| library, // no-op for libraries
        );

        visitor.state.handler.last_err()?;
        visitor.state.ast = ast;

        if !visitor.changed {
            return Ok(());
        }
    }
    panic!("ran out of loops");
}

struct LateOptionalUnwrapVisitor<'a> {
    state: &'a mut CompilerState,
    program: Symbol,
    /// Optional wrapper locals and the atom assigned to each wrapper field.
    optional_fields: IndexMap<Symbol, AtomFields>,
    /// Local atom copies, used only to preserve producer identity.
    aliases: IndexMap<Symbol, Expression>,
    /// Exact `(is_some, val)` SSA paths captured from lowered Optional wrappers.
    optional_selections: IndexSet<(Symbol, Symbol)>,
    /// Atom-only ternary locals and their condition and fallback arms.
    atom_ternaries: IndexMap<Symbol, (Expression, Expression)>,
    /// Struct definitions visible in the current program, including wrappers
    /// introduced after the symbol table was built.
    composites: IndexMap<Location, Composite>,
    changed: bool,
}

impl LateOptionalUnwrapVisitor<'_> {
    fn clear_tracked_values(&mut self) {
        self.optional_fields.clear();
        self.aliases.clear();
        self.optional_selections.clear();
        self.atom_ternaries.clear();
    }

    fn is_optional_wrapper_type(&self, ty: &Type) -> bool {
        let Type::Composite(composite_ty) = ty else {
            return false;
        };
        let location = composite_ty.path.expect_global_location();
        // Option lowering creates wrappers after the symbol table was built, so
        // the current AST definition is authoritative when both sources contain it.
        let Some(composite) =
            self.composites.get(location).or_else(|| self.state.symbol_table.lookup_struct(self.program, location))
        else {
            return false;
        };
        let [is_some, val] = composite.members.as_slice() else {
            return false;
        };
        !composite.is_record
            && is_optional_struct_symbol(composite.identifier.name, &val.type_)
            && is_some.identifier.name == Symbol::intern("is_some")
            && matches!(is_some.type_, Type::Boolean)
            && val.identifier.name == Symbol::intern("val")
    }

    fn is_optional_composite_expression(&self, composite: &CompositeExpression) -> bool {
        self.state.type_table.get(&composite.id()).is_some_and(|ty| self.is_optional_wrapper_type(&ty))
    }

    /// Resolve atom copies to their producer for metadata comparisons. Final
    /// SSA gives each local one definition, and these maps are per-function.
    fn resolve_atom<'a>(&'a self, mut atom: &'a Expression) -> &'a Expression {
        for _ in 0..self.aliases.len() {
            let Expression::Path(path) = atom else {
                break;
            };
            let Some(name) = path.try_local_symbol() else {
                break;
            };
            let Some(source) = self.aliases.get(&name) else {
                break;
            };
            if matches!(source, Expression::Path(path) if path.try_local_symbol() == Some(name)) {
                break;
            }
            atom = source;
        }
        atom
    }

    fn resolved_local_symbol(&self, atom: &Expression) -> Option<Symbol> {
        let Expression::Path(path) = self.resolve_atom(atom) else {
            return None;
        };
        path.try_local_symbol()
    }

    fn same_atom_value(&self, a: &Expression, b: &Expression) -> bool {
        match (self.resolve_atom(a), self.resolve_atom(b)) {
            (Expression::Literal(a), Expression::Literal(b)) => a.variant == b.variant,
            (Expression::Path(a), Expression::Path(b)) => {
                let a = a.try_local_symbol();
                let b = b.try_local_symbol();
                a == b && a.is_some()
            }
            _ => false,
        }
    }

    fn track_optional_fields(&mut self, owner: Symbol, fields: AtomFields) {
        if let (Some(is_some), Some(val)) = (fields.get(&Symbol::intern("is_some")), fields.get(&Symbol::intern("val")))
            && let (Some(is_some), Some(val)) = (self.resolved_local_symbol(is_some), self.resolved_local_symbol(val))
        {
            self.optional_selections.insert((is_some, val));
        }
        self.optional_fields.insert(owner, fields);
    }
}

impl AstReconstructor for LateOptionalUnwrapVisitor<'_> {
    type AdditionalInput = ();
    type AdditionalOutput = ();

    fn reconstruct_member_access(&mut self, mut input: MemberAccess, _additional: &()) -> (Expression, ()) {
        input.inner = self.reconstruct_expression(input.inner, &()).0;

        if let Some(atom) = atom_for_member(&self.optional_fields, &input.inner, input.name.name) {
            self.changed = true;
            return (atom, ());
        }

        (input.into(), ())
    }

    fn reconstruct_ternary(&mut self, input: TernaryExpression, _additional: &()) -> (Expression, ()) {
        let condition = self.reconstruct_expression(input.condition, &()).0;
        let if_true = self.reconstruct_expression(input.if_true, &()).0;
        let if_false = self.reconstruct_expression(input.if_false, &()).0;

        if let (Some(condition_name), Some(value_name)) =
            (self.resolved_local_symbol(&condition), self.resolved_local_symbol(&if_true))
            && self.optional_selections.contains(&(condition_name, value_name))
            && let Some((inner_condition, inner_false)) = self.atom_ternaries.get(&value_name)
            && self.same_atom_value(&condition, inner_condition)
            && self.same_atom_value(&if_false, inner_false)
        {
            // The Optional value has already been selected into `if_true` with
            // the same condition and fallback, so the second selection is equal.
            self.changed = true;
            return (if_true, ());
        }

        (TernaryExpression { condition, if_true, if_false, ..input }.into(), ())
    }

    fn reconstruct_definition(&mut self, mut input: DefinitionStatement) -> (Statement, ()) {
        let value = self.reconstruct_expression(input.value, &()).0;

        if let DefinitionPlace::Single(identifier) = &input.place {
            let fields = match &value {
                Expression::Composite(composite)
                    if composite.base.is_none() && self.is_optional_composite_expression(composite) =>
                {
                    atom_fields_from_composite(composite).filter(|fields| {
                        fields.len() == 2
                            && fields.contains_key(&Symbol::intern("is_some"))
                            && fields.contains_key(&Symbol::intern("val"))
                    })
                }
                Expression::Cast(cast) if self.is_optional_wrapper_type(&cast.type_) => {
                    if let Expression::Tuple(tuple) = &cast.expression
                        && let [is_some, val] = tuple.elements.as_slice()
                        && is_atom(is_some)
                        && is_atom(val)
                    {
                        Some(IndexMap::from([
                            (Symbol::intern("is_some"), is_some.clone()),
                            (Symbol::intern("val"), val.clone()),
                        ]))
                    } else {
                        None
                    }
                }
                Expression::Path(_) => atom_fields_from_alias(&value, &self.optional_fields),
                _ => None,
            };

            if let Some(fields) = fields {
                self.track_optional_fields(identifier.name, fields);
            }
        }

        if let DefinitionPlace::Single(identifier) = &input.place
            && is_atom(&value)
        {
            self.aliases.insert(identifier.name, self.resolve_atom(&value).clone());
        }

        if let (DefinitionPlace::Single(identifier), Expression::Ternary(ternary)) = (&input.place, &value)
            && is_atom(&ternary.condition)
            && is_atom(&ternary.if_true)
            && is_atom(&ternary.if_false)
        {
            self.atom_ternaries.insert(identifier.name, (ternary.condition.clone(), ternary.if_false.clone()));
        }

        input.value = value;
        (input.into(), ())
    }

    fn reconstruct_assign(&mut self, _input: AssignStatement) -> (Statement, ()) {
        panic!("there should be no assignments at this stage");
    }
}

impl UnitReconstructor for LateOptionalUnwrapVisitor<'_> {
    fn reconstruct_library(&mut self, input: Library) -> Library {
        // Library functions have already been inlined into the consuming program.
        input
    }

    fn reconstruct_program(&mut self, input: Program) -> Program {
        let stubs = input.stubs.into_iter().map(|(id, stub)| (id, self.reconstruct_stub(stub))).collect();
        let program_scopes = input
            .program_scopes
            .into_iter()
            .map(|(id, scope)| {
                self.program = scope.program_id.as_symbol();
                self.composites = scope
                    .composites
                    .iter()
                    .map(|(_, composite)| {
                        (Location::new(self.program, vec![composite.identifier.name]), composite.clone())
                    })
                    .collect();
                (id, self.reconstruct_program_scope(scope))
            })
            .collect();
        let modules = input.modules.into_iter().map(|(id, module)| (id, self.reconstruct_module(module))).collect();

        Program { modules, imports: input.imports, stubs, program_scopes }
    }

    fn reconstruct_function(&mut self, mut input: Function) -> Function {
        if !input.variant.is_finalize_context() {
            return input;
        }

        self.clear_tracked_values();
        input.block = self.reconstruct_block(input.block).0;
        self.clear_tracked_values();
        input
    }

    fn reconstruct_constructor(&mut self, input: Constructor) -> Constructor {
        input
    }
}
