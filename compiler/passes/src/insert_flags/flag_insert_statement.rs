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

use crate::{FlagInserter, VariableType};

use leo_ast::*;

use std::mem;

impl StatementReconstructor for FlagInserter<'_> {
    fn reconstruct_block(&mut self, mut input: Block) -> (Block, Self::AdditionalOutput) {
        self.in_scope(input.id(), |slf| {
            let mut statements = Vec::with_capacity(input.statements.len());
            for statement in input.statements.into_iter() {
                let (new_statement, to_defines) = slf.reconstruct_statement(statement);

                for to_define in to_defines {
                    let (variable_type, declaration_type) = if slf.is_const(&to_define.expr) {
                        (VariableType::Const, DeclarationType::Const)
                    } else {
                        (VariableType::Mut, DeclarationType::Let)
                    };
                    // Construct a definition.
                    let name_identifier = Expression::Identifier(Identifier {
                        name: to_define.name,
                        span: to_define.span,
                        id: slf.node_builder.next_id(),
                    });
                    slf.insert_variable(to_define.name, to_define.first_type.clone(), to_define.span, variable_type);
                    let flag_identifier = Expression::Identifier(Identifier {
                        name: to_define.flag,
                        span: to_define.span,
                        id: slf.node_builder.next_id(),
                    });
                    slf.type_table.insert(flag_identifier.id(), Type::Boolean);
                    let mut flag_identifier2 = flag_identifier.clone();
                    flag_identifier2.set_id(slf.node_builder.next_id());
                    slf.insert_variable(to_define.flag, Type::Boolean, to_define.span, variable_type);
                    slf.type_table.insert(flag_identifier2.id(), Type::Boolean);
                    let not_flag = Expression::Unary(UnaryExpression {
                        receiver: Box::new(flag_identifier2),
                        op: UnaryOperation::Not,
                        span: to_define.span,
                        id: slf.node_builder.next_id(),
                    });
                    slf.type_table.insert(not_flag.id(), Type::Boolean);
                    let place = Expression::Tuple(TupleExpression {
                        elements: vec![name_identifier, flag_identifier],
                        span: to_define.span,
                        id: slf.node_builder.next_id(),
                    });
                    let type_ = Type::Tuple(TupleType::new(vec![to_define.first_type, Type::Boolean]));
                    slf.type_table.insert(to_define.expr.id(), type_.clone());
                    let def = Statement::Definition(DefinitionStatement {
                        declaration_type,
                        place,
                        type_,
                        value: to_define.expr,
                        span: to_define.span,
                        id: slf.node_builder.next_id(),
                    });
                    let assert_ = Statement::Assert(AssertStatement {
                        variant: AssertVariant::Assert(not_flag),
                        span: to_define.span,
                        id: slf.node_builder.next_id(),
                    });
                    statements.push(def);
                    statements.push(assert_);
                }

                statements.push(new_statement);
            }

            input.statements = statements;

            (input, Default::default())
        })
    }

    fn reconstruct_iteration(&mut self, mut input: IterationStatement) -> (Statement, Self::AdditionalOutput) {
        self.in_scope(input.id(), |slf| {
            let (start, mut statements) = slf.reconstruct_expression(input.start);
            let (stop, statements2) = slf.reconstruct_expression(input.stop);
            let (block, statements3) = slf.reconstruct_block(input.block);
            input.start = start;
            input.stop = stop;
            input.block = block;
            statements.extend(statements2);
            statements.extend(statements3);
            (Statement::Iteration(Box::new(input)), statements)
        })
    }

    fn reconstruct_assert(&mut self, mut input: AssertStatement) -> (Statement, Self::AdditionalOutput) {
        let to_defines;

        match &mut input.variant {
            AssertVariant::Assert(expr) => {
                let (expr2, to_defines2) = self.reconstruct_expression(mem::take(expr));
                to_defines = to_defines2;
                *expr = expr2;
            }
            AssertVariant::AssertEq(lhs, rhs) | AssertVariant::AssertNeq(lhs, rhs) => {
                let (lhs2, mut to_defines_lhs) = self.reconstruct_expression(mem::take(lhs));
                *lhs = lhs2;
                let (rhs2, to_defines_rhs) = self.reconstruct_expression(mem::take(rhs));
                *rhs = rhs2;
                to_defines_lhs.extend(to_defines_rhs);
                to_defines = to_defines_lhs;
            }
        }

        (Statement::Assert(input), to_defines)
    }

    fn reconstruct_assign(&mut self, mut input: AssignStatement) -> (Statement, Self::AdditionalOutput) {
        let (value, to_defines) = self.reconstruct_expression(input.value);
        input.value = value;
        (Statement::Assign(Box::new(input)), to_defines)
    }

    fn reconstruct_conditional(&mut self, mut input: ConditionalStatement) -> (Statement, Self::AdditionalOutput) {
        let (expr, mut to_defines) = self.reconstruct_expression(input.condition);
        input.condition = expr;
        let (block, to_defines2) = self.reconstruct_block(input.then);
        assert!(to_defines2.is_empty());
        input.then = block;
        if let Some(otherwise) = input.otherwise {
            let (block, to_defines3) = self.reconstruct_statement(*otherwise);
            // If the `otherwise` is a conditional, it may need external definitions.
            to_defines.extend(to_defines3);
            input.otherwise = Some(Box::new(block));
        }
        (Statement::Conditional(input), to_defines)
    }

    fn reconstruct_console(&mut self, mut input: ConsoleStatement) -> (Statement, Self::AdditionalOutput) {
        let to_defines;
        match &mut input.function {
            ConsoleFunction::Assert(expr) => {
                let (expr2, to_defines2) = self.reconstruct_expression(mem::take(expr));
                *expr = expr2;
                to_defines = to_defines2;
            }
            ConsoleFunction::AssertEq(lhs, rhs) | ConsoleFunction::AssertNeq(lhs, rhs) => {
                let (lhs2, mut to_defines_lhs) = self.reconstruct_expression(mem::take(lhs));
                *lhs = lhs2;
                let (rhs2, to_defines_rhs) = self.reconstruct_expression(mem::take(rhs));
                *rhs = rhs2;
                to_defines_lhs.extend(to_defines_rhs);
                to_defines = to_defines_lhs;
            }
        }

        (Statement::Console(input), to_defines)
    }

    fn reconstruct_const(&mut self, mut input: ConstDeclaration) -> (Statement, Self::AdditionalOutput) {
        let (value, to_defines) = self.reconstruct_expression(input.value);
        input.value = value;
        (Statement::Const(input), to_defines)
    }

    fn reconstruct_definition(&mut self, mut input: DefinitionStatement) -> (Statement, Self::AdditionalOutput) {
        let (value, to_defines) = self.reconstruct_expression(input.value);
        input.value = value;
        (Statement::Definition(input), to_defines)
    }

    fn reconstruct_expression_statement(
        &mut self,
        mut input: ExpressionStatement,
    ) -> (Statement, Self::AdditionalOutput) {
        let (expression, to_defines) = self.reconstruct_expression(input.expression);
        input.expression = expression;
        (Statement::Expression(input), to_defines)
    }

    fn reconstruct_return(&mut self, mut input: ReturnStatement) -> (Statement, Self::AdditionalOutput) {
        let (expression, to_defines) = self.reconstruct_expression(input.expression);
        input.expression = expression;
        (Statement::Return(input), to_defines)
    }
}
