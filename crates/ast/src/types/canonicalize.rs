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

//! Scrubs `Span` and `NodeID` while preserving identity-bearing payloads (`Symbol`, `Location`).
//! Applied by [`crate::TypeInterner::intern`] so structurally-equal values sharing only their
//! positions collapse to one [`crate::Type`].
//!
//! Every impl fully destructures via `Self { ... }`, so adding a new field to any AST node is
//! a compile error here — that's what stops a positional field from silently leaking through.

use crate::{
    ArrayAccess,
    ArrayExpression,
    ArrayType,
    AssertStatement,
    AssertVariant,
    AssignStatement,
    AsyncExpression,
    BinaryExpression,
    Block,
    CallExpression,
    CastExpression,
    CompositeExpression,
    CompositeFieldInitializer,
    CompositeType,
    ConditionalStatement,
    ConstDeclaration,
    DefinitionPlace,
    DefinitionStatement,
    DynamicOpExpression,
    DynamicOpKind,
    ErrExpression,
    Expression,
    ExpressionStatement,
    FutureType,
    Identifier,
    IntrinsicExpression,
    IterationStatement,
    Literal,
    MappingType,
    MemberAccess,
    NodeID,
    OptionalType,
    ProgramId,
    RepeatExpression,
    ReturnStatement,
    Statement,
    TernaryExpression,
    TupleAccess,
    TupleExpression,
    TupleType,
    TypeKind,
    TypeNode,
    UnaryExpression,
    UnitExpression,
    VectorType,
};
use leo_span::Span;

pub trait Canonicalize: Sized {
    #[must_use]
    fn canonicalize(self) -> Self;
}

impl<T: Canonicalize> Canonicalize for Box<T> {
    fn canonicalize(self) -> Self {
        Box::new((*self).canonicalize())
    }
}

impl<T: Canonicalize> Canonicalize for Vec<T> {
    fn canonicalize(self) -> Self {
        self.into_iter().map(Canonicalize::canonicalize).collect()
    }
}

impl<T: Canonicalize> Canonicalize for Option<T> {
    fn canonicalize(self) -> Self {
        self.map(Canonicalize::canonicalize)
    }
}

impl Canonicalize for Identifier {
    fn canonicalize(self) -> Self {
        let Self { name, span: _, id: _ } = self;
        Self { name, span: Span::default(), id: NodeID::default() }
    }
}

impl Canonicalize for ProgramId {
    fn canonicalize(self) -> Self {
        let Self { name, network } = self;
        Self { name: name.canonicalize(), network: network.canonicalize() }
    }
}

// `Canonicalize for Path` is implemented in `common/path.rs` so it can see Path's private fields.

impl Canonicalize for TypeNode {
    fn canonicalize(self) -> Self {
        // Canonicalizing `kind` doesn't change its interner key, so `type_` stays valid and we
        // can rebuild via `from_parts` without another intern lookup.
        let (kind, _, type_) = self.into_parts();
        TypeNode::from_parts(kind.canonicalize(), Span::default(), type_)
    }
}

impl Canonicalize for TypeKind {
    fn canonicalize(self) -> Self {
        match self {
            TypeKind::Array(a) => TypeKind::Array(a.canonicalize()),
            TypeKind::Composite(c) => TypeKind::Composite(c.canonicalize()),
            TypeKind::Future(f) => TypeKind::Future(f.canonicalize()),
            TypeKind::Ident(i) => TypeKind::Ident(i.canonicalize()),
            TypeKind::Mapping(m) => TypeKind::Mapping(m.canonicalize()),
            TypeKind::Optional(o) => TypeKind::Optional(o.canonicalize()),
            TypeKind::Tuple(t) => TypeKind::Tuple(t.canonicalize()),
            TypeKind::Vector(v) => TypeKind::Vector(v.canonicalize()),
            t @ (TypeKind::Address
            | TypeKind::Boolean
            | TypeKind::Field
            | TypeKind::Group
            | TypeKind::Integer(_)
            | TypeKind::Identifier
            | TypeKind::DynRecord
            | TypeKind::Scalar
            | TypeKind::Signature
            | TypeKind::String
            | TypeKind::Numeric
            | TypeKind::Unit
            | TypeKind::Err) => t,
        }
    }
}

impl Canonicalize for ArrayType {
    fn canonicalize(self) -> Self {
        let Self { element_type, length } = self;
        Self { element_type: element_type.canonicalize(), length: length.canonicalize() }
    }
}

