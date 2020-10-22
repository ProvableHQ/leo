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

use crate::{DynamicCheckError, FrameError, VariableTableError};
use leo_static_check::{CircuitType, FunctionInputType, FunctionType, SymbolTable, Type, TypeVariable};
use leo_typed::{
    Assignee,
    CircuitVariableDefinition,
    ConditionalNestedOrEndStatement,
    ConditionalStatement,
    ConsoleFunctionCall,
    Declare,
    Expression,
    Function as UnresolvedFunction,
    Identifier,
    Program,
    RangeOrExpression,
    Span,
    SpreadOrExpression,
    Statement as UnresolvedStatement,
    Variables,
};

use leo_typed::integer_type::IntegerType;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Performs a dynamic type inference check over a program.
pub struct DynamicCheck {
    table: SymbolTable,
    functions: Vec<Frame>,
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
        let frame = Frame::new(function.clone(), self.table.clone());

        self.functions.push(frame);
    }

    ///
    /// Returns the result of evaluating all `TypeAssertion` predicates.
    ///
    /// Will attempt to substitute a `Type` for all `TypeVariable`s.
    /// Returns a `LeoResolvedAst` if all `TypeAssertion` predicates are true.
    /// Returns ERROR if a `TypeAssertion` predicate is false or a solution does not exist.
    ///
    pub fn solve(self) -> Result<(), DynamicCheckError> {
        for frame in self.functions {
            frame.solve()?;
        }

        Ok(())
    }
}

/// A vector of `TypeAssertion` predicates created from a function body.
#[derive(Clone)]
pub struct Frame {
    pub function_type: FunctionType,
    pub self_type: Option<CircuitType>,
    pub scopes: Vec<Scope>,
    pub statements: Vec<UnresolvedStatement>,
    pub type_assertions: Vec<TypeAssertion>,
    pub user_defined_types: SymbolTable,
}

impl Frame {
    ///
    /// Collects a vector of `TypeAssertion` predicates from a function.
    ///
    pub fn new(function: UnresolvedFunction, symbol_table: SymbolTable) -> Self {
        let name = &function.identifier.name;

        // Get function type from symbol table.
        let function_type = symbol_table.get_function(name).unwrap().clone();

        // Create a new scope for the function variables.
        let mut scope = Scope::new(None);

        // Initialize function inputs as variables.
        scope.parse_function_inputs(&function_type.inputs);

        // Create new list of scopes for frame.
        let scopes = vec![scope];

        // Create new frame struct.
        // Update variables when encountering let/const variable definitions.
        let mut frame = Self {
            function_type,
            self_type: None,
            scopes,
            statements: function.statements,
            type_assertions: vec![],
            user_defined_types: symbol_table,
        };

        // Create type assertions for function statements
        frame.parse_statements();

        frame
    }

    ///
    /// Pushes a new variable `Scope` to the list of scopes in the current `Frame`.
    ///
    fn push_scope(&mut self, scope: Scope) {
        self.scopes.push(scope)
    }

    ///
    /// Removes and returns the most recent `Scope` from the list of scopes in the current `Frame`.
    ///
    fn pop_scope(&mut self) -> Option<Scope> {
        self.scopes.pop()
    }

    ///
    /// Insert a variable into the symbol table in the current scope.
    ///
    fn insert_variable(&mut self, name: String, type_: Type) -> Option<Type> {
        // Modify the current scope.
        let scope = self.scopes.last_mut().unwrap();

        // Insert the variable name -> type.
        scope.variables.insert(name, type_)
    }

    ///
    /// Get a variable's type from the symbol table in the current scope.
    ///
    fn get_variable(&self, name: &String) -> &Type {
        // Lookup in the current scope.
        let scope = self.scopes.last().unwrap();

        // Get the variable by name.
        scope.get_variable(name).unwrap()
    }

    ///
    /// Creates a new equality type assertion between the given types.
    ///
    fn assert_equal(&mut self, left: Type, right: Type) {
        let type_assertion = TypeAssertion::new_equality(left, right);

        self.type_assertions.push(type_assertion);
    }

    ///
    /// Creates a new membership type assertion between a given and set of types.
    ///
    fn assert_membership(&mut self, given: Type, set: Vec<Type>) {
        let type_assertion = TypeAssertion::new_membership(given, set);

        self.type_assertions.push(type_assertion);
    }

