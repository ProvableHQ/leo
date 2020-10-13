// Copyright (C) 2019-2020 Aleo Systems Inc.
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

use leo_typed::{Expression, Function, Identifier, Program, Span, Statement};

use leo_static_check::{FunctionType, SymbolTable, Type};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

/// Performs a dynamic type inference check over a program.
pub struct DynamicCheck {
    symbol_table: SymbolTable,
    functions: Vec<FunctionBody>,
}

impl DynamicCheck {
    ///
    /// Return a new `DynamicCheck` from a given program and symbol table.
    ///
    pub fn new(program: &Program, symbol_table: SymbolTable) -> Self {
        let mut dynamic_check = Self {
            symbol_table,
            functions: vec![],
        };

        dynamic_check.parse_program(program);

        dynamic_check
    }

    ///
    /// Collects a vector of `TypeAssertion` predicates from a program.
    ///
    fn parse_program(&mut self, program: &Program) {
        let functions = program
            .functions
            .iter()
            .map(|(_identifier, function)| function)
            .collect::<Vec<_>>();

        self.parse_functions(functions);
    }

    ///
    /// Collects a vector of `TypeAssertion` predicates from a vector of functions.
    ///
    fn parse_functions(&mut self, functions: Vec<&Function>) {
        for function in functions {
            self.parse_function(function)
        }
    }

    ///
    /// Collects a vector of `TypeAssertion` predicates from a function.
    ///
    fn parse_function(&mut self, function: &Function) {
        let function_body = FunctionBody::new(function.clone(), self.symbol_table.clone());

        self.functions.push(function_body);
    }

    ///
    /// Return the result of evaluating all `TypeAssertion` predicates.
    ///
    /// Will attempt to substitute a `Type` for all `TypeVariable`s.
    /// Returns `true` if all `TypeAssertion` predicates are true.
    /// Returns ERROR if a `TypeAssertion` predicate is false or a solution does not exist.
    ///
    pub fn solve(self) -> bool {
        for function_body in self.functions {
            function_body.solve();
        }

        true
    }
}

/// A vector of `TypeAssertion` predicates created from a function body.
#[derive(Clone)]
pub struct FunctionBody {
    function_type: FunctionType,
    symbol_table: SymbolTable,
    type_assertions: Vec<TypeAssertion>,
    type_variables: HashSet<TypeVariable>,
}

impl FunctionBody {
    ///
    /// Collects a vector of `TypeAssertion` predicates from a function.
    ///
    pub fn new(function: Function, symbol_table: SymbolTable) -> Self {
        let name = &function.identifier.name;

        // Get function type from symbol table.
        let function_type = symbol_table.get_function(name).unwrap().clone();

        // Create new function body struct.
        let mut function_body = Self {
            function_type,
            symbol_table,
            type_assertions: vec![],
            type_variables: HashSet::new(),
        };

        // Build symbol table for variables.
        // Initialize function inputs as variables.
        // Update inputs when encountering let/const variable definitions.

        // Create type assertions for function statements
        function_body.parse_statements(&function.statements);

        function_body
    }

    ///
    /// Collects a vector of `TypeAssertion` predicates from a vector of statements.
    ///
    fn parse_statements(&mut self, statements: &Vec<Statement>) {
        for statement in statements {
            self.parse_statement(statement);
        }
    }

    ///
    /// Collects a vector of `TypeAssertion` predicates from a statement.
    ///
    fn parse_statement(&mut self, statement: &Statement) {
        match statement {
            Statement::Return(expression, span) => {
                self.parse_statement_return(expression, span);
            }
            statement => unimplemented!("statement {} not implemented", statement),
        }
    }

    ///
    /// Collects a `TypeAssertion` predicate from a statement return.
    ///
    fn parse_statement_return(&mut self, expression: &Expression, _span: &Span) {
        // Get the function output type.
        let output_type = &self.function_type.output.type_;

        // Create the left hand side of a type assertion.
        let left = TypeElement::Type(output_type.clone());

        // Create the right hand side from the statement return expression.
        let right = TypeElement::new(expression, self.symbol_table.clone());

        // Create a new type assertion for the statement return.
        let type_assertion = TypeAssertion::new(left, right);

        // Push the new type assertion to this function's list of type assertions.
        self.type_assertions.push(type_assertion)
    }

