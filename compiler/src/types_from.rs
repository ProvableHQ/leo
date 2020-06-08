//! Logic to convert from an abstract syntax tree (ast) representation to a Leo program.

use crate::{types, Import, ImportSymbol};
use leo_ast::{
    circuits::{
        Circuit,
        CircuitFieldDefinition as AstCircuitFieldDefinition,
        CircuitFunction,
        CircuitMember
    },
    common::{
        Visibility,
        Private,
    },
    files::File,
    functions::{
        Function,
        FunctionInput,
        TestFunction
    },
    imports::{
        Import as AstImport,
        ImportSymbol as AstImportSymbol,
    },
    operations::{
        AssignOperation,
    },
    statements::{
        AssertStatement,
        AssignStatement,
        ConditionalStatement,
        ConditionalNestedOrEndStatement,
        DefinitionStatement,
        ExpressionStatement,
        ForStatement,
        MultipleAssignmentStatement,
        ReturnStatement,
        Statement,
    },
};
use leo_types::{Assignee, Expression, Identifier, Integer, Type, Variable};

use std::collections::HashMap;

/// pest ast -> types::Statement

impl<'ast> From<ReturnStatement<'ast>> for types::Statement {
    fn from(statement: ReturnStatement<'ast>) -> Self {
        types::Statement::Return(
            statement
                .expressions
                .into_iter()
                .map(|expression| Expression::from(expression))
                .collect(),
        )
    }
}

impl<'ast> From<DefinitionStatement<'ast>> for types::Statement {
    fn from(statement: DefinitionStatement<'ast>) -> Self {
        types::Statement::Definition(
            Variable::from(statement.variable),
            Expression::from(statement.expression),
        )
    }
}

impl<'ast> From<AssignStatement<'ast>> for types::Statement {
    fn from(statement: AssignStatement<'ast>) -> Self {
        match statement.assign {
            AssignOperation::Assign(ref _assign) => types::Statement::Assign(
                Assignee::from(statement.assignee),
                Expression::from(statement.expression),
            ),
            operation_assign => {
                // convert assignee into postfix expression
                let converted = Expression::from(statement.assignee.clone());

                match operation_assign {
                    AssignOperation::AddAssign(ref _assign) => types::Statement::Assign(
                        Assignee::from(statement.assignee),
                        Expression::Add(
                            Box::new(converted),
                            Box::new(Expression::from(statement.expression)),
                        ),
                    ),
                    AssignOperation::SubAssign(ref _assign) => types::Statement::Assign(
                        Assignee::from(statement.assignee),
                        Expression::Sub(
                            Box::new(converted),
                            Box::new(Expression::from(statement.expression)),
                        ),
                    ),
                    AssignOperation::MulAssign(ref _assign) => types::Statement::Assign(
                        Assignee::from(statement.assignee),
                        Expression::Mul(
                            Box::new(converted),
                            Box::new(Expression::from(statement.expression)),
                        ),
                    ),
                    AssignOperation::DivAssign(ref _assign) => types::Statement::Assign(
                        Assignee::from(statement.assignee),
                        Expression::Div(
                            Box::new(converted),
                            Box::new(Expression::from(statement.expression)),
                        ),
                    ),
                    AssignOperation::PowAssign(ref _assign) => types::Statement::Assign(
                        Assignee::from(statement.assignee),
                        Expression::Pow(
                            Box::new(converted),
                            Box::new(Expression::from(statement.expression)),
                        ),
                    ),
                    AssignOperation::Assign(ref _assign) => {
                        unimplemented!("cannot assign twice to assign statement")
                    }
                }
            }
        }
    }
}

impl<'ast> From<MultipleAssignmentStatement<'ast>> for types::Statement {
    fn from(statement: MultipleAssignmentStatement<'ast>) -> Self {
        let variables = statement
            .variables
            .into_iter()
            .map(|typed_variable| Variable::from(typed_variable))
            .collect();

        types::Statement::MultipleAssign(
            variables,
            Expression::FunctionCall(
                Box::new(Expression::from(statement.function_name)),
                statement
                    .arguments
                    .into_iter()
                    .map(|e| Expression::from(e))
                    .collect(),
            ),
        )
    }
}

