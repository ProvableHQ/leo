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

use super::{HaltSafeTernaryVisitor, is_checked_arithmetic};

use leo_ast::*;

impl HaltSafeTernaryVisitor<'_> {
    /// Reconstructs each expression in `exprs`, threading the current guard through and
    /// accumulating any produced statements into `statements`.
    fn rec_vec(&mut self, exprs: Vec<Expression>, statements: &mut Vec<Statement>) -> Vec<Expression> {
        exprs
            .into_iter()
            .map(|expr| {
                let (expr, stmts) = self.rec(expr);
                statements.extend(stmts);
                expr
            })
            .collect()
    }
}

impl AstReconstructor for HaltSafeTernaryVisitor<'_> {
    type AdditionalInput = ();
    type AdditionalOutput = Vec<Statement>;

    /* Expressions */

    /// Predicates a ternary's arms so that a halting operation in the untaken arm cannot abort
    /// execution. Each arm is reconstructed under a guard that is true iff that arm is selected.
    fn reconstruct_ternary(&mut self, input: TernaryExpression, _additional: &()) -> (Expression, Vec<Statement>) {
        let TernaryExpression { condition, if_true, if_false, span, id } = input;

        // Whether each arm contains a halting operation that must be guarded.
        let true_has = self.expr_has_halting(&if_true);
        let false_has = self.expr_has_halting(&if_false);

        // The condition is evaluated under the current guard, not under either arm's guard.
        let (condition, mut statements) = self.rec(condition);

        // If neither arm can halt, there is nothing to guard, so the program is left unchanged
        // apart from recursing into the arms (to predicate any deeper, independent ternaries).
        if !true_has && !false_has {
            let if_true = {
                let (if_true, stmts) = self.rec(if_true);
                statements.extend(stmts);
                if_true
            };
            let if_false = {
                let (if_false, stmts) = self.rec(if_false);
                statements.extend(stmts);
                if_false
            };
            return (TernaryExpression { condition, if_true, if_false, span, id }.into(), statements);
        }

        // Hoist the condition into a boolean variable so that the arm guards can reference it
        // (and its negation) without re-evaluating it. The variable's node ID carries the type
        // that the guard references rely on.
        let (base, definition) = self.define("$cond", condition, Type::Boolean);
        statements.push(definition);
        let condition = self.path_expr(base);

        let outer = self.guard;

        // True arm: selected iff `outer && base`.
        let if_true = if true_has {
            let true_guard = if let Some(g) = outer {
                let (guard, definition) = self.conjoin(g, base);
                statements.push(definition);
                guard
            } else {
                base
            };
            self.guard = Some(true_guard);
            let (if_true, stmts) = self.rec(if_true);
            self.guard = outer;
            statements.extend(stmts);
            if_true
        } else {
            let (if_true, stmts) = self.rec(if_true);
            statements.extend(stmts);
            if_true
        };

        // False arm: selected iff `outer && !base`.
        let if_false = if false_has {
            let (negated, definition) = self.negate(base);
            statements.push(definition);
            let false_guard = if let Some(g) = outer {
                let (guard, definition) = self.conjoin(g, negated);
                statements.push(definition);
                guard
            } else {
                negated
            };
            self.guard = Some(false_guard);
            let (if_false, stmts) = self.rec(if_false);
            self.guard = outer;
            statements.extend(stmts);
            if_false
        } else {
            let (if_false, stmts) = self.rec(if_false);
            statements.extend(stmts);
            if_false
        };

        (TernaryExpression { condition, if_true, if_false, span, id }.into(), statements)
    }

    /// Predicates the halt-triggering operands of a checked integer arithmetic or shift
    /// operation when it appears inside a ternary arm (i.e. when a guard is active).
    fn reconstruct_binary(&mut self, input: BinaryExpression, _additional: &()) -> (Expression, Vec<Statement>) {
        let BinaryExpression { op, left, right, span, id } = input;
        let right_id = right.id();

        let (left, mut statements) = self.rec(left);
        let (right, right_statements) = self.rec(right);
        statements.extend(right_statements);

        let result_type = self.state.type_table.get(&id);
        let is_integer = matches!(result_type, Some(Type::Integer(_)));

        let Some(guard) = self.guard else {
            return (BinaryExpression { op, left, right, span, id }.into(), statements);
        };

        if !(is_integer && is_checked_arithmetic(op)) {
            return (BinaryExpression { op, left, right, span, id }.into(), statements);
        }

        let result_type = result_type.expect("Checked to be an integer type above.");

        let (left, right) = match op {
            BinaryOperation::Div | BinaryOperation::Rem | BinaryOperation::Mod => {
                // Predicate the divisor to `1`, which also neutralizes signed `MIN / -1`.
                let neutral = self.numeric_literal("1", &result_type);
                (left, self.predicate(guard, right, neutral, result_type))
            }
            BinaryOperation::Add | BinaryOperation::Sub | BinaryOperation::Mul => {
                // Predicate both operands to `0`, making the operation total when unguarded.
                let left_neutral = self.numeric_literal("0", &result_type);
                let left = self.predicate(guard, left, left_neutral, result_type.clone());
                let right_neutral = self.numeric_literal("0", &result_type);
                (left, self.predicate(guard, right, right_neutral, result_type))
            }
            _ => {
                // `Pow`, `Shl`, `Shr`: predicate the exponent or shift amount to `0`.
                // That operand may have a narrower integer type than the result.
                let right_type =
                    self.state.type_table.get(&right_id).expect("Type checking guarantees the operand is typed.");
                let neutral = self.numeric_literal("0", &right_type);
                (left, self.predicate(guard, right, neutral, right_type))
            }
        };

        (BinaryExpression { op, left, right, span, id }.into(), statements)
    }

    /// Predicates the operand of a narrowing cast so that an out-of-range value in the untaken
    /// arm cannot halt. The operand is replaced with `guard ? operand : 0`.
    fn reconstruct_cast(&mut self, input: CastExpression, _additional: &()) -> (Expression, Vec<Statement>) {
        let CastExpression { expression, type_, span, id } = input;
        let source_type = self.state.type_table.get(&expression.id());

        let (expression, statements) = self.rec(expression);

        let Some(guard) = self.guard else {
            return (CastExpression { expression, type_, span, id }.into(), statements);
        };

        match source_type {
            Some(source_type) if Self::cast_may_halt(&source_type, &type_) => {
                let neutral = self.numeric_literal("0", &source_type);
                let expression = self.predicate(guard, expression, neutral, source_type);
                (CastExpression { expression, type_, span, id }.into(), statements)
            }
            _ => (CastExpression { expression, type_, span, id }.into(), statements),
        }
    }

    fn reconstruct_unary(&mut self, input: UnaryExpression, _additional: &()) -> (Expression, Vec<Statement>) {
        let (receiver, statements) = self.rec(input.receiver);
        (UnaryExpression { receiver, ..input }.into(), statements)
    }

    fn reconstruct_array_access(&mut self, input: ArrayAccess, _additional: &()) -> (Expression, Vec<Statement>) {
        let (array, mut statements) = self.rec(input.array);
        let (index, index_statements) = self.rec(input.index);
        statements.extend(index_statements);
        (ArrayAccess { array, index, ..input }.into(), statements)
    }

    fn reconstruct_member_access(&mut self, input: MemberAccess, _additional: &()) -> (Expression, Vec<Statement>) {
        let (inner, statements) = self.rec(input.inner);
        (MemberAccess { inner, ..input }.into(), statements)
    }

    fn reconstruct_tuple_access(&mut self, input: TupleAccess, _additional: &()) -> (Expression, Vec<Statement>) {
        let (tuple, statements) = self.rec(input.tuple);
        (TupleAccess { tuple, ..input }.into(), statements)
    }

    fn reconstruct_array(&mut self, input: ArrayExpression, _additional: &()) -> (Expression, Vec<Statement>) {
        let mut statements = Vec::new();
        let elements = self.rec_vec(input.elements, &mut statements);
        (ArrayExpression { elements, ..input }.into(), statements)
    }

    fn reconstruct_tuple(&mut self, input: TupleExpression, _additional: &()) -> (Expression, Vec<Statement>) {
        let mut statements = Vec::new();
        let elements = self.rec_vec(input.elements, &mut statements);
        (TupleExpression { elements, ..input }.into(), statements)
    }

    fn reconstruct_call(&mut self, input: CallExpression, _additional: &()) -> (Expression, Vec<Statement>) {
        let mut statements = Vec::new();
        let const_arguments = self.rec_vec(input.const_arguments, &mut statements);
        let arguments = self.rec_vec(input.arguments, &mut statements);
        (CallExpression { const_arguments, arguments, ..input }.into(), statements)
    }

    fn reconstruct_composite_init(
        &mut self,
        input: CompositeExpression,
        _additional: &(),
    ) -> (Expression, Vec<Statement>) {
        let mut statements = Vec::new();
        let const_arguments = self.rec_vec(input.const_arguments, &mut statements);
        let members = input
            .members
            .into_iter()
            .map(|member| {
                let expression = member.expression.map(|expression| {
                    let (expression, stmts) = self.rec(expression);
                    statements.extend(stmts);
                    expression
                });
                CompositeFieldInitializer { expression, ..member }
            })
            .collect();
        (CompositeExpression { const_arguments, members, ..input }.into(), statements)
    }

    fn reconstruct_repeat(&mut self, input: RepeatExpression, _additional: &()) -> (Expression, Vec<Statement>) {
        let (expr, mut statements) = self.rec(input.expr);
        let (count, count_statements) = self.rec(input.count);
        statements.extend(count_statements);
        (RepeatExpression { expr, count, ..input }.into(), statements)
    }

    fn reconstruct_intrinsic(&mut self, input: IntrinsicExpression, _additional: &()) -> (Expression, Vec<Statement>) {
        let mut statements = Vec::new();
        let arguments = self.rec_vec(input.arguments, &mut statements);
        (IntrinsicExpression { arguments, ..input }.into(), statements)
    }

    fn reconstruct_dynamic_op(&mut self, input: DynamicOpExpression, _additional: &()) -> (Expression, Vec<Statement>) {
        let mut statements = Vec::new();
        let (target_program, target_statements) = self.rec(input.target_program);
        statements.extend(target_statements);
        let network = input.network.map(|network| {
            let (network, stmts) = self.rec(network);
            statements.extend(stmts);
            network
        });
        let kind = match input.kind {
            DynamicOpKind::Call { function, arguments } => {
                DynamicOpKind::Call { function, arguments: self.rec_vec(arguments, &mut statements) }
            }
            DynamicOpKind::Read { storage } => DynamicOpKind::Read { storage },
            DynamicOpKind::Op { member, op, arguments } => {
                DynamicOpKind::Op { member, op, arguments: self.rec_vec(arguments, &mut statements) }
            }
        };
        (DynamicOpExpression { target_program, network, kind, ..input }.into(), statements)
    }

    /* Statements */

    /// Flattens each statement's prepended definitions (guard variables and predication
    /// scaffolding) into the block ahead of the statement that uses them.
    fn reconstruct_block(&mut self, block: Block) -> (Block, Vec<Statement>) {
        let mut statements = Vec::with_capacity(block.statements.len());
        for statement in block.statements {
            let (statement, additional) = self.reconstruct_statement(statement);
            statements.extend(additional);
            statements.push(statement);
        }
        (Block { statements, ..block }, Default::default())
    }

    fn reconstruct_definition(&mut self, input: DefinitionStatement) -> (Statement, Vec<Statement>) {
        let (value, statements) = self.rec(input.value);
        (DefinitionStatement { value, ..input }.into(), statements)
    }

    fn reconstruct_assign(&mut self, input: AssignStatement) -> (Statement, Vec<Statement>) {
        let (place, mut statements) = self.rec(input.place);
        let (value, value_statements) = self.rec(input.value);
        statements.extend(value_statements);
        (AssignStatement { place, value, ..input }.into(), statements)
    }

    fn reconstruct_expression_statement(&mut self, input: ExpressionStatement) -> (Statement, Vec<Statement>) {
        let (expression, statements) = self.rec(input.expression);
        (ExpressionStatement { expression, ..input }.into(), statements)
    }

    fn reconstruct_return(&mut self, input: ReturnStatement) -> (Statement, Vec<Statement>) {
        let (expression, statements) = self.rec(input.expression);
        (ReturnStatement { expression, ..input }.into(), statements)
    }

    fn reconstruct_assert(&mut self, input: AssertStatement) -> (Statement, Vec<Statement>) {
        let mut statements = Vec::new();
        let variant = match input.variant {
            AssertVariant::Assert(expression) => {
                let (expression, stmts) = self.rec(expression);
                statements.extend(stmts);
                AssertVariant::Assert(expression)
            }
            AssertVariant::AssertEq(left, right) => {
                let (left, left_statements) = self.rec(left);
                statements.extend(left_statements);
                let (right, right_statements) = self.rec(right);
                statements.extend(right_statements);
                AssertVariant::AssertEq(left, right)
            }
            AssertVariant::AssertNeq(left, right) => {
                let (left, left_statements) = self.rec(left);
                statements.extend(left_statements);
                let (right, right_statements) = self.rec(right);
                statements.extend(right_statements);
                AssertVariant::AssertNeq(left, right)
            }
        };
        (AssertStatement { variant, ..input }.into(), statements)
    }

    /// Reconstructs a conditional statement. The branch condition does not contribute to the
    /// ternary path guard; only ternary-arm conditions are tracked by this pass.
    fn reconstruct_conditional(&mut self, input: ConditionalStatement) -> (Statement, Vec<Statement>) {
        let (condition, statements) = self.rec(input.condition);
        let then = self.reconstruct_block(input.then).0;
        let otherwise = input.otherwise.map(|otherwise| Box::new(self.reconstruct_statement(*otherwise).0));
        (ConditionalStatement { condition, then, otherwise, ..input }.into(), statements)
    }

    /// Constant declarations are left unchanged: their values are compile-time constants and
    /// cannot contain a halting operation guarded by a runtime ternary condition.
    fn reconstruct_const(&mut self, input: ConstDeclaration) -> (Statement, Vec<Statement>) {
        (input.into(), Default::default())
    }

    fn reconstruct_iteration(&mut self, _input: IterationStatement) -> (Statement, Vec<Statement>) {
        panic!("`IterationStatement`s should not be in the AST at this phase of compilation.");
    }
}

// Use the default program traversal, which recurses into every function and constructor body.
impl UnitReconstructor for HaltSafeTernaryVisitor<'_> {}
