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

//! Peephole optimization pass on generated Aleo bytecode instructions.
//!
//! This pass runs after code generation and before serialization. It applies
//! local optimizations on the flat instruction lists (`Vec<AleoStmt>`):
//!
//! - **Identity operation folding**: eliminates `add x 0`, `mul x 1`, `or x false`, etc.
//! - **Trivial assert elimination**: removes `assert.eq <lit> <lit>` when both sides are equal,
//!   preserving the dummy assert required for empty closures/finalizes.
//! - **Dead register elimination**: removes pure instructions whose destination register is never read.
//! - **Consecutive cast folding**: merges adjacent casts through the same intermediate register.
//! - **Register renumbering**: compacts register indices after instruction removal.

use crate::{AleoExpr, AleoReg, AleoStmt, AleoType, CompiledPrograms, GeneratedPrograms, Pass};

use leo_errors::Result;

use std::collections::{HashMap, HashSet};

pub struct PeepholeOptimizing;

impl Pass for PeepholeOptimizing {
    type Input = GeneratedPrograms;
    type Output = CompiledPrograms;

    const NAME: &str = "PeepholeOptimizing";

    fn do_pass(mut input: Self::Input, _state: &mut crate::CompilerState) -> Result<Self::Output> {
        input.for_each_statement_list(|stmts, num_inputs| {
            fold_identity_operations(stmts);
            eliminate_trivial_asserts(stmts);
            eliminate_dead_registers(stmts);
            fold_consecutive_casts(stmts);
            renumber_registers(stmts, num_inputs);
        });
        Ok(input.into_compiled())
    }
}

// Helpers for literal classification

fn is_zero(expr: &AleoExpr) -> bool {
    matches!(
        expr,
        AleoExpr::U8(0)
            | AleoExpr::U16(0)
            | AleoExpr::U32(0)
            | AleoExpr::U64(0)
            | AleoExpr::U128(0)
            | AleoExpr::I8(0)
            | AleoExpr::I16(0)
            | AleoExpr::I32(0)
            | AleoExpr::I64(0)
            | AleoExpr::I128(0)
    ) || matches!(expr, AleoExpr::Field(s) | AleoExpr::Group(s) | AleoExpr::Scalar(s) if s == "0")
}

fn is_one(expr: &AleoExpr) -> bool {
    matches!(
        expr,
        AleoExpr::U8(1)
            | AleoExpr::U16(1)
            | AleoExpr::U32(1)
            | AleoExpr::U64(1)
            | AleoExpr::U128(1)
            | AleoExpr::I8(1)
            | AleoExpr::I16(1)
            | AleoExpr::I32(1)
            | AleoExpr::I64(1)
            | AleoExpr::I128(1)
    ) || matches!(expr, AleoExpr::Field(s) | AleoExpr::Scalar(s) if s == "1")
}

fn is_false(expr: &AleoExpr) -> bool {
    matches!(expr, AleoExpr::Bool(false))
}

fn is_true(expr: &AleoExpr) -> bool {
    matches!(expr, AleoExpr::Bool(true))
}

/// Returns true if the expression is a literal (not a register or composite expression).
fn is_literal(expr: &AleoExpr) -> bool {
    !matches!(
        expr,
        AleoExpr::Reg(_)
            | AleoExpr::Tuple(_)
            | AleoExpr::ArrayAccess(_, _)
            | AleoExpr::MemberAccess(_, _)
            | AleoExpr::RawName(_)
    )
}

// Identity operation folding

/// If `stmt` is an identity operation, returns references to (dest_register, aliased_expr).
fn try_identity(stmt: &AleoStmt) -> Option<(&AleoReg, &AleoExpr)> {
    match stmt {
        // add x 0 / add 0 x
        AleoStmt::Add(a, b, dst) | AleoStmt::AddWrapped(a, b, dst) => {
            if is_zero(b) {
                Some((dst, a))
            } else if is_zero(a) {
                Some((dst, b))
            } else {
                None
            }
        }
        // sub x 0 (NOT sub 0 x, that's negation)
        AleoStmt::Sub(a, b, dst) | AleoStmt::SubWrapped(a, b, dst) => {
            if is_zero(b) {
                Some((dst, a))
            } else {
                None
            }
        }
        // mul x 1 / mul 1 x
        AleoStmt::Mul(a, b, dst) | AleoStmt::MulWrapped(a, b, dst) => {
            if is_one(b) {
                Some((dst, a))
            } else if is_one(a) {
                Some((dst, b))
            } else {
                None
            }
        }
        // or x false / or false x
        AleoStmt::Or(a, b, dst) => {
            if is_false(b) {
                Some((dst, a))
            } else if is_false(a) {
                Some((dst, b))
            } else {
                None
            }
        }
        // and x true / and true x
        AleoStmt::And(a, b, dst) => {
            if is_true(b) {
                Some((dst, a))
            } else if is_true(a) {
                Some((dst, b))
            } else {
                None
            }
        }
        // xor x false / xor false x
        AleoStmt::Xor(a, b, dst) => {
            if is_false(b) {
                Some((dst, a))
            } else if is_false(a) {
                Some((dst, b))
            } else {
                None
            }
        }
        _ => None,
    }
}

