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

//! CEI (Checks-Effects-Interactions) ordering analysis.
//!
//! Within a single execution path through a finalize context, all reads and
//! writes of *mutable persistent state* must precede any interactions (calls
//! to `Final::run()`). An interaction cedes control to another program's
//! finalize, which may modify state that a subsequent read or write depends
//! on — a reentrancy risk.
//!
//! The analysis is a single forward walk over each finalize context. It
//! threads one piece of state: `post: Option<Span>`, the source location of
//! the earliest interaction reachable on some execution path to here.
//! Every read or write encountered while `post.is_some()` produces a
//! warning; branching statements merge post-states with union semantics.
//!
//! Function-call summaries are computed lazily and memoized. Only callees
//! actually reached from a finalize context are summarized.
//!
//! ## What counts as mutable persistent state
//!
//! Only operations that touch state a rival program's finalize could
//! observe or modify are Reads/Writes. Immutable-within-a-transaction
//! reads (`self.address`, `block.height`, …) and pure computations
//! (hashes, SNARK verify, arithmetic) are Pure — freely usable after an
//! interaction. The [`classify_intrinsic`] match is the single source of
//! truth for this categorization and is exhaustive over `Intrinsic`.

use crate::{CompilerState, SymbolTable, VariableType, errors::cei_analyzer};

use leo_ast::*;
use leo_errors::Formatted;
use leo_span::{Span, Symbol};

use indexmap::{IndexMap, IndexSet};

// ---------------------------------------------------------------------------
// Classifier — the single VM-facing surface of this analysis.

/// The CEI category of an operation.
///
/// `Read` and `Write` refer specifically to *mutable persistent state*
/// (mappings, vectors, storage variables, dynamic external storage) that a
/// rival program's finalize could observe or alter. Immutable environment
/// queries and pure computations are `None`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Op {
    /// A read of mutable persistent state.
    Read,
    /// A write to mutable persistent state.
    Write,
    /// Yields control to another program's finalize.
    Interaction,
}

/// Classify an intrinsic. Exhaustive `match` — a new `Intrinsic` variant is a
/// compile error until it is categorized here.
fn classify_intrinsic(i: &Intrinsic) -> Option<Op> {
    use Intrinsic::*;
    match i {
        // Mutable-state reads
        MappingGet | MappingGetOrUse | MappingContains | VectorGet | VectorLen | DynamicContains | DynamicGet
        | DynamicGetOrUse => Some(Op::Read),

        // Mutable-state writes
        MappingSet | MappingRemove | VectorSet | VectorPush | VectorPop | VectorClear | VectorSwapRemove => {
            Some(Op::Write)
        }

        // Interactions
        FinalRun => Some(Op::Interaction),

        // Immutable-within-a-transaction environment queries: a rival
        // finalize cannot change these, so a late read carries no
        // reentrancy risk.
        BlockHeight | BlockTimestamp | NetworkId | SelfProgramOwner | SelfAddress | SelfCaller | SelfChecksum
        | SelfEdition | SelfId | SelfSigner | ProgramOwner | ProgramChecksum | ProgramEdition | FunctionChecksum => {
            None
        }

        // SNARK verifications compute a boolean from arguments. If a
        // verifying key comes from a mapping, that mapping read fires
        // on its own.
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

        // Transition-only. Unreachable in a finalize context; earlier
        // passes reject it there.
        DynamicCall => None,
    }
}