impl<'ast> From<ConditionalNestedOrEndStatement<'ast>> for types::ConditionalNestedOrEndStatement {
    fn from(statement: ConditionalNestedOrEndStatement<'ast>) -> Self {
        match statement {
            ConditionalNestedOrEndStatement::Nested(nested) => types::ConditionalNestedOrEndStatement::Nested(
                Box::new(types::ConditionalStatement::from(*nested)),
            ),
            ConditionalNestedOrEndStatement::End(statements) => types::ConditionalNestedOrEndStatement::End(
                statements
                    .into_iter()
                    .map(|statement| types::Statement::from(statement))
                    .collect(),
            ),
        }
    }
}

impl<'ast> From<ConditionalStatement<'ast>> for types::ConditionalStatement {
    fn from(statement: ConditionalStatement<'ast>) -> Self {
        types::ConditionalStatement {
            condition: Expression::from(statement.condition),
            statements: statement
                .statements
                .into_iter()
                .map(|statement| types::Statement::from(statement))
                .collect(),
            next: statement
                .next
                .map(|n_or_e| Some(types::ConditionalNestedOrEndStatement::from(n_or_e)))
                .unwrap_or(None),
        }
    }
}

impl<'ast> From<ForStatement<'ast>> for types::Statement {
    fn from(statement: ForStatement<'ast>) -> Self {
        let from = match Expression::from(statement.start) {
            Expression::Integer(number) => number,
            Expression::Implicit(string) => Integer::from_implicit(string),
            expression => unimplemented!("Range bounds should be integers, found {}", expression),
        };
        let to = match Expression::from(statement.stop) {
            Expression::Integer(number) => number,
            Expression::Implicit(string) => Integer::from_implicit(string),
            expression => unimplemented!("Range bounds should be integers, found {}", expression),
        };

        types::Statement::For(
            Identifier::from(statement.index),
            from,
            to,
            statement
                .statements
                .into_iter()
                .map(|statement| types::Statement::from(statement))
                .collect(),
        )
    }
}

impl<'ast> From<AssertStatement<'ast>> for types::Statement {
    fn from(statement: AssertStatement<'ast>) -> Self {
        match statement {
            AssertStatement::AssertEq(assert_eq) => types::Statement::AssertEq(
                Expression::from(assert_eq.left),
                Expression::from(assert_eq.right),
            ),
        }
    }
}

impl<'ast> From<ExpressionStatement<'ast>> for types::Statement {
    fn from(statement: ExpressionStatement<'ast>) -> Self {
        types::Statement::Expression(Expression::from(statement.expression))
    }
}

impl<'ast> From<Statement<'ast>> for types::Statement {
    fn from(statement: Statement<'ast>) -> Self {
        match statement {
            Statement::Return(statement) => types::Statement::from(statement),
            Statement::Definition(statement) => types::Statement::from(statement),
            Statement::Assign(statement) => types::Statement::from(statement),
            Statement::MultipleAssignment(statement) => types::Statement::from(statement),
            Statement::Conditional(statement) => {
                types::Statement::Conditional(types::ConditionalStatement::from(statement))
            }
            Statement::Iteration(statement) => types::Statement::from(statement),
            Statement::Assert(statement) => types::Statement::from(statement),
            Statement::Expression(statement) => types::Statement::from(statement),
        }
    }
}

/// pest ast -> types::Circuit

impl<'ast> From<AstCircuitFieldDefinition<'ast>> for types::CircuitMember {
    fn from(circuit_value: AstCircuitFieldDefinition<'ast>) -> Self {
        types::CircuitMember::CircuitField(
            Identifier::from(circuit_value.identifier),
            Type::from(circuit_value._type),
        )
    }
}

impl<'ast> From<CircuitFunction<'ast>> for types::CircuitMember {
    fn from(circuit_function: CircuitFunction<'ast>) -> Self {
        types::CircuitMember::CircuitFunction(
            circuit_function._static.is_some(),
            types::Function::from(circuit_function.function),
        )
    }
}

