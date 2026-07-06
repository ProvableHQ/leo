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

use super::{SymbolTable, VariableType};

use leo_ast::*;
use leo_span::Symbol;

use indexmap::IndexMap;

/// The effect of an operation relevant to on-chain execution.
///
/// Reads and writes refer specifically to mutable persistent state that
/// another program's finalizer could observe or alter.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Op {
    /// A read of mutable persistent state.
    Read,
    /// A write to mutable persistent state.
    Write,
    /// Yields control to another program's finalizer.
    Interaction,
}

/// Classify an intrinsic. Exhaustive `match` — a new `Intrinsic` variant is a
/// compile error until it is categorized here.
pub(crate) fn classify_intrinsic(i: &Intrinsic) -> Option<Op> {
    use Intrinsic::*;
    match i {
        // Mutable-state reads.
        MappingGet | MappingGetOrUse | MappingContains | VectorGet | VectorLen | DynamicContains | DynamicGet
        | DynamicGetOrUse => Some(Op::Read),

        // Mutable-state writes.
        MappingSet | MappingRemove | VectorSet | VectorPush | VectorPop | VectorClear | VectorSwapRemove => {
            Some(Op::Write)
        }

        // Interactions.
        FinalRun => Some(Op::Interaction),

        // Immutable-within-a-transaction environment queries.
        BlockHeight | BlockTimestamp | NetworkId | SelfProgramOwner | SelfAddress | SelfCaller | SelfChecksum
        | SelfEdition | SelfId | SelfSigner | ProgramOwner | ProgramChecksum | ProgramEdition | FunctionChecksum => {
            None
        }

        // Pure verification operations.
        SnarkVerify | SnarkVerifyBatch => None,

        // Pure computations.
        ChaChaRand(_)
        | Commit(_, _)
        | ECDSAVerify(_)
        | Hash(_, _)
        | OptionalUnwrap
        | OptionalUnwrapOr
        | GroupToXCoordinate
        | GroupToYCoordinate
        | GroupGen
        | AleoGenerator
        | AleoGeneratorPowers
        | SignatureVerify
        | Serialize(_)
        | Deserialize(_, _) => None,

        // Transition-only.
        DynamicCall => None,
    }
}

/// A plain storage variable — not a mapping and not a vector, which are
/// only ever accessed through intrinsics.
pub(crate) fn is_storage_var(sym: &SymbolTable, prog: Symbol, p: &Path) -> bool {
    if let Some(loc) = p.try_global_location()
        && let Some(var) = sym.lookup_global(prog, loc)
        && var.declaration == VariableType::Storage
    {
        if let Some(ty) = &var.type_ {
            return !ty.is_mapping() && !ty.is_vector();
        }
        return true;
    }
    false
}

/// Peel wrappers on an assignment LHS to find the root `Path`, if any.
pub(crate) fn peel_assign_root(expr: &Expression) -> Option<&Path> {
    match expr {
        Expression::Path(p) => Some(p),
        Expression::MemberAccess(a) => peel_assign_root(&a.inner),
        Expression::TupleAccess(a) => peel_assign_root(&a.tuple),
        Expression::ArrayAccess(a) => peel_assign_root(&a.array),
        _ => None,
    }
}

/// The effects a function transitively performs.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(crate) struct Summary {
    pub(crate) reads: bool,
    pub(crate) writes: bool,
    pub(crate) interacts: bool,
}

impl Summary {
    pub(crate) fn merge(&mut self, other: Summary) {
        self.reads |= other.reads;
        self.writes |= other.writes;
        self.interacts |= other.interacts;
    }
}

pub(crate) fn function_writes_state(symbol_table: &SymbolTable, function: &Path) -> bool {
    Summarizer::new(symbol_table, &mut IndexMap::new()).summary_of(function).writes
}

/// Computes function summaries with an order-insensitive walk, borrowing
/// function bodies directly from the symbol table.
pub(crate) struct Summarizer<'a> {
    sym: &'a SymbolTable,
    summaries: &'a mut IndexMap<Location, Summary>,
}

impl<'a> Summarizer<'a> {
    pub(crate) fn new(sym: &'a SymbolTable, summaries: &'a mut IndexMap<Location, Summary>) -> Self {
        Self { sym, summaries }
    }

    /// Get or compute a callee's summary. Off-chain callees (regular `fn`)
    /// and unresolved paths return the empty summary.
    pub(crate) fn summary_of(&mut self, callee: &Path) -> Summary {
        let sym = self.sym;
        let Some(loc) = callee.try_global_location() else { return Summary::default() };
        if let Some(s) = self.summaries.get(loc) {
            return *s;
        }
        let loc = loc.clone();
        // Seed the cache so self-references terminate. Recursion is rejected
        // by an earlier pass.
        self.summaries.insert(loc.clone(), Summary::default());
        let Some(func) = sym.lookup_function(loc.program, &loc) else {
            return Summary::default();
        };
        if !func.function.variant.is_onchain() {
            return Summary::default();
        }
        let variant = func.function.variant;
        let s = if func.is_stub {
            // The Leo body of an external stub may be unavailable, so use a
            // conservative summary based on its variant.
            match variant {
                Variant::View => Summary { reads: true, writes: false, interacts: false },
                Variant::FinalFn | Variant::Finalize => Summary { reads: true, writes: true, interacts: true },
                Variant::Fn | Variant::EntryPoint => Summary::default(),
            }
        } else {
            self.summarize_block(&func.function.block, loc.program)
        };
        self.summaries.insert(loc, s);
        s
    }