/// A plain storage variable — not a mapping and not a vector, which are
/// only ever accessed through intrinsics.
fn is_storage_var(sym: &SymbolTable, prog: Symbol, p: &Path) -> bool {
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

/// Peel wrappers on an assignment LHS to find the root `Path` (the write
/// target). Returns `None` if the root is not a `Path`.
fn peel_assign_root(expr: &Expression) -> Option<&Path> {
    match expr {
        Expression::Path(p) => Some(p),
        Expression::MemberAccess(a) => peel_assign_root(&a.inner),
        Expression::TupleAccess(a) => peel_assign_root(&a.tuple),
        Expression::ArrayAccess(a) => peel_assign_root(&a.array),
        _ => None,
    }
}

// ---------------------------------------------------------------------------
// Effect summaries

/// What CEI operations a callee transitively performs.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
struct Summary {
    reads: bool,
    writes: bool,
    interacts: bool,
}

impl Summary {
    fn merge(&mut self, other: Summary) {
        self.reads |= other.reads;
        self.writes |= other.writes;
        self.interacts |= other.interacts;
    }
}

// ---------------------------------------------------------------------------
// Scanner

/// The CEI ordering scanner.
///
/// One instance is created per pass invocation and walks every finalize
/// context in the AST. Its two mutable pieces of state are:
///
/// - `summaries`: lazily populated as `summary_of` is called.
/// - `warned`: dedup set so a single violation site never emits twice.
/// - `suppress`: set to `true` inside a loop body that has already fired
///   the CEI03 loop-level warning, so we don't repeat the same root cause
///   as per-statement warnings.
struct Scanner<'a> {
    state: &'a mut CompilerState,
    program: Symbol,
    summaries: IndexMap<Location, Summary>,
    warned: IndexSet<(Span, i32)>,
    suppress: bool,
}

// CEI warning codes (must match `errors/cei_analyzer.rs`).
const CODE_CHECK: i32 = 13000;
const CODE_EFFECT: i32 = 13001;
const CODE_CALLEE: i32 = 13002;
const CODE_LOOP: i32 = 13003;

impl<'a> Scanner<'a> {
    fn new(state: &'a mut CompilerState) -> Self {
        Self {
            state,
            program: Symbol::intern(""),
            summaries: IndexMap::new(),
            warned: IndexSet::new(),
            suppress: false,
        }
    }

    /// Merge two branch post-states (union). Prefer the first argument's
    /// span (source-order stability: then-branch before else-branch).
    fn merge(a: Option<Span>, b: Option<Span>) -> Option<Span> {
        a.or(b)
    }

    /// Emit a warning, deduped by (span, code) and gated by `suppress`.
    fn emit(&mut self, span: Span, code: i32, w: Formatted) {
        if !self.suppress && self.warned.insert((span, code)) {
            self.state.handler.emit_warning(w);
        }
    }

    /// Apply an operation's effect to the post-state.
    ///
    /// - Interaction: advance post-state (unless already set).
    /// - Read/Write: if post-state is set, emit a warning at `span`.
    fn apply(&mut self, op: Op, span: Span, post: Option<Span>, desc: &str) -> Option<Span> {
        match op {
            Op::Interaction => post.or(Some(span)),
            Op::Read => {
                if post.is_some() {
                    self.emit(span, CODE_CHECK, cei_analyzer::check_after_interaction(desc, span));
                }
                post
            }
            Op::Write => {
                if post.is_some() {
                    self.emit(span, CODE_EFFECT, cei_analyzer::effect_after_interaction(desc, span));
                }
                post
            }
        }
    }

    /// Apply a callee summary at a call site.
    fn apply_summary(&mut self, s: Summary, callee: &Path, span: Span, post: Option<Span>) -> Option<Span> {
        if post.is_some() && (s.reads || s.writes) {
            self.emit(span, CODE_CALLEE, cei_analyzer::callee_has_effects_after_interaction(callee, span));
        }
        if s.interacts { post.or(Some(span)) } else { post }
    }

    // -----------------------------------------------------------------
    // Summaries — order-insensitive walk that returns a `Summary`.

