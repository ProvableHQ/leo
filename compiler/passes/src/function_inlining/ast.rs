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

use super::FunctionInliningVisitor;
use crate::{Replacer, SsaFormingInput, static_single_assignment::visitor::SsaFormingVisitor};

use leo_ast::*;

use indexmap::IndexMap;
use itertools::Itertools;

impl AstReconstructor for FunctionInliningVisitor<'_> {
    type AdditionalInput = ();
    type AdditionalOutput = Vec<Statement>;

    /* Expressions */
    fn reconstruct_call(&mut self, input: CallExpression, _additional: &()) -> (Expression, Self::AdditionalOutput) {
        // Type checking guarantees that only functions local to the program scope can be inlined.
        if input.function.expect_global_location().program != self.program {
            return (input.into(), Default::default());
        }

        // Lookup the reconstructed callee function.
        // Since this pass processes functions in post-order, the callee function is guaranteed to exist in `self.reconstructed_functions`
        let function_location = input.function.expect_global_location();
        let (_, callee) = self
            .reconstructed_functions
            .iter()
            .find(|(path, _)| *path == function_location.path)
            .expect("guaranteed to exist due to post-order traversal of the call graph.");

        // Inline the callee function, if required, otherwise, return the call expression.
        match callee.variant {
            Variant::Inline => {
                // Construct a mapping from input variables of the callee function to arguments passed to the callee.
                let parameter_to_argument = callee
                    .input
                    .iter()
                    .map(|input| input.identifier().name)
                    .zip_eq(input.arguments)
                    .collect::<IndexMap<_, _>>();

                // Function to replace path expressions with their corresponding const argument or keep them unchanged.
                let replace_path = |expr: &Expression| match expr {
                    Expression::Path(path) => parameter_to_argument
                        .get(&path.identifier().name)
                        .map_or(Expression::Path(path.clone()), |expr| expr.clone()),
                    _ => expr.clone(),
                };

                // Replace path expressions with their corresponding const argument or keep them unchanged.
                let reconstructed_block = Replacer::new(replace_path, false /* refresh IDs */, self.state)
                    .reconstruct_block(callee.block.clone())
                    .0;

                // Run SSA formation on the inlined block and rename definitions. Renaming is necessary to avoid shadowing variables.
                let mut inlined_statements =
                    SsaFormingVisitor::new(self.state, SsaFormingInput { rename_defs: true }, self.program)
                        .consume_block(reconstructed_block);

                // If the inlined block returns a value, then use the value in place of the call expression; otherwise, use the unit expression.
                let result = match inlined_statements.last() {
                    Some(Statement::Return(_)) => {
                        // Note that this unwrap is safe since we know that the last statement is a return statement.
                        match inlined_statements.pop().unwrap() {
                            Statement::Return(ReturnStatement { expression, .. }) => expression,
                            _ => panic!("This branch checks that the last statement is a return statement."),
                        }
                    }
                    _ => {
                        let id = self.state.node_builder.next_id();
                        self.state.type_table.insert(id, Type::Unit);
                        UnitExpression { span: Default::default(), id }.into()
                    }
                };

                (result, inlined_statements)
            }
            Variant::Function
            | Variant::Script
            | Variant::AsyncFunction
            | Variant::Transition
            | Variant::AsyncTransition => (input.into(), Default::default()),
        }
    }

    /* Statements */
    fn reconstruct_assign(&mut self, _input: AssignStatement) -> (Statement, Self::AdditionalOutput) {
        panic!("`AssignStatement`s should not exist in the AST at this phase of compilation.")
    }

    /// Reconstructs the statements inside a basic block, accumulating any statements produced by function inlining.
    fn reconstruct_block(&mut self, block: Block) -> (Block, Self::AdditionalOutput) {
        let mut statements = Vec::with_capacity(block.statements.len());

        for statement in block.statements {
            let (reconstructed_statement, additional_statements) = self.reconstruct_statement(statement);
            statements.extend(additional_statements);
            statements.push(reconstructed_statement);
        }

        (Block { span: block.span, statements, id: block.id }, Default::default())
    }

    /// Flattening removes conditional statements from the program.
    fn reconstruct_conditional(&mut self, input: ConditionalStatement) -> (Statement, Self::AdditionalOutput) {
        if !self.is_async {
            panic!("`ConditionalStatement`s should not be in the AST at this phase of compilation.")
        } else {
            (
                ConditionalStatement {
                    condition: self.reconstruct_expression(input.condition, &()).0,
                    then: self.reconstruct_block(input.then).0,
                    otherwise: input.otherwise.map(|n| Box::new(self.reconstruct_statement(*n).0)),
                    span: input.span,
                    id: input.id,
                }
                .into(),
                Default::default(),
            )
        }
    }

    /// Reconstruct a definition statement by inlining any function calls.
    /// This function also segments tuple assignment statements into multiple assignment statements.
    fn reconstruct_definition(&mut self, mut input: DefinitionStatement) -> (Statement, Self::AdditionalOutput) {
        let (value, mut statements) = self.reconstruct_expression(input.value, &());
        match (input.place, value) {
            // If we just inlined the production of a tuple literal, we need multiple definition statements.
            (DefinitionPlace::Multiple(left), Expression::Tuple(right)) => {
                assert_eq!(left.len(), right.elements.len());
                for (identifier, rhs_value) in left.into_iter().zip(right.elements) {
                    let stmt = DefinitionStatement {
                        place: DefinitionPlace::Single(identifier),
                        type_: None,
                        value: rhs_value,
                        span: Default::default(),
                        id: self.state.node_builder.next_id(),
                    }
                    .into();

                    statements.push(stmt);
                }
                (Statement::dummy(), statements)
            }

            (place, value) => {
                input.value = value;
                input.place = place;
                (input.into(), statements)
            }
        }
    }

    /// Reconstructs expression statements by inlining any function calls.
    fn reconstruct_expression_statement(&mut self, input: ExpressionStatement) -> (Statement, Self::AdditionalOutput) {
        // Reconstruct the expression.
        // Note that type checking guarantees that the expression is a function call.
        let (expression, additional_statements) = self.reconstruct_expression(input.expression, &());

        // If the resulting expression is a unit expression, return a dummy statement.
        let statement = match expression {
            Expression::Unit(_) => Statement::dummy(),
            _ => ExpressionStatement { expression, ..input }.into(),
        };

        (statement, additional_statements)
    }

    /// Loop unrolling unrolls and removes iteration statements from the program.
    fn reconstruct_iteration(&mut self, _: IterationStatement) -> (Statement, Self::AdditionalOutput) {
        panic!("`IterationStatement`s should not be in the AST at this phase of compilation.");
    }
}