impl Canonicalize for CompositeType {
    fn canonicalize(self) -> Self {
        let Self { path, const_arguments } = self;
        Self { path: path.canonicalize(), const_arguments: const_arguments.canonicalize() }
    }
}

impl Canonicalize for FutureType {
    fn canonicalize(self) -> Self {
        let Self { inputs, location, is_explicit } = self;
        Self { inputs: inputs.canonicalize(), location, is_explicit }
    }
}

impl Canonicalize for MappingType {
    fn canonicalize(self) -> Self {
        let Self { key, value } = self;
        Self { key: key.canonicalize(), value: value.canonicalize() }
    }
}

impl Canonicalize for OptionalType {
    fn canonicalize(self) -> Self {
        let Self { inner } = self;
        Self { inner: inner.canonicalize() }
    }
}

impl Canonicalize for TupleType {
    fn canonicalize(self) -> Self {
        let Self { elements } = self;
        Self { elements: elements.canonicalize() }
    }
}

impl Canonicalize for VectorType {
    fn canonicalize(self) -> Self {
        let Self { element_type } = self;
        Self { element_type: element_type.canonicalize() }
    }
}

impl Canonicalize for Expression {
    fn canonicalize(self) -> Self {
        match self {
            Expression::ArrayAccess(n) => Expression::ArrayAccess(n.canonicalize()),
            Expression::Async(n) => Expression::Async(n.canonicalize()),
            Expression::Array(n) => Expression::Array(n.canonicalize()),
            Expression::Binary(n) => Expression::Binary(n.canonicalize()),
            Expression::Intrinsic(n) => Expression::Intrinsic(n.canonicalize()),
            Expression::Call(n) => Expression::Call(n.canonicalize()),
            Expression::DynamicOp(n) => Expression::DynamicOp(n.canonicalize()),
            Expression::Cast(n) => Expression::Cast(n.canonicalize()),
            Expression::Composite(n) => Expression::Composite(n.canonicalize()),
            Expression::Err(n) => Expression::Err(n.canonicalize()),
            Expression::Path(n) => Expression::Path(n.canonicalize()),
            Expression::Literal(n) => Expression::Literal(n.canonicalize()),
            Expression::MemberAccess(n) => Expression::MemberAccess(n.canonicalize()),
            Expression::Repeat(n) => Expression::Repeat(n.canonicalize()),
            Expression::Ternary(n) => Expression::Ternary(n.canonicalize()),
            Expression::Tuple(n) => Expression::Tuple(n.canonicalize()),
            Expression::TupleAccess(n) => Expression::TupleAccess(n.canonicalize()),
            Expression::Unary(n) => Expression::Unary(n.canonicalize()),
            Expression::Unit(n) => Expression::Unit(n.canonicalize()),
        }
    }
}

impl Canonicalize for ArrayAccess {
    fn canonicalize(self) -> Self {
        let Self { array, index, span: _, id: _ } = self;
        Self { array: array.canonicalize(), index: index.canonicalize(), span: Span::default(), id: NodeID::default() }
    }
}

impl Canonicalize for AsyncExpression {
    fn canonicalize(self) -> Self {
        let Self { block, span: _, id: _ } = self;
        Self { block: block.canonicalize(), span: Span::default(), id: NodeID::default() }
    }
}

impl Canonicalize for ArrayExpression {
    fn canonicalize(self) -> Self {
        let Self { elements, span: _, id: _ } = self;
        Self { elements: elements.canonicalize(), span: Span::default(), id: NodeID::default() }
    }
}

impl Canonicalize for BinaryExpression {
    fn canonicalize(self) -> Self {
        let Self { left, right, op, span: _, id: _ } = self;
        Self {
            left: left.canonicalize(),
            right: right.canonicalize(),
            op,
            span: Span::default(),
            id: NodeID::default(),
        }
    }
}

impl Canonicalize for IntrinsicExpression {
    fn canonicalize(self) -> Self {
        let Self { name, type_parameters, input_types, return_types, arguments, span: _, id: _ } = self;
        Self {
            name,
            type_parameters: type_parameters.into_iter().map(|(ty, _)| (ty.canonicalize(), Span::default())).collect(),
            input_types: input_types
                .into_iter()
                .map(|(mode, ty, _)| (mode, ty.canonicalize(), Span::default()))
                .collect(),
            return_types: return_types
                .into_iter()
                .map(|(mode, ty, _)| (mode, ty.canonicalize(), Span::default()))
                .collect(),
            arguments: arguments.canonicalize(),
            span: Span::default(),
            id: NodeID::default(),
        }
    }
}