    ///
    /// Creates a new membership type assertion between a given and the set of negative integer types.
    ///
    fn assert_negative_integer(&mut self, given: &Type) {
        let negative_integer_types = Type::negative_integer_types();

        self.assert_membership(given.clone(), negative_integer_types)
    }

    ///
    /// Creates a new membership type assertion between a given and the set of all integer types.
    ///
    fn assert_integer(&mut self, given: &Type) {
        let integer_types = Type::integer_types();

        self.assert_membership(given.clone(), integer_types)
    }

    ///
    /// Creates a new membership type assertion between a given and the set of index types.
    ///
    fn assert_index(&mut self, given: &Type) {
        let index_types = Type::index_types();

        self.assert_membership(given.clone(), index_types)
    }

    ///
    /// Collects a vector of `TypeAssertion` predicates from a vector of statements.
    ///
    fn parse_statements(&mut self) {
        for statement in self.statements.clone() {
            self.parse_statement(&statement);
        }
    }

    ///
    /// Collects a vector of `TypeAssertion` predicates from a statement.
    ///
    fn parse_statement(&mut self, statement: &UnresolvedStatement) {
        match statement {
            UnresolvedStatement::Return(expression, span) => {
                self.parse_return(expression, span);
            }
            UnresolvedStatement::Definition(declare, variables, expression, span) => {
                self.parse_definition(declare, variables, expression, span)
            }
            UnresolvedStatement::Assign(assignee, expression, span) => self.parse_assign(assignee, expression, span),
            UnresolvedStatement::Conditional(conditional, span) => self.parse_statement_conditional(conditional, span),
            UnresolvedStatement::Iteration(identifier, from, to, statements, span) => {
                self.parse_iteration(identifier, from, to, statements, span)
            }
            UnresolvedStatement::Expression(expression, span) => self.parse_statement_expression(expression, span),
            UnresolvedStatement::Console(console_call) => self.parse_console_function_call(console_call),
        }
    }

    ///
    /// Collects `TypeAssertion` predicates from a return statement.
    ///
    fn parse_return(&mut self, expression: &Expression, _span: &Span) {
        // Get the function output type.
        let output_type = &self.function_type.output.type_;

        // Create the left hand side of a type assertion.
        let left = output_type.clone();

        // Create the right hand side from the statement return expression.
        let right = self.parse_expression(expression);

        // Create a new type assertion for the statement return.
        self.assert_equal(left, right);
    }

    ///
    /// Collects `Type Assertion` predicates from a definition statement.
    ///
    fn parse_definition(&mut self, _declare: &Declare, variables: &Variables, expression: &Expression, span: &Span) {
        // Parse the definition expression.
        let actual_type = self.parse_expression(expression);

        // Check if an explicit type is given.
        if let Some(type_) = variables.type_.clone() {
            // Convert the expected type into a dynamic check type.
            let expected_type = match self.self_type.clone() {
                Some(circuit_type) => {
                    Type::new_from_circuit(&self.user_defined_types, type_, circuit_type.identifier, span.clone())
                        .unwrap()
                }
                None => Type::new(&self.user_defined_types, type_, span.clone()).unwrap(),
            };

            // Assert that the expected type is equal to the actual type.
            self.assert_equal(expected_type.clone(), actual_type.clone())
        }

        // Check for multiple defined variables.
        if variables.names.len() == 1 {
            // Insert variable into symbol table
            let variable = variables.names[0].clone();

            // TODO (collinc97) throw error for duplicate variable definitions.
            self.insert_variable(variable.identifier.name, actual_type).unwrap();
        } else {
            // Expect a tuple type.
            let types = match actual_type {
                Type::Tuple(types) => types,
                _ => unimplemented!("expected a tuple type for multiple defined variables"),
            };

            // Check number of variables == number of types.
            if types.len() != variables.names.len() {
                unimplemented!("Incorrect number of defined variables")
            }

            // Insert variables into symbol table
            for (variable, type_) in variables.names.iter().zip(types) {
                // TODO (collinc97) throw error for duplicate variable definitions.
                self.insert_variable(variable.identifier.name.clone(), type_).unwrap();
            }
        }
    }

    ///
    /// Collects `TypeAssertion` predicates from an assignment statement.
    ///
    fn parse_assign(&mut self, _assignee: &Assignee, _expression: &Expression, _span: &Span) {
        // TODO (collinc97) impl locations.
    }

