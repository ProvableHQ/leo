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

use leo_ast::Identifier;

use crate::{accesses::*, expression::*, program::*, statement::*, AsgContext};

#[allow(unused_variables)]
pub trait ReconstructingReducerExpression<'a> {
    fn reduce_expression(&mut self, input: &'a Expression<'a>, value: Expression<'a>) -> Expression<'a> {
        value
    }

    fn reduce_array_init(&mut self, input: ArrayInitExpression<'a>, element: &'a Expression<'a>) -> Expression<'a> {
        Expression::ArrayInit(ArrayInitExpression {
            parent: input.parent,
            element: Cell::new(element),
            len: input.len,
            span: input.span,
        })
    }

    fn reduce_array_inline(
        &mut self,
        input: ArrayInlineExpression<'a>,
        elements: Vec<(&'a Expression<'a>, bool)>,
    ) -> Expression<'a> {
        Expression::ArrayInline(ArrayInlineExpression {
            parent: input.parent,
            elements: elements.into_iter().map(|x| (Cell::new(x.0), x.1)).collect(),
            span: input.span,
        })
    }

    fn reduce_binary(
        &mut self,
        input: BinaryExpression<'a>,
        left: &'a Expression<'a>,
        right: &'a Expression<'a>,
    ) -> Expression<'a> {
        Expression::Binary(BinaryExpression {
            parent: input.parent,
            left: Cell::new(left),
            right: Cell::new(right),
            span: input.span,
            operation: input.operation,
        })
    }

    fn reduce_call(
        &mut self,
        input: CallExpression<'a>,
        target: Option<&'a Expression<'a>>,
        arguments: Vec<&'a Expression<'a>>,
    ) -> Expression<'a> {
        Expression::Call(CallExpression {
            parent: input.parent,
            function: input.function,
            target: Cell::new(target),
            arguments: arguments.into_iter().map(Cell::new).collect(),
            span: input.span,
        })
    }

    fn reduce_circuit_init(
        &mut self,
        input: CircuitInitExpression<'a>,
        values: Vec<(Identifier, &'a Expression<'a>)>,
    ) -> Expression<'a> {
        Expression::CircuitInit(CircuitInitExpression {
            parent: input.parent,
            circuit: input.circuit,
            values: values.into_iter().map(|x| (x.0, Cell::new(x.1))).collect(),
            span: input.span,
        })
    }

    fn reduce_ternary_expression(
        &mut self,
        input: TernaryExpression<'a>,
        condition: &'a Expression<'a>,
        if_true: &'a Expression<'a>,
        if_false: &'a Expression<'a>,
    ) -> Expression<'a> {
        Expression::Ternary(TernaryExpression {
            parent: input.parent,
            condition: Cell::new(condition),
            if_true: Cell::new(if_true),
            if_false: Cell::new(if_false),
            span: input.span,
        })
    }

    fn reduce_cast_expression(&mut self, input: CastExpression<'a>, inner: &'a Expression<'a>) -> Expression<'a> {
        Expression::Cast(CastExpression {
            parent: input.parent,
            inner: Cell::new(inner),
            target_type: input.target_type,
            span: input.span,
        })
    }

    fn reduce_array_access(
        &mut self,
        input: ArrayAccess<'a>,
        array: &'a Expression<'a>,
        index: &'a Expression<'a>,
    ) -> AccessExpression<'a> {
        AccessExpression::Array(ArrayAccess {
            parent: input.parent,
            array: Cell::new(array),
            index: Cell::new(index),
            span: input.span,
        })
    }

    fn reduce_array_range_access(
        &mut self,
        input: ArrayRangeAccess<'a>,
        array: &'a Expression<'a>,
        left: Option<&'a Expression<'a>>,
        right: Option<&'a Expression<'a>>,
    ) -> AccessExpression<'a> {
        AccessExpression::ArrayRange(ArrayRangeAccess {
            parent: input.parent,
            array: Cell::new(array),
            left: Cell::new(left),
            right: Cell::new(right),
            span: input.span,
            length: input.length,
        })
    }

    fn reduce_circuit_access(
        &mut self,
        input: CircuitAccess<'a>,
        target: Option<&'a Expression<'a>>,
    ) -> AccessExpression<'a> {
        AccessExpression::Circuit(CircuitAccess {
            parent: input.parent,
            circuit: input.circuit,
            target: Cell::new(target),
            member: input.member,
            span: input.span,
        })
    }

    fn reduce_named_access(
        &mut self,
        input: NamedTypeAccess<'a>,
        named_type: &'a Expression<'a>,
    ) -> AccessExpression<'a> {
        AccessExpression::Named(NamedTypeAccess {
            parent: input.parent,
            named_type: Cell::new(named_type),
            access: input.access,
            span: input.span,
        })
    }

    fn reduce_tuple_access(&mut self, input: TupleAccess<'a>, tuple_ref: &'a Expression<'a>) -> AccessExpression<'a> {
        AccessExpression::Tuple(TupleAccess {
            parent: input.parent,
            tuple_ref: Cell::new(tuple_ref),
            index: input.index,
            span: input.span,
        })
    }

    fn reduce_value_access(&mut self, input: ValueAccess<'a>, target: &'a Expression<'a>) -> AccessExpression<'a> {
        AccessExpression::Value(ValueAccess {
            parent: input.parent,
            target: Cell::new(target),
            access: input.access.clone(),
            span: input.span,
        })
    }

    fn reduce_access_expression(&mut self, input: AccessExpression<'a>) -> Expression<'a> {
        Expression::Access(input)
    }

    fn reduce_named_type_expression(&mut self, input: NamedTypeExpression<'a>) -> Expression<'a> {
        Expression::NamedType(NamedTypeExpression {
            parent: input.parent,
            named_type: input.named_type,
            span: input.span,
        })
    }

    fn reduce_constant(&mut self, input: Constant<'a>) -> Expression<'a> {
        Expression::Constant(input)
    }

    fn reduce_tuple_init(&mut self, input: TupleInitExpression<'a>, values: Vec<&'a Expression<'a>>) -> Expression<'a> {
        Expression::TupleInit(TupleInitExpression {
            parent: input.parent,
            elements: values.into_iter().map(Cell::new).collect(),
            span: input.span,
        })
    }

    fn reduce_unary(&mut self, input: UnaryExpression<'a>, inner: &'a Expression<'a>) -> Expression<'a> {
        Expression::Unary(UnaryExpression {
            parent: input.parent,
            inner: Cell::new(inner),
            span: input.span,
            operation: input.operation,
        })
    }

    fn reduce_variable_ref(&mut self, input: VariableRef<'a>) -> Expression<'a> {
        Expression::VariableRef(input)
    }
}

#[allow(unused_variables)]
pub trait ReconstructingReducerStatement<'a>: ReconstructingReducerExpression<'a> {
    fn reduce_statement_alloc(
        &mut self,
        context: AsgContext<'a>,
        input: &'a Statement<'a>,
        value: Statement<'a>,
    ) -> &'a Statement<'a> {
        context.alloc_statement(value)
    }

    fn reduce_statement(&mut self, input: &'a Statement<'a>, value: Statement<'a>) -> Statement<'a> {
        value
    }

    fn reduce_assign_access_range(
        &mut self,
        input: AssignAccess<'a>,
        left: Option<&'a Expression<'a>>,
        right: Option<&'a Expression<'a>>,
    ) -> AssignAccess<'a> {
        AssignAccess::ArrayRange(Cell::new(left), Cell::new(right))
    }

    fn reduce_assign_access_index(&mut self, input: AssignAccess<'a>, index: &'a Expression<'a>) -> AssignAccess<'a> {
        AssignAccess::ArrayIndex(Cell::new(index))
    }

    fn reduce_assign_access(&mut self, input: AssignAccess<'a>) -> AssignAccess<'a> {
        input
    }

    fn reduce_assign(
        &mut self,
        input: AssignStatement<'a>,
        accesses: Vec<AssignAccess<'a>>,
        value: &'a Expression<'a>,
    ) -> Statement<'a> {
        Statement::Assign(AssignStatement {
            parent: input.parent,
            span: input.span,
            operation: input.operation,
            target_accesses: accesses,
            target_variable: input.target_variable,
            value: Cell::new(value),
        })
    }

    fn reduce_block(&mut self, input: BlockStatement<'a>, statements: Vec<&'a Statement<'a>>) -> Statement<'a> {
        Statement::Block(BlockStatement {
            parent: input.parent,
            span: input.span,
            statements: statements.into_iter().map(Cell::new).collect(),
            scope: input.scope,
        })
    }

    fn reduce_conditional_statement(
        &mut self,
        input: ConditionalStatement<'a>,
        condition: &'a Expression<'a>,
        if_true: &'a Statement<'a>,
        if_false: Option<&'a Statement<'a>>,
    ) -> Statement<'a> {
        Statement::Conditional(ConditionalStatement {
            parent: input.parent,
            span: input.span,
            condition: Cell::new(condition),
            result: Cell::new(if_true),
            next: Cell::new(if_false),
        })
    }

    fn reduce_formatted_string(
        &mut self,
        input: ConsoleArgs<'a>,
        parameters: Vec<&'a Expression<'a>>,
    ) -> ConsoleArgs<'a> {
        ConsoleArgs {
            span: input.span,
            string: input.string,
            parameters: parameters.into_iter().map(Cell::new).collect(),
        }
    }

    fn reduce_console_assert(&mut self, input: ConsoleStatement<'a>, argument: &'a Expression<'a>) -> Statement<'a> {
        assert!(matches!(input.function, ConsoleFunction::Assert(_)));
        Statement::Console(ConsoleStatement {
            parent: input.parent,
            span: input.span,
            function: ConsoleFunction::Assert(Cell::new(argument)),
        })
    }

    fn reduce_console_log(&mut self, input: ConsoleStatement<'a>, argument: ConsoleArgs<'a>) -> Statement<'a> {
        assert!(!matches!(input.function, ConsoleFunction::Assert(_)));
        Statement::Console(ConsoleStatement {
            parent: input.parent,
            span: input.span,
            function: match input.function {
                ConsoleFunction::Assert(_) => unimplemented!(),
                ConsoleFunction::Error(_) => ConsoleFunction::Error(argument),
                ConsoleFunction::Log(_) => ConsoleFunction::Log(argument),
            },
        })
    }

    fn reduce_definition(&mut self, input: DefinitionStatement<'a>, value: &'a Expression<'a>) -> Statement<'a> {
        Statement::Definition(DefinitionStatement {
            parent: input.parent,
            span: input.span,
            variables: input.variables,
            value: Cell::new(value),
        })
    }

    fn reduce_expression_statement(
        &mut self,
        input: ExpressionStatement<'a>,
        expression: &'a Expression<'a>,
    ) -> Statement<'a> {
        Statement::Expression(ExpressionStatement {
            parent: input.parent,
            span: input.span,
            expression: Cell::new(expression),
        })
    }

    fn reduce_iteration(
        &mut self,
        input: IterationStatement<'a>,
        start: &'a Expression<'a>,
        stop: &'a Expression<'a>,
        body: &'a Statement<'a>,
    ) -> Statement<'a> {
        Statement::Iteration(IterationStatement {
            parent: input.parent,
            span: input.span,
            variable: input.variable,
            start: Cell::new(start),
            stop: Cell::new(stop),
            inclusive: input.inclusive,
            body: Cell::new(body),
        })
    }

    fn reduce_return(&mut self, input: ReturnStatement<'a>, value: &'a Expression<'a>) -> Statement<'a> {
        Statement::Return(ReturnStatement {
            parent: input.parent,
            span: input.span,
            expression: Cell::new(value),
        })
    }
}

#[allow(unused_variables)]
pub trait ReconstructingReducerProgram<'a>: ReconstructingReducerStatement<'a> {
    // todo @protryon: this is kind of hacky
    fn reduce_function(&mut self, input: &'a Function<'a>, body: Option<&'a Statement<'a>>) -> &'a Function<'a> {
        input.body.set(body);
        input
    }

    fn reduce_circuit_member_variable(&mut self, input: CircuitMember<'a>) -> CircuitMember<'a> {
        input
    }

    fn reduce_circuit_member_function(
        &mut self,
        input: CircuitMember<'a>,
        function: &'a Function<'a>,
    ) -> CircuitMember<'a> {
        CircuitMember::Function(function)
    }

    // todo @protryon: this is kind of hacky
    fn reduce_circuit(&mut self, input: &'a Circuit<'a>, members: Vec<CircuitMember<'a>>) -> &'a Circuit<'a> {
        let mut input_members = input.members.borrow_mut();
        for ((name, input_member), member) in input_members.iter_mut().zip(members) {
            *input_member = member;
        }
        input
    }

    fn reduce_global_const(
        &mut self,
        input: &'a DefinitionStatement<'a>,
        value: &'a Expression<'a>,
    ) -> &'a DefinitionStatement<'a> {
        input.value.set(value);
        input
    }

    fn reduce_program(
        &mut self,
        input: Program<'a>,
        imported_modules: Vec<(String, Program<'a>)>,
        aliases: Vec<(String, &'a Alias<'a>)>,
        functions: Vec<(String, &'a Function<'a>)>,
        circuits: Vec<(String, &'a Circuit<'a>)>,
        global_consts: Vec<(String, &'a DefinitionStatement<'a>)>,
    ) -> Program<'a> {
        Program {
            context: input.context,
            id: input.id,
            name: input.name,
            imported_modules: imported_modules.into_iter().collect(),
            aliases: aliases.into_iter().collect(),
            functions: functions.into_iter().collect(),
            circuits: circuits.into_iter().collect(),
            scope: input.scope,
            global_consts: global_consts.into_iter().collect(),
        }
    }
}