impl<'ast> From<CircuitMember<'ast>> for types::CircuitMember {
    fn from(object: CircuitMember<'ast>) -> Self {
        match object {
            CircuitMember::CircuitFieldDefinition(circuit_value) => {
                types::CircuitMember::from(circuit_value)
            }
            CircuitMember::CircuitFunction(circuit_function) => {
                types::CircuitMember::from(circuit_function)
            }
        }
    }
}

impl<'ast> From<Circuit<'ast>> for types::Circuit {
    fn from(circuit: Circuit<'ast>) -> Self {
        let variable = Identifier::from(circuit.identifier);
        let members = circuit
            .members
            .into_iter()
            .map(|member| types::CircuitMember::from(member))
            .collect();

        types::Circuit {
            identifier: variable,
            members,
        }
    }
}

/// pest ast -> function types::Parameters

impl<'ast> From<FunctionInput<'ast>> for types::InputModel {
    fn from(parameter: FunctionInput<'ast>) -> Self {
        types::InputModel {
            identifier: Identifier::from(parameter.identifier),
            mutable: parameter.mutable.is_some(),
            // private by default
            private: parameter.visibility.map_or(true, |visibility| {
                visibility.eq(&Visibility::Private(Private {}))
            }),
            _type: Type::from(parameter._type),
        }
    }
}

/// pest ast -> types::Function

impl<'ast> From<Function<'ast>> for types::Function {
    fn from(function_definition: Function<'ast>) -> Self {
        let function_name = Identifier::from(function_definition.function_name);
        let parameters = function_definition
            .parameters
            .into_iter()
            .map(|parameter| types::InputModel::from(parameter))
            .collect();
        let returns = function_definition
            .returns
            .into_iter()
            .map(|return_type| Type::from(return_type))
            .collect();
        let statements = function_definition
            .statements
            .into_iter()
            .map(|statement| types::Statement::from(statement))
            .collect();

        types::Function {
            function_name,
            inputs: parameters,
            returns,
            statements,
        }
    }
}

/// pest ast -> Import

impl<'ast> From<AstImportSymbol<'ast>> for ImportSymbol {
    fn from(symbol: AstImportSymbol<'ast>) -> Self {
        ImportSymbol {
            symbol: Identifier::from(symbol.value),
            alias: symbol.alias.map(|alias| Identifier::from(alias)),
        }
    }
}

impl<'ast> From<AstImport<'ast>> for Import {
    fn from(import: AstImport<'ast>) -> Self {
        Import {
            path_string: import.source.value,
            symbols: import
                .symbols
                .into_iter()
                .map(|symbol| ImportSymbol::from(symbol))
                .collect(),
        }
    }
}

/// pest ast -> Test
impl<'ast> From<TestFunction<'ast>> for types::Test {
    fn from(test: TestFunction) -> Self {
        types::Test(types::Function::from(test.function))
    }
}

/// pest ast -> types::Program

impl<'ast> types::Program {
    pub fn from(file: File<'ast>, name: String) -> Self {
        // Compiled ast -> aleo program representation
        let imports = file
            .imports
            .into_iter()
            .map(|import| Import::from(import))
            .collect::<Vec<Import>>();

        let mut circuits = HashMap::new();
        let mut functions = HashMap::new();
        let mut tests = HashMap::new();
        let mut num_parameters = 0usize;

        file.circuits.into_iter().for_each(|circuit| {
            circuits.insert(
                Identifier::from(circuit.identifier.clone()),
                types::Circuit::from(circuit),
            );
        });
        file.functions.into_iter().for_each(|function_def| {
            functions.insert(
                Identifier::from(function_def.function_name.clone()),
                types::Function::from(function_def),
            );
        });
        file.tests.into_iter().for_each(|test_def| {
            tests.insert(
                Identifier::from(test_def.function.function_name.clone()),
                types::Test::from(test_def),
            );
        });

        if let Some(main_function) = functions.get(&Identifier::new("main".into())) {
            num_parameters = main_function.inputs.len();
        }

        types::Program {
            name: Identifier::new(name),
            num_parameters,
            imports,
            circuits,
            functions,
            tests,
        }
    }
}