    ///
    /// Collects `TypeAssertion` predicates from a block of statements.
    ///
    fn parse_block(&mut self, statements: &Vec<UnresolvedStatement>, _span: &Span) {
        // Push new scope.
        let scope = Scope::new(self.scopes.last().map(|scope| scope.clone()));
        self.push_scope(scope);

        // Parse all statements.
        for statement in statements.iter() {
            self.parse_statement(statement);
        }

        // Pop out of scope.
        let _scope = self.pop_scope();
    }

    ///
    /// Collects `TypeAssertion` predicates from a conditional statement.
    ///
    /// Creates a new scope for each code block in the conditional.
    ///
    fn parse_statement_conditional(&mut self, conditional: &ConditionalStatement, span: &Span) {
        // Parse the condition expression.
        let condition = self.parse_expression(&conditional.condition);

        // Assert that the condition is a boolean type.
        let boolean_type = Type::Boolean;
        self.assert_equal(boolean_type, condition);

        // Parse conditional statements.
        self.parse_block(&conditional.statements, span);

        // Parse conditional or end.
        match &conditional.next {
            Some(cond_or_end) => self.parse_conditional_nested_or_end(cond_or_end, span),
            None => {}
        }
    }

    ///
    /// Collects `TypeAssertion` predicates from a conditional statement.
    ///
    fn parse_conditional_nested_or_end(&mut self, cond_or_end: &ConditionalNestedOrEndStatement, span: &Span) {
        match cond_or_end {
            ConditionalNestedOrEndStatement::Nested(nested) => {
                self.parse_statement_conditional(nested, span);
            }
            ConditionalNestedOrEndStatement::End(statements) => self.parse_block(statements, span),
        }
    }

    ///
    /// Collects `TypeAssertion` predicates from an iteration statement.
    ///
    fn parse_iteration(
        &mut self,
        identifier: &Identifier,
        from: &Expression,
        to: &Expression,
        statements: &Vec<UnresolvedStatement>,
        span: &Span,
    ) {
        // Insert variable into symbol table with u32 type.
        let u32_type = Type::IntegerType(IntegerType::U32);
        self.insert_variable(identifier.name.to_owned(), u32_type.clone());

        // Parse `from` and `to` expressions.
        let from_type = self.parse_expression(from);
        let to_type = self.parse_expression(to);

        // Assert `from` and `to` types are a u32 or implicit.
        self.assert_equal(u32_type.clone(), from_type);
        self.assert_equal(u32_type, to_type);

        // Parse block of statements.
        self.parse_block(statements, span);
    }

    ///
    /// Asserts that the statement `UnresolvedExpression` returns an empty tuple.
    ///
    fn parse_statement_expression(&mut self, expression: &Expression, _span: &Span) {
        // Create empty tuple type.
        let expected_type = Type::Tuple(Vec::with_capacity(0));

        // Parse the actual type of the expression.
        let actual_type = self.parse_expression(expression);

        self.assert_equal(expected_type, actual_type);
    }

    ///
    /// Collects `TypeAssertion` predicates from a console statement.
    ///
    fn parse_console_function_call(&mut self, _console_function_call: &ConsoleFunctionCall) {
        // TODO (collinc97) find a way to fetch console function call types here
    }

