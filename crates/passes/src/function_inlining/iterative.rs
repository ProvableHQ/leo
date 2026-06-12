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

//! Lowering for *iterative inlines*: inlined `final fn` bodies whose `return`s sit under live
//! control flow.
//!
//! `final fn` bodies execute iteratively on-chain: their effects cannot be executed
//! speculatively, so flattening leaves their conditionals as real branches instead of collapsing
//! them into ternaries the way it does off-chain. AVM finalize registers are write-once with no
//! merge mechanism, which rules out every lowering that funnels the per-path return values into
//! one place after a join (a single trailing return, a shared result variable, a branch to an
//! exit label). The only sound lowering is tail duplication: each return path binds the value it
//! returns and carries its own copy of what follows the call.
//!
//! Only values are barred from crossing a join — control may rejoin freely. So the duplication
//! is bounded to the continuation's value-dependent prefix, and the independent remainder
//! executes once after the woven region.
//!
//! The lowering spans three hand-offs in the surrounding [`TransformVisitor`]:
//! 1. `reconstruct_call` detects a qualifying body ([`PendingIterativeInline::try_from_body`])
//!    and stashes it, since no single value can replace the call expression.
//! 2. The statement holding the call claims the result ([`PendingIterativeInline::bind_to`] for
//!    a definition; discarded otherwise) and dissolves into a dummy.
//! 3. The enclosing block hands the rest of itself over as the continuation
//!    ([`TransformVisitor::lower_iterative_inline`]).

use crate::{SsaFormingInput, common::SymbolAccessCollector, static_single_assignment::visitor::SsaFormingVisitor};

use super::transform::TransformVisitor;

use indexmap::IndexSet;
use leo_ast::*;
use leo_span::Symbol;

/// How the value of an inlined call is consumed at the callsite.
pub enum InlineBinder {
    /// The call result is bound by a definition: `let x = f(…);`.
    Definition(DefinitionPlace),
    /// The call result is discarded: `f(…);`.
    Discard,
}

/// An inlined callee body awaiting the lowering described in the module docs.
pub struct PendingIterativeInline {
    statements: Vec<Statement>,
    binder: InlineBinder,
}

impl PendingIterativeInline {
    /// A freshly inlined body qualifies when some `return` sits under live control flow — i.e.
    /// anywhere but as the trailing statement. Plain bodies are handed back to the caller, whose
    /// simple value substitution remains correct for them.
    pub fn try_from_body(statements: Vec<Statement>) -> Result<Self, Vec<Statement>> {
        let under_live_control_flow = match statements.last() {
            Some(Statement::Return(_)) => &statements[..statements.len() - 1],
            _ => &statements[..],
        };
        if statements_contain_return(under_live_control_flow) {
            Ok(Self { statements, binder: InlineBinder::Discard })
        } else {
            Err(statements)
        }
    }

    /// Claims the call result for a definition place; the lowering binds it at every return site.
    pub fn bind_to(&mut self, place: DefinitionPlace) {
        self.binder = InlineBinder::Definition(place);
    }
}

