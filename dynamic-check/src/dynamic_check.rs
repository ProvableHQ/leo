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

use crate::{DynamicCheckError, Function, FunctionError, LeoResolvedAst};
use leo_static_check::{FunctionInputType, FunctionType, SymbolTable, Type, TypeVariable};
use leo_typed::{
    Expression,
    Function as UnresolvedFunction,
    Identifier,
    Program,
    Span,
    Statement as UnresolvedStatement,
};

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Performs a dynamic type inference check over a program.
pub struct DynamicCheck {
    table: SymbolTable,
    functions: Vec<FunctionBody>,
}

impl DynamicCheck {
    ///
    /// Returns a new `DynamicCheck` from a given program and symbol table.
    ///
    pub fn new(program: &Program, symbol_table: SymbolTable) -> Self {
        let mut dynamic_check = Self {
            table: symbol_table,
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
    fn parse_functions(&mut self, functions: Vec<&UnresolvedFunction>) {
        for function in functions {
            self.parse_function(function)
        }
    }

    ///
    /// Collects a vector of `TypeAssertion` predicates from a function.
    ///
    fn parse_function(&mut self, function: &UnresolvedFunction) {
        let function_body = FunctionBody::new(function.clone(), self.table.clone());

        self.functions.push(function_body);
    }

    ///
    /// Returns the result of evaluating all `TypeAssertion` predicates.
    ///
    /// Will attempt to substitute a `Type` for all `TypeVariable`s.
    /// Returns a `LeoResolvedAst` if all `TypeAssertion` predicates are true.
    /// Returns ERROR if a `TypeAssertion` predicate is false or a solution does not exist.
    ///
    pub fn solve(self) -> Result<LeoResolvedAst, DynamicCheckError> {
        for function_body in self.functions {
            function_body.solve();
        }

        Ok(LeoResolvedAst::new())
    }
}

/// A vector of `TypeAssertion` predicates created from a function body.
#[derive(Clone)]
pub struct FunctionBody {
    function_type: FunctionType,
    statements: Vec<UnresolvedStatement>,
    user_defined_types: SymbolTable,
    type_assertions: Vec<TypeAssertion>,
    variable_table: VariableTable,
}

impl FunctionBody {
    ///
    /// Collects a vector of `TypeAssertion` predicates from a function.
    ///
    pub fn new(function: UnresolvedFunction, symbol_table: SymbolTable) -> Self {
        let name = &function.identifier.name;

        // Get function type from symbol table.
        let function_type = symbol_table.get_function(name).unwrap().clone();

        // Create a new mapping of variables to types.
        let mut variable_table = VariableTable::new();

        // Initialize function inputs as variables.
        variable_table.parse_function_inputs(&function_type.inputs);

        // Create new function body struct.
        // Update variables when encountering let/const variable definitions.
        let mut function_body = Self {
            statements: function.statements,
            function_type,
            user_defined_types: symbol_table,
            type_assertions: vec![],
            variable_table,
        };

        // Create type assertions for function statements
        function_body.parse_statements();

        function_body
    }

    ///
    /// Collects a vector of `TypeAssertion` predicates from a vector of statements.
    ///
    fn parse_statements(&mut self) {
        for statement in &self.statements {
            self.parse_statement(statement);
        }
    }

    ///
    /// Collects a vector of `TypeAssertion` predicates from a statement.
    ///
    fn parse_statement(&mut self, statement: &UnresolvedStatement) {
        match statement {
            UnresolvedStatement::Return(expression, span) => {
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
        let left = output_type.clone();

        // Create the right hand side from the statement return expression.
        let right = self.parse_expression(expression);

        // Create a new type assertion for the statement return.
        let type_assertion = TypeAssertion::new(left, right);

        // Push the new type assertion to this function's list of type assertions.
        self.type_assertions.push(type_assertion)
    }

    ///
    /// Returns the type of an expression.
    ///
    fn parse_expression(&mut self, expression: &Expression) -> Type {
        match expression {
            // Type variables
            Expression::Identifier(identifier) => Self::parse_identifier(&identifier),
            Expression::Implicit(name, _) => Self::parse_implicit(name),

            // Explicit types
            Expression::Boolean(_, _) => Type::Boolean,
            Expression::Address(_, _) => Type::Address,
            Expression::Field(_, _) => Type::Field,
            Expression::Group(_) => Type::Group,
            Expression::Integer(integer_type, _, _) => Type::IntegerType(integer_type.clone()),

            // Number operations
            Expression::Add(left, right, span) => self.parse_binary_expression(left, right, span),
            Expression::Sub(left, right, span) => self.parse_binary_expression(left, right, span),
            Expression::Mul(left, right, span) => self.parse_binary_expression(left, right, span),
            Expression::Div(left, right, span) => self.parse_binary_expression(left, right, span),
            Expression::Pow(left, right, span) => self.parse_binary_expression(left, right, span),
            Expression::Negate(expression, _span) => self.parse_expression(expression),

            // Boolean operations
            Expression::Not(expression, _span) => self.parse_boolean_expression(expression),
            Expression::Or(left, right, span) => self.parse_boolean_binary_expression(left, right, span),
            Expression::And(left, right, span) => self.parse_boolean_binary_expression(left, right, span),
            Expression::Eq(left, right, span) => self.parse_boolean_binary_expression(left, right, span),
            Expression::Ge(left, right, span) => self.parse_boolean_binary_expression(left, right, span),
            Expression::Le(left, right, span) => self.parse_boolean_binary_expression(left, right, span),
            Expression::Lt(left, right, span) => self.parse_boolean_binary_expression(left, right, span),

            // Conditionals
            Expression::IfElse(condition, first, second, span) => {
                self.parse_conditional_expression(condition, first, second, span)
            }

            expression => unimplemented!("expression {} not implemented", expression),
        }
    }

    ///
    /// Returns a new type variable from a given identifier
    ///
    fn parse_identifier(identifier: &Identifier) -> Type {
        Type::TypeVariable(TypeVariable::from(identifier.name.clone()))
    }

    ///
    /// Returns a new type variable from a given identifier
    ///
    fn parse_implicit(name: &String) -> Type {
        Type::TypeVariable(TypeVariable::from(name.clone()))
    }

    ///
    /// Returns the type of a binary expression.
    ///
    fn parse_binary_expression(&mut self, left: &Expression, right: &Expression, _span: &Span) -> Type {
        // Get the left expression type.
        let left_type = self.parse_expression(left);

        // Get the right expression type.
        let right_type = self.parse_expression(right);

        // TODO (collinc97) throw an error if left type does not match right type
        if left_type.ne(&right_type) {
            unimplemented!("Mismatched types parse_binary_expression")
        }

        left_type
    }

    ///
    /// Returns the `Boolean` type if the expression is a `Boolean` type.
    ///
    fn parse_boolean_expression(&mut self, expression: &Expression) -> Type {
        // Return the `Boolean` type
        let boolean_type = Type::Boolean;

        // Get the type of the expression
        let expression_type = self.parse_expression(expression);

        // TODO (collinc97) throw an error if the expression is not a `Boolean` type.
        if expression_type.ne(&boolean_type) {
            unimplemented!("Mismatched types parse_boolean_expression")
        }

        boolean_type
    }

    ///
    /// Returns the `Boolean` type if the binary expression is a `Boolean` type.
    ///
    fn parse_boolean_binary_expression(&mut self, left: &Expression, right: &Expression, _span: &Span) -> Type {
        // Return the `Boolean` type.
        let boolean_type = Type::Boolean;

        // Get the type of the binary expression.
        let binary_expression_type = self.parse_binary_expression(left, right, _span);

        // TODO (collinc97) throw an error if the binary expression is not a `Boolean` type.
        if binary_expression_type.ne(&boolean_type) {
            unimplemented!("Mismatched types parse_boolean_binary_expression")
        }

        boolean_type
    }

    ///
    /// Returns the type of the conditional expression.
    ///
    fn parse_conditional_expression(
        &mut self,
        condition: &Expression,
        first: &Expression,
        second: &Expression,
        _span: &Span,
    ) -> Type {
        // Check that the type of the condition expression is a boolean.
        let _condition_type = self.parse_boolean_expression(condition);

        // Check that the types of the first and second expression are equal.
        self.parse_binary_expression(first, second, _span)
    }

    ///
    /// Returns a new `Function` if all `TypeAssertions` can be solved successfully.
    ///
    fn solve(self) -> Result<Function, FunctionError> {
        let mut unsolved = self.type_assertions.clone();

        while !unsolved.is_empty() {
            // Pop type assertion from list
            let type_assertion = unsolved.pop().unwrap();

            println!("assertion: {:?}", type_assertion);

            // Get type variable and type
            if let Some((type_variable, type_)) = type_assertion.get_pair() {
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

        // Return a new resolved function struct.
        Function::new(self.variable_table, self.function_type, self.statements)
    }
}

/// A structure for tracking the types of user defined variables in a program.
#[derive(Clone)]
pub struct VariableTable(pub HashMap<String, Type>);

impl VariableTable {
    ///
    /// Returns a new variable table
    ///
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    ///
    /// Insert a name -> type pair into the variable table.
    ///
    /// If the variable table did not have this key present, [`None`] is returned.
    ///
    /// If the variable table did have this key present, the type is updated, and the old
    /// type is returned.
    ///
    pub fn insert(&mut self, name: String, type_: Type) -> Option<Type> {
        self.0.insert(name, type_)
    }

    ///
    /// Returns a reference to the type corresponding to the name.
    ///
    /// If the variable table did not have this name present, [`None`] is returned.
    ///
    pub fn get(&self, name: &String) -> Option<&Type> {
        self.0.get(name)
    }

    ///
    /// Inserts a vector of function input types into the variable table.
    ///
    pub fn parse_function_inputs(&mut self, function_inputs: &Vec<FunctionInputType>) {
        for input in function_inputs {
            let input_name = input.identifier().name.clone();
            let input_type = input.type_().clone();

            // TODO (collinc97) throw an error for duplicate function input names.
            self.insert(input_name, input_type);
        }
    }
}

/// A predicate that evaluates equality between two `Types`s.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TypeAssertion {
    left: Type,
    right: Type,
}

impl TypeAssertion {
    ///
    /// Returns a `TypeAssertion` predicate from given left and right `Types`s
    ///
    pub fn new(left: Type, right: Type) -> Self {
        Self { left, right }
    }

    ///
    /// Substitutes the given `TypeVariable` for each `Types` in the `TypeAssertion`.
    ///
    pub fn substitute(&mut self, _variable: &TypeVariable, _type_: &Type) {
        // self.left.substitute(variable, type_);
        // self.right.substitute(variable, type_);
    }

    ///
    /// Returns true if the left `Types` is equal to the right `Types`.
    ///
    pub fn evaluate(&self) -> bool {
        self.left.eq(&self.right)
    }

    ///
    /// Returns the (type variable, type) pair from this assertion.
    ///
    pub fn get_pair(&self) -> Option<(TypeVariable, Type)> {
        match (&self.left, &self.right) {
            (Type::TypeVariable(variable), type_) => Some((variable.clone(), type_.clone())),
            (type_, Type::TypeVariable(variable)) => Some((variable.clone(), type_.clone())),
            (_type1, _type2) => None, // No (type variable, type) pair can be returned from two types
        }
    }
}