    ///
    /// Returns the type of an expression.
    ///
    fn parse_expression(&mut self, expression: &Expression) -> Type {
        match expression {
            // Type variables
            Expression::Identifier(identifier) => self.parse_identifier(identifier),

            // Explicit types
            Expression::Boolean(_, _) => Type::Boolean,
            Expression::Address(_, _) => Type::Address,
            Expression::Field(_, _) => Type::Field,
            Expression::Group(_) => Type::Group,
            Expression::Implicit(name, _) => Self::parse_implicit(name),
            Expression::Integer(integer_type, _, _) => Type::IntegerType(integer_type.clone()),

            // Number operations
            Expression::Add(left, right, span) => self.parse_integer_binary_expression(left, right, span),
            Expression::Sub(left, right, span) => self.parse_integer_binary_expression(left, right, span),
            Expression::Mul(left, right, span) => self.parse_integer_binary_expression(left, right, span),
            Expression::Div(left, right, span) => self.parse_integer_binary_expression(left, right, span),
            Expression::Pow(left, right, span) => self.parse_integer_binary_expression(left, right, span),
            Expression::Negate(expression, _span) => self.parse_negate_expression(expression),

            // Boolean operations
            Expression::Not(expression, _span) => self.parse_boolean_expression(expression),
            Expression::Or(left, right, span) => self.parse_boolean_binary_expression(left, right, span),
            Expression::And(left, right, span) => self.parse_boolean_binary_expression(left, right, span),
            Expression::Eq(left, right, span) => self.parse_boolean_binary_expression(left, right, span),
            Expression::Ge(left, right, span) => self.parse_boolean_binary_expression(left, right, span),
            Expression::Gt(left, right, span) => self.parse_boolean_binary_expression(left, right, span),
            Expression::Le(left, right, span) => self.parse_boolean_binary_expression(left, right, span),
            Expression::Lt(left, right, span) => self.parse_boolean_binary_expression(left, right, span),

            // Conditionals
            Expression::IfElse(condition, first, second, span) => {
                self.parse_conditional_expression(condition, first, second, span)
            }

            // Arrays
            Expression::Array(expressions, span) => self.parse_array(expressions, span),
            Expression::ArrayAccess(array, access, span) => self.parse_array_access(array, access, span),

            // Tuples
            Expression::Tuple(expressions, span) => self.parse_tuple(expressions, span),
            Expression::TupleAccess(tuple, index, span) => self.parse_tuple_access(tuple, *index, span),

            // Circuits
            Expression::Circuit(identifier, members, span) => self.parse_circuit(identifier, members, span),
            Expression::CircuitMemberAccess(expression, identifier, span) => {
                self.parse_circuit_member_access(expression, identifier, span)
            }
            Expression::CircuitStaticFunctionAccess(expression, identifier, span) => {
                self.parse_circuit_static_function_access(expression, identifier, span)
            }

            // Functions
            Expression::FunctionCall(name, arguments, span) => self.parse_function_call(name, arguments, span),
            Expression::CoreFunctionCall(name, arguments, span) => self.parse_core_function_call(name, arguments, span),
        }
    }

    ///
    /// Returns the type of the identifier in the symbol table.
    ///
    fn parse_identifier(&self, identifier: &Identifier) -> Type {
        // TODO (collinc97) throw an error if identifier is not present.
        self.get_variable(&identifier.name).clone()
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

        // Create a type assertion left_type == right_type.
        self.assert_equal(left_type.clone(), right_type);

        left_type
    }

    ///
    /// Returns the `Type` of the expression after the binary operation.
    ///
    /// Asserts that the `Type` is an integer.
    ///
    fn parse_integer_binary_expression(&mut self, left: &Expression, right: &Expression, _span: &Span) -> Type {
        let type_ = self.parse_binary_expression(left, right, _span);

        // Assert that the type is an integer.
        self.assert_integer(&type_);

        type_
    }

    ///
    /// Returns the `Boolean` type if the expression is a `Boolean` type.
    ///
    fn parse_boolean_expression(&mut self, expression: &Expression) -> Type {
        // Return the `Boolean` type
        let boolean_type = Type::Boolean;

        // Get the type of the expression
        let expression_type = self.parse_expression(expression);

        // Assert that the type is a boolean.
        self.assert_equal(boolean_type.clone(), expression_type);

        boolean_type
    }

    ///
    /// Returns the `Type` of the expression being negated. Must be a negative integer type.
    ///
    fn parse_negate_expression(&mut self, expression: &Expression) -> Type {
        // Parse the expression type.
        let type_ = self.parse_expression(expression);

        // Assert that this integer can be negated.
        self.assert_negative_integer(&type_);

        type_
    }

