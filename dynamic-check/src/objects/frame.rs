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

use crate::{FrameError, Scope, TypeAssertion};
use leo_symbol_table::{Attribute, CircuitFunctionType, CircuitType, FunctionType, SymbolTable, Type, TypeVariable};
use leo_typed::{
    Assignee,
    AssigneeAccess,
    CircuitVariableDefinition,
    ConditionalNestedOrEndStatement,
    ConditionalStatement,
    Declare,
    Expression,
    Function,
    Identifier,
    IntegerType,
    RangeOrExpression,
    Span,
    SpreadOrExpression,
    Statement,
    Variables,
};

/// A vector of `TypeAssertion` predicates created from a function body.
#[derive(Clone)]
pub struct Frame {
    pub function_type: FunctionType,
    pub self_type: Option<CircuitType>,
    pub scopes: Vec<Scope>,
    pub statements: Vec<Statement>,
    pub type_assertions: Vec<TypeAssertion>,
    pub user_defined_types: SymbolTable,
}

impl Frame {
    ///
    /// Collects a vector of `TypeAssertion` predicates from a function.
    ///
    pub fn new_function(
        function: Function,
        self_type: Option<CircuitType>,
        parent_scope: Option<Scope>,
        user_defined_types: SymbolTable,
    ) -> Result<Self, FrameError> {
        let name = &function.identifier.name;

        // Get function type from symbol table.
        let function_type = user_defined_types.get_function_type(name).unwrap().clone();

        // Create a new scope for the function variables.
        let mut scope = Scope::new(parent_scope);

        // Initialize function inputs as variables.
        scope.insert_function_inputs(&function_type.inputs)?;

        // Create new list of scopes for frame.
        let scopes = vec![scope];

        // Create new frame struct.
        // Update variables when encountering let/const variable definitions.
        let mut frame = Self {
            function_type,
            self_type,
            scopes,
            statements: function.statements,
            type_assertions: vec![],
            user_defined_types,
        };

        // Create type assertions for function statements
        frame.parse_statements()?;

        Ok(frame)
    }