/// Strength-reduce a statement if possible, returning a simpler equivalent statement.
/// - `nand(x, true)` / `nand(true, x)` → `not(x)`
/// - `nor(x, false)` / `nor(false, x)` → `not(x)`
fn try_strength_reduce(stmt: &AleoStmt) -> Option<AleoStmt> {
    match stmt {
        AleoStmt::Nand(a, b, dst) => {
            if is_true(b) {
                Some(AleoStmt::Not(a.clone(), dst.clone()))
            } else if is_true(a) {
                Some(AleoStmt::Not(b.clone(), dst.clone()))
            } else {
                None
            }
        }
        AleoStmt::Nor(a, b, dst) => {
            if is_false(b) {
                Some(AleoStmt::Not(a.clone(), dst.clone()))
            } else if is_false(a) {
                Some(AleoStmt::Not(b.clone(), dst.clone()))
            } else {
                None
            }
        }
        _ => None,
    }
}

/// Resolve an expression through the alias map, following chains transitively.
/// Also recurses into composite expressions (Tuple, ArrayAccess, MemberAccess)
/// to resolve any nested register references.
fn resolve_alias(expr: &AleoExpr, aliases: &HashMap<AleoReg, AleoExpr>) -> AleoExpr {
    match expr {
        AleoExpr::Reg(reg) => {
            if let Some(target) = aliases.get(reg) {
                // Follow alias chains transitively.
                resolve_alias(target, aliases)
            } else {
                expr.clone()
            }
        }
        AleoExpr::Tuple(exprs) => AleoExpr::Tuple(exprs.iter().map(|e| resolve_alias(e, aliases)).collect()),
        AleoExpr::ArrayAccess(a, b) => {
            AleoExpr::ArrayAccess(Box::new(resolve_alias(a, aliases)), Box::new(resolve_alias(b, aliases)))
        }
        AleoExpr::MemberAccess(a, member) => {
            AleoExpr::MemberAccess(Box::new(resolve_alias(a, aliases)), member.clone())
        }
        _ => expr.clone(), // literals and names
    }
}

/// Apply alias substitution to every register operand in a statement.
fn substitute_aliases_in_stmt(stmt: &mut AleoStmt, aliases: &HashMap<AleoReg, AleoExpr>) {
    // Helper: substitute in a single expression.
    fn sub(expr: &mut AleoExpr, aliases: &HashMap<AleoReg, AleoExpr>) {
        let resolved = resolve_alias(expr, aliases);
        if resolved != *expr {
            *expr = resolved;
        }
    }

    match stmt {
        AleoStmt::Output(e, _, _) => sub(e, aliases),
        AleoStmt::AssertEq(a, b) | AleoStmt::AssertNeq(a, b) => {
            sub(a, aliases);
            sub(b, aliases);
        }
        AleoStmt::Cast(e, _, _) => sub(e, aliases),
        AleoStmt::Abs(e, _)
        | AleoStmt::AbsW(e, _)
        | AleoStmt::Double(e, _)
        | AleoStmt::Inv(e, _)
        | AleoStmt::Not(e, _)
        | AleoStmt::Neg(e, _)
        | AleoStmt::Square(e, _)
        | AleoStmt::Sqrt(e, _) => sub(e, aliases),
        AleoStmt::Ternary(a, b, c, _) => {
            sub(a, aliases);
            sub(b, aliases);
            sub(c, aliases);
        }
        AleoStmt::Commit(_, a, b, _, _) => {
            sub(a, aliases);
            sub(b, aliases);
        }
        AleoStmt::Hash(_, a, _, _) => sub(a, aliases),
        AleoStmt::Get(a, b, _) | AleoStmt::Contains(a, b, _) => {
            sub(a, aliases);
            sub(b, aliases);
        }
        AleoStmt::GetOrUse(a, b, c, _) | AleoStmt::Set(a, b, c) => {
            sub(a, aliases);
            sub(b, aliases);
            sub(c, aliases);
        }
        AleoStmt::Remove(a, b) => {
            sub(a, aliases);
            sub(b, aliases);
        }
        AleoStmt::RandChacha(_, _) => {}
        AleoStmt::SignVerify(a, b, c, _) => {
            sub(a, aliases);
            sub(b, aliases);
            sub(c, aliases);
        }
        AleoStmt::EcdsaVerify(_, a, b, c, _) => {
            sub(a, aliases);
            sub(b, aliases);
            sub(c, aliases);
        }
        AleoStmt::SnarkVerify(_, a, b, c, d, _) => {
            sub(a, aliases);
            sub(b, aliases);
            sub(c, aliases);
            sub(d, aliases);
        }
        AleoStmt::Await(e) => sub(e, aliases),
        AleoStmt::Serialize(_, a, _, _, _) | AleoStmt::Deserialize(_, a, _, _, _) => sub(a, aliases),
        AleoStmt::Call(_, inputs, _) | AleoStmt::Async(_, inputs, _) => {
            for e in inputs.iter_mut() {
                sub(e, aliases);
            }
        }
        AleoStmt::CallDynamic(prog, net, fun, inputs, _, _, _) => {
            sub(prog, aliases);
            sub(net, aliases);
            sub(fun, aliases);
            for e in inputs.iter_mut() {
                sub(e, aliases);
            }
        }
        AleoStmt::GetRecordDynamic(e, _, _, _) => sub(e, aliases),
        AleoStmt::ContainsDynamic(a, b, c, d, _) => {
            sub(a, aliases);
            sub(b, aliases);
            sub(c, aliases);
            sub(d, aliases);
        }
        AleoStmt::GetDynamic(a, b, c, d, _, _) => {
            sub(a, aliases);
            sub(b, aliases);
            sub(c, aliases);
            sub(d, aliases);
        }
        AleoStmt::GetOrUseDynamic(a, b, c, d, e, _, _) => {
            sub(a, aliases);
            sub(b, aliases);
            sub(c, aliases);
            sub(d, aliases);
            sub(e, aliases);
        }
        AleoStmt::BranchEq(a, b, _) => {
            sub(a, aliases);
            sub(b, aliases);
        }
        AleoStmt::Position(_) => {}
        // Binary operations
        AleoStmt::Add(a, b, _)
        | AleoStmt::AddWrapped(a, b, _)
        | AleoStmt::And(a, b, _)
        | AleoStmt::Div(a, b, _)
        | AleoStmt::DivWrapped(a, b, _)
        | AleoStmt::Eq(a, b, _)
        | AleoStmt::Gte(a, b, _)
        | AleoStmt::Gt(a, b, _)
        | AleoStmt::Lte(a, b, _)
        | AleoStmt::Lt(a, b, _)
        | AleoStmt::Mod(a, b, _)
        | AleoStmt::Mul(a, b, _)
        | AleoStmt::MulWrapped(a, b, _)
        | AleoStmt::Nand(a, b, _)
        | AleoStmt::Neq(a, b, _)
        | AleoStmt::Nor(a, b, _)
        | AleoStmt::Or(a, b, _)
        | AleoStmt::Pow(a, b, _)
        | AleoStmt::PowWrapped(a, b, _)
        | AleoStmt::Rem(a, b, _)
        | AleoStmt::RemWrapped(a, b, _)
        | AleoStmt::Shl(a, b, _)
        | AleoStmt::ShlWrapped(a, b, _)
        | AleoStmt::Shr(a, b, _)
        | AleoStmt::ShrWrapped(a, b, _)
        | AleoStmt::Sub(a, b, _)
        | AleoStmt::SubWrapped(a, b, _)
        | AleoStmt::Xor(a, b, _) => {
            sub(a, aliases);
            sub(b, aliases);
        }
    }
}