    ///
    /// Returns the `Boolean` type if the binary expression is a `Boolean` type.
    ///
    fn parse_boolean_binary_expression(&mut self, left: &Expression, right: &Expression, _span: &Span) -> Type {
        // Return the `Boolean` type.
        let boolean_type = Type::Boolean;

        // Get the type of the binary expression.
        let binary_expression_type = self.parse_binary_expression(left, right, _span);

        // Create a type assertion boolean_type == expression_type.
        self.assert_equal(boolean_type.clone(), binary_expression_type);

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
    /// Returns the type of the tuple expression.
    ///
    fn parse_tuple(&mut self, expressions: &Vec<Expression>, _span: &Span) -> Type {
        let mut types = vec![];

        // Parse all tuple expressions.
        for expression in expressions {
            let type_ = self.parse_expression(expression);

            types.push(type_)
        }

        Type::Tuple(types)
    }

    ///
    /// Returns the type of the accessed tuple element.
    ///
    fn parse_tuple_access(&mut self, expression: &Expression, index: usize, _span: &Span) -> Type {
        // Parse the tuple expression which could be a variable with type tuple.
        let type_ = self.parse_expression(expression);

        // Check the type is a tuple.
        let elements = match type_ {
            Type::Tuple(elements) => elements,
            _ => unimplemented!("expected a tuple type"),
        };

        let element_type = elements[index].clone();

        element_type
    }

    ///
    /// Returns the type of the array expression.
    ///
    fn parse_array(&mut self, expressions: &Vec<Box<SpreadOrExpression>>, _span: &Span) -> Type {
        // Store array element type.
        let mut element_type = None;
        let mut count = 0usize;

        // Parse all array elements.
        for expression in expressions {
            // Get the type and count of elements in each spread or expression.
            let (type_, element_count) = self.parse_spread_or_expression(expression);

            // Assert that array element types are the same.
            if let Some(prev_type) = element_type {
                self.assert_equal(prev_type, type_.clone())
            }

            // Update array element type.
            element_type = Some(type_);

            // Update number of array elements.
            count += element_count;
        }

        // Return an error for empty arrays.
        let type_ = match element_type {
            Some(type_) => type_,
            None => unimplemented!("return empty array error"),
        };

        Type::Array(Box::new(type_), vec![count])
    }

    ///
    /// Returns the type and count of elements in a spread or expression.
    ///
    fn parse_spread_or_expression(&mut self, s_or_e: &SpreadOrExpression) -> (Type, usize) {
        match s_or_e {
            SpreadOrExpression::Spread(expression) => {
                // Parse the type of the spread array expression.
                let array_type = self.parse_expression(expression);

                // Check that the type is an array.
                let (element_type, mut dimensions) = match array_type {
                    Type::Array(element_type, dimensions) => (element_type, dimensions),
                    _ => unimplemented!("Spread type must be an array"),
                };

                // A spread copies the elements of an array.
                // If the array has elements of type array, we must return a new array type with proper dimensions.
                // If the array has elements of any other type, we can return the type and count directly.
                let count = dimensions.pop().unwrap();

                let type_ = if dimensions.is_empty() {
                    *element_type
                } else {
                    Type::Array(element_type, dimensions)
                };

                (type_, count)
            }
            SpreadOrExpression::Expression(expression) => (self.parse_expression(expression), 1),
        }
    }

    ///
    /// Returns the type of the accessed array element.
    ///
    fn parse_array_access(&mut self, expression: &Expression, r_or_e: &RangeOrExpression, _span: &Span) -> Type {
        // Parse the array expression which could be a variable with type array.
        let type_ = self.parse_expression(expression);

        // Check the type is an array.
        let (element_type, dimensions) = match type_ {
            Type::Array(type_, dimensions) => (type_, dimensions),
            _ => unimplemented!("expected an array type"),
        };

        // Get the length of the array.
        // let length = *dimensions.last().unwrap();

        // Evaluate the range as an array type or the expression as the element type.
        match r_or_e {
            RangeOrExpression::Range(from, to) => {
                if let Some(expression) = from {
                    // Parse the expression type.
                    let type_ = self.parse_expression(expression);

                    self.assert_index(&type_);
                }

                if let Some(expression) = to {
                    // Parse the expression type.
                    let type_ = self.parse_expression(expression);

                    self.assert_index(&type_);
                }
            }
            RangeOrExpression::Expression(expression) => {
                // Parse the expression type.
                let type_ = self.parse_expression(expression);

                // Assert the type is an index.
                self.assert_index(&type_);
            }
        }

        *element_type
    }

    ///
    /// Returns the type of inline circuit expression.
    ///
    fn parse_circuit(
        &mut self,
        identifier: &Identifier,
        members: &Vec<CircuitVariableDefinition>,
        _span: &Span,
    ) -> Type {
        // Check if identifier is Self circuit type.
        let circuit_type = if identifier.is_self() {
            self.self_type.clone()
        } else {
            // Get circuit type.
            self.user_defined_types
                .get_circuit(&identifier.name)
                .map(|circuit_type| circuit_type.clone())
        }
        .unwrap();

        // Check the length of the circuit members.
        if circuit_type.variables.len() != members.len() {
            unimplemented!("Number of circuit arguments invalid")
        }

        // Assert members are circuit type member types.
        for (expected_variable, actual_variable) in circuit_type.variables.iter().zip(members) {
            // Parse actual variable expression.
            let actual_type = self.parse_expression(&actual_variable.expression);

            // Assert expected variable type == actual variable type.
            self.assert_equal(expected_variable.type_.clone(), actual_type)
        }

        Type::Circuit(identifier.clone())
    }

    ///
    /// Returns the type of the accessed circuit member.
    ///
    fn parse_circuit_member_access(&mut self, expression: &Expression, identifier: &Identifier, _span: &Span) -> Type {
        // Parse circuit name.
        let type_ = self.parse_expression(expression);

        // Check that type is a circuit type.
        let circuit_type = match type_ {
            Type::Circuit(identifier) => {
                // Lookup circuit.
                self.user_defined_types.get_circuit(&identifier.name).unwrap()
            }
            _ => unimplemented!("expected circuit type to access member"),
        };

        // Look for member with matching name.
        circuit_type.member_type(&identifier).unwrap().clone()
    }

    ///
    /// Returns the type returned by calling the static circuit function.
    ///
    fn parse_circuit_static_function_access(
        &mut self,
        expression: &Expression,
        identifier: &Identifier,
        _span: &Span,
    ) -> Type {
        // Parse the circuit name.
        let type_ = self.parse_expression(expression);

        // Check that type is a circuit type.
        let circuit_type = match type_ {
            Type::Circuit(identifier) => {
                // Lookup circuit.
                self.user_defined_types.get_circuit(&identifier.name).unwrap()
            }
            _ => unimplemented!("expected circuit type to access static member"),
        };

        // Look for member with matching name.
        circuit_type.member_type(&identifier).unwrap().clone()
    }

    ///
    /// Returns the type returned by calling the function.
    ///
    fn parse_function_call(&mut self, expression: &Expression, inputs: &Vec<Expression>, _span: &Span) -> Type {
        // Parse the function name.
        let type_ = self.parse_expression(expression);

        // Check that type is a function type.
        let function_type = match type_ {
            Type::Function(identifier) => {
                // Lookup function.
                self.user_defined_types.get_function(&identifier.name).unwrap().clone()
            }
            _ => unimplemented!("expected function type for function call"),
        };

        // Check the length of arguments
        if function_type.inputs.len() != inputs.len() {
            unimplemented!("Number of function arguments invalid")
        }

        // Assert function inputs are correct types.
        for (expected_input, actual_input) in function_type.inputs.iter().zip(inputs) {
            // Parse actual input expression.
            let actual_type = self.parse_expression(actual_input);

            // Assert expected input type == actual input type.
            self.assert_equal(expected_input.type_().clone(), actual_type);
        }

        // Return the function output type.
        function_type.output.type_.clone()
    }

    ///
    /// Returns the type returned by calling the core function.
    ///
    fn parse_core_function_call(&mut self, _name: &String, _arguments: &Vec<Expression>, _span: &Span) -> Type {
        unimplemented!("type checks for core function calls not implemented")
    }

    ///
    /// Returns a new `Function` if all `TypeAssertions` can be solved successfully.
    ///
    fn solve(self) -> Result<(), FrameError> {
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
        // Function::new(self)

        Ok(())
    }
}

/// A structure for tracking the types of defined variables in a block of code.
#[derive(Clone)]
pub struct Scope {
    pub loop_variables: VariableTable,
    pub variables: VariableTable,
}

impl Scope {
    ///
    /// Returns a new `Scope` from an optional given `Scope`.
    ///
    /// The new scope will contain the variables of the optional given `Scope`.
    ///
    pub fn new(parent: Option<Scope>) -> Self {
        match parent {
            Some(scope) => scope.clone(),
            None => Self::empty(),
        }
    }