    ///
    /// Collects vector of `TypeAssertion` predicates from a circuit function.
    ///
    pub fn new_circuit_function(
        function: Function,
        self_type: CircuitType,
        parent_scope: Scope,
        user_defined_types: SymbolTable,
    ) -> Result<Self, FrameError> {
        let identifier = &function.identifier;

        // Find function name in circuit members.
        let circuit_function_type = self_type.member_function_type(identifier).unwrap().to_owned();

        // Create a new scope for the function variables.
        let mut scope = Scope::new(Some(parent_scope));

        // Initialize function inputs as variables.
        scope.insert_function_inputs(&circuit_function_type.function.inputs)?;

        // Create new list of scopes for frame.
        let scopes = vec![scope];

        // Create new frame struct.
        // Update variables when encountering let/const variable definitions.
        let mut frame = Self {
            function_type: circuit_function_type.function,
            self_type: Some(self_type),
            scopes,
            statements: function.statements,
            type_assertions: Vec::new(),
            user_defined_types,
        };

        // Create type assertions for function statements
        frame.parse_statements()?;

        Ok(frame)
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
    fn insert_variable(&mut self, name: String, type_: Type, span: &Span) -> Result<(), FrameError> {
        // Modify the current scope.
        let scope = self.scopes.last_mut().unwrap();

        // Insert the variable name -> type.
        match scope.variables.insert(name.clone(), type_) {
            Some(_type) => Err(FrameError::duplicate_variable(&name, span)),
            None => Ok(()),
        }
    }

    ///
    /// Get a variable's type from the symbol table in the current scope.
    ///
    fn get_variable(&self, name: &str) -> Option<&Type> {
        // Lookup in the current scope.
        let scope = self.scopes.last().unwrap();

        // Get the variable by name.
        scope.get_variable(name)
    }

    ///
    /// Get a function's type from the user defined types in the current scope.
    ///
    fn get_function(&self, name: &str) -> Option<&FunctionType> {
        self.user_defined_types.get_function_type(name)
    }

    ///
    /// Get a circuit's type from the user defined types in the current scope.
    ///
    fn get_circuit(&self, name: &str) -> Option<&CircuitType> {
        self.user_defined_types.get_circuit_type(name)
    }

    ///
    /// Creates a new equality type assertion between the given types.
    ///
    fn assert_equal(&mut self, left: Type, right: Type, span: &Span) {
        let type_assertion = TypeAssertion::new_equality(left, right, span);

        self.type_assertions.push(type_assertion);
    }

    ///
    /// Creates a new membership type assertion between a given and set of types.
    ///
    fn assert_membership(&mut self, given: Type, set: Vec<Type>, span: &Span) {
        let type_assertion = TypeAssertion::new_membership(given, set, span);

        self.type_assertions.push(type_assertion);
    }

    ///
    /// Creates a new membership type assertion between a given and the set of negative integer types.
    ///
    fn assert_negative_integer(&mut self, given: &Type, span: &Span) {
        let negative_integer_types = Type::negative_integer_types();

        self.assert_membership(given.clone(), negative_integer_types, span)
    }

    ///
    /// Creates a new membership type assertion between a given and the set of all integer types.
    ///
    fn assert_integer(&mut self, given: &Type, span: &Span) {
        let integer_types = Type::integer_types();

        self.assert_membership(given.clone(), integer_types, span)
    }

    ///
    /// Creates a new membership type assertion between a given and the set of index types.
    ///
    fn assert_index(&mut self, given: &Type, span: &Span) {
        let index_types = Type::index_types();

        self.assert_membership(given.clone(), index_types, span)
    }

    ///
    /// Collects a vector of `TypeAssertion` predicates from a vector of statements.
    ///
    fn parse_statements(&mut self) -> Result<(), FrameError> {
        for statement in self.statements.clone() {
            self.parse_statement(&statement)?;
        }

        Ok(())
    }

    ///
    /// Collects a vector of `TypeAssertion` predicates from a statement.
    ///
    fn parse_statement(&mut self, statement: &Statement) -> Result<(), FrameError> {
        match statement {
            Statement::Return(expression, span) => self.parse_return(expression, span),
            Statement::Definition(declare, variables, expression, span) => {
                self.parse_definition(declare, variables, expression, span)
            }
            Statement::Assign(assignee, expression, span) => self.parse_assign(assignee, expression, span),
            Statement::Conditional(conditional, span) => self.parse_statement_conditional(conditional, span),
            Statement::Iteration(identifier, from_to, statements, span) => {
                self.parse_iteration(identifier, from_to, statements, span)
            }
            Statement::Expression(expression, span) => self.parse_statement_expression(expression, span),
            Statement::Console(_console_call) => Ok(()), // Console function calls do not generate type assertions.
        }
    }

    ///
    /// Collects `TypeAssertion` predicates from a return statement.
    ///
    fn parse_return(&mut self, expression: &Expression, span: &Span) -> Result<(), FrameError> {
        // Get the function output type.
        let output_type = &self.function_type.output.type_;

        // Create the left hand side of a type assertion.
        let left = output_type.clone();

        // Create the right hand side from the statement return expression.
        let right = self.parse_expression(expression)?;

        // Create a new type assertion for the statement return.
        self.assert_equal(left, right, span);

        Ok(())
    }

    ///
    /// Collects `Type Assertion` predicates from a definition statement.
    ///
    fn parse_definition(
        &mut self,
        _declare: &Declare,
        variables: &Variables,
        expression: &Expression,
        span: &Span,
    ) -> Result<(), FrameError> {
        // Parse the definition expression.
        let actual_type = self.parse_expression(expression)?;

        // Check if an explicit type is given.
        if let Some(type_) = variables.type_.clone() {
            // Convert the expected type into a dynamic check type.
            let expected_type = match self.self_type {
                Some(ref circuit_type) => Type::new_from_circuit(
                    &self.user_defined_types,
                    type_,
                    circuit_type.identifier.clone(),
                    span.clone(),
                )
                .unwrap(),
                None => Type::new(&self.user_defined_types, type_, span.clone()).unwrap(),
            };

            // Assert that the expected type is equal to the actual type.
            self.assert_equal(expected_type.clone(), actual_type.clone(), span)
        }

        // Check for multiple defined variables.
        if variables.names.len() == 1 {
            // Insert variable into symbol table
            let variable = variables.names[0].clone();

            self.insert_variable(variable.identifier.name, actual_type, span)?;
        } else {
            // Expect a tuple type.
            let types = match actual_type {
                Type::Tuple(types) => types,
                _ => return Err(FrameError::not_enough_values(span)),
            };

            // Check number of variables == number of types.
            if types.len() != variables.names.len() {
                return Err(FrameError::invalid_number_of_values(
                    types.len(),
                    variables.names.len(),
                    span,
                ));
            }

            // Insert variables into symbol table
            for (variable, type_) in variables.names.iter().zip(types) {
                self.insert_variable(variable.identifier.name.clone(), type_, span)?;
            }
        }

        Ok(())
    }

    ///
    /// Asserts that the assignee's type is equal to the `Expression` type.
    ///
    fn parse_assign(&mut self, assignee: &Assignee, expression: &Expression, span: &Span) -> Result<(), FrameError> {
        // Parse assignee type.
        let assignee_type = self.parse_assignee(assignee, span)?;

        // Parse expression type.
        let expression_type = self.parse_expression(expression)?;

        // Assert that the assignee_type == expression_type.
        self.assert_equal(assignee_type, expression_type, span);

        Ok(())
    }

    ///
    /// Returns the type of the assignee.
    ///
    fn parse_assignee(&mut self, assignee: &Assignee, span: &Span) -> Result<Type, FrameError> {
        // Get the type of the assignee variable.
        let mut type_ = if assignee.identifier.is_self() {
            // If the variable is the self keyword, then return the self.circuit_type
            let self_type = self.self_type_or_error(span)?;

            Type::Circuit(self_type.identifier)
        } else {
            // Otherwise, lookup the variable by name in the symbol table.
            self.get_variable(&assignee.identifier.name)
                .map(|type_| type_.to_owned())
                .ok_or_else(|| FrameError::undefined_variable(&assignee.identifier))?
        };

        // Iteratively evaluate assignee access types.
        for access in &assignee.accesses {
            let access_type = match access {
                AssigneeAccess::Array(r_or_e) => self.parse_array_access(type_, r_or_e, span),
                AssigneeAccess::Tuple(index) => self.parse_tuple_access(type_, *index, span),
                AssigneeAccess::Member(identifier) => self.parse_circuit_member_access(type_, identifier, span),
            }?;

            type_ = access_type;
        }

        Ok(type_)
    }

    ///
    /// Collects `TypeAssertion` predicates from a block of statements.
    ///
    fn parse_block(&mut self, statements: &[Statement], _span: &Span) -> Result<(), FrameError> {
        // Push new scope.
        let scope = Scope::new(self.scopes.last().cloned());
        self.push_scope(scope);

        // Parse all statements.
        for statement in statements {
            self.parse_statement(&statement)?;
        }

        // Pop out of scope.
        let _scope = self.pop_scope();

        Ok(())
    }

    ///
    /// Collects `TypeAssertion` predicates from a conditional statement.
    ///
    /// Creates a new scope for each code block in the conditional.
    ///
    fn parse_statement_conditional(
        &mut self,
        conditional: &ConditionalStatement,
        span: &Span,
    ) -> Result<(), FrameError> {
        // Parse the condition expression.
        let condition = self.parse_expression(&conditional.condition)?;

        // Assert that the condition is a boolean type.
        let boolean_type = Type::Boolean;
        self.assert_equal(boolean_type, condition, span);

        // Parse conditional statements.
        self.parse_block(&conditional.statements, span)?;

        // Parse conditional or end.
        if let Some(cond_or_end) = &conditional.next {
            self.parse_conditional_nested_or_end(cond_or_end, span)?;
        }

        Ok(())
    }

    ///
    /// Collects `TypeAssertion` predicates from a conditional statement.
    ///
    fn parse_conditional_nested_or_end(
        &mut self,
        cond_or_end: &ConditionalNestedOrEndStatement,
        span: &Span,
    ) -> Result<(), FrameError> {
        match cond_or_end {
            ConditionalNestedOrEndStatement::Nested(nested) => self.parse_statement_conditional(nested, span),
            ConditionalNestedOrEndStatement::End(statements) => self.parse_block(statements, span),
        }
    }

    ///
    /// Collects `TypeAssertion` predicates from an iteration statement.
    ///
    fn parse_iteration(
        &mut self,
        identifier: &Identifier,
        from_to: &(Expression, Expression),
        statements: &[Statement],
        span: &Span,
    ) -> Result<(), FrameError> {
        // Insert variable into symbol table with u32 type.
        let u32_type = Type::IntegerType(IntegerType::U32);
        let _expect_none = self.insert_variable(identifier.name.to_owned(), u32_type.clone(), span);

        // Parse `from` and `to` expressions.
        let from_type = self.parse_expression(&from_to.0)?;
        let to_type = self.parse_expression(&from_to.1)?;

        // Assert `from` and `to` types are a u32 or implicit.
        self.assert_equal(u32_type.clone(), from_type, span);
        self.assert_equal(u32_type, to_type, span);

        // Parse block of statements.
        self.parse_block(statements, span)
    }

    ///
    /// Asserts that the statement `UnresolvedExpression` returns an empty tuple.
    ///
    fn parse_statement_expression(&mut self, expression: &Expression, span: &Span) -> Result<(), FrameError> {
        // Create empty tuple type.
        let expected_type = Type::Tuple(Vec::new());

        // Parse the actual type of the expression.
        let actual_type = self.parse_expression(expression)?;

        self.assert_equal(expected_type, actual_type, span);

        Ok(())
    }

    ///
    /// Returns the type of an expression.
    ///
    fn parse_expression(&mut self, expression: &Expression) -> Result<Type, FrameError> {
        match expression {
            // Type variables
            Expression::Identifier(identifier) => self.parse_identifier(identifier),

            // Explicit types
            Expression::Boolean(_, _) => Ok(Type::Boolean),
            Expression::Address(_, _) => Ok(Type::Address),
            Expression::Field(_, _) => Ok(Type::Field),
            Expression::Group(_) => Ok(Type::Group),
            Expression::Implicit(name, span) => Ok(Self::parse_implicit(Identifier::new_with_span(name, span))),
            Expression::Integer(integer_type, _, _) => Ok(Type::IntegerType(integer_type.clone())),

            // Number operations
            Expression::Add(left_right, span) => {
                self.parse_integer_binary_expression(&left_right.0, &left_right.1, span)
            }
            Expression::Sub(left_right, span) => {
                self.parse_integer_binary_expression(&left_right.0, &left_right.1, span)
            }
            Expression::Mul(left_right, span) => {
                self.parse_integer_binary_expression(&left_right.0, &left_right.1, span)
            }
            Expression::Div(left_right, span) => {
                self.parse_integer_binary_expression(&left_right.0, &left_right.1, span)
            }
            Expression::Pow(left_right, span) => {
                self.parse_integer_binary_expression(&left_right.0, &left_right.1, span)
            }
            Expression::Negate(expression, span) => self.parse_negate_expression(expression, span),

            // Boolean operations
            Expression::Not(expression, span) => self.parse_boolean_expression(expression, span),
            Expression::Or(left_right, span) => {
                self.parse_boolean_binary_expression(&left_right.0, &left_right.1, span)
            }
            Expression::And(left_right, span) => {
                self.parse_boolean_binary_expression(&left_right.0, &left_right.1, span)
            }
            Expression::Eq(left_right, span) => {
                self.parse_boolean_binary_expression(&left_right.0, &left_right.1, span)
            }
            Expression::Ge(left_right, span) => {
                self.parse_boolean_binary_expression(&left_right.0, &left_right.1, span)
            }
            Expression::Gt(left_right, span) => {
                self.parse_boolean_binary_expression(&left_right.0, &left_right.1, span)
            }
            Expression::Le(left_right, span) => {
                self.parse_boolean_binary_expression(&left_right.0, &left_right.1, span)
            }
            Expression::Lt(left_right, span) => {
                self.parse_boolean_binary_expression(&left_right.0, &left_right.1, span)
            }

            // Conditionals
            Expression::IfElse(triplet, span) => {
                self.parse_conditional_expression(&triplet.0, &triplet.1, &triplet.2, span)
            }

            // Arrays
            Expression::Array(expressions, span) => self.parse_array(expressions, span),
            Expression::ArrayAccess(array_w_index, span) => {
                self.parse_expression_array_access(&array_w_index.0, &array_w_index.1, span)
            }

            // Tuples
            Expression::Tuple(expressions, span) => self.parse_tuple(expressions, span),
            Expression::TupleAccess(tuple, index, span) => self.parse_expression_tuple_access(tuple, *index, span),

            // Circuits
            Expression::Circuit(identifier, members, span) => self.parse_circuit(identifier, members, span),
            Expression::CircuitMemberAccess(expression, identifier, span) => {
                self.parse_expression_circuit_member_access(expression, identifier, span)
            }
            Expression::CircuitStaticFunctionAccess(expression, identifier, span) => {
                self.parse_static_circuit_function_access(expression, identifier, span)
            }

            // Functions
            Expression::FunctionCall(name, arguments, span) => self.parse_function_call(name, arguments, span),
            Expression::CoreFunctionCall(name, arguments, span) => self.parse_core_function_call(name, arguments, span),
        }
    }

    ///
    /// Returns the type of the identifier in the symbol table.
    ///
    fn parse_identifier(&self, identifier: &Identifier) -> Result<Type, FrameError> {
        // Check Self type.
        if identifier.is_self() {
            // Check for frame circuit self type.
            let circuit_type = self.self_type_or_error(&identifier.span)?;

            // Return new type with circuit identifier.
            return Ok(Type::Circuit(circuit_type.identifier));
        }

        // Check variable symbol table.
        if let Some(type_) = self.get_variable(&identifier.name) {
            return Ok(type_.to_owned());
        };

        // Check function symbol table.
        if let Some(function_type) = self.get_function(&identifier.name) {
            return Ok(Type::Function(function_type.identifier.to_owned()));
        };

        // Check circuit symbol table.
        if let Some(circuit_type) = self.get_circuit(&identifier.name) {
            return Ok(Type::Circuit(circuit_type.identifier.to_owned()));
        }

        Ok(Self::parse_implicit(identifier.to_owned()))
    }

    ///
    /// Returns a new type variable from a given identifier
    ///
    fn parse_implicit(identifier: Identifier) -> Type {
        Type::TypeVariable(TypeVariable::from(identifier))
    }

    ///
    /// Returns the type of a binary expression.
    ///
    fn parse_binary_expression(
        &mut self,
        left: &Expression,
        right: &Expression,
        span: &Span,
    ) -> Result<Type, FrameError> {
        // Get the left expression type.
        let left_type = self.parse_expression(left)?;

        // Get the right expression type.
        let right_type = self.parse_expression(right)?;

        // Create a type assertion left_type == right_type.
        self.assert_equal(left_type.clone(), right_type, span);

        Ok(left_type)
    }

    ///
    /// Returns the `Type` of the expression after the binary operation.
    ///
    /// Asserts that the `Type` is an integer.
    ///
    fn parse_integer_binary_expression(
        &mut self,
        left: &Expression,
        right: &Expression,
        span: &Span,
    ) -> Result<Type, FrameError> {
        let type_ = self.parse_binary_expression(left, right, span)?;

        // Assert that the type is an integer.
        self.assert_integer(&type_, span);

        Ok(type_)
    }

    ///
    /// Returns the `Boolean` type if the expression is a `Boolean` type.
    ///
    fn parse_boolean_expression(&mut self, expression: &Expression, span: &Span) -> Result<Type, FrameError> {
        // Return the `Boolean` type
        let boolean_type = Type::Boolean;

        // Get the type of the expression
        let expression_type = self.parse_expression(expression)?;

        // Assert that the type is a boolean.
        self.assert_equal(boolean_type.clone(), expression_type, span);

        Ok(boolean_type)
    }

    ///
    /// Returns the `Type` of the expression being negated. Must be a negative integer type.
    ///
    fn parse_negate_expression(&mut self, expression: &Expression, span: &Span) -> Result<Type, FrameError> {
        // Parse the expression type.
        let type_ = self.parse_expression(expression)?;

        // Assert that this integer can be negated.
        self.assert_negative_integer(&type_, span);

        Ok(type_)
    }

    ///
    /// Returns the `Boolean` type if the binary expression is a `Boolean` type.
    ///
    fn parse_boolean_binary_expression(
        &mut self,
        left: &Expression,
        right: &Expression,
        span: &Span,
    ) -> Result<Type, FrameError> {
        // Create the `Boolean` type.
        let boolean_type = Type::Boolean;

        // Create a new type assertion for the binary expression
        let _binary_expression_type = self.parse_binary_expression(left, right, span)?;

        // Return the `Boolean` type.
        Ok(boolean_type)
    }

    ///
    /// Returns the type of the conditional expression.
    ///
    fn parse_conditional_expression(
        &mut self,
        condition: &Expression,
        first: &Expression,
        second: &Expression,
        span: &Span,
    ) -> Result<Type, FrameError> {
        // Check that the type of the condition expression is a boolean.
        let _condition_type = self.parse_boolean_expression(condition, span)?;

        // Check that the types of the first and second expression are equal.
        self.parse_binary_expression(first, second, span)
    }

    ///
    /// Returns the type of the tuple expression.
    ///
    fn parse_tuple(&mut self, expressions: &[Expression], _span: &Span) -> Result<Type, FrameError> {
        let mut types = Vec::with_capacity(expressions.len());

        // Parse all tuple expressions.
        for expression in expressions {
            let type_ = self.parse_expression(expression)?;

            types.push(type_)
        }

        Ok(Type::Tuple(types))
    }

    ///
    /// Returns the type of the accessed tuple element when called as an expression.
    ///
    fn parse_expression_tuple_access(
        &mut self,
        expression: &Expression,
        index: usize,
        span: &Span,
    ) -> Result<Type, FrameError> {
        // Parse the tuple expression which could be a variable with type tuple.
        let type_ = self.parse_expression(expression)?;

        // Parse the tuple access.
        self.parse_tuple_access(type_, index, span)
    }

    ///
    /// Returns the type of the accessed tuple element.
    ///
    fn parse_tuple_access(&mut self, type_: Type, index: usize, span: &Span) -> Result<Type, FrameError> {
        // Check the type is a tuple.
        let mut elements = match type_ {
            Type::Tuple(elements) => elements,
            type_ => return Err(FrameError::tuple_access(&type_, span)),
        };

        let element_type = elements.swap_remove(index);

        Ok(element_type)
    }

    ///
    /// Returns the type of the array expression.
    ///
    fn parse_array(&mut self, expressions: &[SpreadOrExpression], span: &Span) -> Result<Type, FrameError> {
        // Store array element type.
        let mut element_type = None;
        let mut count = 0usize;

        // Parse all array elements.
        for expression in expressions {
            // Get the type and count of elements in each spread or expression.
            let (type_, element_count) = self.parse_spread_or_expression(expression, span)?;

            // Assert that array element types are the same.
            if let Some(prev_type) = element_type {
                self.assert_equal(prev_type, type_.clone(), span);
            }

            // Update array element type.
            element_type = Some(type_);

            // Update number of array elements.
            count += element_count;
        }

        // Return an error for empty arrays.
        let type_ = match element_type {
            Some(type_) => type_,
            None => return Err(FrameError::empty_array(span)),
        };

        Ok(Type::Array(Box::new(type_), vec![count]))
    }

    ///
    /// Returns the type and count of elements in a spread or expression.
    ///
    fn parse_spread_or_expression(
        &mut self,
        s_or_e: &SpreadOrExpression,
        span: &Span,
    ) -> Result<(Type, usize), FrameError> {
        Ok(match s_or_e {
            SpreadOrExpression::Spread(expression) => {
                // Parse the type of the spread array expression.
                let array_type = self.parse_expression(expression)?;

                // Check that the type is an array.
                let (element_type, mut dimensions) = match array_type {
                    Type::Array(element_type, dimensions) => (element_type, dimensions),
                    type_ => return Err(FrameError::invalid_spread(type_, span)),
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
            SpreadOrExpression::Expression(expression) => (self.parse_expression(expression)?, 1),
        })
    }

    ///
    /// Returns the type of the accessed array element when called as an expression.
    ///
    fn parse_expression_array_access(
        &mut self,
        expression: &Expression,
        r_or_e: &RangeOrExpression,
        span: &Span,
    ) -> Result<Type, FrameError> {
        // Parse the array expression which could be a variable with type array.
        let type_ = self.parse_expression(expression)?;

        // Parse the array access.
        self.parse_array_access(type_, r_or_e, span)
    }

    ///
    /// Returns the type of the accessed array element.
    ///
    fn parse_array_access(&mut self, type_: Type, r_or_e: &RangeOrExpression, span: &Span) -> Result<Type, FrameError> {
        // Check the type is an array.
        let (element_type, _dimensions) = match type_ {
            Type::Array(type_, dimensions) => (type_, dimensions),
            type_ => return Err(FrameError::array_access(&type_, span)),
        };

        // Get the length of the array.
        // let length = *dimensions.last().unwrap();

        // Evaluate the range as an array type or the expression as the element type.
        match r_or_e {
            RangeOrExpression::Range(from, to) => {
                if let Some(expression) = from {
                    // Parse the expression type.
                    let type_ = self.parse_expression(expression)?;

                    self.assert_index(&type_, span);
                }

                if let Some(expression) = to {
                    // Parse the expression type.
                    let type_ = self.parse_expression(expression)?;

                    self.assert_index(&type_, span);
                }
            }
            RangeOrExpression::Expression(expression) => {
                // Parse the expression type.
                let type_ = self.parse_expression(expression)?;

                // Assert the type is an index.
                self.assert_index(&type_, span);
            }
        }

        Ok(*element_type)
    }

    ///
    /// Returns the Self type of the frame or an error if it does not exist.
    ///
    fn self_type_or_error(&self, span: &Span) -> Result<CircuitType, FrameError> {
        self.self_type.clone().ok_or_else(|| FrameError::circuit_self(span))
    }

    ///
    /// Returns the type of inline circuit expression.
    ///
    fn parse_circuit(
        &mut self,
        identifier: &Identifier,
        members: &[CircuitVariableDefinition],
        span: &Span,
    ) -> Result<Type, FrameError> {
        // Check if identifier is Self circuit type.
        let circuit_type = if identifier.is_self() {
            // Get the Self type of the frame.
            self.self_type_or_error(span)?
        } else {
            // Get circuit type.
            self.user_defined_types
                .get_circuit_type(&identifier.name)
                .cloned()
                .ok_or_else(|| FrameError::undefined_circuit(identifier))?
        };

        // Check the length of the circuit members.
        if circuit_type.variables.len() != members.len() {
            return Err(FrameError::num_circuit_variables(
                circuit_type.variables.len(),
                members.len(),
                span,
            ));
        }

        // Assert members are circuit type member types.
        for (expected_variable, actual_variable) in circuit_type.variables.iter().zip(members) {
            // Parse actual variable expression.
            let actual_type = self.parse_expression(&actual_variable.expression)?;

            // Assert expected variable type == actual variable type.
            self.assert_equal(expected_variable.type_.clone(), actual_type, span)
        }

        Ok(Type::Circuit(circuit_type.identifier))
    }

    ///
    /// Returns the type of the accessed circuit member when called as an expression.
    ///
    fn parse_expression_circuit_member_access(
        &mut self,
        expression: &Expression,
        identifier: &Identifier,
        span: &Span,
    ) -> Result<Type, FrameError> {
        // Parse circuit name.
        let type_ = self.parse_expression(expression)?;

        // Parse the circuit member access.
        self.parse_circuit_member_access(type_, identifier, span)
    }

    ///
    /// Returns the type of the accessed circuit member.
    ///
    fn parse_circuit_member_access(
        &mut self,
        type_: Type,
        identifier: &Identifier,
        span: &Span,
    ) -> Result<Type, FrameError> {
        // Check that type is a circuit type.
        let circuit_type = self.parse_circuit_name(type_, span)?;

        // Look for member with matching name.
        Ok(circuit_type.member_type(&identifier)?)
    }

    ///
    /// Returns the type returned by calling the static circuit function.
    ///
    fn parse_static_circuit_function_access(
        &mut self,
        expression: &Expression,
        identifier: &Identifier,
        span: &Span,
    ) -> Result<Type, FrameError> {
        // Parse the circuit name.
        let type_ = self.parse_expression(expression)?;

        self.parse_circuit_member_access(type_, identifier, span)
    }

    ///
    /// Returns a `CircuitType` given a circuit expression.
    ///
    fn parse_circuit_name(&mut self, type_: Type, span: &Span) -> Result<&CircuitType, FrameError> {
        // Check that type is a circuit type.
        match type_ {
            Type::Circuit(identifier) => {
                // Lookup circuit identifier.
                self.user_defined_types
                    .get_circuit_type(&identifier.name)
                    .ok_or_else(|| FrameError::undefined_circuit(&identifier))
            }
            type_ => Err(FrameError::invalid_circuit(type_, span)),
        }
    }

    ///
    /// Returns a `FunctionType` given a function expression.
    ///
    fn parse_function_name(&mut self, expression: &Expression, span: &Span) -> Result<FunctionType, FrameError> {
        // Case 1: Call a function defined in the program file.
        // Case 2: Call a circuit function.
        // Case 3: Call a static circuit function.
        // Return an Error in any other case.
        match expression {
            Expression::Identifier(identifier) => self.parse_program_function(identifier, span),
            Expression::CircuitMemberAccess(expression, identifier, span) => {
                self.parse_circuit_function(expression, identifier, span)
            }
            Expression::CircuitStaticFunctionAccess(expression, identifier, span) => {
                self.parse_static_circuit_function(expression, identifier, span)
            }
            expression => Err(FrameError::invalid_function(expression, span)),
        }
    }

    ///
    /// Returns a `FunctionType` given a function identifier.
    ///
    fn parse_program_function(&mut self, identifier: &Identifier, _span: &Span) -> Result<FunctionType, FrameError> {
        self.user_defined_types
            .get_function_type(&identifier.name)
            .cloned()
            .ok_or_else(|| FrameError::undefined_function(identifier))
    }

    ///
    /// Returns a `CircuitFunctionType` given a circuit expression and function identifier.
    ///
    fn parse_circuit_function_type(
        &mut self,
        expression: &Expression,
        identifier: &Identifier,
        span: &Span,
    ) -> Result<&CircuitFunctionType, FrameError> {
        // Parse circuit name.
        let type_ = self.parse_expression(expression)?;

        // Get circuit type.
        let circuit_type = self.parse_circuit_name(type_, span)?;

        // Find circuit function by identifier.
        circuit_type
            .member_function_type(identifier)
            .ok_or_else(|| FrameError::undefined_circuit_function(identifier))
    }

    ///
    /// Returns a `FunctionType` given a circuit expression and non-static function identifier.
    ///
    fn parse_circuit_function(
        &mut self,
        expression: &Expression,
        identifier: &Identifier,
        span: &Span,
    ) -> Result<FunctionType, FrameError> {
        // Find circuit function type.
        let circuit_function_type = self.parse_circuit_function_type(expression, identifier, span)?;

        // Check that the function is non-static.
        if let Some(Attribute::Static) = circuit_function_type.attribute {
            return Err(FrameError::invalid_static_access(identifier));
        }

        // Return the function type.
        Ok(circuit_function_type.function.to_owned())
    }

    ///
    /// Returns a `FunctionType` given a circuit expression and static function identifier.
    ///
    fn parse_static_circuit_function(
        &mut self,
        expression: &Expression,
        identifier: &Identifier,
        span: &Span,
    ) -> Result<FunctionType, FrameError> {
        // Find circuit function type.
        let circuit_function_type = self.parse_circuit_function_type(expression, identifier, span)?;

        // Check that the function is static.
        if let Some(Attribute::Static) = circuit_function_type.attribute {
            Ok(circuit_function_type.function.to_owned())
        } else {
            Err(FrameError::invalid_member_access(identifier))
        }
    }

    ///
    /// Returns the type returned by calling the function.
    ///
    /// Does not attempt to evaluate the function call. We are just checking types at this step.
    ///
    fn parse_function_call(
        &mut self,
        expression: &Expression,
        inputs: &[Expression],
        span: &Span,
    ) -> Result<Type, FrameError> {
        // Parse the function name.
        let function_type = self.parse_function_name(expression, span)?;

        // Check the length of arguments
        if function_type.inputs.len() != inputs.len() {
            return Err(FrameError::num_inputs(function_type.inputs.len(), inputs.len(), span));
        }

        // Assert function inputs are correct types.
        for (expected_input, actual_input) in function_type.inputs.iter().zip(inputs) {
            // Parse expected input type.
            let expected_type = expected_input.type_();

            // Parse actual input type.
            let actual_type = self.parse_expression(actual_input)?;

            // Assert expected input type == actual input type.
            self.assert_equal(expected_type, actual_type, span);
        }

        // Return the function output type.
        Ok(function_type.output.type_.clone())
    }

    ///
    /// Returns the type returned by calling the core function.
    ///
    fn parse_core_function_call(
        &mut self,
        _name: &str,
        _arguments: &[Expression],
        _span: &Span,
    ) -> Result<Type, FrameError> {
        unimplemented!("type checks for core function calls not implemented")
    }

    ///
    /// Returns `Ok` if all `TypeAssertions` can be solved successfully.
    ///
    pub(crate) fn check(self) -> Result<(), FrameError> {
        let mut unsolved = self.type_assertions;

        // Solve all type equality assertions first.
        let mut unsolved_membership = Vec::new();

        while !unsolved.is_empty() {
            // Pop type assertion from list
            let type_assertion = unsolved.pop().unwrap();

            // If it is a membership assertion, then skip it for now.
            if let TypeAssertion::Membership(membership) = type_assertion {
                unsolved_membership.push(membership);

                continue;
            }

            // Collect `TypeVariablePairs` from the `TypeAssertion`.
            let pairs = type_assertion.pairs()?;

            // If no pairs are found, attempt to evaluate the `TypeAssertion`.
            if pairs.is_empty() {
                // Evaluate the `TypeAssertion`.
                type_assertion.evaluate()?
            } else {
                // Iterate over each `TypeVariable` -> `Type` pair.
                for pair in pairs.get_pairs() {
                    // Substitute the `TypeVariable` for it's paired `Type` in all `TypeAssertion`s.
                    for original in &mut unsolved {
                        original.substitute(pair.first(), pair.second())
                    }

                    for original in &mut unsolved_membership {
                        original.substitute(pair.first(), pair.second())
                    }
                }
            }
        }

        // Solve all type membership assertions.
        while !unsolved_membership.is_empty() {
            // Pop type assertion from list
            let type_assertion = unsolved_membership.pop().unwrap();

            // Solve the membership assertion.
            type_assertion.evaluate()?;
        }

        Ok(())
    }
}
