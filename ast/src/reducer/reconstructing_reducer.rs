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

use crate::*;
use indexmap::IndexMap;

// Needed to fix clippy bug.
#[allow(clippy::redundant_closure)]
pub trait ReconstructingReducer {
    fn reduce_type(&mut self, _type_: &Type, new: Type) -> Type {
        new
    }

    // Expressions
    fn reduce_expression(&mut self, _expression: &Expression, new: Expression) -> Expression {
        new
    }

    fn reduce_identifier(&mut self, identifier: &Identifier, span: Span) -> Identifier {
        Identifier {
            name: identifier.name.clone(),
            span,
        }
    }

    fn reduce_group_tuple(&mut self, group_tuple: &GroupTuple, span: Span) -> GroupTuple {
        GroupTuple {
            x: group_tuple.x.clone(),
            y: group_tuple.y.clone(),
            span,
        }
    }

    fn reduce_group_value(&mut self, _group_value: &GroupValue, new: GroupValue) -> GroupValue {
        new
    }

    fn reduce_value(&mut self, _value: &ValueExpression, new: ValueExpression) -> ValueExpression {
        new
    }

    fn reduce_binary(
        &mut self,
        _binary: &BinaryExpression,
        left: Expression,
        right: Expression,
        op: BinaryOperation,
        span: Span,
    ) -> BinaryExpression {
        BinaryExpression {
            left: Box::new(left),
            right: Box::new(right),
            op,
            span,
        }
    }

    fn reduce_unary(
        &mut self,
        _unary: &UnaryExpression,
        inner: Expression,
        op: UnaryOperation,
        span: Span,
    ) -> UnaryExpression {
        UnaryExpression {
            inner: Box::new(inner),
            op,
            span,
        }
    }

    fn reduce_ternary(
        &mut self,
        _ternary: &TernaryExpression,
        condition: Expression,
        if_true: Expression,
        if_false: Expression,
        span: Span,
    ) -> TernaryExpression {
        TernaryExpression {
            condition: Box::new(condition),
            if_true: Box::new(if_true),
            if_false: Box::new(if_false),
            span,
        }
    }

    fn reduce_cast(
        &mut self,
        _cast: &CastExpression,
        inner: Expression,
        target_type: Type,
        span: Span,
    ) -> CastExpression {
        CastExpression {
            inner: Box::new(inner),
            target_type,
            span,
        }
    }

    fn reduce_array_inline(
        &mut self,
        _array_inline: &ArrayInlineExpression,
        elements: Vec<SpreadOrExpression>,
        span: Span,
    ) -> ArrayInlineExpression {
        ArrayInlineExpression { elements, span }
    }

    fn reduce_array_init(
        &mut self,
        array_init: &ArrayInitExpression,
        element: Expression,
        span: Span,
    ) -> ArrayInitExpression {
        ArrayInitExpression {
            element: Box::new(element),
            dimensions: array_init.dimensions.clone(),
            span,
        }
    }

    fn reduce_array_access(
        &mut self,
        _array_access: &ArrayAccessExpression,
        array: Expression,
        index: Expression,
        span: Span,
    ) -> ArrayAccessExpression {
        ArrayAccessExpression {
            array: Box::new(array),
            index: Box::new(index),
            span,
        }
    }

    fn reduce_array_range_access(
        &mut self,
        _array_rage_access: &ArrayRangeAccessExpression,
        array: Expression,
        left: Option<Expression>,
        right: Option<Expression>,
        span: Span,
    ) -> ArrayRangeAccessExpression {
        ArrayRangeAccessExpression {
            array: Box::new(array),
            left: left.map(|expr| Box::new(expr)),
            right: right.map(|expr| Box::new(expr)),
            span,
        }
    }

    fn reduce_tuple_init(
        &mut self,
        _tuple_init: &TupleInitExpression,
        elements: Vec<Expression>,
        span: Span,
    ) -> TupleInitExpression {
        TupleInitExpression { elements, span }
    }

    fn reduce_tuple_access(
        &mut self,
        tuple_access: &TupleAccessExpression,
        tuple: Expression,
        span: Span,
    ) -> TupleAccessExpression {
        TupleAccessExpression {
            tuple: Box::new(tuple),
            index: tuple_access.index.clone(),
            span,
        }
    }

    fn reduce_circuit_init(
        &mut self,
        _circuit_init: &CircuitInitExpression,
        name: Identifier,
        members: Vec<CircuitImpliedVariableDefinition>,
        span: Span,
    ) -> CircuitInitExpression {
        CircuitInitExpression { name, members, span }
    }

    fn reduce_circuit_member_access(
        &mut self,
        _circuit_member_access: &CircuitMemberAccessExpression,
        circuit: Expression,
        name: Identifier,
        span: Span,
    ) -> CircuitMemberAccessExpression {
        CircuitMemberAccessExpression {
            circuit: Box::new(circuit),
            name,
            span,
        }
    }

    fn reduce_circuit_static_fn_access(
        &mut self,
        _circuit_static_fn_access: &CircuitStaticFunctionAccessExpression,
        circuit: Expression,
        name: Identifier,
        span: Span,
    ) -> CircuitStaticFunctionAccessExpression {
        CircuitStaticFunctionAccessExpression {
            circuit: Box::new(circuit),
            name,
            span,
        }
    }

