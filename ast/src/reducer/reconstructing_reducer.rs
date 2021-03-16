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
    fn reduce_type(
        &self,
        _type_: &Type,
        new: Type,
        _in_circuit: bool,
        _span: &Span,
    ) -> Result<Type, CanonicalizeError> {
        Ok(new)
    }

    // Expressions
    fn reduce_expression(
        &self,
        _expression: &Expression,
        new: Expression,
        _in_circuit: bool,
    ) -> Result<Expression, CanonicalizeError> {
        Ok(new)
    }

    fn reduce_identifier(&self, identifier: &Identifier) -> Result<Identifier, CanonicalizeError> {
        Ok(Identifier {
            name: identifier.name.clone(),
            span: identifier.span.clone(),
        })
    }

    fn reduce_group_tuple(&self, group_tuple: &GroupTuple) -> Result<GroupTuple, CanonicalizeError> {
        Ok(GroupTuple {
            x: group_tuple.x.clone(),
            y: group_tuple.y.clone(),
            span: group_tuple.span.clone(),
        })
    }

    fn reduce_group_value(&self, _group_value: &GroupValue, new: GroupValue) -> Result<GroupValue, CanonicalizeError> {
        Ok(new)
    }

    fn reduce_value(
        &self,
        _value: &ValueExpression,
        new: ValueExpression,
    ) -> Result<ValueExpression, CanonicalizeError> {
        Ok(new)
    }

    fn reduce_binary(
        &self,
        binary: &BinaryExpression,
        left: Expression,
        right: Expression,
        op: BinaryOperation,
        _in_circuit: bool,
    ) -> Result<BinaryExpression, CanonicalizeError> {
        Ok(BinaryExpression {
            left: Box::new(left),
            right: Box::new(right),
            op,
            span: binary.span.clone(),
        })
    }

    fn reduce_unary(
        &self,
        unary: &UnaryExpression,
        inner: Expression,
        op: UnaryOperation,
        _in_circuit: bool,
    ) -> Result<UnaryExpression, CanonicalizeError> {
        Ok(UnaryExpression {
            inner: Box::new(inner),
            op,
            span: unary.span.clone(),
        })
    }

    fn reduce_ternary(
        &self,
        ternary: &TernaryExpression,
        condition: Expression,
        if_true: Expression,
        if_false: Expression,
        _in_circuit: bool,
    ) -> Result<TernaryExpression, CanonicalizeError> {
        Ok(TernaryExpression {
            condition: Box::new(condition),
            if_true: Box::new(if_true),
            if_false: Box::new(if_false),
            span: ternary.span.clone(),
        })
    }

    fn reduce_cast(
        &self,
        cast: &CastExpression,
        inner: Expression,
        target_type: Type,
        _in_circuit: bool,
    ) -> Result<CastExpression, CanonicalizeError> {
        Ok(CastExpression {
            inner: Box::new(inner),
            target_type,
            span: cast.span.clone(),
        })
    }

    fn reduce_array_inline(
        &self,
        array_inline: &ArrayInlineExpression,
        elements: Vec<SpreadOrExpression>,
        _in_circuit: bool,
    ) -> Result<ArrayInlineExpression, CanonicalizeError> {
        Ok(ArrayInlineExpression {
            elements,
            span: array_inline.span.clone(),
        })
    }

    fn reduce_array_init(
        &self,
        array_init: &ArrayInitExpression,
        element: Expression,
        _in_circuit: bool,
    ) -> Result<ArrayInitExpression, CanonicalizeError> {
        Ok(ArrayInitExpression {
            element: Box::new(element),
            dimensions: array_init.dimensions.clone(),
            span: array_init.span.clone(),
        })
    }

    fn reduce_array_access(
        &self,
        array_access: &ArrayAccessExpression,
        array: Expression,
        index: Expression,
        _in_circuit: bool,
    ) -> Result<ArrayAccessExpression, CanonicalizeError> {
        Ok(ArrayAccessExpression {
            array: Box::new(array),
            index: Box::new(index),
            span: array_access.span.clone(),
        })
    }

    fn reduce_array_range_access(
        &self,
        array_rage_access: &ArrayRangeAccessExpression,
        array: Expression,
        left: Option<Expression>,
        right: Option<Expression>,
        _in_circuit: bool,
    ) -> Result<ArrayRangeAccessExpression, CanonicalizeError> {
        Ok(ArrayRangeAccessExpression {
            array: Box::new(array),
            left: left.map(|expr| Box::new(expr)),
            right: right.map(|expr| Box::new(expr)),
            span: array_rage_access.span.clone(),
        })
    }

    fn reduce_tuple_init(
        &self,
        tuple_init: &TupleInitExpression,
        elements: Vec<Expression>,
        _in_circuit: bool,
    ) -> Result<TupleInitExpression, CanonicalizeError> {
        Ok(TupleInitExpression {
            elements,
            span: tuple_init.span.clone(),
        })
    }

    fn reduce_tuple_access(
        &self,
        tuple_access: &TupleAccessExpression,
        tuple: Expression,
        _in_circuit: bool,
    ) -> Result<TupleAccessExpression, CanonicalizeError> {
        Ok(TupleAccessExpression {
            tuple: Box::new(tuple),
            index: tuple_access.index.clone(),
            span: tuple_access.span.clone(),
        })
    }

    fn reduce_circuit_implied_variable_definition(
        &self,
        _variable: &CircuitImpliedVariableDefinition,
        identifier: Identifier,
        expression: Option<Expression>,
        _in_circuit: bool,
    ) -> Result<CircuitImpliedVariableDefinition, CanonicalizeError> {
        Ok(CircuitImpliedVariableDefinition { identifier, expression })
    }

    fn reduce_circuit_init(
        &self,
        circuit_init: &CircuitInitExpression,
        name: Identifier,
        members: Vec<CircuitImpliedVariableDefinition>,
        _in_circuit: bool,
    ) -> Result<CircuitInitExpression, CanonicalizeError> {
        Ok(CircuitInitExpression {
            name,
            members,
            span: circuit_init.span.clone(),
        })
    }

    fn reduce_circuit_member_access(
        &self,
        circuit_member_access: &CircuitMemberAccessExpression,
        circuit: Expression,
        name: Identifier,
        _in_circuit: bool,
    ) -> Result<CircuitMemberAccessExpression, CanonicalizeError> {
        Ok(CircuitMemberAccessExpression {
            circuit: Box::new(circuit),
            name,
            span: circuit_member_access.span.clone(),
        })
    }

    fn reduce_circuit_static_fn_access(
        &self,
        circuit_static_fn_access: &CircuitStaticFunctionAccessExpression,
        circuit: Expression,
        name: Identifier,
        _in_circuit: bool,
    ) -> Result<CircuitStaticFunctionAccessExpression, CanonicalizeError> {
        Ok(CircuitStaticFunctionAccessExpression {
            circuit: Box::new(circuit),
            name,
            span: circuit_static_fn_access.span.clone(),
        })
    }

    fn reduce_call(
        &self,
        call: &CallExpression,
        function: Expression,
        arguments: Vec<Expression>,
        _in_circuit: bool,
    ) -> Result<CallExpression, CanonicalizeError> {
        Ok(CallExpression {
            function: Box::new(function),
            arguments,
            span: call.span.clone(),
        })
    }

    // Statements
    fn reduce_statement(
        &self,
        _statement: &Statement,
        new: Statement,
        _in_circuit: bool,
    ) -> Result<Statement, CanonicalizeError> {
        Ok(new)
    }

    fn reduce_return(
        &self,
        return_statement: &ReturnStatement,
        expression: Expression,
        _in_circuit: bool,
    ) -> Result<ReturnStatement, CanonicalizeError> {
        Ok(ReturnStatement {
            expression,
            span: return_statement.span.clone(),
        })
    }

    fn reduce_variable_name(
        &self,
        variable_name: &VariableName,
        identifier: Identifier,
    ) -> Result<VariableName, CanonicalizeError> {
        Ok(VariableName {
            mutable: variable_name.mutable,
            identifier,
            span: variable_name.span.clone(),
        })
    }

    fn reduce_definition(
        &self,
        definition: &DefinitionStatement,
        variable_names: Vec<VariableName>,
        type_: Option<Type>,
        value: Expression,
        _in_circuit: bool,
    ) -> Result<DefinitionStatement, CanonicalizeError> {
        Ok(DefinitionStatement {
            declaration_type: definition.declaration_type.clone(),
            variable_names,
            type_,
            value,
            span: definition.span.clone(),
        })
    }

    fn reduce_assignee_access(
        &self,
        _access: &AssigneeAccess,
        new: AssigneeAccess,
        _in_circuit: bool,
    ) -> Result<AssigneeAccess, CanonicalizeError> {
        Ok(new)
    }

    fn reduce_assignee(
        &self,
        assignee: &Assignee,
        identifier: Identifier,
        accesses: Vec<AssigneeAccess>,
        _in_circuit: bool,
    ) -> Result<Assignee, CanonicalizeError> {
        Ok(Assignee {
            identifier,
            accesses,
            span: assignee.span.clone(),
        })
    }

    fn reduce_assign(
        &self,
        assign: &AssignStatement,
        assignee: Assignee,
        value: Expression,
        _in_circuit: bool,
    ) -> Result<AssignStatement, CanonicalizeError> {
        Ok(AssignStatement {
            operation: assign.operation.clone(),
            assignee,
            value,
            span: assign.span.clone(),
        })
    }

    fn reduce_conditional(
        &self,
        conditional: &ConditionalStatement,
        condition: Expression,
        block: Block,
        statement: Option<Statement>,
        _in_circuit: bool,
    ) -> Result<ConditionalStatement, CanonicalizeError> {
        Ok(ConditionalStatement {
            condition,
            block,
            next: statement.map(|statement| Box::new(statement)),
            span: conditional.span.clone(),
        })
    }

    fn reduce_iteration(
        &self,
        iteration: &IterationStatement,
        variable: Identifier,
        start: Expression,
        stop: Expression,
        block: Block,
        _in_circuit: bool,
    ) -> Result<IterationStatement, CanonicalizeError> {
        Ok(IterationStatement {
            variable,
            start,
            stop,
            block,
            span: iteration.span.clone(),
        })
    }

    fn reduce_console(
        &self,
        console: &ConsoleStatement,
        function: ConsoleFunction,
        _in_circuit: bool,
    ) -> Result<ConsoleStatement, CanonicalizeError> {
        Ok(ConsoleStatement {
            function,
            span: console.span.clone(),
        })
    }

    fn reduce_expression_statement(
        &self,
        expression_statement: &ExpressionStatement,
        expression: Expression,
        _in_circuit: bool,
    ) -> Result<ExpressionStatement, CanonicalizeError> {
        Ok(ExpressionStatement {
            expression,
            span: expression_statement.span.clone(),
        })
    }

    fn reduce_block(
        &self,
        block: &Block,
        statements: Vec<Statement>,
        _in_circuit: bool,
    ) -> Result<Block, CanonicalizeError> {
        Ok(Block {
            statements,
            span: block.span.clone(),
        })
    }

    // Program
    fn reduce_program(
        &self,
        program: &Program,
        expected_input: Vec<FunctionInput>,
        imports: Vec<ImportStatement>,
        circuits: IndexMap<Identifier, Circuit>,
        functions: IndexMap<Identifier, Function>,
    ) -> Result<Program, CanonicalizeError> {
        Ok(Program {
            name: program.name.clone(),
            expected_input,
            imports,
            circuits,
            functions,
        })
    }

    fn reduce_function_input_variable(
        &self,
        variable: &FunctionInputVariable,
        identifier: Identifier,
        type_: Type,
        _in_circuit: bool,
    ) -> Result<FunctionInputVariable, CanonicalizeError> {
        Ok(FunctionInputVariable {
            identifier,
            const_: variable.const_,
            mutable: variable.mutable,
            type_,
            span: variable.span.clone(),
        })
    }

    fn reduce_function_input(
        &self,
        _input: &FunctionInput,
        new: FunctionInput,
        _in_circuit: bool,
    ) -> Result<FunctionInput, CanonicalizeError> {
        Ok(new)
    }

    fn reduce_package_or_packages(
        &self,
        _package_or_packages: &PackageOrPackages,
        new: PackageOrPackages,
    ) -> Result<PackageOrPackages, CanonicalizeError> {
        Ok(new)
    }

    fn reduce_import(
        &self,
        import: &ImportStatement,
        package_or_packages: PackageOrPackages,
    ) -> Result<ImportStatement, CanonicalizeError> {
        Ok(ImportStatement {
            package_or_packages,
            span: import.span.clone(),
        })
    }

    fn reduce_circuit_member(
        &self,
        _circuit_member: &CircuitMember,
        new: CircuitMember,
    ) -> Result<CircuitMember, CanonicalizeError> {
        Ok(new)
    }

    fn reduce_circuit(
        &self,
        _circuit: &Circuit,
        circuit_name: Identifier,
        members: Vec<CircuitMember>,
    ) -> Result<Circuit, CanonicalizeError> {
        Ok(Circuit { circuit_name, members })
    }

    fn reduce_annotation(&self, annotation: &Annotation, name: Identifier) -> Result<Annotation, CanonicalizeError> {
        Ok(Annotation {
            span: annotation.span.clone(),
            name,
            arguments: annotation.arguments.clone(),
        })
    }

    #[allow(clippy::too_many_arguments)]
    fn reduce_function(
        &self,
        function: &Function,
        identifier: Identifier,
        annotations: Vec<Annotation>,
        input: Vec<FunctionInput>,
        output: Option<Type>,
        block: Block,
        _in_circuit: bool,
    ) -> Result<Function, CanonicalizeError> {
        Ok(Function {
            identifier,
            annotations,
            input,
            output,
            block,
            span: function.span.clone(),
        })
    }
}