    ///
    /// Returns a new `Scope` with no variables.
    ///
    fn empty() -> Self {
        Self {
            loop_variables: VariableTable::new(),
            variables: VariableTable::new(),
        }
    }

    ///
    /// Inserts a variable name -> type mapping into the loop variable table.
    ///
    pub fn insert_loop_variable(&mut self, name: String, type_: Type) -> Option<Type> {
        self.loop_variables.insert(name, type_)
    }

    ///
    /// Inserts a variable name -> type mapping into the variable table.
    ///
    pub fn insert_variable(&mut self, name: String, type_: Type) -> Option<Type> {
        self.variables.insert(name, type_)
    }

    ///
    /// Returns a reference to the type corresponding to the loop variable name.
    ///
    pub fn get_loop_variable(&self, name: &String) -> Option<&Type> {
        self.loop_variables.get(name)
    }

    ///
    /// Returns a reference to the type corresponding to the variable name.
    ///
    /// Checks loop variables first, then non-loop variables.
    ///
    pub fn get_variable(&self, name: &String) -> Option<&Type> {
        match self.get_loop_variable(name) {
            Some(loop_variable_type) => Some(loop_variable_type),
            None => self.variables.get(name),
        }
    }

    ///
    /// Inserts a vector of function input types into the `Scope` variable table.
    ///
    pub fn parse_function_inputs(&mut self, function_inputs: &Vec<FunctionInputType>) {
        self.variables.parse_function_inputs(function_inputs)
    }
}

/// Mapping of variable names to types
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
    /// If the variable table did not have this key present, throw an undefined variable error
    /// using the given span.
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
pub enum TypeAssertion {
    Equality(TypeEquality),
    Membership(TypeMembership),
}

impl TypeAssertion {
    ///
    /// Returns a `TypeAssertion::Equality` predicate from given left and right `Types`s.
    ///
    pub fn new_equality(left: Type, right: Type) -> Self {
        Self::Equality(TypeEquality::new(left, right))
    }