impl Canonicalize for CallExpression {
    fn canonicalize(self) -> Self {
        let Self { function, const_arguments, arguments, span: _, id: _ } = self;
        Self {
            function: function.canonicalize(),
            const_arguments: const_arguments.canonicalize(),
            arguments: arguments.canonicalize(),
            span: Span::default(),
            id: NodeID::default(),
        }
    }
}

impl Canonicalize for DynamicOpExpression {
    fn canonicalize(self) -> Self {
        let Self { interface, target_program, network, kind, span: _, id: _ } = self;
        Self {
            interface: interface.canonicalize(),
            target_program: target_program.canonicalize(),
            network: network.canonicalize(),
            kind: kind.canonicalize(),
            span: Span::default(),
            id: NodeID::default(),
        }
    }
}

impl Canonicalize for DynamicOpKind {
    fn canonicalize(self) -> Self {
        match self {
            DynamicOpKind::Call { function, arguments } => {
                DynamicOpKind::Call { function: function.canonicalize(), arguments: arguments.canonicalize() }
            }
            DynamicOpKind::Read { storage } => DynamicOpKind::Read { storage: storage.canonicalize() },
            DynamicOpKind::Op { member, op, arguments } => DynamicOpKind::Op {
                member: member.canonicalize(),
                op: op.canonicalize(),
                arguments: arguments.canonicalize(),
            },
        }
    }
}

impl Canonicalize for CastExpression {
    fn canonicalize(self) -> Self {
        let Self { expression, type_, span: _, id: _ } = self;
        Self {
            expression: expression.canonicalize(),
            type_: type_.canonicalize(),
            span: Span::default(),
            id: NodeID::default(),
        }
    }
}

impl Canonicalize for CompositeExpression {
    fn canonicalize(self) -> Self {
        let Self { path, const_arguments, members, base, span: _, id: _ } = self;
        Self {
            path: path.canonicalize(),
            const_arguments: const_arguments.canonicalize(),
            members: members.canonicalize(),
            base: base.canonicalize(),
            span: Span::default(),
            id: NodeID::default(),
        }
    }
}

impl Canonicalize for CompositeFieldInitializer {
    fn canonicalize(self) -> Self {
        let Self { identifier, expression, span: _, id: _ } = self;
        Self {
            identifier: identifier.canonicalize(),
            expression: expression.canonicalize(),
            span: Span::default(),
            id: NodeID::default(),
        }
    }
}

impl Canonicalize for ErrExpression {
    fn canonicalize(self) -> Self {
        let Self { span: _, id: _ } = self;
        Self { span: Span::default(), id: NodeID::default() }
    }
}

impl Canonicalize for Literal {
    fn canonicalize(self) -> Self {
        let Self { variant, span: _, id: _ } = self;
        // `variant: LiteralVariant` carries the identity (Address(String), Integer(IntegerType, String), …).
        Self { variant, span: Span::default(), id: NodeID::default() }
    }
}

impl Canonicalize for MemberAccess {
    fn canonicalize(self) -> Self {
        let Self { inner, name, span: _, id: _ } = self;
        Self { inner: inner.canonicalize(), name: name.canonicalize(), span: Span::default(), id: NodeID::default() }
    }
}

impl Canonicalize for RepeatExpression {
    fn canonicalize(self) -> Self {
        let Self { expr, count, span: _, id: _ } = self;
        Self { expr: expr.canonicalize(), count: count.canonicalize(), span: Span::default(), id: NodeID::default() }
    }
}

impl Canonicalize for TernaryExpression {
    fn canonicalize(self) -> Self {
        let Self { condition, if_true, if_false, span: _, id: _ } = self;
        Self {
            condition: condition.canonicalize(),
            if_true: if_true.canonicalize(),
            if_false: if_false.canonicalize(),
            span: Span::default(),
            id: NodeID::default(),
        }
    }
}

impl Canonicalize for TupleExpression {
    fn canonicalize(self) -> Self {
        let Self { elements, span: _, id: _ } = self;
        Self { elements: elements.canonicalize(), span: Span::default(), id: NodeID::default() }
    }
}

impl Canonicalize for TupleAccess {
    fn canonicalize(self) -> Self {
        let Self { tuple, index, span: _, id: _ } = self;
        Self { tuple: tuple.canonicalize(), index, span: Span::default(), id: NodeID::default() }
    }
}

impl Canonicalize for UnaryExpression {
    fn canonicalize(self) -> Self {
        let Self { receiver, op, span: _, id: _ } = self;
        Self { receiver: receiver.canonicalize(), op, span: Span::default(), id: NodeID::default() }
    }
}

