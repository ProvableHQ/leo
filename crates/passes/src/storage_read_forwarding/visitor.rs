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

use leo_ast::{Expression, Identifier, IntrinsicExpression, LiteralVariant, Location, Node as _, Path};
use leo_span::{Symbol, sym};

use indexmap::IndexMap;

#[derive(Clone, Eq, PartialEq, Hash)]
pub(super) enum Atom {
    Local(Symbol),
    Global(Location),
    Literal(LiteralVariant),
}

#[derive(Eq, PartialEq, Hash)]
pub(super) enum StorageRead {
    Get { mapping: Atom, key: Atom },
    GetOrUse { mapping: Atom, key: Atom, default: Atom },
    Contains { mapping: Atom, key: Atom },
}

pub struct StorageReadForwardingVisitor<'a> {
    pub state: &'a mut CompilerState,
    pub(super) reads: IndexMap<StorageRead, Symbol>,
    pub(super) aliases: IndexMap<Symbol, Symbol>,
    pub(super) then_join_aliases: IndexMap<Symbol, Symbol>,
    pub(super) otherwise_join_aliases: IndexMap<Symbol, Symbol>,
    pub(super) join_condition: Option<Expression>,
    pub(super) in_finalize_context: bool,
}

impl StorageReadForwardingVisitor<'_> {
    pub(super) fn clear_reads(&mut self) {
        self.reads.clear();
    }

    pub(super) fn clear_function_state(&mut self) {
        self.reads.clear();
        self.aliases.clear();
        self.clear_join_aliases();
    }

    pub(super) fn clear_join_aliases(&mut self) {
        self.then_join_aliases.clear();
        self.otherwise_join_aliases.clear();
        self.join_condition = None;
    }

    pub(super) fn local_alias(&self, name: Symbol) -> Option<Symbol> {
        let mut current = name;
        for _ in 0..=self.aliases.len() {
            let Some(next) = self.aliases.get(&current).copied() else {
                return (current != name).then_some(current);
            };
            if next == current {
                return Some(current);
            }
            current = next;
        }
        None
    }

    pub(super) fn canonical_local(&self, name: Symbol) -> Symbol {
        self.local_alias(name).unwrap_or(name)
    }

    pub(super) fn insert_alias(&mut self, alias: Symbol, target: Symbol) {
        let target = self.canonical_local(target);
        if alias != target {
            self.aliases.insert(alias, target);
        }
    }

    pub(super) fn same_join_condition(&self, condition: &Expression) -> bool {
        let Some(join_condition) = &self.join_condition else {
            return false;
        };

        match (condition, join_condition) {
            (Expression::Path(left), Expression::Path(right)) => {
                let left = left.try_local_symbol().map(|name| self.canonical_local(name));
                let right = right.try_local_symbol().map(|name| self.canonical_local(name));
                left == right && left.is_some()
            }
            (Expression::Literal(left), Expression::Literal(right)) => left.variant == right.variant,
            _ => false,
        }
    }

    pub(super) fn is_matching_join_ternary(&self, expression: &Expression) -> bool {
        matches!(expression, Expression::Ternary(ternary) if self.same_join_condition(&ternary.condition))
    }

    pub(super) fn atom(&self, expr: &Expression) -> Option<Atom> {
        match expr {
            Expression::Literal(lit) => Some(Atom::Literal(lit.variant.clone())),
            Expression::Path(path) => path
                .try_local_symbol()
                .map(|name| Atom::Local(self.canonical_local(name)))
                .or_else(|| path.try_global_location().cloned().map(Atom::Global)),
            _ => None,
        }
    }

    pub(super) fn storage_read(&self, intrinsic: &IntrinsicExpression) -> Option<StorageRead> {
        match intrinsic.name {
            sym::_mapping_get => Some(StorageRead::Get {
                mapping: self.atom(intrinsic.arguments.first()?)?,
                key: self.atom(intrinsic.arguments.get(1)?)?,
            }),
            sym::_mapping_get_or_use => Some(StorageRead::GetOrUse {
                mapping: self.atom(intrinsic.arguments.first()?)?,
                key: self.atom(intrinsic.arguments.get(1)?)?,
                default: self.atom(intrinsic.arguments.get(2)?)?,
            }),
            sym::_mapping_contains => Some(StorageRead::Contains {
                mapping: self.atom(intrinsic.arguments.first()?)?,
                key: self.atom(intrinsic.arguments.get(1)?)?,
            }),
            _ => None,
        }
    }

    pub(super) fn local_expression_like(&mut self, symbol: Symbol, old_value: &Expression) -> Expression {
        let ty = self.state.type_table.get(&old_value.id());
        let path = Path::from(Identifier::new(symbol, self.state.node_builder.next_id())).to_local();
        if let Some(ty) = ty {
            self.state.type_table.insert(path.id(), ty);
        }
        path.into()
    }

    pub(super) fn is_effect_boundary(intrinsic: &IntrinsicExpression) -> bool {
        matches!(intrinsic.name, sym::_mapping_set | sym::_mapping_remove | sym::_final_run)
    }
}
