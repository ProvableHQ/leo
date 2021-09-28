// Copyright (C) 2019-2021 Aleo Systems Inc.
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

use std::cell::Cell;

use leo_asg::*;
use leo_errors::Result;

pub struct AsgPrinting<'a, 'b> {
    program: &'b Program<'a>,
}

impl<'a, 'b> ExpressionVisitor<'a> for AsgPrinting<'a, 'b> {
    fn visit_expression(&mut self, input: &Cell<&'a Expression<'a>>) -> VisitResult {
        println!("expression");
        VisitResult::VisitChildren
    }

    fn visit_array_access(&mut self, input: &ArrayAccessExpression<'a>) -> VisitResult {
        println!("array_access");
        VisitResult::VisitChildren
    }

    fn visit_array_init(&mut self, input: &ArrayInitExpression<'a>) -> VisitResult {
        println!("array_init");
        VisitResult::VisitChildren
    }

    fn visit_array_inline(&mut self, input: &ArrayInlineExpression<'a>) -> VisitResult {
        println!("array_inline");
        VisitResult::VisitChildren
    }

    fn visit_array_range_access(&mut self, input: &ArrayRangeAccessExpression<'a>) -> VisitResult {
        println!("range_access");
        VisitResult::VisitChildren
    }

    fn visit_binary(&mut self, input: &BinaryExpression<'a>) -> VisitResult {
        println!("binary");
        VisitResult::VisitChildren
    }

    fn visit_call(&mut self, input: &CallExpression<'a>) -> VisitResult {
        println!("call");
        VisitResult::VisitChildren
    }

    fn visit_circuit_access(&mut self, input: &CircuitAccessExpression<'a>) -> VisitResult {
        println!("circuit_access");
        VisitResult::VisitChildren
    }

    fn visit_circuit_init(&mut self, input: &CircuitInitExpression<'a>) -> VisitResult {
        println!("circuit_init");
        VisitResult::VisitChildren
    }

    fn visit_ternary_expression(&mut self, input: &TernaryExpression<'a>) -> VisitResult {
        println!("ternary_expression");
        VisitResult::VisitChildren
    }

    fn visit_cast_expression(&mut self, input: &CastExpression<'a>) -> VisitResult {
        println!("cast_expression");
        VisitResult::VisitChildren
    }

    fn visit_lengthof_expression(&mut self, input: &LengthOfExpression<'a>) -> VisitResult {
        println!("lengthof_expression");
        VisitResult::VisitChildren
    }

    fn visit_constant(&mut self, input: &Constant<'a>) -> VisitResult {
        println!("constant");
        VisitResult::VisitChildren
    }

    fn visit_tuple_access(&mut self, input: &TupleAccessExpression<'a>) -> VisitResult {
        println!("tuple_access");
        VisitResult::VisitChildren
    }

    fn visit_tuple_init(&mut self, input: &TupleInitExpression<'a>) -> VisitResult {
        println!("tuple_init");
        VisitResult::VisitChildren
    }

    fn visit_unary(&mut self, input: &UnaryExpression<'a>) -> VisitResult {
        println!("unary");
        VisitResult::VisitChildren
    }

    fn visit_variable_ref(&mut self, input: &VariableRef<'a>) -> VisitResult {
        println!("variable_ref");
        VisitResult::VisitChildren
    }
}

impl<'a, 'b> StatementVisitor<'a> for AsgPrinting<'a, 'b> {
    fn visit_statement(&mut self, input: &Cell<&'a Statement<'a>>) -> VisitResult {
        println!("statement");
        VisitResult::VisitChildren
    }

    // left = Some(ArrayIndex.0) always if AssignAccess::ArrayIndex. if member/tuple, always None
    fn visit_assign_access(&mut self, input: &AssignAccess<'a>) -> VisitResult {
        println!("assign_access");
        VisitResult::VisitChildren
    }

    fn visit_assign(&mut self, input: &AssignStatement<'a>) -> VisitResult {
        println!("assign");
        VisitResult::VisitChildren
    }

    fn visit_block(&mut self, input: &BlockStatement<'a>) -> VisitResult {
        println!("block");
        VisitResult::VisitChildren
    }

    fn visit_conditional_statement(&mut self, input: &ConditionalStatement<'a>) -> VisitResult {
        println!("conditional_statement");
        VisitResult::VisitChildren
    }

    fn visit_formatted_string(&mut self, input: &ConsoleArgs<'a>) -> VisitResult {
        println!("formatted_string");
        VisitResult::VisitChildren
    }

    fn visit_console(&mut self, input: &ConsoleStatement<'a>) -> VisitResult {
        println!("console");
        VisitResult::VisitChildren
    }

    fn visit_definition(&mut self, input: &DefinitionStatement<'a>) -> VisitResult {
        println!("definition");
        VisitResult::VisitChildren
    }

    fn visit_expression_statement(&mut self, input: &ExpressionStatement<'a>) -> VisitResult {
        println!("expression_statement");
        VisitResult::VisitChildren
    }

    fn visit_iteration(&mut self, input: &IterationStatement<'a>) -> VisitResult {
        println!("iteration");
        VisitResult::VisitChildren
    }

    fn visit_return(&mut self, input: &ReturnStatement<'a>) -> VisitResult {
        println!("return");
        VisitResult::VisitChildren
    }
}

impl<'a, 'b> ProgramVisitor<'a> for AsgPrinting<'a, 'b> {
    fn visit_function(&mut self, input: &'a Function<'a>) -> VisitResult {
        println!("function");
        VisitResult::VisitChildren
    }

    fn visit_circuit_member(&mut self, input: &CircuitMember<'a>) -> VisitResult {
        println!("circuit_member");
        VisitResult::VisitChildren
    }

    fn visit_circuit(&mut self, input: &'a Circuit<'a>) -> VisitResult {
        println!("circuit");
        VisitResult::VisitChildren
    }

    fn visit_global_const(&mut self, input: &'a DefinitionStatement<'a>) -> VisitResult {
        println!("global_const");
        VisitResult::VisitChildren
    }

    fn visit_program(&mut self, input: &Program<'a>) -> VisitResult {
        println!("program");
        VisitResult::VisitChildren
    }
}

impl<'a, 'b> AsgPass<'a> for AsgPrinting<'a, 'b> {
    fn do_pass(asg: Program<'a>) -> Result<Program<'a>> {
        let pass = AsgPrinting { program: &asg };
        let mut director = VisitorDirector::new(pass);
        director.visit_program(&asg).ok();
        Ok(asg)
    }
}
