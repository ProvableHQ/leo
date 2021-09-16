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

use super::*;
use crate::{accesses::*, expression::*, program::*, statement::*};

use std::{cell::Cell, marker::PhantomData};

pub struct VisitorDirector<'a, R: ExpressionVisitor<'a>> {
    visitor: R,
    lifetime: PhantomData<&'a ()>,
}

pub type ConcreteVisitResult = Result<(), ()>;

impl Into<ConcreteVisitResult> for VisitResult {
    fn into(self) -> ConcreteVisitResult {
        match self {
            VisitResult::VisitChildren => Ok(()),
            VisitResult::SkipChildren => Ok(()),
            VisitResult::Exit => Err(()),
        }
    }
}

impl<'a, R: ExpressionVisitor<'a>> VisitorDirector<'a, R> {
    pub fn new(visitor: R) -> Self {
        Self {
            visitor,
            lifetime: PhantomData,
        }
    }

    pub fn visitor(self) -> R {
        self.visitor
    }

    pub fn visit_expression(&mut self, input: &Cell<&'a Expression<'a>>) -> ConcreteVisitResult {
        match self.visitor.visit_expression(input) {
            VisitResult::VisitChildren => match input.get() {
                Expression::ArrayInit(e) => self.visit_array_init(e),
                Expression::ArrayInline(e) => self.visit_array_inline(e),
                Expression::Binary(e) => self.visit_binary(e),
                Expression::Call(e) => self.visit_call(e),
                Expression::CircuitInit(e) => self.visit_circuit_init(e),
                Expression::Ternary(e) => self.visit_ternary_expression(e),
                Expression::Cast(e) => self.visit_cast_expression(e),
                Expression::Access(e) => self.visit_access_expression(e),
                Expression::NamedType(e) => self.visit_named_type_expression(e),
                Expression::LengthOf(e) => self.visit_lengthof_expression(e),
                Expression::Constant(e) => self.visit_constant(e),
                Expression::TupleInit(e) => self.visit_tuple_init(e),
                Expression::Unary(e) => self.visit_unary(e),
                Expression::VariableRef(e) => self.visit_variable_ref(e),
            },
            x => x.into(),
        }
    }

    fn visit_opt_expression(&mut self, input: &Cell<Option<&'a Expression<'a>>>) -> ConcreteVisitResult {
        let interior = input.get().map(Cell::new);
        if let Some(interior) = interior.as_ref() {
            let result = self.visit_expression(interior);
            input.replace(Some(interior.get()));
            result
        } else {
            Ok(())
        }
    }

    pub fn visit_array_init(&mut self, input: &ArrayInitExpression<'a>) -> ConcreteVisitResult {
        match self.visitor.visit_array_init(input) {
            VisitResult::VisitChildren => {
                self.visit_expression(&input.element)?;
                Ok(())
            }
            x => x.into(),
        }
    }

    pub fn visit_array_inline(&mut self, input: &ArrayInlineExpression<'a>) -> ConcreteVisitResult {
        match self.visitor.visit_array_inline(input) {
            VisitResult::VisitChildren => {
                for (element, _) in input.elements.iter() {
                    self.visit_expression(element)?;
                }
                Ok(())
            }
            x => x.into(),
        }
    }

    pub fn visit_binary(&mut self, input: &BinaryExpression<'a>) -> ConcreteVisitResult {
        match self.visitor.visit_binary(input) {
            VisitResult::VisitChildren => {
                self.visit_expression(&input.left)?;
                self.visit_expression(&input.right)?;
                Ok(())
            }
            x => x.into(),
        }
    }

    pub fn visit_call(&mut self, input: &CallExpression<'a>) -> ConcreteVisitResult {
        match self.visitor.visit_call(input) {
            VisitResult::VisitChildren => {
                self.visit_opt_expression(&input.target)?;
                for argument in input.arguments.iter() {
                    self.visit_expression(argument)?;
                }
                Ok(())
            }
            x => x.into(),
        }
    }

    pub fn visit_circuit_init(&mut self, input: &CircuitInitExpression<'a>) -> ConcreteVisitResult {
        match self.visitor.visit_circuit_init(input) {
            VisitResult::VisitChildren => {
                for (_, argument) in input.values.iter() {
                    self.visit_expression(argument)?;
                }
                Ok(())
            }
            x => x.into(),
        }
    }

    pub fn visit_ternary_expression(&mut self, input: &TernaryExpression<'a>) -> ConcreteVisitResult {
        match self.visitor.visit_ternary_expression(input) {
            VisitResult::VisitChildren => {
                self.visit_expression(&input.condition)?;
                self.visit_expression(&input.if_true)?;
                self.visit_expression(&input.if_false)?;
                Ok(())
            }
            x => x.into(),
        }
    }

    pub fn visit_cast_expression(&mut self, input: &CastExpression<'a>) -> ConcreteVisitResult {
        match self.visitor.visit_cast_expression(input) {
            VisitResult::VisitChildren => {
                self.visit_expression(&input.inner)?;
                Ok(())
            }
            x => x.into(),
        }
    }

    pub fn visit_array_access(&mut self, input: &ArrayAccess<'a>) -> ConcreteVisitResult {
        match self.visitor.visit_array_access(input) {
            VisitResult::VisitChildren => {
                self.visit_expression(&input.array)?;
                self.visit_expression(&input.index)?;
                Ok(())
            }
            x => x.into(),
        }
    }
    
    pub fn visit_lengthof_expression(&mut self, input: &LengthOfExpression<'a>) -> ConcreteVisitResult {
        match self.visitor.visit_lengthof_expression(input) {
            VisitResult::VisitChildren => {
                self.visit_expression(&input.inner)?;
                Ok(())
            }
            x => x.into(),
        }
    }

    pub fn visit_array_range_access(&mut self, input: &ArrayRangeAccess<'a>) -> ConcreteVisitResult {
        match self.visitor.visit_array_range_access(input) {
            VisitResult::VisitChildren => {
                self.visit_expression(&input.array)?;
                self.visit_opt_expression(&input.left)?;
                self.visit_opt_expression(&input.right)?;
                Ok(())
            }
            x => x.into(),
        }
    }

    pub fn visit_circuit_access(&mut self, input: &CircuitAccess<'a>) -> ConcreteVisitResult {
        match self.visitor.visit_circuit_access(input) {
            VisitResult::VisitChildren => {
                self.visit_opt_expression(&input.target)?;
                Ok(())
            }
            x => x.into(),
        }
    }

    pub fn visit_named_access(&mut self, input: &NamedTypeAccess<'a>) -> ConcreteVisitResult {
        match self.visitor.visit_named_access(input) {
            VisitResult::VisitChildren => {
                self.visit_expression(&input.named_type)?;
                self.visit_expression(&input.access)?;
                Ok(())
            }
            x => x.into(),
        }
    }

    pub fn visit_tuple_access(&mut self, input: &TupleAccess<'a>) -> ConcreteVisitResult {
        match self.visitor.visit_tuple_access(input) {
            VisitResult::VisitChildren => {
                self.visit_expression(&input.tuple_ref)?;
                Ok(())
            }
            x => x.into(),
        }
    }

    pub fn visit_value_access(&mut self, input: &ValueAccess<'a>) -> ConcreteVisitResult {
        match self.visitor.visit_value_access(input) {
            VisitResult::VisitChildren => {
                self.visit_expression(&input.target)?;
                self.visit_expression(&input.access)?;
                Ok(())
            }
            x => x.into(),
        }
    }

    pub fn visit_access_expression(&mut self, input: &AccessExpression<'a>) -> ConcreteVisitResult {
        use AccessExpression::*;

        match input {
            Array(a) => self.visit_array_access(a),
            ArrayRange(a) => self.visit_array_range_access(a),
            Circuit(a) => self.visit_circuit_access(a),
            Named(a) => self.visit_named_access(a),
            Tuple(a) => self.visit_tuple_access(a),
            Value(a) => self.visit_value_access(a),
        }
    }

    pub fn visit_named_type_expression(&mut self, input: &NamedTypeExpression<'a>) -> ConcreteVisitResult {
        match self.visitor.visit_named_type_expression(input) {
            VisitResult::VisitChildren => Ok(()),
            x => x.into(),
        }
    }

    pub fn visit_constant(&mut self, input: &Constant<'a>) -> ConcreteVisitResult {
        self.visitor.visit_constant(input).into()
    }

    pub fn visit_tuple_init(&mut self, input: &TupleInitExpression<'a>) -> ConcreteVisitResult {
        match self.visitor.visit_tuple_init(input) {
            VisitResult::VisitChildren => {
                for argument in input.elements.iter() {
                    self.visit_expression(argument)?;
                }
                Ok(())
            }
            x => x.into(),
        }
    }

    pub fn visit_unary(&mut self, input: &UnaryExpression<'a>) -> ConcreteVisitResult {
        match self.visitor.visit_unary(input) {
            VisitResult::VisitChildren => {
                self.visit_expression(&input.inner)?;
                Ok(())
            }
            x => x.into(),
        }
    }

    pub fn visit_variable_ref(&mut self, input: &VariableRef<'a>) -> ConcreteVisitResult {
        self.visitor.visit_variable_ref(input).into()
    }
}