    ///
    /// Iteratively solves all `TypeAssertions`.
    ///
    fn solve(self) {
        let mut unsolved = self.type_assertions.clone();

        while !unsolved.is_empty() {
            // Pop type assertion from list
            let type_assertion = unsolved.pop().unwrap();

            println!("assertion: {:?}", type_assertion);

            // Get type variable and type
            if let Some((type_variable, type_)) = type_assertion.get_substitute() {
                // Substitute type variable for type in unsolved
                for original in &mut unsolved {
                    original.substitute(&type_variable, &type_)
                }
            }
        }

        // for type_assertion in unsolved.pop() {
        //     if let Some((type_variable, type_)) = type_assertion.get_substitute() {
        //         // Substitute type variable in unsolved type assertions
        //         for mut original in unsolved {
        //             original.substitute(type_variable, type_)
        //         }
        //     }
        // }
    }
}

/// A predicate that evaluates equality between two `TypeElement`s.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TypeAssertion {
    left: TypeElement,
    right: TypeElement,
}

impl TypeAssertion {
    ///
    /// Return a `TypeAssertion` predicate from given left and right `TypeElement`s
    ///
    pub fn new(left: TypeElement, right: TypeElement) -> Self {
        Self { left, right }
    }

    ///
    /// Substitute the given `TypeVariable` for each `TypeElement` in the `TypeAssertion`.
    ///
    pub fn substitute(&mut self, variable: &TypeVariable, type_: &TypeElement) {
        self.left.substitute(variable, type_);
        self.right.substitute(variable, type_);
    }

    ///
    /// Returns true if the left `TypeElement` is equal to the right `TypeElement`.
    ///
    pub fn evaluate(&self) -> bool {
        self.left.eq(&self.right)
    }

    pub fn get_substitute(&self) -> Option<(TypeVariable, TypeElement)> {
        match (&self.left, &self.right) {
            (TypeElement::Variable(variable), element) => Some((variable.clone(), element.clone())),
            (TypeElement::Type(type_), TypeElement::Variable(variable)) => {
                Some((variable.clone(), TypeElement::Type(type_.clone())))
            }
            (TypeElement::Type(_), TypeElement::Type(_)) => None,
        }
    }
}

/// A `Type` or a `TypeVariable` in a `TypeAssertion`.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum TypeElement {
    Type(Type),
    Variable(TypeVariable),
}

impl TypeElement {
    ///
    /// Return a new `TypeElement` from the given expression and symbol table.
    ///
    pub fn new(expression: &Expression, _symbol_table: SymbolTable) -> Self {
        match expression {
            Expression::Identifier(identifier) => Self::from(identifier.clone()),
            Expression::Implicit(name, _) => Self::from(name.clone()),
            Expression::Boolean(_, _) => Self::boolean(),
            expression => unimplemented!("expression {} not implemented", expression),
        }
    }

    ///
    /// Return a boolean `TypeElement`.
    ///
    pub fn boolean() -> Self {
        TypeElement::Type(Type::Boolean)
    }

    ///
    /// Substitute the given `TypeElement` if self is equal to the given `TypeVariable`.
    ///
    pub fn substitute(&mut self, variable: &TypeVariable, type_: &TypeElement) {
        match self {
            TypeElement::Type(_) => {}
            TypeElement::Variable(original) => {
                if original.eq(&variable) {
                    *self = type_.clone()
                }
            }
        }
    }
}

impl From<String> for TypeElement {
    fn from(name: String) -> Self {
        Self::Variable(TypeVariable::from(name))
    }
}

impl From<Identifier> for TypeElement {
    fn from(identifier: Identifier) -> Self {
        Self::Variable(TypeVariable::from(identifier))
    }
}

/// An unknown type in a `TypeAssertion`.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TypeVariable {
    name: String,
}

impl From<String> for TypeVariable {
    fn from(name: String) -> Self {
        Self { name }
    }
}

impl From<Identifier> for TypeVariable {
    fn from(identifier: Identifier) -> Self {
        Self::from(identifier.name)
    }
}