impl TransformVisitor<'_> {
    /// Entry point of the lowering: replaces the callsite statement and everything after it in
    /// the enclosing block.
    pub(super) fn lower_iterative_inline(
        &mut self,
        inline: PendingIterativeInline,
        continuation: Vec<Statement>,
    ) -> Vec<Statement> {
        let (prefix, suffix) = self.split_continuation(&inline.binder, continuation);
        let mut out = self.weave(inline.statements, &inline.binder, &prefix);
        // Control rejoins here: the suffix references nothing defined past the callsite.
        out.extend(suffix);
        out
    }

    /// Tail duplication itself: after this rewrite, every path through the callee body ends by
    /// binding the value it returns and running its own copy of `continuation`.
    fn weave(
        &mut self,
        statements: Vec<Statement>,
        binder: &InlineBinder,
        continuation: &[Statement],
    ) -> Vec<Statement> {
        let mut out = Vec::new();
        let mut iter = statements.into_iter();
        while let Some(statement) = iter.next() {
            match statement {
                Statement::Return(ret) => {
                    // Anything after a `return` in the same block is unreachable.
                    out.extend(self.bind_and_continue(ret.expression, binder, continuation));
                    return out;
                }
                Statement::Block(block) if statements_contain_return(&block.statements) => {
                    // Post-SSA, names are unique across the function, so block scopes carry no
                    // meaning and the nested block can take over the rest of this one.
                    let mut merged = block.statements;
                    merged.extend(iter);
                    out.extend(self.weave(merged, binder, continuation));
                    return out;
                }
                Statement::Conditional(conditional) if conditional_contains_return(&conditional) => {
                    // Control may not rejoin after a conditional that returns, so each branch
                    // takes over the rest of the block.
                    let rest: Vec<Statement> = iter.collect();

                    let mut then_statements = conditional.then.statements;
                    then_statements.extend(self.fresh_clone(rest.clone()));

                    let mut else_statements = match conditional.otherwise.map(|s| *s) {
                        Some(Statement::Block(block)) => block.statements,
                        Some(other) => vec![other],
                        None => Vec::new(),
                    };
                    else_statements.extend(rest);

                    let then = Block {
                        statements: self.weave(then_statements, binder, continuation),
                        span: conditional.then.span,
                        id: conditional.then.id,
                    };
                    let otherwise = Block {
                        statements: self.weave(else_statements, binder, continuation),
                        span: Default::default(),
                        id: self.state.node_builder.next_id(),
                    };
                    out.push(
                        ConditionalStatement {
                            condition: conditional.condition,
                            then,
                            otherwise: Some(Box::new(otherwise.into())),
                            span: conditional.span,
                            id: conditional.id,
                        }
                        .into(),
                    );
                    return out;
                }
                other => out.push(other),
            }
        }
        // A body that falls through its end returns unit implicitly.
        let id = self.state.node_builder.next_id();
        self.state.type_table.insert(id, Type::Unit);
        out.extend(self.bind_and_continue(
            UnitExpression { span: Default::default(), id }.into(),
            binder,
            continuation,
        ));
        out
    }

    /// Materializes one return site: the binding of the returned value, then the caller's
    /// continuation. The whole fragment is cloned fresh because sibling sites live in disjoint
    /// branches, yet downstream passes assume definition names are unique per function.
    fn bind_and_continue(
        &mut self,
        value: Expression,
        binder: &InlineBinder,
        continuation: &[Statement],
    ) -> Vec<Statement> {
        let mut fragment = Vec::with_capacity(continuation.len() + 1);
        match binder {
            InlineBinder::Discard => {}
            InlineBinder::Definition(place) => match (place, value) {
                // Tuple literals bound to multiple places are segmented into per-element
                // definitions, like any other inlined definition.
                (DefinitionPlace::Multiple(left), Expression::Tuple(right)) => {
                    assert_eq!(left.len(), right.elements.len());
                    for (identifier, rhs_value) in left.iter().zip(right.elements) {
                        fragment.push(
                            DefinitionStatement {
                                place: DefinitionPlace::Single(*identifier),
                                type_: None,
                                value: rhs_value,
                                span: Default::default(),
                                id: self.state.node_builder.next_id(),
                            }
                            .into(),
                        );
                    }
                }
                (place, value) => fragment.push(
                    DefinitionStatement {
                        place: place.clone(),
                        type_: None,
                        value,
                        span: Default::default(),
                        id: self.state.node_builder.next_id(),
                    }
                    .into(),
                ),
            },
        }
        fragment.extend(continuation.to_vec());
        self.fresh_clone(fragment)
    }

    /// Splits a continuation into the prefix that must be duplicated into every return path and
    /// the suffix that may execute once after the woven region.
    ///
    /// Duplication is forced for statements that transitively depend on the bound return value.
    /// It then extends by def-closure: every name defined inside the prefix is branch-local (and
    /// freshly renamed) after cloning, so a later statement reading one cannot be shared either —
    /// at the AVM level that read would target a register the taken path never wrote. The suffix
    /// that remains references only names defined before the callsite, which is exactly what
    /// makes rejoining control after the weave sound.
    fn split_continuation(
        &mut self,
        binder: &InlineBinder,
        continuation: Vec<Statement>,
    ) -> (Vec<Statement>, Vec<Statement>) {
        let mut dup_names: IndexSet<Symbol> = match binder {
            InlineBinder::Discard => IndexSet::new(),
            InlineBinder::Definition(DefinitionPlace::Single(identifier)) => IndexSet::from([identifier.name]),
            InlineBinder::Definition(DefinitionPlace::Multiple(identifiers)) => {
                identifiers.iter().map(|i| i.name).collect()
            }
        };

        let uses: Vec<IndexSet<Symbol>> = continuation.iter().map(|s| self.local_uses(s)).collect();
        let defs: Vec<Vec<Symbol>> = continuation.iter().map(defs_of).collect();

        // Forward scan suffices for transitive dependence: post-SSA, definitions precede uses.
        let mut cut: Option<usize> = None;
        for (i, statement_uses) in uses.iter().enumerate() {
            if !statement_uses.is_disjoint(&dup_names) {
                cut = Some(i);
                dup_names.extend(defs[i].iter().copied());
            }
        }

        // Def-closure fixpoint; each round extends the cut by at least one statement.
        while let Some(c) = cut {
            let prefix_defs: IndexSet<Symbol> = defs[..=c].iter().flatten().copied().collect();
            match uses.iter().enumerate().skip(c + 1).find(|(_, u)| !u.is_disjoint(&prefix_defs)) {
                Some((i, _)) => cut = Some(i),
                None => break,
            }
        }

        match cut {
            None => (Vec::new(), continuation),
            Some(c) => {
                let mut prefix = continuation;
                let suffix = prefix.split_off(c + 1);
                (prefix, suffix)
            }
        }
    }

    /// Collects the local variable names a statement reads, recursively.
    fn local_uses(&mut self, statement: &Statement) -> IndexSet<Symbol> {
        let mut collector = SymbolAccessCollector::new(self.state);
        collector.visit_statement(statement);
        collector
            .symbol_accesses
            .iter()
            .filter(|(path, _)| path.try_global_location().is_none())
            .map(|(path, _)| path.identifier().name)
            .collect()
    }

    /// Clones a sequence of statements, renaming its definitions so the clone can live alongside
    /// sibling copies in disjoint branches without colliding in downstream SSA-based passes.
    fn fresh_clone(&mut self, statements: Vec<Statement>) -> Vec<Statement> {
        if statements.is_empty() {
            return Vec::new();
        }
        let block = Block { statements, span: Default::default(), id: self.state.node_builder.next_id() };
        SsaFormingVisitor::new(self.state, SsaFormingInput { rename_defs: true }, self.program).consume_block(block)
    }
}

