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

use leo_ast::{
    BinaryExpression,
    BinaryOperation,
    Expression,
    Identifier,
    Literal,
    Node,
    Path,
    Statement,
    TernaryExpression,
    Type,
    UnaryExpression,
    UnaryOperation,
};

pub struct HaltSafeTernaryVisitor<'a> {
    pub state: &'a mut CompilerState,

    /// The current path guard: a boolean variable that is true iff all the enclosing
    /// ternary-arm conditions hold. `None` means we are not inside any ternary arm, in which
    /// case halting operations are left unguarded (a genuine fault should still halt).
    pub guard: Option<Identifier>,
}

/// Returns true iff `op` is a checked operation that halts on overflow or a zero divisor.
/// The wrapping variants (`add_wrapped`, ...) never halt and are excluded.
pub fn is_checked_arithmetic(op: BinaryOperation) -> bool {
    use BinaryOperation::*;
    matches!(op, Add | Sub | Mul | Div | Rem | Mod | Pow | Shl | Shr)
}

impl HaltSafeTernaryVisitor<'_> {
    /// Reconstructs an expression, threading the current guard through its children.
    pub fn rec(&mut self, expr: Expression) -> (Expression, Vec<Statement>) {
        use leo_ast::AstReconstructor as _;
        self.reconstruct_expression(expr, &())
    }

    /// Builds a `Path` expression referring to a local variable.
    /// The path reuses the identifier's node ID, whose type is already in the type table.
    pub fn path_expr(&self, identifier: Identifier) -> Expression {
        Path::from(identifier).to_local().into()
    }

    /// Creates a definition `let place = value;` with a fresh, unique name, recording the
    /// type of the new variable. Returns the new identifier and the definition statement.
    pub fn define(&mut self, hint: &str, value: Expression, type_: Type) -> (Identifier, Statement) {
        let name = self.state.assigner.unique_symbol(hint, "$");
        let identifier = Identifier { name, span: Default::default(), id: self.state.node_builder.next_id() };
        self.state.type_table.insert(identifier.id, type_);
        let statement = self.state.assigner.simple_definition(identifier, value, self.state.node_builder.next_id());
        (identifier, statement)
    }

    /// Builds `let g = left && right;` and returns `g` along with the definition.
    pub fn conjoin(&mut self, left: Identifier, right: Identifier) -> (Identifier, Statement) {
        let binary = BinaryExpression {
            op: BinaryOperation::And,
            left: self.path_expr(left),
            right: self.path_expr(right),
            span: Default::default(),
            id: self.state.node_builder.next_id(),
        };
        self.state.type_table.insert(binary.id, Type::Boolean);
        self.define("$guard", binary.into(), Type::Boolean)
    }

    /// Builds `let g = !inner;` and returns `g` along with the definition.
    pub fn negate(&mut self, inner: Identifier) -> (Identifier, Statement) {
        let unary = UnaryExpression {
            op: UnaryOperation::Not,
            receiver: self.path_expr(inner),
            span: Default::default(),
            id: self.state.node_builder.next_id(),
        };
        self.state.type_table.insert(unary.id, Type::Boolean);
        self.define("$guard", unary.into(), Type::Boolean)
    }

    /// Wraps `operand` in `guard ? operand : neutral`, producing a halt-free selection that
    /// yields the real operand only when `guard` holds. `type_` is the shared type of the
    /// operand and the neutral value.
    pub fn predicate(
        &mut self,
        guard: Identifier,
        operand: Expression,
        neutral: Expression,
        type_: Type,
    ) -> Expression {
        let ternary = TernaryExpression {
            condition: self.path_expr(guard),
            if_true: operand,
            if_false: neutral,
            span: Default::default(),
            id: self.state.node_builder.next_id(),
        };
        self.state.type_table.insert(ternary.id, type_);
        ternary.into()
    }

    /// Builds a literal of value `value` ("0" or "1") for the given numeric type, recording
    /// its type. Used to construct neutral operands for predication.
    pub fn numeric_literal(&mut self, value: &str, type_: &Type) -> Expression {
        let id = self.state.node_builder.next_id();
        let literal = match type_ {
            Type::Integer(integer_type) => Literal::integer(*integer_type, value.to_string(), Default::default(), id),
            Type::Field => Literal::field(value.to_string(), Default::default(), id),
            Type::Scalar => Literal::scalar(value.to_string(), Default::default(), id),
            _ => panic!("Neutral values are only constructed for integer, field, and scalar types."),
        };
        self.state.type_table.insert(id, type_.clone());
        literal.into()
    }

    /// Returns true iff a narrowing cast from `source` to `target` can halt at runtime.
    /// A cast can overflow only when the target is an integer type and the source is a wider
    /// numeric type (another integer, a field, or a scalar).
    pub fn cast_may_halt(source: &Type, target: &Type) -> bool {
        matches!(target, Type::Integer(_)) && matches!(source, Type::Integer(_) | Type::Field | Type::Scalar)
    }

    /// Returns true iff `expr` contains a halting operation (a checked integer arithmetic or
    /// shift operation, or a narrowing cast) anywhere in its subtree. Nested ternaries are
    /// traversed because a halting operation inside a deeper arm is guarded by the conjunction
    /// of all enclosing conditions, including this one.
    pub fn expr_has_halting(&self, expr: &Expression) -> bool {
        match expr {
            Expression::Binary(binary) => {
                (is_checked_arithmetic(binary.op)
                    && matches!(self.state.type_table.get(&binary.id), Some(Type::Integer(_))))
                    || self.expr_has_halting(&binary.left)
                    || self.expr_has_halting(&binary.right)
            }
            Expression::Cast(cast) => {
                let source_halts = self
                    .state
                    .type_table
                    .get(&cast.expression.id())
                    .is_some_and(|source| Self::cast_may_halt(&source, &cast.type_));
                source_halts || self.expr_has_halting(&cast.expression)
            }
            Expression::Ternary(ternary) => {
                self.expr_has_halting(&ternary.condition)
                    || self.expr_has_halting(&ternary.if_true)
                    || self.expr_has_halting(&ternary.if_false)
            }
            Expression::Unary(unary) => self.expr_has_halting(&unary.receiver),
            Expression::ArrayAccess(access) => {
                self.expr_has_halting(&access.array) || self.expr_has_halting(&access.index)
            }
            Expression::MemberAccess(access) => self.expr_has_halting(&access.inner),
            Expression::TupleAccess(access) => self.expr_has_halting(&access.tuple),
            Expression::Array(array) => array.elements.iter().any(|element| self.expr_has_halting(element)),
            Expression::Tuple(tuple) => tuple.elements.iter().any(|element| self.expr_has_halting(element)),
            Expression::Call(call) => {
                call.arguments.iter().any(|argument| self.expr_has_halting(argument))
                    || call.const_arguments.iter().any(|argument| self.expr_has_halting(argument))
            }
            Expression::Composite(composite) => {
                composite.const_arguments.iter().any(|argument| self.expr_has_halting(argument))
                    || composite.members.iter().any(|member| {
                        member.expression.as_ref().is_some_and(|expression| self.expr_has_halting(expression))
                    })
            }
            Expression::Repeat(repeat) => self.expr_has_halting(&repeat.expr) || self.expr_has_halting(&repeat.count),
            Expression::Intrinsic(intrinsic) => {
                intrinsic.arguments.iter().any(|argument| self.expr_has_halting(argument))
            }
            // An async block is a separate execution context; its ternaries are predicated when
            // the block itself is reconstructed, not as part of this arm.
            Expression::Async(_)
            | Expression::DynamicOp(_)
            | Expression::Path(_)
            | Expression::Literal(_)
            | Expression::Unit(_)
            | Expression::Err(_) => false,
        }
    }
}