    /// Get or compute a callee's summary. Off-chain callees (regular `fn`)
    /// and unresolved paths return the empty summary.
    fn summary_of(&mut self, callee: &Path) -> Summary {
        let Some(loc) = callee.try_global_location() else { return Summary::default() };
        if let Some(s) = self.summaries.get(loc) {
            return *s;
        }
        let loc = loc.clone();
        // Seed with default so any self-reference terminates. Recursion is
        // rejected by earlier passes; this is defensive.
        self.summaries.insert(loc.clone(), Summary::default());
        let Some(func) = self.state.symbol_table.lookup_function(loc.program, &loc) else {
            return Summary::default();
        };
        if !func.function.variant.is_onchain() {
            // Regular helper `fn`s can't touch state or interact.
            return Summary::default();
        }
        let variant = func.function.variant;
        let block = func.function.block.clone();
        let s = if block.statements.is_empty() {
            // Callee is a stub. Its body isn't visible as Leo AST, so we
            // can't summarize it. Fall back to a conservative summary based on
            // the variant, so callee-has-effects warnings still fire against
            // externals.
            match variant {
                Variant::View => Summary { reads: true, writes: false, interacts: false },
                Variant::FinalFn | Variant::Finalize => Summary { reads: true, writes: true, interacts: true },
                Variant::Fn | Variant::EntryPoint => Summary::default(),
            }
        } else {
            let prog = loc.program;
            self.summarize_block(&block, prog)
        };
        self.summaries.insert(loc, s);
        s
    }