/// Collects the names a statement defines, recursively. Definitions are the only def sites at
/// this stage — assignments no longer exist post-SSA.
fn defs_of(statement: &Statement) -> Vec<Symbol> {
    match statement {
        Statement::Definition(def) => match &def.place {
            DefinitionPlace::Single(identifier) => vec![identifier.name],
            DefinitionPlace::Multiple(identifiers) => identifiers.iter().map(|i| i.name).collect(),
        },
        Statement::Block(block) => block.statements.iter().flat_map(defs_of).collect(),
        Statement::Conditional(conditional) => {
            let mut defs: Vec<Symbol> = conditional.then.statements.iter().flat_map(defs_of).collect();
            if let Some(otherwise) = &conditional.otherwise {
                defs.extend(defs_of(otherwise));
            }
            defs
        }
        _ => Vec::new(),
    }
}

/// Returns `true` if any of the statements is or contains a `ReturnStatement`.
fn statements_contain_return(statements: &[Statement]) -> bool {
    statements.iter().any(statement_contains_return)
}

fn statement_contains_return(statement: &Statement) -> bool {
    match statement {
        Statement::Return(_) => true,
        Statement::Block(block) => statements_contain_return(&block.statements),
        Statement::Conditional(conditional) => conditional_contains_return(conditional),
        _ => false,
    }
}

fn conditional_contains_return(conditional: &ConditionalStatement) -> bool {
    statements_contain_return(&conditional.then.statements)
        || conditional.otherwise.as_ref().is_some_and(|s| statement_contains_return(s))
}
