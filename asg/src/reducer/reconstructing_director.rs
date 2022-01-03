// Copyright (C) 2019-2022 Aleo Systems Inc.
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
use crate::{accesses::*, expression::*, program::*, statement::*, AsgContext};

/*
reconstructing director tries to maintain a normalized ASG but may require renormalization under the following circumstances:
* breaking strict reducer model (i.e. live mutations)
* dropping or duplicating branches
*/
pub struct ReconstructingDirector<'a, R: ReconstructingReducerExpression<'a>> {
    context: AsgContext<'a>,
    reducer: R,
}

impl<'a, R: ReconstructingReducerExpression<'a>> ReconstructingDirector<'a, R> {
    pub fn new(context: AsgContext<'a>, reducer: R) -> Self {
        Self { context, reducer }
    }

    pub fn reducer(self) -> R {
        self.reducer
    }

    pub fn reduce_expression(&mut self, input: &'a Expression<'a>) -> &'a Expression<'a> {
        let value = match input.clone() {
            Expression::Err(e) => self.reduce_err(e),
            Expression::ArrayInit(e) => self.reduce_array_init(e),
            Expression::ArrayInline(e) => self.reduce_array_inline(e),
            Expression::Binary(e) => self.reduce_binary(e),
            Expression::Call(e) => self.reduce_call(e),
            Expression::CircuitInit(e) => self.reduce_circuit_init(e),
            Expression::Ternary(e) => self.reduce_ternary_expression(e),
            Expression::Cast(e) => self.reduce_cast_expression(e),
            Expression::Access(e) => self.reduce_access_expression(e),
            Expression::Constant(e) => self.reduce_constant(e),
            Expression::TupleInit(e) => self.reduce_tuple_init(e),
            Expression::Unary(e) => self.reduce_unary(e),
            Expression::VariableRef(e) => {
                {
                    let mut variable = e.variable.borrow_mut();
                    let index = variable.references.iter().position(|x| (*x).ptr_eq(input));
                    if let Some(index) = index {
                        variable.references.remove(index);
                    }
                }
                self.reduce_variable_ref(e)
            }
        };

        let allocated = self
            .context
            .alloc_expression(self.reducer.reduce_expression(input, value));
        if let Expression::VariableRef(reference) = allocated {
            let mut variable = reference.variable.borrow_mut();
            variable.references.push(allocated);
        }
        allocated
    }

    pub fn reduce_err(&mut self, input: ErrExpression<'a>) -> Expression<'a> {
        self.reducer.reduce_err(input)
    }

    pub fn reduce_array_init(&mut self, input: ArrayInitExpression<'a>) -> Expression<'a> {
        let element = self.reduce_expression(input.element.get());

        self.reducer.reduce_array_init(input, element)
    }

    pub fn reduce_array_inline(&mut self, input: ArrayInlineExpression<'a>) -> Expression<'a> {
        let elements = input
            .elements
            .iter()
            .map(|(x, spread)| (self.reduce_expression(x.get()), *spread))
            .collect();

        self.reducer.reduce_array_inline(input, elements)
    }

    pub fn reduce_binary(&mut self, input: BinaryExpression<'a>) -> Expression<'a> {
        let left = self.reduce_expression(input.left.get());
        let right = self.reduce_expression(input.right.get());

        self.reducer.reduce_binary(input, left, right)
    }

    pub fn reduce_call(&mut self, input: CallExpression<'a>) -> Expression<'a> {
        let target = input.target.get().map(|e| self.reduce_expression(e));
        let arguments = input
            .arguments
            .iter()
            .map(|e| self.reduce_expression(e.get()))
            .collect();

        self.reducer.reduce_call(input, target, arguments)
    }

    pub fn reduce_circuit_init(&mut self, input: CircuitInitExpression<'a>) -> Expression<'a> {
        let values = input
            .values
            .iter()
            .map(|(ident, e)| (ident.clone(), self.reduce_expression(e.get())))
            .collect();

        self.reducer.reduce_circuit_init(input, values)
    }

    pub fn reduce_ternary_expression(&mut self, input: TernaryExpression<'a>) -> Expression<'a> {
        let condition = self.reduce_expression(input.condition.get());
        let if_true = self.reduce_expression(input.if_true.get());
        let if_false = self.reduce_expression(input.if_false.get());

        self.reducer
            .reduce_ternary_expression(input, condition, if_true, if_false)
    }

    pub fn reduce_cast_expression(&mut self, input: CastExpression<'a>) -> Expression<'a> {
        let inner = self.reduce_expression(input.inner.get());

        self.reducer.reduce_cast_expression(input, inner)
    }

    pub fn reduce_array_access(&mut self, input: ArrayAccess<'a>) -> AccessExpression<'a> {
        let array = self.reduce_expression(input.array.get());
        let index = self.reduce_expression(input.index.get());

        self.reducer.reduce_array_access(input, array, index)
    }

    pub fn reduce_array_range_access(&mut self, input: ArrayRangeAccess<'a>) -> AccessExpression<'a> {
        let array = self.reduce_expression(input.array.get());
        let left = input.left.get().map(|e| self.reduce_expression(e));
        let right = input.right.get().map(|e| self.reduce_expression(e));

        self.reducer.reduce_array_range_access(input, array, left, right)
    }

    pub fn reduce_circuit_access(&mut self, input: CircuitAccess<'a>) -> AccessExpression<'a> {
        let target = input.target.get().map(|e| self.reduce_expression(e));

        self.reducer.reduce_circuit_access(input, target)
    }

    pub fn reduce_tuple_access(&mut self, input: TupleAccess<'a>) -> AccessExpression<'a> {
        let tuple_ref = self.reduce_expression(input.tuple_ref.get());

        self.reducer.reduce_tuple_access(input, tuple_ref)
    }

    pub fn reduce_access_expression(&mut self, input: AccessExpression<'a>) -> Expression<'a> {
        use AccessExpression::*;

        let access = match input {
            Array(a) => self.reduce_array_access(a),
            ArrayRange(a) => self.reduce_array_range_access(a),
            Circuit(a) => self.reduce_circuit_access(a),
            Tuple(a) => self.reduce_tuple_access(a),
        };

        self.reducer.reduce_access_expression(access)
    }

    pub fn reduce_constant(&mut self, input: Constant<'a>) -> Expression<'a> {
        self.reducer.reduce_constant(input)
    }

    pub fn reduce_tuple_init(&mut self, input: TupleInitExpression<'a>) -> Expression<'a> {
        let values = input.elements.iter().map(|e| self.reduce_expression(e.get())).collect();

        self.reducer.reduce_tuple_init(input, values)
    }

    pub fn reduce_unary(&mut self, input: UnaryExpression<'a>) -> Expression<'a> {
        let inner = self.reduce_expression(input.inner.get());

        self.reducer.reduce_unary(input, inner)
    }

    pub fn reduce_variable_ref(&mut self, input: VariableRef<'a>) -> Expression<'a> {
        self.reducer.reduce_variable_ref(input)
    }
}

impl<'a, R: ReconstructingReducerStatement<'a>> ReconstructingDirector<'a, R> {
    pub fn reduce_statement(&mut self, input: &'a Statement<'a>) -> &'a Statement<'a> {
        let value = match input.clone() {
            Statement::Assign(s) => self.reduce_assign(s),
            Statement::Block(s) => self.reduce_block(s),
            Statement::Conditional(s) => self.reduce_conditional_statement(s),
            Statement::Console(s) => self.reduce_console(s),
            Statement::Definition(s) => self.reduce_definition(s),
            Statement::Expression(s) => self.reduce_expression_statement(s),
            Statement::Iteration(s) => self.reduce_iteration(s),
            Statement::Return(s) => self.reduce_return(s),
            x @ Statement::Empty(_) => x,
        };

        self.reducer.reduce_statement_alloc(self.context, input, value)
    }

    pub fn reduce_assign_access(&mut self, input: AssignAccess<'a>) -> AssignAccess<'a> {
        match &input {
            AssignAccess::ArrayRange(left, right) => {
                let left = left.get().map(|e| self.reduce_expression(e));
                let right = right.get().map(|e| self.reduce_expression(e));
                self.reducer.reduce_assign_access_range(input, left, right)
            }
            AssignAccess::ArrayIndex(index) => {
                let index = self.reduce_expression(index.get());
                self.reducer.reduce_assign_access_index(input, index)
            }
            _ => self.reducer.reduce_assign_access(input),
        }
    }

    pub fn reduce_assign(&mut self, input: AssignStatement<'a>) -> Statement<'a> {
        let accesses = input
            .target_accesses
            .iter()
            .map(|x| self.reduce_assign_access(x.clone()))
            .collect();
        let value = self.reduce_expression(input.value.get());

        self.reducer.reduce_assign(input, accesses, value)
    }

    pub fn reduce_block(&mut self, input: BlockStatement<'a>) -> Statement<'a> {
        let statements = input
            .statements
            .iter()
            .map(|x| self.reduce_statement(x.get()))
            .collect();

        self.reducer.reduce_block(input, statements)
    }

    pub fn reduce_conditional_statement(&mut self, input: ConditionalStatement<'a>) -> Statement<'a> {
        let condition = self.reduce_expression(input.condition.get());
        let if_true = self.reduce_statement(input.result.get());
        let if_false = input.next.get().map(|s| self.reduce_statement(s));

        self.reducer
            .reduce_conditional_statement(input, condition, if_true, if_false)
    }

    pub fn reduce_formatted_string(&mut self, input: ConsoleArgs<'a>) -> ConsoleArgs<'a> {
        let parameters = input
            .parameters
            .iter()
            .map(|e| self.reduce_expression(e.get()))
            .collect();

        self.reducer.reduce_formatted_string(input, parameters)
    }

    pub fn reduce_console(&mut self, input: ConsoleStatement<'a>) -> Statement<'a> {
        match &input.function {
            ConsoleFunction::Assert(argument) => {
                let argument = self.reduce_expression(argument.get());
                self.reducer.reduce_console_assert(input, argument)
            }
            ConsoleFunction::Error(f) | ConsoleFunction::Log(f) => {
                let formatted = self.reduce_formatted_string(f.clone());
                self.reducer.reduce_console_log(input, formatted)
            }
        }
    }

    pub fn reduce_definition(&mut self, input: DefinitionStatement<'a>) -> Statement<'a> {
        let value = self.reduce_expression(input.value.get());

        self.reducer.reduce_definition(input, value)
    }

    pub fn reduce_expression_statement(&mut self, input: ExpressionStatement<'a>) -> Statement<'a> {
        let value = self.reduce_expression(input.expression.get());

        self.reducer.reduce_expression_statement(input, value)
    }

    pub fn reduce_iteration(&mut self, input: IterationStatement<'a>) -> Statement<'a> {
        let start = self.reduce_expression(input.start.get());
        let stop = self.reduce_expression(input.stop.get());
        let body = self.reduce_statement(input.body.get());

        self.reducer.reduce_iteration(input, start, stop, body)
    }

    pub fn reduce_return(&mut self, input: ReturnStatement<'a>) -> Statement<'a> {
        let value = self.reduce_expression(input.expression.get());

        self.reducer.reduce_return(input, value)
    }
}

#[allow(dead_code)]
impl<'a, R: ReconstructingReducerProgram<'a>> ReconstructingDirector<'a, R> {
    fn reduce_function(&mut self, input: &'a Function<'a>) -> &'a Function<'a> {
        let body = input.body.get().map(|s| self.reduce_statement(s));

        self.reducer.reduce_function(input, body)
    }

    pub fn reduce_circuit_member(&mut self, input: CircuitMember<'a>) -> CircuitMember<'a> {
        match input {
            CircuitMember::Const(_) => self.reducer.reduce_circuit_member_const(input),
            CircuitMember::Function(function) => {
                let function = self.reduce_function(function);
                self.reducer.reduce_circuit_member_function(input, function)
            }
            CircuitMember::Variable(_) => self.reducer.reduce_circuit_member_variable(input),
        }
    }

    pub fn reduce_circuit(&mut self, input: &'a Circuit<'a>) -> &'a Circuit<'a> {
        let members = input
            .members
            .borrow()
            .iter()
            .map(|(_, member)| self.reduce_circuit_member(member.clone()))
            .collect();

        self.reducer.reduce_circuit(input, members)
    }

    pub fn reduce_global_const(&mut self, input: &'a DefinitionStatement<'a>) -> &'a DefinitionStatement<'a> {
        let value = self.reduce_expression(input.value.get());

        self.reducer.reduce_global_const(input, value)
    }

    pub fn reduce_program(&mut self, input: Program<'a>) -> Program<'a> {
        let imported_modules = input
            .imported_modules
            .iter()
            .map(|(module, import)| (module.clone(), self.reduce_program(import.clone())))
            .collect();
        let aliases = input.aliases.iter().map(|(name, a)| (name.clone(), *a)).collect();
        let functions = input
            .functions
            .iter()
            .map(|(name, f)| (name.clone(), self.reduce_function(f)))
            .collect();
        let circuits = input
            .circuits
            .iter()
            .map(|(name, c)| (name.clone(), self.reduce_circuit(c)))
            .collect();

        let global_consts = input
            .global_consts
            .iter()
            .map(|(name, gc)| (name.clone(), self.reduce_global_const(gc)))
            .collect();

        self.reducer
            .reduce_program(input, imported_modules, aliases, functions, circuits, global_consts)
    }
}