    fn summarize_block(&mut self, b: &Block, prog: Symbol) -> Summary {
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
                    && is_storage_var(&self.state.symbol_table, prog, root)
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
                if is_storage_var(&self.state.symbol_table, prog, p) {
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
            }
            Expression::Repeat(r) => {
                self.summarize_expr(&r.expr, prog, s);
                self.summarize_expr(&r.count, prog, s);
            }
            Expression::Async(a) => s.merge(self.summarize_block(&a.block, prog)),
            Expression::Literal(_) | Expression::Unit(_) | Expression::Err(_) => {}
        }
    }

    // -----------------------------------------------------------------
    // Path-sensitive scan of a finalize context.

    fn scan_block(&mut self, b: &Block, mut post: Option<Span>) -> Option<Span> {
        for stmt in &b.statements {
            post = self.scan_stmt(stmt, post);
        }
        post
    }

    fn scan_stmt(&mut self, s: &Statement, post: Option<Span>) -> Option<Span> {
        match s {
            // FP2: `assert` is not itself a Read. Only its sub-expressions count.
            Statement::Assert(a) => self.scan_assert(a, post),
            Statement::Assign(a) => self.scan_assign(a, post),
            Statement::Block(b) => self.scan_block(b, post),
            Statement::Conditional(c) => {
                let post = self.scan_expr(&c.condition, post);
                let t = self.scan_block(&c.then, post);
                let e = if let Some(o) = &c.otherwise { self.scan_stmt(o, post) } else { post };
                Self::merge(t, e)
            }
            Statement::Const(d) => self.scan_expr(&d.value, post),
            Statement::Definition(d) => self.scan_expr(&d.value, post),
            Statement::Expression(e) => self.scan_expr(&e.expression, post),
            Statement::Iteration(it) => self.scan_loop(it, post),
            Statement::Return(r) => self.scan_expr(&r.expression, post),
        }
    }

    fn scan_assert(&mut self, a: &AssertStatement, post: Option<Span>) -> Option<Span> {
        match &a.variant {
            AssertVariant::Assert(e) => self.scan_expr(e, post),
            AssertVariant::AssertEq(l, r) | AssertVariant::AssertNeq(l, r) => {
                let post = self.scan_expr(l, post);
                self.scan_expr(r, post)
            }
        }
    }

    fn scan_assign(&mut self, a: &AssignStatement, post: Option<Span>) -> Option<Span> {
        let post = self.scan_lhs_indices(&a.place, post);
        let post = self.scan_expr(&a.value, post);
        if let Some(root) = peel_assign_root(&a.place)
            && is_storage_var(&self.state.symbol_table, self.program, root)
        {
            self.apply(Op::Write, a.span, post, "a storage variable write")
        } else {
            post
        }
    }

    fn scan_lhs_indices(&mut self, expr: &Expression, post: Option<Span>) -> Option<Span> {
        match expr {
            Expression::Path(_) => post,
            Expression::MemberAccess(a) => self.scan_lhs_indices(&a.inner, post),
            Expression::TupleAccess(a) => self.scan_lhs_indices(&a.tuple, post),
            Expression::ArrayAccess(a) => {
                let post = self.scan_lhs_indices(&a.array, post);
                self.scan_expr(&a.index, post)
            }
            _ => post,
        }
    }

    fn scan_loop(&mut self, it: &IterationStatement, post: Option<Span>) -> Option<Span> {
        let post = self.scan_expr(&it.start, post);
        let post = self.scan_expr(&it.stop, post);

        // Iteration i's non-interactions come after iteration i-1's
        // interaction, so a body containing both violates CEI.
        let body_summary = {
            let mut s = Summary::default();
            let prog = self.program;
            let stmts = it.block.statements.clone();
            for st in &stmts {
                self.summarize_stmt(st, prog, &mut s);
            }
            s
        };
        let mixed = body_summary.interacts && (body_summary.reads || body_summary.writes);
        if mixed {
            let span = it.variable.span();
            self.emit(span, CODE_LOOP, cei_analyzer::cei_violation_in_loop(span));
        }

        // FP4: suppress per-statement warnings inside a body that has
        // already fired the loop-level warning.
        let new_suppress = mixed || self.suppress;
        let saved = std::mem::replace(&mut self.suppress, new_suppress);
        let after = self.scan_block(&it.block, post);
        self.suppress = saved;

        // If the body performs an interaction, downstream code is post.
        if body_summary.interacts { after.or(Some(it.span)) } else { after }
    }

    fn scan_expr(&mut self, e: &Expression, post: Option<Span>) -> Option<Span> {
        match e {
            Expression::Intrinsic(i) => {
                // Arguments evaluate before the intrinsic.
                let post = self.scan_args(&i.arguments, post);
                if let Some(intr) = Intrinsic::from_symbol(i.name, &i.type_parameters)
                    && let Some(op) = classify_intrinsic(&intr)
                {
                    let desc = format!("a call to `{}`", i.name);
                    self.apply(op, i.span, post, &desc)
                } else {
                    post
                }
            }
            Expression::Call(c) => {
                let post = self.scan_args(&c.arguments, post);
                let s = self.summary_of(&c.function);
                self.apply_summary(s, &c.function, c.span, post)
            }
            Expression::DynamicOp(d) => self.scan_dynamic_op(d, post),
            // FP5: merge branches with the same `post` snapshot.
            Expression::Ternary(t) => {
                let post = self.scan_expr(&t.condition, post);
                let a = self.scan_expr(&t.if_true, post);
                let b = self.scan_expr(&t.if_false, post);
                Self::merge(a, b)
            }
            Expression::Binary(b) => {
                let post = self.scan_expr(&b.left, post);
                self.scan_expr(&b.right, post)
            }
            Expression::Unary(u) => self.scan_expr(&u.receiver, post),
            Expression::Cast(c) => self.scan_expr(&c.expression, post),
            Expression::Tuple(t) => self.scan_seq(&t.elements, post),
            Expression::Array(a) => self.scan_seq(&a.elements, post),
            Expression::ArrayAccess(a) => {
                let post = self.scan_expr(&a.array, post);
                self.scan_expr(&a.index, post)
            }
            Expression::MemberAccess(a) => self.scan_expr(&a.inner, post),
            Expression::TupleAccess(a) => self.scan_expr(&a.tuple, post),
            Expression::Composite(c) => {
                let mut post = post;
                for m in &c.members {
                    if let Some(e) = &m.expression {
                        post = self.scan_expr(e, post);
                    }
                }
                post
            }
            Expression::Repeat(r) => {
                let post = self.scan_expr(&r.expr, post);
                self.scan_expr(&r.count, post)
            }
            // An inline `async { ... }` inside a finalize context shouldn't
            // arise (they live only in EntryPoint bodies), but if the AST
            // grows to allow it, treat it as a fresh finalize sub-context.
            Expression::Async(a) => {
                let _ = self.scan_block(&a.block, None);
                post
            }
            Expression::Path(p) => {
                if is_storage_var(&self.state.symbol_table, self.program, p) {
                    self.apply(Op::Read, p.span, post, "a storage variable read")
                } else {
                    post
                }
            }
            Expression::Literal(_) | Expression::Unit(_) | Expression::Err(_) => post,
        }
    }

    fn scan_args(&mut self, args: &[Expression], mut post: Option<Span>) -> Option<Span> {
        for a in args {
            post = self.scan_expr(a, post);
        }
        post
    }

    fn scan_seq(&mut self, xs: &[Expression], mut post: Option<Span>) -> Option<Span> {
        for x in xs {
            post = self.scan_expr(x, post);
        }
        post
    }

    fn scan_dynamic_op(&mut self, d: &DynamicOpExpression, post: Option<Span>) -> Option<Span> {
        let post = self.scan_expr(&d.target_program, post);
        let post = if let Some(n) = &d.network { self.scan_expr(n, post) } else { post };
        match &d.kind {
            DynamicOpKind::Call { arguments, .. } => {
                // Transition-only; not expected in finalize, but visit args.
                self.scan_args(arguments, post)
            }
            DynamicOpKind::Read { storage } => {
                let desc = format!("a read of dynamic storage `{storage}`");
                self.apply(Op::Read, d.span, post, &desc)
            }
            DynamicOpKind::Op { op, arguments, .. } => {
                let post = self.scan_args(arguments, post);
                let desc = format!("a `{op}` call on dynamic storage");
                self.apply(Op::Read, d.span, post, &desc)
            }
        }
    }

    // -----------------------------------------------------------------
    // EntryPoint driver: locate inline `async { ... }` blocks and scan
    // each as a fresh finalize context. Non-async code in the transition
    // body is proof-context and outside CEI's remit.

    fn scan_entrypoint(&mut self, b: &Block) {
        for s in &b.statements {
            self.scan_entrypoint_stmt(s);
        }
    }

    fn scan_entrypoint_stmt(&mut self, s: &Statement) {
        match s {
            Statement::Assert(a) => match &a.variant {
                AssertVariant::Assert(e) => self.scan_entrypoint_expr(e),
                AssertVariant::AssertEq(l, r) | AssertVariant::AssertNeq(l, r) => {
                    self.scan_entrypoint_expr(l);
                    self.scan_entrypoint_expr(r);
                }
            },
            Statement::Assign(a) => {
                self.scan_entrypoint_expr(&a.place);
                self.scan_entrypoint_expr(&a.value);
            }
            Statement::Block(b) => self.scan_entrypoint(b),
            Statement::Conditional(c) => {
                self.scan_entrypoint_expr(&c.condition);
                self.scan_entrypoint(&c.then);
                if let Some(o) = &c.otherwise {
                    self.scan_entrypoint_stmt(o);
                }
            }
            Statement::Const(d) => self.scan_entrypoint_expr(&d.value),
            Statement::Definition(d) => self.scan_entrypoint_expr(&d.value),
            Statement::Expression(e) => self.scan_entrypoint_expr(&e.expression),
            Statement::Iteration(it) => {
                self.scan_entrypoint_expr(&it.start);
                self.scan_entrypoint_expr(&it.stop);
                self.scan_entrypoint(&it.block);
            }
            Statement::Return(r) => self.scan_entrypoint_expr(&r.expression),
        }
    }

    fn scan_entrypoint_expr(&mut self, e: &Expression) {
        match e {
            Expression::Async(a) => {
                let _ = self.scan_block(&a.block, None);
            }
            Expression::Intrinsic(i) => {
                for arg in &i.arguments {
                    self.scan_entrypoint_expr(arg);
                }
            }
            Expression::Call(c) => {
                for arg in &c.arguments {
                    self.scan_entrypoint_expr(arg);
                }
            }
            Expression::DynamicOp(d) => {
                self.scan_entrypoint_expr(&d.target_program);
                if let Some(n) = &d.network {
                    self.scan_entrypoint_expr(n);
                }
                match &d.kind {
                    DynamicOpKind::Call { arguments, .. } | DynamicOpKind::Op { arguments, .. } => {
                        for arg in arguments {
                            self.scan_entrypoint_expr(arg);
                        }
                    }
                    DynamicOpKind::Read { .. } => {}
                }
            }
            Expression::Binary(b) => {
                self.scan_entrypoint_expr(&b.left);
                self.scan_entrypoint_expr(&b.right);
            }
            Expression::Unary(u) => self.scan_entrypoint_expr(&u.receiver),
            Expression::Ternary(t) => {
                self.scan_entrypoint_expr(&t.condition);
                self.scan_entrypoint_expr(&t.if_true);
                self.scan_entrypoint_expr(&t.if_false);
            }
            Expression::Cast(c) => self.scan_entrypoint_expr(&c.expression),
            Expression::Tuple(t) => {
                for e in &t.elements {
                    self.scan_entrypoint_expr(e);
                }
            }
            Expression::Array(a) => {
                for e in &a.elements {
                    self.scan_entrypoint_expr(e);
                }
            }
            Expression::ArrayAccess(a) => {
                self.scan_entrypoint_expr(&a.array);
                self.scan_entrypoint_expr(&a.index);
            }
            Expression::MemberAccess(a) => self.scan_entrypoint_expr(&a.inner),
            Expression::TupleAccess(a) => self.scan_entrypoint_expr(&a.tuple),
            Expression::Composite(c) => {
                for m in &c.members {
                    if let Some(e) = &m.expression {
                        self.scan_entrypoint_expr(e);
                    }
                }
            }
            Expression::Repeat(r) => {
                self.scan_entrypoint_expr(&r.expr);
                self.scan_entrypoint_expr(&r.count);
            }
            Expression::Path(_) | Expression::Literal(_) | Expression::Unit(_) | Expression::Err(_) => {}
        }
    }
}