    fn reduce_call(
        &mut self,
        _call: &CallExpression,
        function: Expression,
        arguments: Vec<Expression>,
        span: Span,
    ) -> CallExpression {
        CallExpression {
            function: Box::new(function),
            arguments,
            span,
        }
    }

    // Statements
    fn reduce_statement(&mut self, _statement: &Statement, new: Statement) -> Statement {
        new
    }

    fn reduce_return(
        &mut self,
        _return_statement: &ReturnStatement,
        expression: Expression,
        span: Span,
    ) -> ReturnStatement {
        ReturnStatement { expression, span }
    }

    fn reduce_variable_name(
        &mut self,
        variable_name: &VariableName,
        identifier: Identifier,
        span: Span,
    ) -> VariableName {
        VariableName {
            mutable: variable_name.mutable,
            identifier,
            span,
        }
    }

    fn reduce_definition(
        &mut self,
        definition: &DefinitionStatement,
        variable_names: Vec<VariableName>,
        type_: Option<Type>,
        value: Expression,
        span: Span,
    ) -> DefinitionStatement {
        DefinitionStatement {
            declaration_type: definition.declaration_type.clone(),
            variable_names,
            type_,
            value,
            span,
        }
    }

    fn reduce_assignee_access(&mut self, _access: &AssigneeAccess, new: AssigneeAccess) -> AssigneeAccess {
        new
    }

    fn reduce_assignee(
        &mut self,
        _assignee: &Assignee,
        identifier: Identifier,
        accesses: Vec<AssigneeAccess>,
        span: Span,
    ) -> Assignee {
        Assignee {
            identifier,
            accesses,
            span,
        }
    }

    fn reduce_assign(
        &mut self,
        assign: &AssignStatement,
        assignee: Assignee,
        value: Expression,
        span: Span,
    ) -> AssignStatement {
        AssignStatement {
            operation: assign.operation.clone(),
            assignee,
            value,
            span,
        }
    }

    fn reduce_conditional(
        &mut self,
        _conditional: &ConditionalStatement,
        condition: Expression,
        block: Block,
        statement: Option<Statement>,
        span: Span,
    ) -> ConditionalStatement {
        ConditionalStatement {
            condition,
            block,
            next: statement.map(|statement| Box::new(statement)),
            span,
        }
    }

    fn reduce_iteration(
        &mut self,
        _iteration: &IterationStatement,
        variable: Identifier,
        start: Expression,
        stop: Expression,
        block: Block,
        span: Span,
    ) -> IterationStatement {
        IterationStatement {
            variable,
            start,
            stop,
            block,
            span,
        }
    }

    fn reduce_console(
        &mut self,
        _console: &ConsoleStatement,
        function: ConsoleFunction,
        span: Span,
    ) -> ConsoleStatement {
        ConsoleStatement { function, span }
    }

    fn reduce_expression_statement(
        &mut self,
        _expression_statement: &ExpressionStatement,
        expression: Expression,
        span: Span,
    ) -> ExpressionStatement {
        ExpressionStatement { expression, span }
    }

    fn reduce_block(&mut self, _block: &Block, statements: Vec<Statement>, span: Span) -> Block {
        Block { statements, span }
    }

    // Program
    fn reduce_program(
        &mut self,
        program: &Program,
        expected_input: Vec<FunctionInput>,
        imports: Vec<ImportStatement>,
        circuits: IndexMap<Identifier, Circuit>,
        functions: IndexMap<Identifier, Function>,
    ) -> Program {
        Program {
            name: program.name.clone(),
            expected_input,
            imports,
            circuits,
            functions,
        }
    }

    fn reduce_function_input_variable(
        &mut self,
        variable: &FunctionInputVariable,
        identifier: Identifier,
        type_: Type,
        span: Span,
    ) -> FunctionInputVariable {
        FunctionInputVariable {
            identifier,
            const_: variable.const_,
            mutable: variable.mutable,
            type_,
            span,
        }
    }

    fn reduce_function_input(&mut self, _input: &FunctionInput, new: FunctionInput) -> FunctionInput {
        new
    }

    fn reduce_package_or_packages(
        &mut self,
        _package_or_packages: &PackageOrPackages,
        new: PackageOrPackages,
    ) -> PackageOrPackages {
        new
    }

    fn reduce_import(
        &mut self,
        _import: &ImportStatement,
        package_or_packages: PackageOrPackages,
        span: Span,
    ) -> ImportStatement {
        ImportStatement {
            package_or_packages,
            span,
        }
    }

    fn reduce_circuit_member(&mut self, _circuit_member: &CircuitMember, new: CircuitMember) -> CircuitMember {
        new
    }

    fn reduce_circuit(&mut self, _circuit: &Circuit, circuit_name: Identifier, members: Vec<CircuitMember>) -> Circuit {
        Circuit { circuit_name, members }
    }

    fn reduce_annotation(&mut self, annotation: &Annotation, span: Span, name: Identifier) -> Annotation {
        Annotation {
            span,
            name,
            arguments: annotation.arguments.clone(),
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn reduce_function(
        &mut self,
        _function: &Function,
        identifier: Identifier,
        annotations: Vec<Annotation>,
        input: Vec<FunctionInput>,
        output: Option<Type>,
        block: Block,
        span: Span,
    ) -> Function {
        Function {
            identifier,
            annotations,
            input,
            output,
            block,
            span,
        }
    }
}