    ///
    /// Returns a `TypeAssertion::Membership` predicate from given and set `Type`s.
    ///
    pub fn new_membership(given: Type, set: Vec<Type>) -> Self {
        Self::Membership(TypeMembership::new(given, set))
    }

    pub fn substitute(&mut self, _variable: &TypeVariable, _type: &Type) {
        unimplemented!("substitution not impl")
    }

    ///
    /// Returns true if the given type assertion outputs true.
    ///
    pub fn evaluate(&self) -> bool {
        match self {
            TypeAssertion::Equality(equality) => equality.evaluate(),
            TypeAssertion::Membership(membership) => membership.evaluate(),
        }
    }

    pub fn get_pair(&self) -> Option<(TypeVariable, Type)> {
        unimplemented!("pair resolution not impl")
    }
}

/// A predicate that evaluates to true if the given type is equal to a member in the set vector of types.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TypeMembership {
    given: Type,
    set: Vec<Type>,
}

impl TypeMembership {
    ///
    /// Returns a `TypeMembership` predicate from given and set `Type`s.
    ///
    pub fn new(given: Type, set: Vec<Type>) -> Self {
        Self { given, set }
    }

    ///
    /// Substitutes the given `TypeVariable` for each `Type` in the `TypeMembership`.
    ///
    pub fn substitute(&mut self, _variable: &TypeVariable, _type_: &Type) {
        // self.left.substitute(variable, type_);
        // self.right.substitute(variable, type_);
    }

    ///
    /// Returns true if the given type is equal to a member of the set.
    ///
    pub fn evaluate(&self) -> bool {
        self.set.contains(&self.given)
    }

    ///
    /// Returns the (type variable, type) pair from this assertion.
    ///
    pub fn get_pair(&self) -> Option<(TypeVariable, Type)> {
        // match (&self.left, &self.right) {
        //     (Type::TypeVariable(variable), type_) => Some((variable.clone(), type_.clone())),
        //     (type_, Type::TypeVariable(variable)) => Some((variable.clone(), type_.clone())),
        //     (_type1, _type2) => None, // No (type variable, type) pair can be returned from two types
        // }
        unimplemented!("pair resolution not impl")
    }
}

/// A predicate that evaluates equality between two `Type`s.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TypeEquality {
    left: Type,
    right: Type,
}

impl TypeEquality {
    ///
    /// Returns a `TypeEquality` predicate from given left and right `Types`s
    ///
    pub fn new(left: Type, right: Type) -> Self {
        Self { left, right }
    }

    ///
    /// Substitutes the given `TypeVariable` for each `Types` in the `TypeEquality`.
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