// ---------------------------------------------------------------------------
// UnitVisitor plumbing — the driver's only job is to identify each
// finalize-context function and dispatch into `scan_block`, and each
// EntryPoint into `scan_entrypoint`.

impl AstVisitor for Scanner<'_> {
    type AdditionalInput = ();
    type Output = ();
}

impl UnitVisitor for Scanner<'_> {
    fn visit_program_scope(&mut self, input: &ProgramScope) {
        let saved = self.program;
        self.program = input.program_id.as_symbol();
        for (_, f) in input.functions.iter() {
            self.visit_function(f);
        }
        self.program = saved;
    }

    fn visit_function(&mut self, input: &Function) {
        if input.variant.is_finalize_context() {
            let _ = self.scan_block(&input.block, None);
        } else if input.variant == Variant::EntryPoint {
            self.scan_entrypoint(&input.block);
        }
        // Regular helpers (`Fn`) and `View` are analyzed via `summary_of`
        // when called from a finalize context.
    }
}

// ---------------------------------------------------------------------------
// Entry point

pub fn run(state: &mut CompilerState) {
    let ast = std::mem::take(&mut state.ast);
    let mut scanner = Scanner::new(state);
    match &ast {
        Ast::Program(p) => scanner.visit_program(p),
        Ast::Library(l) => scanner.visit_library(l),
    }
    scanner.state.ast = ast;
}