fn fold_identity_operations(stmts: &mut Vec<AleoStmt>) {
    // Apply strength reductions first (e.g. nand(x, true) → not(x)).
    for stmt in stmts.iter_mut() {
        if let Some(reduced) = try_strength_reduce(stmt) {
            *stmt = reduced;
        }
    }

    let mut aliases: HashMap<AleoReg, AleoExpr> = HashMap::new();
    let mut to_remove = Vec::new();

    for (i, stmt) in stmts.iter().enumerate() {
        if let Some((dst, value)) = try_identity(stmt) {
            // Resolve the alias target transitively — clone only here.
            let resolved = resolve_alias(value, &aliases);
            aliases.insert(dst.clone(), resolved);
            to_remove.push(i);
        }
    }

    if aliases.is_empty() {
        return;
    }

    // Propagate aliases through remaining instructions.
    for (i, stmt) in stmts.iter_mut().enumerate() {
        if !to_remove.contains(&i) {
            substitute_aliases_in_stmt(stmt, &aliases);
        }
    }

    // Remove identity instructions in reverse order.
    for i in to_remove.into_iter().rev() {
        stmts.remove(i);
    }
}

// Trivial assert elimination

fn eliminate_trivial_asserts(stmts: &mut Vec<AleoStmt>) {
    fn is_trivial_assert(s: &AleoStmt) -> bool {
        matches!(s, AleoStmt::AssertEq(a, b) if is_literal(a) && is_literal(b) && a == b)
    }

    // The Aleo VM requires closures/finalizes to have at least one non-output
    // instruction. Codegen inserts a dummy `assert.eq true true` to satisfy this.
    let non_output_non_trivial =
        stmts.iter().filter(|s| !matches!(s, AleoStmt::Output(..)) && !is_trivial_assert(s)).count();

    if non_output_non_trivial == 0 {
        // No real instructions — keep exactly one trivial assert as a dummy.
        let mut kept_one = false;
        stmts.retain(|stmt| {
            if is_trivial_assert(stmt) {
                if kept_one {
                    return false;
                }
                kept_one = true;
            }
            true
        });
    } else {
        // Real instructions exist — remove all trivial asserts.
        stmts.retain(|stmt| !is_trivial_assert(stmt));
    }
}

// Dead register elimination

