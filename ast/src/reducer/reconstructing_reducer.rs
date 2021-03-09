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

//! This module contains the reducer which iterates through ast nodes - converting them into
//! asg nodes and saving relevant information.

use crate::{
    AssigneeAccess,
    Block,
    Circuit,
    CircuitMember,
    ConditionalStatement,
    Expression,
    Function,
    FunctionInput,
    FunctionInputVariable,
    Identifier,
    ImportStatement,
    Package,
    PackageOrPackages,
    Packages,
    Program,
    Statement,
    TestFunction,
    Type,
    VariableName,
};
use indexmap::IndexMap;

#[allow(unused_variables)]
#[allow(clippy::redundant_closure)] // Clippy bug line 188
pub trait ReconstructingReducer {
    // ciruits/functions/tests map identifier -> (identifier_item, value_item)
    fn reduce_program(
        &mut self,
        program: &Program,
        expected_input: Vec<FunctionInput>,
        imports: Vec<ImportStatement>,
        circuits: IndexMap<Identifier, Circuit>,
        functions: IndexMap<Identifier, Function>,
        tests: IndexMap<Identifier, TestFunction>,
    ) -> Program {
        Program {
            name: program.name.clone(),
            expected_input,
            imports,
            circuits,
            functions,
            tests,
        }
    }

    fn reduce_function_input(&mut self, input: &FunctionInput, item: FunctionInput) -> Option<FunctionInput> {
        Some(item)
    }

    fn reduce_import_statement(
        &mut self,
        import: &ImportStatement,
        package_or_packages: PackageOrPackages,
    ) -> Option<ImportStatement> {
        Some(ImportStatement {
            package_or_packages,
            span: import.span.clone(),
        })
    }

    fn reduce_circuit(
        &mut self,
        circuit: &Circuit,
        circuit_name: Identifier,
        members: Vec<CircuitMember>,
    ) -> Option<Circuit> {
        Some(Circuit { circuit_name, members })
    }

    fn reduce_function(
        &mut self,
        function: &Function,
        identifier: Identifier,
        input: Vec<FunctionInput>,
        output: Option<Type>,
        block: Block,
    ) -> Option<Function> {
        Some(Function {
            identifier,
            input,
            output,
            block,
            span: function.span.clone(),
        })
    }

    fn reduce_test_function(
        &mut self,
        test_function: &TestFunction,
        function: Function,
        input_file: Option<Identifier>,
    ) -> Option<TestFunction> {
        Some(TestFunction { function, input_file })
    }

    fn reduce_identifier(&mut self, identifier: &Identifier) -> Identifier {
        identifier.clone()
    }

    fn reduce_function_input_variable(
        &mut self,
        function_input_variable: &FunctionInputVariable,
        identifier: Identifier,
        type_: Type,
    ) -> FunctionInputVariable {
        FunctionInputVariable {
            identifier,
            const_: function_input_variable.const_,
            mutable: function_input_variable.mutable,
            type_,
            span: function_input_variable.span.clone(),
        }
    }

    fn reduce_type(&mut self, type_: &Type, items: Type) -> Type {
        items
    }

    fn reduce_package(&mut self, package_or_packages: &PackageOrPackages) -> PackageOrPackages {
        match package_or_packages {
            PackageOrPackages::Package(package) => {
                let name = self.reduce_identifier(&package.name);

                PackageOrPackages::Package(Package {
                    name,
                    access: package.access.clone(),
                    span: package.span.clone(),
                })
            }
            PackageOrPackages::Packages(packages) => {
                let name = self.reduce_identifier(&packages.name);

                PackageOrPackages::Packages(Packages {
                    name,
                    accesses: packages.accesses.clone(),
                    span: packages.span.clone(),
                })
            }
        }
    }

    fn reduce_circuit_member(&mut self, circuit_member: &CircuitMember, items: CircuitMember) -> Option<CircuitMember> {
        Some(items)
    }

    fn reduce_statement(&mut self, statement: &Statement, items: Statement) -> Statement {
        items
    }

    fn reduce_assignee_access(
        &mut self,
        assignee_access: &AssigneeAccess,
        item: AssigneeAccess,
    ) -> Option<AssigneeAccess> {
        Some(item)
    }

    fn reduce_conditional_statement(
        &mut self,
        statement: &ConditionalStatement,
        condition: Expression,
        block: Block,
        next: Option<Statement>,
    ) -> ConditionalStatement {
        ConditionalStatement {
            condition,
            block,
            next: next.map(|item| Box::new(item)),
            span: statement.span.clone(),
        }
    }

    fn reduce_variable_name(&mut self, variable_name: &VariableName, identifier: Identifier) -> VariableName {
        VariableName {
            mutable: variable_name.mutable,
            identifier,
            span: variable_name.span.clone(),
        }
    }

    fn reduce_expression(&mut self, expression: &Expression, items: Expression) -> Expression {
        items
    }
}