impl<'a, R: StatementVisitor<'a>> VisitorDirector<'a, R> {
    pub fn visit_statement(&mut self, input: &Cell<&'a Statement<'a>>) -> ConcreteVisitResult {
        match self.visitor.visit_statement(input) {
            VisitResult::VisitChildren => match input.get() {
                Statement::Assign(s) => self.visit_assign(s),
                Statement::Block(s) => self.visit_block(s),
                Statement::Conditional(s) => self.visit_conditional_statement(s),
                Statement::Console(s) => self.visit_console(s),
                Statement::Definition(s) => self.visit_definition(s),
                Statement::Expression(s) => self.visit_expression_statement(s),
                Statement::Iteration(s) => self.visit_iteration(s),
                Statement::Return(s) => self.visit_return(s),
                Statement::Empty(_) => Ok(()),
            },
            x => x.into(),
        }
    }

    fn visit_opt_statement(&mut self, input: &Cell<Option<&'a Statement<'a>>>) -> ConcreteVisitResult {
        let interior = input.get().map(Cell::new);
        if let Some(interior) = interior.as_ref() {
            let result = self.visit_statement(interior);
            input.replace(Some(interior.get()));
            result
        } else {
            Ok(())
        }
    }

    pub fn visit_assign_access(&mut self, input: &AssignAccess<'a>) -> ConcreteVisitResult {
        match self.visitor.visit_assign_access(input) {
            VisitResult::VisitChildren => {
                match input {
                    AssignAccess::ArrayRange(left, right) => {
                        self.visit_opt_expression(left)?;
                        self.visit_opt_expression(right)?;
                    }
                    AssignAccess::ArrayIndex(index) => self.visit_expression(index)?,
                    _ => (),
                }
                Ok(())
            }
            x => x.into(),
        }
    }

    pub fn visit_assign(&mut self, input: &AssignStatement<'a>) -> ConcreteVisitResult {
        match self.visitor.visit_assign(input) {
            VisitResult::VisitChildren => {
                for access in input.target_accesses.iter() {
                    self.visit_assign_access(access)?;
                }
                self.visit_expression(&input.value)?;
                Ok(())
            }
            x => x.into(),
        }
    }

    pub fn visit_block(&mut self, input: &BlockStatement<'a>) -> ConcreteVisitResult {
        match self.visitor.visit_block(input) {
            VisitResult::VisitChildren => {
                for statement in input.statements.iter() {
                    self.visit_statement(statement)?;
                }
                Ok(())
            }
            x => x.into(),
        }
    }

    pub fn visit_conditional_statement(&mut self, input: &ConditionalStatement<'a>) -> ConcreteVisitResult {
        match self.visitor.visit_conditional_statement(input) {
            VisitResult::VisitChildren => {
                self.visit_expression(&input.condition)?;
                self.visit_statement(&input.result)?;
                self.visit_opt_statement(&input.next)?;
                Ok(())
            }
            x => x.into(),
        }
    }

    pub fn visit_formatted_string(&mut self, input: &ConsoleArgs<'a>) -> ConcreteVisitResult {
        match self.visitor.visit_formatted_string(input) {
            VisitResult::VisitChildren => {
                for parameter in input.parameters.iter() {
                    self.visit_expression(parameter)?;
                }
                Ok(())
            }
            x => x.into(),
        }
    }

    pub fn visit_console(&mut self, input: &ConsoleStatement<'a>) -> ConcreteVisitResult {
        match self.visitor.visit_console(input) {
            VisitResult::VisitChildren => {
                match &input.function {
                    ConsoleFunction::Assert(e) => self.visit_expression(e)?,
                    ConsoleFunction::Error(f) | ConsoleFunction::Log(f) => self.visit_formatted_string(f)?,
                }
                Ok(())
            }
            x => x.into(),
        }
    }

    pub fn visit_definition(&mut self, input: &DefinitionStatement<'a>) -> ConcreteVisitResult {
        match self.visitor.visit_definition(input) {
            VisitResult::VisitChildren => {
                self.visit_expression(&input.value)?;
                Ok(())
            }
            x => x.into(),
        }
    }

    pub fn visit_expression_statement(&mut self, input: &ExpressionStatement<'a>) -> ConcreteVisitResult {
        match self.visitor.visit_expression_statement(input) {
            VisitResult::VisitChildren => {
                self.visit_expression(&input.expression)?;
                Ok(())
            }
            x => x.into(),
        }
    }

    pub fn visit_iteration(&mut self, input: &IterationStatement<'a>) -> ConcreteVisitResult {
        match self.visitor.visit_iteration(input) {
            VisitResult::VisitChildren => {
                self.visit_expression(&input.start)?;
                self.visit_expression(&input.stop)?;
                self.visit_statement(&input.body)?;
                Ok(())
            }
            x => x.into(),
        }
    }

    pub fn visit_return(&mut self, input: &ReturnStatement<'a>) -> ConcreteVisitResult {
        match self.visitor.visit_return(input) {
            VisitResult::VisitChildren => {
                self.visit_expression(&input.expression)?;
                Ok(())
            }
            x => x.into(),
        }
    }
}