/// Collect all registers that are read (used as operands) in a statement.
fn collect_read_registers(stmt: &AleoStmt, used: &mut HashSet<AleoReg>) {
    fn collect_from_expr(expr: &AleoExpr, used: &mut HashSet<AleoReg>) {
        match expr {
            AleoExpr::Reg(r) => {
                used.insert(r.clone());
            }
            AleoExpr::Tuple(exprs) => {
                for e in exprs {
                    collect_from_expr(e, used);
                }
            }
            AleoExpr::ArrayAccess(a, b) => {
                collect_from_expr(a, used);
                collect_from_expr(b, used);
            }
            AleoExpr::MemberAccess(a, _) => {
                collect_from_expr(a, used);
            }
            _ => {} // literals and names don't reference registers
        }
    }

    match stmt {
        AleoStmt::Output(e, _, _) => collect_from_expr(e, used),
        AleoStmt::AssertEq(a, b) | AleoStmt::AssertNeq(a, b) => {
            collect_from_expr(a, used);
            collect_from_expr(b, used);
        }
        AleoStmt::Cast(e, _, _) => collect_from_expr(e, used),
        AleoStmt::Abs(e, _)
        | AleoStmt::AbsW(e, _)
        | AleoStmt::Double(e, _)
        | AleoStmt::Inv(e, _)
        | AleoStmt::Not(e, _)
        | AleoStmt::Neg(e, _)
        | AleoStmt::Square(e, _)
        | AleoStmt::Sqrt(e, _) => collect_from_expr(e, used),
        AleoStmt::Ternary(a, b, c, _) => {
            collect_from_expr(a, used);
            collect_from_expr(b, used);
            collect_from_expr(c, used);
        }
        AleoStmt::Commit(_, a, b, _, _) => {
            collect_from_expr(a, used);
            collect_from_expr(b, used);
        }
        AleoStmt::Hash(_, a, _, _) => collect_from_expr(a, used),
        AleoStmt::Get(a, b, _) | AleoStmt::Contains(a, b, _) => {
            collect_from_expr(a, used);
            collect_from_expr(b, used);
        }
        AleoStmt::GetOrUse(a, b, c, _) | AleoStmt::Set(a, b, c) => {
            collect_from_expr(a, used);
            collect_from_expr(b, used);
            collect_from_expr(c, used);
        }
        AleoStmt::Remove(a, b) => {
            collect_from_expr(a, used);
            collect_from_expr(b, used);
        }
        AleoStmt::RandChacha(_, _) => {}
        AleoStmt::SignVerify(a, b, c, _) => {
            collect_from_expr(a, used);
            collect_from_expr(b, used);
            collect_from_expr(c, used);
        }
        AleoStmt::EcdsaVerify(_, a, b, c, _) => {
            collect_from_expr(a, used);
            collect_from_expr(b, used);
            collect_from_expr(c, used);
        }
        AleoStmt::SnarkVerify(_, a, b, c, d, _) => {
            collect_from_expr(a, used);
            collect_from_expr(b, used);
            collect_from_expr(c, used);
            collect_from_expr(d, used);
        }
        AleoStmt::Await(e) => collect_from_expr(e, used),
        AleoStmt::Serialize(_, a, _, _, _) | AleoStmt::Deserialize(_, a, _, _, _) => collect_from_expr(a, used),
        AleoStmt::Call(_, inputs, _) | AleoStmt::Async(_, inputs, _) => {
            for e in inputs {
                collect_from_expr(e, used);
            }
        }
        AleoStmt::CallDynamic(prog, net, fun, inputs, _, _, _) => {
            collect_from_expr(prog, used);
            collect_from_expr(net, used);
            collect_from_expr(fun, used);
            for e in inputs {
                collect_from_expr(e, used);
            }
        }
        AleoStmt::GetRecordDynamic(e, _, _, _) => collect_from_expr(e, used),
        AleoStmt::ContainsDynamic(a, b, c, d, _) => {
            collect_from_expr(a, used);
            collect_from_expr(b, used);
            collect_from_expr(c, used);
            collect_from_expr(d, used);
        }
        AleoStmt::GetDynamic(a, b, c, d, _, _) => {
            collect_from_expr(a, used);
            collect_from_expr(b, used);
            collect_from_expr(c, used);
            collect_from_expr(d, used);
        }
        AleoStmt::GetOrUseDynamic(a, b, c, d, e, _, _) => {
            collect_from_expr(a, used);
            collect_from_expr(b, used);
            collect_from_expr(c, used);
            collect_from_expr(d, used);
            collect_from_expr(e, used);
        }
        AleoStmt::BranchEq(a, b, _) => {
            collect_from_expr(a, used);
            collect_from_expr(b, used);
        }
        AleoStmt::Position(_) => {}
        // Binary operations
        AleoStmt::Add(a, b, _)
        | AleoStmt::AddWrapped(a, b, _)
        | AleoStmt::And(a, b, _)
        | AleoStmt::Div(a, b, _)
        | AleoStmt::DivWrapped(a, b, _)
        | AleoStmt::Eq(a, b, _)
        | AleoStmt::Gte(a, b, _)
        | AleoStmt::Gt(a, b, _)
        | AleoStmt::Lte(a, b, _)
        | AleoStmt::Lt(a, b, _)
        | AleoStmt::Mod(a, b, _)
        | AleoStmt::Mul(a, b, _)
        | AleoStmt::MulWrapped(a, b, _)
        | AleoStmt::Nand(a, b, _)
        | AleoStmt::Neq(a, b, _)
        | AleoStmt::Nor(a, b, _)
        | AleoStmt::Or(a, b, _)
        | AleoStmt::Pow(a, b, _)
        | AleoStmt::PowWrapped(a, b, _)
        | AleoStmt::Rem(a, b, _)
        | AleoStmt::RemWrapped(a, b, _)
        | AleoStmt::Shl(a, b, _)
        | AleoStmt::ShlWrapped(a, b, _)
        | AleoStmt::Shr(a, b, _)
        | AleoStmt::ShrWrapped(a, b, _)
        | AleoStmt::Sub(a, b, _)
        | AleoStmt::SubWrapped(a, b, _)
        | AleoStmt::Xor(a, b, _) => {
            collect_from_expr(a, used);
            collect_from_expr(b, used);
        }
    }
}

