// Copyright (C) 2019-2025 Provable Inc.
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

use leo_ast::{
    AssociatedFunctionExpression,
    Block,
    Constructor,
    DefinitionPlace,
    DefinitionStatement,
    Expression,
    ExpressionReconstructor,
    Literal,
    LocatorExpression,
    MemberAccess,
    Node,
    ProgramId,
    ProgramReconstructor,
    Statement,
    StatementReconstructor,
    TypeReconstructor,
};

/// A `Normalizer` normalizes all NodeID and Span values in the AST.
/// Note: This implementation is **NOT** complete and only normalizes up to `Constructor`.
pub struct Normalizer;

impl ExpressionReconstructor for Normalizer {
    type AdditionalOutput = ();

    fn reconstruct_expression(&mut self, input: Expression) -> (Expression, Self::AdditionalOutput) {
        // Set the ID and span to 0.
        let input = normalize_node(input);
        // Reconstruct the sub-expressions.
        match input {
            Expression::AssociatedConstant(constant) => self.reconstruct_associated_constant(constant),
            Expression::AssociatedFunction(function) => self.reconstruct_associated_function(function),
            Expression::Array(array) => self.reconstruct_array(array),
            Expression::ArrayAccess(access) => self.reconstruct_array_access(*access),
            Expression::Binary(binary) => self.reconstruct_binary(*binary),
            Expression::Call(call) => self.reconstruct_call(*call),
            Expression::Cast(cast) => self.reconstruct_cast(*cast),
            Expression::Struct(struct_) => self.reconstruct_struct_init(struct_),
            Expression::Err(err) => self.reconstruct_err(err),
            Expression::Identifier(identifier) => self.reconstruct_identifier(identifier),
            Expression::Literal(value) => self.reconstruct_literal(value),
            Expression::Locator(locator) => self.reconstruct_locator(locator),
            Expression::MemberAccess(access) => self.reconstruct_member_access(*access),
            Expression::Repeat(repeat) => self.reconstruct_repeat(*repeat),
            Expression::Ternary(ternary) => self.reconstruct_ternary(*ternary),
            Expression::Tuple(tuple) => self.reconstruct_tuple(tuple),
            Expression::TupleAccess(access) => self.reconstruct_tuple_access(*access),
            Expression::Unary(unary) => self.reconstruct_unary(*unary),
            Expression::Unit(unit) => self.reconstruct_unit(unit),
        }
    }

    fn reconstruct_associated_function(
        &mut self,
        input: AssociatedFunctionExpression,
    ) -> (Expression, Self::AdditionalOutput) {
        (
            AssociatedFunctionExpression {
                arguments: input.arguments.into_iter().map(|arg| self.reconstruct_expression(arg).0).collect(),
                name: {
                    let mut name = input.name;
                    name.set_id(0);
                    name.set_span(Default::default());
                    name
                },
                variant: normalize_node(input.variant),
                ..input
            }
            .into(),
            Default::default(),
        )
    }

    fn reconstruct_literal(&mut self, input: Literal) -> (Expression, Self::AdditionalOutput) {
        let input = normalize_node(input);
        (Expression::Literal(input), Default::default())
    }

    fn reconstruct_locator(&mut self, input: LocatorExpression) -> (Expression, Self::AdditionalOutput) {
        let input = normalize_node(input);
        (
            LocatorExpression {
                program: ProgramId {
                    name: normalize_node(input.program.name),
                    network: normalize_node(input.program.network),
                },
                ..input
            }
            .into(),
            Default::default(),
        )
    }

    fn reconstruct_member_access(&mut self, input: MemberAccess) -> (Expression, Self::AdditionalOutput) {
        (
            MemberAccess {
                inner: self.reconstruct_expression(input.inner).0,
                name: normalize_node(input.name),
                ..input
            }
            .into(),
            Default::default(),
        )
    }
}

impl StatementReconstructor for Normalizer {
    fn reconstruct_statement(&mut self, input: leo_ast::Statement) -> (leo_ast::Statement, Self::AdditionalOutput) {
        // Set the ID and span to 0.
        let input = normalize_node(input);
        // Reconstruct the sub-components.
        match input {
            Statement::Assert(assert) => self.reconstruct_assert(assert),
            Statement::Assign(stmt) => self.reconstruct_assign(*stmt),
            Statement::Block(stmt) => {
                let (stmt, output) = self.reconstruct_block(stmt);
                (stmt.into(), output)
            }
            Statement::Conditional(stmt) => self.reconstruct_conditional(stmt),
            Statement::Const(stmt) => self.reconstruct_const(stmt),
            Statement::Definition(stmt) => self.reconstruct_definition(stmt),
            Statement::Expression(stmt) => self.reconstruct_expression_statement(stmt),
            Statement::Iteration(stmt) => self.reconstruct_iteration(*stmt),
            Statement::Return(stmt) => self.reconstruct_return(stmt),
        }
    }

    fn reconstruct_block(&mut self, input: Block) -> (Block, Self::AdditionalOutput) {
        let input = normalize_node(input);
        (
            Block {
                statements: input.statements.into_iter().map(|s| self.reconstruct_statement(s).0).collect(),
                ..input
            },
            Default::default(),
        )
    }

    fn reconstruct_definition(&mut self, input: DefinitionStatement) -> (Statement, Self::AdditionalOutput) {
        (
            DefinitionStatement {
                value: self.reconstruct_expression(input.value).0,
                place: match input.place {
                    DefinitionPlace::Single(identifier) => DefinitionPlace::Single(normalize_node(identifier)),
                    DefinitionPlace::Multiple(identifiers) => {
                        DefinitionPlace::Multiple(identifiers.into_iter().map(normalize_node).collect())
                    }
                },
                type_: input.type_.map(|t| self.reconstruct_type(t).0),
                ..input
            }
            .into(),
            Default::default(),
        )
    }
}

impl TypeReconstructor for Normalizer {}

impl ProgramReconstructor for Normalizer {
    fn reconstruct_constructor(&mut self, input: Constructor) -> Constructor {
        let input = normalize_node(input);
        Constructor {
            annotations: input.annotations,
            block: self.reconstruct_block(input.block).0,
            span: input.span,
            id: input.id,
        }
    }
}

// A helper function to normalize a `Node`.
fn normalize_node<N: Node>(mut node: N) -> N {
    node.set_id(0);
    node.set_span(Default::default());
    node
}