    pub(crate) fn summarize_block(&mut self, b: &Block, prog: Symbol) -> Summary {
        let mut s = Summary::default();
        for stmt in &b.statements {
            self.summarize_stmt(stmt, prog, &mut s);
        }
        s
    }

    fn summarize_stmt(&mut self, stmt: &Statement, prog: Symbol, s: &mut Summary) {
        match stmt {
            Statement::Assert(a) => match &a.variant {
                AssertVariant::Assert(e) => self.summarize_expr(e, prog, s),
                AssertVariant::AssertEq(l, r) | AssertVariant::AssertNeq(l, r) => {
                    self.summarize_expr(l, prog, s);
                    self.summarize_expr(r, prog, s);
                }
            },
            Statement::Assign(a) => {
                if let Some(root) = peel_assign_root(&a.place)
                    && is_storage_var(self.sym, prog, root)
                {
                    s.writes = true;
                }
                self.summarize_lhs_indices(&a.place, prog, s);
                self.summarize_expr(&a.value, prog, s);
            }
            Statement::Block(b) => s.merge(self.summarize_block(b, prog)),
            Statement::Conditional(c) => {
                self.summarize_expr(&c.condition, prog, s);
                s.merge(self.summarize_block(&c.then, prog));
                if let Some(o) = &c.otherwise {
                    self.summarize_stmt(o, prog, s);
                }
            }
            Statement::Const(d) => self.summarize_expr(&d.value, prog, s),
            Statement::Definition(d) => self.summarize_expr(&d.value, prog, s),
            Statement::Expression(e) => self.summarize_expr(&e.expression, prog, s),
            Statement::Iteration(it) => {
                self.summarize_expr(&it.start, prog, s);
                self.summarize_expr(&it.stop, prog, s);
                s.merge(self.summarize_block(&it.block, prog));
            }
            Statement::Return(r) => self.summarize_expr(&r.expression, prog, s),
        }
    }

    fn summarize_lhs_indices(&mut self, expr: &Expression, prog: Symbol, s: &mut Summary) {
        match expr {
            Expression::Path(_) => {}
            Expression::MemberAccess(a) => self.summarize_lhs_indices(&a.inner, prog, s),
            Expression::TupleAccess(a) => self.summarize_lhs_indices(&a.tuple, prog, s),
            Expression::ArrayAccess(a) => {
                self.summarize_lhs_indices(&a.array, prog, s);
                self.summarize_expr(&a.index, prog, s);
            }
            _ => {}
        }
    }

    fn summarize_expr(&mut self, e: &Expression, prog: Symbol, s: &mut Summary) {
        match e {
            Expression::Intrinsic(i) => {
                for arg in &i.arguments {
                    self.summarize_expr(arg, prog, s);
                }
                if let Some(intr) = Intrinsic::from_symbol(i.name, &i.type_parameters)
                    && let Some(op) = classify_intrinsic(&intr)
                {
                    match op {
                        Op::Read => s.reads = true,
                        Op::Write => s.writes = true,
                        Op::Interaction => s.interacts = true,
                    }
                }
            }
            Expression::Call(c) => {
                for arg in &c.arguments {
                    self.summarize_expr(arg, prog, s);
                }
                let cs = self.summary_of(&c.function);
                s.merge(cs);
            }
            Expression::DynamicOp(d) => {
                self.summarize_expr(&d.target_program, prog, s);
                if let Some(n) = &d.network {
                    self.summarize_expr(n, prog, s);
                }
                match &d.kind {
                    DynamicOpKind::Call { arguments, .. } => {
                        for arg in arguments {
                            self.summarize_expr(arg, prog, s);
                        }
                    }
                    DynamicOpKind::Read { .. } => s.reads = true,
                    DynamicOpKind::Op { arguments, .. } => {
                        s.reads = true;
                        for arg in arguments {
                            self.summarize_expr(arg, prog, s);
                        }
                    }
                }
            }
            Expression::Path(p) => {
                if is_storage_var(self.sym, prog, p) {
                    s.reads = true;
                }
            }
            Expression::Binary(b) => {
                self.summarize_expr(&b.left, prog, s);
                self.summarize_expr(&b.right, prog, s);
            }
            Expression::Unary(u) => self.summarize_expr(&u.receiver, prog, s),
            Expression::Ternary(t) => {
                self.summarize_expr(&t.condition, prog, s);
                self.summarize_expr(&t.if_true, prog, s);
                self.summarize_expr(&t.if_false, prog, s);
            }
            Expression::Cast(c) => self.summarize_expr(&c.expression, prog, s),
            Expression::Tuple(t) => {
                for e in &t.elements {
                    self.summarize_expr(e, prog, s);
                }
            }
            Expression::Array(a) => {
                for e in &a.elements {
                    self.summarize_expr(e, prog, s);
                }
            }
            Expression::ArrayAccess(a) => {
                self.summarize_expr(&a.array, prog, s);
                self.summarize_expr(&a.index, prog, s);
            }
            Expression::MemberAccess(a) => self.summarize_expr(&a.inner, prog, s),
            Expression::TupleAccess(a) => self.summarize_expr(&a.tuple, prog, s),
            Expression::Composite(c) => {
                for m in &c.members {
                    if let Some(e) = &m.expression {
                        self.summarize_expr(e, prog, s);
                    }
                }
                if let Some(base) = &c.base {
                    self.summarize_expr(base, prog, s);
                }
            }
            Expression::Repeat(r) => {
                self.summarize_expr(&r.expr, prog, s);
                self.summarize_expr(&r.count, prog, s);
            }
            Expression::Async(a) => s.merge(self.summarize_block(&a.block, prog)),
            Expression::Literal(_) | Expression::Unit(_) | Expression::Err(_) => {}
        }
    }
}