/// Returns the destination register of a pure instruction, or None for side-effecting instructions.
fn dest_register_if_pure(stmt: &AleoStmt) -> Option<&AleoReg> {
    match stmt {
        // Side-effecting: never remove.
        AleoStmt::Output(..)
        | AleoStmt::AssertEq(..)
        | AleoStmt::AssertNeq(..)
        | AleoStmt::Set(..)
        | AleoStmt::Remove(..)
        | AleoStmt::Await(..)
        | AleoStmt::Call(..)
        | AleoStmt::CallDynamic(..)
        | AleoStmt::Async(..)
        | AleoStmt::BranchEq(..)
        | AleoStmt::Position(..) => None,

        // Checked arithmetic: halts on integer overflow/underflow.
        AleoStmt::Add(..)
        | AleoStmt::Sub(..)
        | AleoStmt::Mul(..)
        | AleoStmt::Pow(..)
        | AleoStmt::Abs(..)  // halts on I::MIN
        | AleoStmt::Neg(..)  // halts on I::MIN and unsigned types
        => None,

        // Division/remainder: halts on zero divisor (even the wrapping variants).
        AleoStmt::Div(..)          // also halts on I::MIN / -1
        | AleoStmt::DivWrapped(..)
        | AleoStmt::Rem(..)        // also halts on I::MIN % -1
        | AleoStmt::RemWrapped(..)
        | AleoStmt::Mod(..)
        => None,

        // Checked shifts: halts when shift amount >= bit width.
        AleoStmt::Shl(..)
        | AleoStmt::Shr(..)
        => None,

        // Field inverse: halts on zero.
        AleoStmt::Inv(..) => None,

        // Square root: halts for quadratic non-residues.
        AleoStmt::Sqrt(..) => None,

        // Lossy cast: narrowing conversions can halt on out-of-range values.
        AleoStmt::Cast(..) => None,

        // Non-deterministic: removing would change subsequent randomness.
        AleoStmt::RandChacha(..) => None,

        // Crypto operations: input/type validation can halt.
        AleoStmt::Commit(..)
        | AleoStmt::Hash(..)
        | AleoStmt::SignVerify(..)
        | AleoStmt::Serialize(..)
        | AleoStmt::Deserialize(..)
        => None,

        // Finalize-only operations: halt outside finalize context.
        AleoStmt::EcdsaVerify(..)
        | AleoStmt::SnarkVerify(..)
        => None,

        // Storage reads: may halt or have observable effects.
        AleoStmt::Get(..)
        | AleoStmt::GetOrUse(..)
        | AleoStmt::Contains(..)
        | AleoStmt::ContainsDynamic(..)
        | AleoStmt::GetDynamic(..)
        | AleoStmt::GetOrUseDynamic(..)
        | AleoStmt::GetRecordDynamic(..) => None,

        // Pure (infallible, no side effects): safe to remove if destination is unused.
        // Wrapping arithmetic, bitwise/boolean ops, comparisons, and total field ops.
        AleoStmt::AbsW(_, d)
        | AleoStmt::Double(_, d)
        | AleoStmt::Not(_, d)
        | AleoStmt::Square(_, d)
        | AleoStmt::Ternary(_, _, _, d)
        | AleoStmt::PowWrapped(_, _, d)
        | AleoStmt::AddWrapped(_, _, d)
        | AleoStmt::SubWrapped(_, _, d)
        | AleoStmt::MulWrapped(_, _, d)
        | AleoStmt::ShlWrapped(_, _, d)
        | AleoStmt::ShrWrapped(_, _, d)
        | AleoStmt::And(_, _, d)
        | AleoStmt::Or(_, _, d)
        | AleoStmt::Xor(_, _, d)
        | AleoStmt::Nand(_, _, d)
        | AleoStmt::Nor(_, _, d)
        | AleoStmt::Eq(_, _, d)
        | AleoStmt::Neq(_, _, d)
        | AleoStmt::Gt(_, _, d)
        | AleoStmt::Gte(_, _, d)
        | AleoStmt::Lt(_, _, d)
        | AleoStmt::Lte(_, _, d)
        => Some(d),
    }
}

fn eliminate_dead_registers(stmts: &mut Vec<AleoStmt>) {
    // Collect all registers that are read.
    let mut used = HashSet::new();
    for stmt in stmts.iter() {
        collect_read_registers(stmt, &mut used);
    }

    // Remove pure instructions whose destination is not in the used set.
    stmts.retain(|stmt| {
        if let Some(dest) = dest_register_if_pure(stmt) {
            used.contains(dest)
        } else {
            true // side-effecting, keep
        }
    });
}

// Consecutive cast folding

