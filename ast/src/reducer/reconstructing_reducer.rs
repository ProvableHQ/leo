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
    fn reduce_type(&mut self, _type_: &Type, new: Type, _in_circuit: bool) -> Type {
        new
    }

    // Expressions
    fn reduce_expression(&mut self, _expression: &Expression, new: Expression) -> Expression {
        new
    }

    fn reduce_identifier(&mut self, identifier: &Identifier) -> Identifier {
        Identifier {
            name: identifier.name.clone(),
            span: identifier.span.clone(),
        }
    }

    fn reduce_group_tuple(&mut self, group_tuple: &GroupTuple) -> GroupTuple {
        GroupTuple {
            x: group_tuple.x.clone(),
            y: group_tuple.y.clone(),
            span: group_tuple.span.clone(),
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
        binary: &BinaryExpression,
        left: Expression,
        right: Expression,
        op: BinaryOperation,
    ) -> BinaryExpression {
        BinaryExpression {
            left: Box::new(left),
            right: Box::new(right),
            op,
            span: binary.span.clone(),
        }
    }

    fn reduce_unary(&mut self, unary: &UnaryExpression, inner: Expression, op: UnaryOperation) -> UnaryExpression {
        UnaryExpression {
            inner: Box::new(inner),
            op,
            span: unary.span.clone(),
        }
    }

    fn reduce_ternary(
        &mut self,
        ternary: &TernaryExpression,
        condition: Expression,
        if_true: Expression,
        if_false: Expression,
    ) -> TernaryExpression {
        TernaryExpression {
            condition: Box::new(condition),
            if_true: Box::new(if_true),
            if_false: Box::new(if_false),
            span: ternary.span.clone(),
        }
    }

    fn reduce_cast(&mut self, cast: &CastExpression, inner: Expression, target_type: Type) -> CastExpression {
        CastExpression {
            inner: Box::new(inner),
            target_type,
            span: cast.span.clone(),
        }
    }

    fn reduce_array_inline(
        &mut self,
        array_inline: &ArrayInlineExpression,
        elements: Vec<SpreadOrExpression>,
    ) -> ArrayInlineExpression {
        ArrayInlineExpression {
            elements,
            span: array_inline.span.clone(),
        }
    }

    fn reduce_array_init(&mut self, array_init: &ArrayInitExpression, element: Expression) -> ArrayInitExpression {
        ArrayInitExpression {
            element: Box::new(element),
            dimensions: array_init.dimensions.clone(),
            span: array_init.span.clone(),
        }
    }

    fn reduce_array_access(
        &mut self,
        array_access: &ArrayAccessExpression,
        array: Expression,
        index: Expression,
    ) -> ArrayAccessExpression {
        ArrayAccessExpression {
            array: Box::new(array),
            index: Box::new(index),
            span: array_access.span.clone(),
        }
    }

    fn reduce_array_range_access(
        &mut self,
        array_rage_access: &ArrayRangeAccessExpression,
        array: Expression,
        left: Option<Expression>,
        right: Option<Expression>,
    ) -> ArrayRangeAccessExpression {
        ArrayRangeAccessExpression {
            array: Box::new(array),
            left: left.map(|expr| Box::new(expr)),
            right: right.map(|expr| Box::new(expr)),
            span: array_rage_access.span.clone(),
        }
    }

    fn reduce_tuple_init(
        &mut self,
        tuple_init: &TupleInitExpression,
        elements: Vec<Expression>,
    ) -> TupleInitExpression {
        TupleInitExpression {
            elements,
            span: tuple_init.span.clone(),
        }
    }

    fn reduce_tuple_access(
        &mut self,
        tuple_access: &TupleAccessExpression,
        tuple: Expression,
    ) -> TupleAccessExpression {
        TupleAccessExpression {
            tuple: Box::new(tuple),
            index: tuple_access.index.clone(),
            span: tuple_access.span.clone(),
        }
    }

    fn reduce_circuit_init(
        &mut self,
        circuit_init: &CircuitInitExpression,
        name: Identifier,
        members: Vec<CircuitImpliedVariableDefinition>,
    ) -> CircuitInitExpression {
        CircuitInitExpression {
            name,
            members,
            span: circuit_init.span.clone(),
        }
    }

    fn reduce_circuit_member_access(
        &mut self,
        circuit_member_access: &CircuitMemberAccessExpression,
        circuit: Expression,
        name: Identifier,
    ) -> CircuitMemberAccessExpression {
        CircuitMemberAccessExpression {
            circuit: Box::new(circuit),
            name,
            span: circuit_member_access.span.clone(),
        }
    }

    fn reduce_circuit_static_fn_access(
        &mut self,
        circuit_static_fn_access: &CircuitStaticFunctionAccessExpression,
        circuit: Expression,
        name: Identifier,
    ) -> CircuitStaticFunctionAccessExpression {
        CircuitStaticFunctionAccessExpression {
            circuit: Box::new(circuit),
            name,
            span: circuit_static_fn_access.span.clone(),
        }
    }

    fn reduce_call(
        &mut self,
        call: &CallExpression,
        function: Expression,
        arguments: Vec<Expression>,
    ) -> CallExpression {
        CallExpression {
            function: Box::new(function),
            arguments,
            span: call.span.clone(),
        }
    }

    // Statements
    fn reduce_statement(&mut self, _statement: &Statement, new: Statement, _in_circuit: bool) -> Statement {
        new
    }

    fn reduce_return(&mut self, return_statement: &ReturnStatement, expression: Expression) -> ReturnStatement {
        ReturnStatement {
            expression,
            span: return_statement.span.clone(),
        }
    }

    fn reduce_variable_name(&mut self, variable_name: &VariableName, identifier: Identifier) -> VariableName {
        VariableName {
            mutable: variable_name.mutable,
            identifier,
            span: variable_name.span.clone(),
        }
    }

    fn reduce_definition(
        &mut self,
        definition: &DefinitionStatement,
        variable_names: Vec<VariableName>,
        type_: Option<Type>,
        value: Expression,
        _in_circuit: bool,
    ) -> DefinitionStatement {
        DefinitionStatement {
            declaration_type: definition.declaration_type.clone(),
            variable_names,
            type_,
            value,
            span: definition.span.clone(),
        }
    }

    fn reduce_assignee_access(&mut self, _access: &AssigneeAccess, new: AssigneeAccess) -> AssigneeAccess {
        new
    }

    fn reduce_assignee(
        &mut self,
        assignee: &Assignee,
        identifier: Identifier,
        accesses: Vec<AssigneeAccess>,
    ) -> Assignee {
        Assignee {
            identifier,
            accesses,
            span: assignee.span.clone(),
        }
    }

    fn reduce_assign(&mut self, assign: &AssignStatement, assignee: Assignee, value: Expression) -> AssignStatement {
        AssignStatement {
            operation: assign.operation.clone(),
            assignee,
            value,
            span: assign.span.clone(),
        }
    }

    fn reduce_conditional(
        &mut self,
        conditional: &ConditionalStatement,
        condition: Expression,
        block: Block,
        statement: Option<Statement>,
        _in_circuit: bool,
    ) -> ConditionalStatement {
        ConditionalStatement {
            condition,
            block,
            next: statement.map(|statement| Box::new(statement)),
            span: conditional.span.clone(),
        }
    }

    fn reduce_iteration(
        &mut self,
        iteration: &IterationStatement,
        variable: Identifier,
        start: Expression,
        stop: Expression,
        block: Block,
        _in_circuit: bool,
    ) -> IterationStatement {
        IterationStatement {
            variable,
            start,
            stop,
            block,
            span: iteration.span.clone(),
        }
    }

    fn reduce_console(&mut self, console: &ConsoleStatement, function: ConsoleFunction) -> ConsoleStatement {
        ConsoleStatement {
            function,
            span: console.span.clone(),
        }
    }

    fn reduce_expression_statement(
        &mut self,
        expression_statement: &ExpressionStatement,
        expression: Expression,
    ) -> ExpressionStatement {
        ExpressionStatement {
            expression,
            span: expression_statement.span.clone(),
        }
    }

    fn reduce_block(&mut self, block: &Block, statements: Vec<Statement>, _in_circuit: bool) -> Block {
        Block {
            statements,
            span: block.span.clone(),
        }
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
        _in_circuit: bool,
    ) -> FunctionInputVariable {
        FunctionInputVariable {
            identifier,
            const_: variable.const_,
            mutable: variable.mutable,
            type_,
            span: variable.span.clone(),
        }
    }

    fn reduce_function_input(
        &mut self,
        _input: &FunctionInput,
        new: FunctionInput,
        _in_circuit: bool,
    ) -> FunctionInput {
        new
    }

    fn reduce_package_or_packages(
        &mut self,
        _package_or_packages: &PackageOrPackages,
        new: PackageOrPackages,
    ) -> PackageOrPackages {
        new
    }

    fn reduce_import(&mut self, import: &ImportStatement, package_or_packages: PackageOrPackages) -> ImportStatement {
        ImportStatement {
            package_or_packages,
            span: import.span.clone(),
        }
    }

    fn reduce_circuit_member(&mut self, _circuit_member: &CircuitMember, new: CircuitMember) -> CircuitMember {
        new
    }

    fn reduce_circuit(&mut self, _circuit: &Circuit, circuit_name: Identifier, members: Vec<CircuitMember>) -> Circuit {
        Circuit { circuit_name, members }
    }

    fn reduce_annotation(&mut self, annotation: &Annotation, name: Identifier) -> Annotation {
        Annotation {
            span: annotation.span.clone(),
            name,
            arguments: annotation.arguments.clone(),
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn reduce_function(
        &mut self,
        function: &Function,
        identifier: Identifier,
        annotations: Vec<Annotation>,
        input: Vec<FunctionInput>,
        output: Option<Type>,
        block: Block,
        _in_circuit: bool,
    ) -> Function {
        Function {
            identifier,
            annotations,
            input,
            output,
            block,
            span: function.span.clone(),
        }
    }
}