impl Canonicalize for UnitExpression {
    fn canonicalize(self) -> Self {
        let Self { span: _, id: _ } = self;
        Self { span: Span::default(), id: NodeID::default() }
    }
}

impl Canonicalize for Block {
    fn canonicalize(self) -> Self {
        let Self { statements, span: _, id: _ } = self;
        Self { statements: statements.canonicalize(), span: Span::default(), id: NodeID::default() }
    }
}

impl Canonicalize for Statement {
    fn canonicalize(self) -> Self {
        match self {
            Statement::Assert(s) => Statement::Assert(s.canonicalize()),
            Statement::Assign(s) => Statement::Assign(s.canonicalize()),
            Statement::Block(s) => Statement::Block(s.canonicalize()),
            Statement::Conditional(s) => Statement::Conditional(s.canonicalize()),
            Statement::Const(s) => Statement::Const(s.canonicalize()),
            Statement::Definition(s) => Statement::Definition(s.canonicalize()),
            Statement::Expression(s) => Statement::Expression(s.canonicalize()),
            Statement::Iteration(s) => Statement::Iteration(s.canonicalize()),
            Statement::Return(s) => Statement::Return(s.canonicalize()),
        }
    }
}

impl Canonicalize for AssertStatement {
    fn canonicalize(self) -> Self {
        let Self { variant, span: _, id: _ } = self;
        Self { variant: variant.canonicalize(), span: Span::default(), id: NodeID::default() }
    }
}

impl Canonicalize for AssertVariant {
    fn canonicalize(self) -> Self {
        match self {
            AssertVariant::Assert(e) => AssertVariant::Assert(e.canonicalize()),
            AssertVariant::AssertEq(a, b) => AssertVariant::AssertEq(a.canonicalize(), b.canonicalize()),
            AssertVariant::AssertNeq(a, b) => AssertVariant::AssertNeq(a.canonicalize(), b.canonicalize()),
        }
    }
}

impl Canonicalize for AssignStatement {
    fn canonicalize(self) -> Self {
        let Self { place, value, span: _, id: _ } = self;
        Self { place: place.canonicalize(), value: value.canonicalize(), span: Span::default(), id: NodeID::default() }
    }
}

impl Canonicalize for ConditionalStatement {
    fn canonicalize(self) -> Self {
        let Self { condition, then, otherwise, span: _, id: _ } = self;
        Self {
            condition: condition.canonicalize(),
            then: then.canonicalize(),
            otherwise: otherwise.canonicalize(),
            span: Span::default(),
            id: NodeID::default(),
        }
    }
}

impl Canonicalize for ConstDeclaration {
    fn canonicalize(self) -> Self {
        let Self { is_exported, place, type_, value, span: _, id: _ } = self;
        Self {
            is_exported,
            place: place.canonicalize(),
            type_: type_.canonicalize(),
            value: value.canonicalize(),
            span: Span::default(),
            id: NodeID::default(),
        }
    }
}

impl Canonicalize for DefinitionStatement {
    fn canonicalize(self) -> Self {
        let Self { place, type_, value, span: _, id: _ } = self;
        Self {
            place: place.canonicalize(),
            type_: type_.canonicalize(),
            value: value.canonicalize(),
            span: Span::default(),
            id: NodeID::default(),
        }
    }
}

impl Canonicalize for DefinitionPlace {
    fn canonicalize(self) -> Self {
        match self {
            DefinitionPlace::Single(id) => DefinitionPlace::Single(id.canonicalize()),
            DefinitionPlace::Multiple(ids) => DefinitionPlace::Multiple(ids.canonicalize()),
        }
    }
}

impl Canonicalize for ExpressionStatement {
    fn canonicalize(self) -> Self {
        let Self { expression, span: _, id: _ } = self;
        Self { expression: expression.canonicalize(), span: Span::default(), id: NodeID::default() }
    }
}

impl Canonicalize for IterationStatement {
    fn canonicalize(self) -> Self {
        let Self { variable, type_, start, stop, inclusive, block, span: _, id: _ } = self;
        Self {
            variable: variable.canonicalize(),
            type_: type_.canonicalize(),
            start: start.canonicalize(),
            stop: stop.canonicalize(),
            inclusive,
            block: block.canonicalize(),
            span: Span::default(),
            id: NodeID::default(),
        }
    }
}

impl Canonicalize for ReturnStatement {
    fn canonicalize(self) -> Self {
        let Self { expression, span: _, id: _ } = self;
        Self { expression: expression.canonicalize(), span: Span::default(), id: NodeID::default() }
    }
}