fn fold_consecutive_casts(stmts: &mut Vec<AleoStmt>) {
    if stmts.len() < 2 {
        return;
    }

    // Build a set of registers that are used more than once (as operands), so we
    // can tell if an intermediate register from a cast is only used by the next cast.
    let mut reg_use_count: HashMap<AleoReg, usize> = HashMap::new();
    for stmt in stmts.iter() {
        let mut used = HashSet::new();
        collect_read_registers(stmt, &mut used);
        for reg in used {
            *reg_use_count.entry(reg).or_insert(0) += 1;
        }
    }

    #[derive(Clone, Copy)]
    enum PrimitiveCastDomain {
        Signed { bits: u32 },
        Unsigned { bits: u32 },
    }

    let primitive_cast_domain = |ty: &AleoType| -> Option<PrimitiveCastDomain> {
        match ty {
            AleoType::Boolean => Some(PrimitiveCastDomain::Unsigned { bits: 1 }),
            AleoType::I8 => Some(PrimitiveCastDomain::Signed { bits: 8 }),
            AleoType::I16 => Some(PrimitiveCastDomain::Signed { bits: 16 }),
            AleoType::I32 => Some(PrimitiveCastDomain::Signed { bits: 32 }),
            AleoType::I64 => Some(PrimitiveCastDomain::Signed { bits: 64 }),
            AleoType::I128 => Some(PrimitiveCastDomain::Signed { bits: 128 }),
            AleoType::U8 => Some(PrimitiveCastDomain::Unsigned { bits: 8 }),
            AleoType::U16 => Some(PrimitiveCastDomain::Unsigned { bits: 16 }),
            AleoType::U32 => Some(PrimitiveCastDomain::Unsigned { bits: 32 }),
            AleoType::U64 => Some(PrimitiveCastDomain::Unsigned { bits: 64 }),
            AleoType::U128 => Some(PrimitiveCastDomain::Unsigned { bits: 128 }),
            _ => None,
        }
    };

    let intermediate_accepts_all_final_values = |intermediate: &AleoType, final_target: &AleoType| -> bool {
        use PrimitiveCastDomain::*;
        match (primitive_cast_domain(intermediate), primitive_cast_domain(final_target)) {
            (Some(Signed { bits: intermediate_bits }), Some(Signed { bits: final_bits })) => {
                intermediate_bits >= final_bits
            }
            (Some(Signed { bits: intermediate_bits }), Some(Unsigned { bits: final_bits })) => {
                intermediate_bits > final_bits
            }
            (Some(Unsigned { bits: intermediate_bits }), Some(Unsigned { bits: final_bits })) => {
                intermediate_bits >= final_bits
            }
            (Some(Unsigned { .. }), Some(Signed { .. })) => false,
            _ => false,
        }
    };

    let mut i = 0;
    while i + 1 < stmts.len() {
        let can_fold = {
            if let (AleoStmt::Cast(src, r0, t1), AleoStmt::Cast(operand, _, t2)) = (&stmts[i], &stmts[i + 1])
            // Only fold single-operand casts (type coercions), not multi-operand
            // struct/record constructions like `cast r0 r1 r2 into r3 as Foo.record`.
            && !matches!(src, AleoExpr::Tuple(_))
            // The second cast reads from the first cast's destination.
            && let AleoExpr::Reg(inner_reg) = operand
            {
                inner_reg == r0
                    && reg_use_count.get(r0).copied().unwrap_or(0) == 1
                    // This source-agnostic fold is only safe when every value
                    // accepted by the final target is also accepted by the
                    // intermediate target. Equal bit widths are not enough
                    // across signed and unsigned primitive domains.
                    && intermediate_accepts_all_final_values(t1, t2)
            } else {
                false
            }
        };

        if can_fold {
            // Extract the source from the first cast and the dest+type from the second.
            let source = if let AleoStmt::Cast(ref src, _, _) = stmts[i] { src.clone() } else { unreachable!() };
            if let AleoStmt::Cast(ref mut operand, _, _) = stmts[i + 1] {
                *operand = source;
            }
            stmts.remove(i);
            // Don't advance i, check if the merged cast can fold with the next one too.
        } else {
            i += 1;
        }
    }
}

// Register renumbering