impl<'a, R: ProgramVisitor<'a>> VisitorDirector<'a, R> {
    pub fn visit_function(&mut self, input: &'a Function<'a>) -> ConcreteVisitResult {
        match self.visitor.visit_function(input) {
            VisitResult::VisitChildren => {
                self.visit_opt_statement(&input.body)?;
                Ok(())
            }
            x => x.into(),
        }
    }

    pub fn visit_circuit_member(&mut self, input: &CircuitMember<'a>) -> ConcreteVisitResult {
        match self.visitor.visit_circuit_member(input) {
            VisitResult::VisitChildren => {
                if let CircuitMember::Function(f) = input {
                    self.visit_function(f)?;
                }
                Ok(())
            }
            x => x.into(),
        }
    }

    pub fn visit_circuit(&mut self, input: &'a Circuit<'a>) -> ConcreteVisitResult {
        match self.visitor.visit_circuit(input) {
            VisitResult::VisitChildren => {
                for (_, member) in input.members.borrow().iter() {
                    self.visit_circuit_member(member)?;
                }
                Ok(())
            }
            x => x.into(),
        }
    }

    pub fn visit_global_const(&mut self, input: &'a DefinitionStatement<'a>) -> ConcreteVisitResult {
        match self.visitor.visit_global_const(input) {
            VisitResult::VisitChildren => {
                self.visit_expression(&input.value)?;
                Ok(())
            }
            x => x.into(),
        }
    }

    pub fn visit_program(&mut self, input: &Program<'a>) -> ConcreteVisitResult {
        match self.visitor.visit_program(input) {
            VisitResult::VisitChildren => {
                for (_, import) in input.imported_modules.iter() {
                    self.visit_program(import)?;
                }
                for (_, function) in input.functions.iter() {
                    self.visit_function(function)?;
                }
                for (_, circuit) in input.circuits.iter() {
                    self.visit_circuit(circuit)?;
                }
                for (_, global_const) in input.global_consts.iter() {
                    self.visit_global_const(global_const)?;
                }
                Ok(())
            }
            x => x.into(),
        }
    }
}