/// Returns all destination registers produced by a statement.
fn dest_registers(stmt: &AleoStmt) -> Vec<&AleoReg> {
    match stmt {
        // No destination register.
        AleoStmt::Output(..)
        | AleoStmt::AssertEq(..)
        | AleoStmt::AssertNeq(..)
        | AleoStmt::Set(..)
        | AleoStmt::Remove(..)
        | AleoStmt::Await(..)
        | AleoStmt::BranchEq(..)
        | AleoStmt::Position(..) => vec![],

        // Multi-dest.
        AleoStmt::Call(_, _, dests) | AleoStmt::Async(_, _, dests) => dests.iter().collect(),
        AleoStmt::CallDynamic(_, _, _, _, _, outputs, _) => outputs.iter().collect(),

        // Single dest — all remaining variants.
        AleoStmt::Cast(_, d, _)
        | AleoStmt::Abs(_, d)
        | AleoStmt::AbsW(_, d)
        | AleoStmt::Double(_, d)
        | AleoStmt::Inv(_, d)
        | AleoStmt::Not(_, d)
        | AleoStmt::Neg(_, d)
        | AleoStmt::Square(_, d)
        | AleoStmt::Sqrt(_, d)
        | AleoStmt::Ternary(_, _, _, d)
        | AleoStmt::Commit(_, _, _, d, _)
        | AleoStmt::Hash(_, _, d, _)
        | AleoStmt::RandChacha(d, _)
        | AleoStmt::SignVerify(_, _, _, d)
        | AleoStmt::EcdsaVerify(_, _, _, _, d)
        | AleoStmt::SnarkVerify(_, _, _, _, _, d)
        | AleoStmt::Serialize(_, _, _, d, _)
        | AleoStmt::Deserialize(_, _, _, d, _)
        | AleoStmt::GetRecordDynamic(_, _, d, _)
        | AleoStmt::Get(_, _, d)
        | AleoStmt::GetOrUse(_, _, _, d)
        | AleoStmt::Contains(_, _, d)
        | AleoStmt::ContainsDynamic(_, _, _, _, d)
        | AleoStmt::GetDynamic(_, _, _, _, d, _)
        | AleoStmt::GetOrUseDynamic(_, _, _, _, _, d, _)
        | AleoStmt::Add(_, _, d)
        | AleoStmt::AddWrapped(_, _, d)
        | AleoStmt::And(_, _, d)
        | AleoStmt::Div(_, _, d)
        | AleoStmt::DivWrapped(_, _, d)
        | AleoStmt::Eq(_, _, d)
        | AleoStmt::Gte(_, _, d)
        | AleoStmt::Gt(_, _, d)
        | AleoStmt::Lte(_, _, d)
        | AleoStmt::Lt(_, _, d)
        | AleoStmt::Mod(_, _, d)
        | AleoStmt::Mul(_, _, d)
        | AleoStmt::MulWrapped(_, _, d)
        | AleoStmt::Nand(_, _, d)
        | AleoStmt::Neq(_, _, d)
        | AleoStmt::Nor(_, _, d)
        | AleoStmt::Or(_, _, d)
        | AleoStmt::Pow(_, _, d)
        | AleoStmt::PowWrapped(_, _, d)
        | AleoStmt::Rem(_, _, d)
        | AleoStmt::RemWrapped(_, _, d)
        | AleoStmt::Shl(_, _, d)
        | AleoStmt::ShlWrapped(_, _, d)
        | AleoStmt::Shr(_, _, d)
        | AleoStmt::ShrWrapped(_, _, d)
        | AleoStmt::Sub(_, _, d)
        | AleoStmt::SubWrapped(_, _, d)
        | AleoStmt::Xor(_, _, d) => vec![d],
    }
}

fn renumber_registers(stmts: &mut [AleoStmt], num_inputs: usize) {
    // Build the old-to-new register mapping by scanning in order.
    let mut mapping: HashMap<u64, u64> = HashMap::new();
    let mut next_reg = num_inputs as u64;

    // First, map input registers to themselves (they're already sequential from 0).
    for i in 0..num_inputs as u64 {
        mapping.insert(i, i);
    }

    // Then scan all instructions for destination registers.
    for stmt in stmts.iter() {
        for d in dest_registers(stmt) {
            if let AleoReg::R(n) = d
                && !mapping.contains_key(n)
            {
                mapping.insert(*n, next_reg);
                next_reg += 1;
            }
        }
    }

    // Check if any renumbering is actually needed.
    if mapping.iter().all(|(old, new)| old == new) {
        return;
    }

    // Apply the mapping.
    fn remap_reg(reg: &mut AleoReg, mapping: &HashMap<u64, u64>) {
        if let AleoReg::R(n) = reg
            && let Some(&new_n) = mapping.get(n)
        {
            *n = new_n;
        }
    }

    fn remap_expr(expr: &mut AleoExpr, mapping: &HashMap<u64, u64>) {
        match expr {
            AleoExpr::Reg(r) => remap_reg(r, mapping),
            AleoExpr::Tuple(exprs) => {
                for e in exprs {
                    remap_expr(e, mapping);
                }
            }
            AleoExpr::ArrayAccess(a, b) => {
                remap_expr(a, mapping);
                remap_expr(b, mapping);
            }
            AleoExpr::MemberAccess(a, _) => remap_expr(a, mapping),
            _ => {} // literals
        }
    }

    for stmt in stmts.iter_mut() {
        match stmt {
            AleoStmt::Output(e, _, _) => remap_expr(e, &mapping),
            AleoStmt::AssertEq(a, b) | AleoStmt::AssertNeq(a, b) => {
                remap_expr(a, &mapping);
                remap_expr(b, &mapping);
            }
            AleoStmt::Cast(e, d, _) => {
                remap_expr(e, &mapping);
                remap_reg(d, &mapping);
            }
            AleoStmt::Abs(e, d)
            | AleoStmt::AbsW(e, d)
            | AleoStmt::Double(e, d)
            | AleoStmt::Inv(e, d)
            | AleoStmt::Not(e, d)
            | AleoStmt::Neg(e, d)
            | AleoStmt::Square(e, d)
            | AleoStmt::Sqrt(e, d) => {
                remap_expr(e, &mapping);
                remap_reg(d, &mapping);
            }
            AleoStmt::Ternary(a, b, c, d) => {
                remap_expr(a, &mapping);
                remap_expr(b, &mapping);
                remap_expr(c, &mapping);
                remap_reg(d, &mapping);
            }
            AleoStmt::Commit(_, a, b, d, _) => {
                remap_expr(a, &mapping);
                remap_expr(b, &mapping);
                remap_reg(d, &mapping);
            }
            AleoStmt::Hash(_, a, d, _) => {
                remap_expr(a, &mapping);
                remap_reg(d, &mapping);
            }
            AleoStmt::Get(a, b, d) | AleoStmt::Contains(a, b, d) => {
                remap_expr(a, &mapping);
                remap_expr(b, &mapping);
                remap_reg(d, &mapping);
            }
            AleoStmt::GetOrUse(a, b, c, d) => {
                remap_expr(a, &mapping);
                remap_expr(b, &mapping);
                remap_expr(c, &mapping);
                remap_reg(d, &mapping);
            }
            AleoStmt::Set(a, b, c) => {
                remap_expr(a, &mapping);
                remap_expr(b, &mapping);
                remap_expr(c, &mapping);
            }
            AleoStmt::Remove(a, b) => {
                remap_expr(a, &mapping);
                remap_expr(b, &mapping);
            }
            AleoStmt::RandChacha(d, _) => remap_reg(d, &mapping),
            AleoStmt::SignVerify(a, b, c, d) => {
                remap_expr(a, &mapping);
                remap_expr(b, &mapping);
                remap_expr(c, &mapping);
                remap_reg(d, &mapping);
            }
            AleoStmt::EcdsaVerify(_, a, b, c, d) => {
                remap_expr(a, &mapping);
                remap_expr(b, &mapping);
                remap_expr(c, &mapping);
                remap_reg(d, &mapping);
            }
            AleoStmt::SnarkVerify(_, a, b, c, d2, e) => {
                remap_expr(a, &mapping);
                remap_expr(b, &mapping);
                remap_expr(c, &mapping);
                remap_expr(d2, &mapping);
                remap_reg(e, &mapping);
            }
            AleoStmt::Await(e) => remap_expr(e, &mapping),
            AleoStmt::Serialize(_, a, _, d, _) | AleoStmt::Deserialize(_, a, _, d, _) => {
                remap_expr(a, &mapping);
                remap_reg(d, &mapping);
            }
            AleoStmt::Call(_, inputs, dests) | AleoStmt::Async(_, inputs, dests) => {
                for e in inputs.iter_mut() {
                    remap_expr(e, &mapping);
                }
                for d in dests.iter_mut() {
                    remap_reg(d, &mapping);
                }
            }
            AleoStmt::CallDynamic(prog, net, fun, inputs, _, outputs, _) => {
                remap_expr(prog, &mapping);
                remap_expr(net, &mapping);
                remap_expr(fun, &mapping);
                for e in inputs.iter_mut() {
                    remap_expr(e, &mapping);
                }
                for d in outputs.iter_mut() {
                    remap_reg(d, &mapping);
                }
            }
            AleoStmt::GetRecordDynamic(e, _, d, _) => {
                remap_expr(e, &mapping);
                remap_reg(d, &mapping);
            }
            AleoStmt::ContainsDynamic(a, b, c, d2, e) => {
                remap_expr(a, &mapping);
                remap_expr(b, &mapping);
                remap_expr(c, &mapping);
                remap_expr(d2, &mapping);
                remap_reg(e, &mapping);
            }
            AleoStmt::GetDynamic(a, b, c, d2, e, _) => {
                remap_expr(a, &mapping);
                remap_expr(b, &mapping);
                remap_expr(c, &mapping);
                remap_expr(d2, &mapping);
                remap_reg(e, &mapping);
            }
            AleoStmt::GetOrUseDynamic(a, b, c, d2, e2, f, _) => {
                remap_expr(a, &mapping);
                remap_expr(b, &mapping);
                remap_expr(c, &mapping);
                remap_expr(d2, &mapping);
                remap_expr(e2, &mapping);
                remap_reg(f, &mapping);
            }
            AleoStmt::BranchEq(a, b, _) => {
                remap_expr(a, &mapping);
                remap_expr(b, &mapping);
            }
            AleoStmt::Position(_) => {}
            AleoStmt::Add(a, b, d)
            | AleoStmt::AddWrapped(a, b, d)
            | AleoStmt::And(a, b, d)
            | AleoStmt::Div(a, b, d)
            | AleoStmt::DivWrapped(a, b, d)
            | AleoStmt::Eq(a, b, d)
            | AleoStmt::Gte(a, b, d)
            | AleoStmt::Gt(a, b, d)
            | AleoStmt::Lte(a, b, d)
            | AleoStmt::Lt(a, b, d)
            | AleoStmt::Mod(a, b, d)
            | AleoStmt::Mul(a, b, d)
            | AleoStmt::MulWrapped(a, b, d)
            | AleoStmt::Nand(a, b, d)
            | AleoStmt::Neq(a, b, d)
            | AleoStmt::Nor(a, b, d)
            | AleoStmt::Or(a, b, d)
            | AleoStmt::Pow(a, b, d)
            | AleoStmt::PowWrapped(a, b, d)
            | AleoStmt::Rem(a, b, d)
            | AleoStmt::RemWrapped(a, b, d)
            | AleoStmt::Shl(a, b, d)
            | AleoStmt::ShlWrapped(a, b, d)
            | AleoStmt::Shr(a, b, d)
            | AleoStmt::ShrWrapped(a, b, d)
            | AleoStmt::Sub(a, b, d)
            | AleoStmt::SubWrapped(a, b, d)
            | AleoStmt::Xor(a, b, d) => {
                remap_expr(a, &mapping);
                remap_expr(b, &mapping);
                remap_reg(d, &mapping);
            }
        }
    }
}
