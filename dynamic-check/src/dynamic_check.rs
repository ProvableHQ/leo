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

use crate::{DynamicCheckError, FrameError, ScopeError, TypeAssertionError, VariableTableError};
use leo_static_check::{
    flatten_array_type,
    Attribute,
    CircuitFunctionType,
    CircuitType,
    FunctionInputType,
    FunctionType,
    SymbolTable,
    Type,
    TypeVariable,
};
use leo_typed::{
    Assignee,
    AssigneeAccess,
    Circuit,
    CircuitMember,
    CircuitVariableDefinition,
    ConditionalNestedOrEndStatement,
    ConditionalStatement,
    ConsoleFunctionCall,
    Declare,
    Expression,
    Function,
    Identifier,
    Program,
    RangeOrExpression,
    Span,
    SpreadOrExpression,
    Statement,
    Variables,
};

use leo_typed::integer_type::IntegerType;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Performs a dynamic type inference check over a program.
pub struct DynamicCheck {
    table: SymbolTable,
    frames: Vec<Frame>,
}

pub struct VariableEnvironment {}

impl DynamicCheck {
    ///
    /// Creates a new `DynamicCheck` from a given program and symbol table.
    ///
    /// Evaluates all `TypeAssertion` predicates.
    ///
    pub fn run(program: &Program, symbol_table: SymbolTable) -> Result<(), DynamicCheckError> {
        let dynamic_check = Self::new(program, symbol_table)?;

        dynamic_check.solve()
    }

    ///
    /// Returns a new `DynamicCheck` from a given program and symbol table.
    ///
    pub fn new(program: &Program, symbol_table: SymbolTable) -> Result<Self, DynamicCheckError> {
        let mut dynamic_check = Self {
            table: symbol_table,
            frames: Vec::new(),
        };

        dynamic_check.parse_program(program)?;

        Ok(dynamic_check)
    }

    ///
    /// Collects a vector of `TypeAssertion` predicates from a program.
    ///
    fn parse_program(&mut self, program: &Program) -> Result<(), DynamicCheckError> {
        let circuits = program
            .circuits
            .iter()
            .map(|(_identifier, circuit)| circuit)
            .collect::<Vec<_>>();

        self.parse_circuits(circuits)?;

        let functions = program
            .functions
            .iter()
            .map(|(_identifier, function)| function)
            .collect::<Vec<_>>();

        self.parse_functions(functions)
    }

    ///
    /// Collects a vector of `TypeAssertion` predicates from a vector of functions.
    ///
    fn parse_functions(&mut self, functions: Vec<&Function>) -> Result<(), DynamicCheckError> {
        for function in functions {
            self.parse_function(function)?;
        }

        Ok(())
    }

    ///
    /// Collects a vector of `TypeAssertion` predicates from a function.
    ///
    fn parse_function(&mut self, function: &Function) -> Result<(), DynamicCheckError> {
        let frame = Frame::new_function(function.to_owned(), None, None, self.table.clone())?;

        self.frames.push(frame);

        Ok(())
    }

    ///
    /// Collects a vector of `Frames`s from a vector of circuit functions.
    ///
    fn parse_circuits(&mut self, circuits: Vec<&Circuit>) -> Result<(), DynamicCheckError> {
        for circuit in circuits {
            self.parse_circuit(circuit)?;
        }

        Ok(())
    }

    ///
    /// Collects a vector of `Frames`s from a circuit function.
    ///
    /// Each frame collects a vector of `TypeAssertion` predicates from each function.
    ///
    fn parse_circuit(&mut self, circuit: &Circuit) -> Result<(), DynamicCheckError> {
        let name = &circuit.circuit_name.name;

        // Get circuit type from circuit symbol table.
        let circuit_type = self.table.get_circuit(name).unwrap().clone();

        // Create a new function for each circuit member function.
        for circuit_member in &circuit.members {
            // ignore circuit member variables
            if let CircuitMember::CircuitFunction(_, function) = circuit_member {
                // Collect `TypeAssertion` predicates from the function.
                // Pass down circuit self type and circuit variable types to each function.
                let frame = Frame::new_circuit_function(
                    function.to_owned(),
                    circuit_type.clone(),
                    Scope::empty(),
                    self.table.clone(),
                )?;

                self.frames.push(frame)
            }
        }

        Ok(())
    }

    ///
    /// Returns the result of evaluating all `TypeAssertion` predicates.
    ///
    /// Will attempt to substitute a `Type` for all `TypeVariable`s.
    /// Returns a `LeoResolvedAst` if all `TypeAssertion` predicates are true.
    /// Returns ERROR if a `TypeAssertion` predicate is false or a solution does not exist.
    ///
    pub fn solve(self) -> Result<(), DynamicCheckError> {
        for frame in self.frames {
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
        let function_type = user_defined_types.get_function(name).unwrap().clone();

        // Create a new scope for the function variables.
        let mut scope = Scope::new(parent_scope);

        // Initialize function inputs as variables.
        scope.parse_function_inputs(&function_type.inputs)?;

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
        scope.parse_function_inputs(&circuit_function_type.function.inputs)?;

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
    fn insert_variable(&mut self, name: String, type_: Type) -> Option<Type> {
        // Modify the current scope.
        let scope = self.scopes.last_mut().unwrap();

        // Insert the variable name -> type.
        scope.variables.insert(name, type_)
    }

    ///
    /// Get a variable's type from the symbol table in the current scope.
    ///
    fn get_variable(&self, name: &String) -> Option<&Type> {
        // Lookup in the current scope.
        let scope = self.scopes.last().unwrap();

        // Get the variable by name.
        scope.get_variable(name)
    }

    ///
    /// Get a function's type from the user defined types in the current scope.
    ///
    fn get_function(&self, name: &String) -> Option<&FunctionType> {
        self.user_defined_types.get_function(name)
    }

    ///
    /// Get a circuit's type from the user defined types in the current scope.
    ///
    fn get_circuit(&self, name: &String) -> Option<&CircuitType> {
        self.user_defined_types.get_circuit(name)
    }

    ///
    /// Creates a new equality type assertion between the given types.
    ///
    fn assert_equal(&mut self, left: Type, right: Type, span: &Span) {
        let type_assertion = TypeAssertion::new_equality(left, right, span);

        println!("equality: {:?}", type_assertion);

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
            Statement::Iteration(identifier, from, to, statements, span) => {
                self.parse_iteration(identifier, from, to, statements, span)
            }
            Statement::Expression(expression, span) => self.parse_statement_expression(expression, span),
            Statement::Console(console_call) => self.parse_console_function_call(console_call),
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
            let expected_type = match self.self_type.clone() {
                Some(circuit_type) => {
                    Type::new_from_circuit(&self.user_defined_types, type_, circuit_type.identifier, span.clone())
                        .unwrap()
                }
                None => Type::new(&self.user_defined_types, type_, span.clone()).unwrap(),
            };

            // Assert that the expected type is equal to the actual type.
            self.assert_equal(expected_type.clone(), actual_type.clone(), span)
        }

        // Check for multiple defined variables.
        if variables.names.len() == 1 {
            // Insert variable into symbol table
            let variable = variables.names[0].clone();

            // TODO (collinc97) throw error for duplicate variable definitions.
            let _expect_none = self.insert_variable(variable.identifier.name, actual_type);
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
                let _expect_none = self.insert_variable(variable.identifier.name.clone(), type_);
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
        let mut type_ = self.get_variable(&assignee.identifier.name).unwrap().to_owned();

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
    fn parse_block(&mut self, statements: &Vec<Statement>, _span: &Span) -> Result<(), FrameError> {
        // Push new scope.
        let scope = Scope::new(self.scopes.last().map(|scope| scope.clone()));
        self.push_scope(scope);

        // Parse all statements.
        for statement in statements.iter() {
            self.parse_statement(statement)?;
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
        match &conditional.next {
            Some(cond_or_end) => self.parse_conditional_nested_or_end(cond_or_end, span)?,
            None => {}
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
        from: &Expression,
        to: &Expression,
        statements: &Vec<Statement>,
        span: &Span,
    ) -> Result<(), FrameError> {
        // Insert variable into symbol table with u32 type.
        let u32_type = Type::IntegerType(IntegerType::U32);
        let _expect_none = self.insert_variable(identifier.name.to_owned(), u32_type.clone());

        // Parse `from` and `to` expressions.
        let from_type = self.parse_expression(from)?;
        let to_type = self.parse_expression(to)?;

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
        let expected_type = Type::Tuple(Vec::with_capacity(0));

        // Parse the actual type of the expression.
        let actual_type = self.parse_expression(expression)?;

        self.assert_equal(expected_type, actual_type, span);

        Ok(())
    }

    ///
    /// Collects `TypeAssertion` predicates from a console statement.
    ///
    fn parse_console_function_call(&mut self, _console_function_call: &ConsoleFunctionCall) -> Result<(), FrameError> {
        // TODO (collinc97) find a way to fetch console function call types here
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
            Expression::Implicit(name, _) => Ok(Self::parse_implicit(name)),
            Expression::Integer(integer_type, _, _) => Ok(Type::IntegerType(integer_type.clone())),

            // Number operations
            Expression::Add(left, right, span) => self.parse_integer_binary_expression(left, right, span),
            Expression::Sub(left, right, span) => self.parse_integer_binary_expression(left, right, span),
            Expression::Mul(left, right, span) => self.parse_integer_binary_expression(left, right, span),
            Expression::Div(left, right, span) => self.parse_integer_binary_expression(left, right, span),
            Expression::Pow(left, right, span) => self.parse_integer_binary_expression(left, right, span),
            Expression::Negate(expression, span) => self.parse_negate_expression(expression, span),

            // Boolean operations
            Expression::Not(expression, span) => self.parse_boolean_expression(expression, span),
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
            Expression::ArrayAccess(array, access, span) => self.parse_expression_array_access(array, access, span),

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
        if identifier.is_self_type() {
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

        Ok(Self::parse_implicit(&identifier.name))
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
    fn parse_tuple(&mut self, expressions: &Vec<Expression>, _span: &Span) -> Result<Type, FrameError> {
        let mut types = vec![];

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
    fn parse_tuple_access(&mut self, type_: Type, index: usize, _span: &Span) -> Result<Type, FrameError> {
        // Check the type is a tuple.
        let elements = match type_ {
            Type::Tuple(elements) => elements,
            _ => unimplemented!("expected a tuple type"),
        };

        let element_type = elements[index].clone();

        Ok(element_type)
    }

    ///
    /// Returns the type of the array expression.
    ///
    fn parse_array(&mut self, expressions: &Vec<Box<SpreadOrExpression>>, span: &Span) -> Result<Type, FrameError> {
        // Store array element type.
        let mut element_type = None;
        let mut count = 0usize;

        // Parse all array elements.
        for expression in expressions {
            // Get the type and count of elements in each spread or expression.
            let (type_, element_count) = self.parse_spread_or_expression(expression)?;

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
            None => unimplemented!("return empty array error"),
        };

        Ok(Type::Array(Box::new(type_), vec![count]))
    }

    ///
    /// Returns the type and count of elements in a spread or expression.
    ///
    fn parse_spread_or_expression(&mut self, s_or_e: &SpreadOrExpression) -> Result<(Type, usize), FrameError> {
        Ok(match s_or_e {
            SpreadOrExpression::Spread(expression) => {
                // Parse the type of the spread array expression.
                let array_type = self.parse_expression(expression)?;

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
            _ => unimplemented!("expected an array type"),
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
        self.self_type
            .as_ref()
            .map(|circuit_type| circuit_type.clone())
            .ok_or_else(|| FrameError::circuit_self(span))
    }

    ///
    /// Returns the type of inline circuit expression.
    ///
    fn parse_circuit(
        &mut self,
        identifier: &Identifier,
        members: &Vec<CircuitVariableDefinition>,
        span: &Span,
    ) -> Result<Type, FrameError> {
        // Check if identifier is Self circuit type.
        let circuit_type = if identifier.is_self() {
            // Get the Self type of the frame.
            self.self_type_or_error(span)?
        } else {
            // Get circuit type.
            self.user_defined_types
                .get_circuit(&identifier.name)
                .map(|circuit_type| circuit_type.clone())
                .ok_or_else(|| FrameError::undefined_circuit(identifier))?
        };

        // Check the length of the circuit members.
        if circuit_type.variables.len() != members.len() {
            return Err(FrameError::num_variables(
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

        Ok(Type::Circuit(identifier.to_owned()))
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
    fn parse_circuit_name(&mut self, type_: Type, _span: &Span) -> Result<&CircuitType, FrameError> {
        // Check that type is a circuit type.
        Ok(match type_ {
            Type::Circuit(identifier) => {
                // Lookup circuit identifier.
                self.user_defined_types.get_circuit(&identifier.name).unwrap()
            }
            type_ => unimplemented!("expected circuit type {:?}", type_),
        })
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
            _ => unimplemented!("Invalid function name"),
        }
    }

    ///
    /// Returns a `FunctionType` given a function identifier.
    ///
    fn parse_program_function(&mut self, identifier: &Identifier, _span: &Span) -> Result<FunctionType, FrameError> {
        Ok(self
            .user_defined_types
            .get_function(&identifier.name)
            .unwrap()
            .to_owned())
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
        if circuit_function_type.attributes.contains(&Attribute::Static) {
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
        if !circuit_function_type.attributes.contains(&Attribute::Static) {
            return Err(FrameError::invalid_member_access(identifier));
        }

        Ok(circuit_function_type.function.to_owned())
    }

    ///
    /// Returns the type returned by calling the function.
    ///
    /// Does not attempt to evaluate the function call. We are just checking types at this step.
    ///
    fn parse_function_call(
        &mut self,
        expression: &Expression,
        inputs: &Vec<Expression>,
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
            // Parse actual input expression.
            let actual_type = self.parse_expression(actual_input)?;

            // Assert expected input type == actual input type.
            self.assert_equal(expected_input.type_().clone(), actual_type, span);
        }

        // Return the function output type.
        Ok(function_type.output.type_.clone())
    }

    ///
    /// Returns the type returned by calling the core function.
    ///
    fn parse_core_function_call(
        &mut self,
        _name: &String,
        _arguments: &Vec<Expression>,
        _span: &Span,
    ) -> Result<Type, FrameError> {
        unimplemented!("type checks for core function calls not implemented")
    }

    ///
    /// Returns a new `Function` if all `TypeAssertions` can be solved successfully.
    ///
    fn solve(self) -> Result<(), FrameError> {
        let mut unsolved = self.type_assertions.clone();

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

            println!("assertion: {:?}", type_assertion);

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
                        original.substitute(&pair.0, &pair.1)
                    }

                    for original in &mut unsolved_membership {
                        original.substitute(&pair.0, &pair.1)
                    }
                }
            }

            // Solve the `TypeAssertion`.
            //
            // If the `TypeAssertion` has a solution, then continue the loop.
            // If the `TypeAssertion` returns a `TypeVariablePair`, then substitute the `TypeVariable`
            // for it's paired `Type` in all `TypeAssertion`s.
            // if let Some(pair) = type_assertion.solve()? {
            //     // Substitute the `TypeVariable` for it's paired `Type` in all `TypeAssertion`s.
            //     for original in &mut unsolved {
            //         original.substitute(&pair.0, &pair.1)
            //     }
            //
            //     for original in &mut unsolved_membership {
            //         original.substitute(&pair.0, &pair.1)
            //     }
            // };
        }

        // Solve all type membership assertions.
        while !unsolved_membership.is_empty() {
            // Pop type assertion from list
            let type_assertion = unsolved_membership.pop().unwrap();

            // Solve the membership assertion.
            type_assertion.evaluate()?;
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
    pub fn parse_function_inputs(&mut self, function_inputs: &Vec<FunctionInputType>) -> Result<(), ScopeError> {
        self.variables
            .parse_function_inputs(function_inputs)
            .map_err(|err| ScopeError::VariableTableError(err))
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
    pub fn parse_function_inputs(
        &mut self,
        function_inputs: &Vec<FunctionInputType>,
    ) -> Result<(), VariableTableError> {
        for input in function_inputs {
            let input_name = input.identifier().name.clone();
            let input_type = input.type_().clone();

            // TODO (collinc97) throw an error for duplicate function input names.
            self.insert(input_name, input_type);
        }
        Ok(())
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
    pub fn new_equality(left: Type, right: Type, span: &Span) -> Self {
        Self::Equality(TypeEquality::new(left, right, span))
    }

    ///
    /// Returns a `TypeAssertion::Membership` predicate from given and set `Type`s.
    ///
    pub fn new_membership(given: Type, set: Vec<Type>, span: &Span) -> Self {
        Self::Membership(TypeMembership::new(given, set, span))
    }

    ///
    /// Returns one or more `TypeVariablePairs` generated by the given `TypeAssertion`.
    ///
    pub fn pairs(&self) -> Result<TypeVariablePairs, TypeAssertionError> {
        match self {
            TypeAssertion::Equality(equality) => equality.pairs(),
            TypeAssertion::Membership(_) => unimplemented!("Cannot generate pairs from type membership"),
        }
    }

    ///
    /// Substitutes the given type for self if self is equal to the type variable.
    ///
    pub fn substitute(&mut self, variable: &TypeVariable, type_: &Type) {
        match self {
            TypeAssertion::Equality(equality) => equality.substitute(variable, type_),
            TypeAssertion::Membership(membership) => membership.substitute(variable, type_),
        }
    }

    ///
    /// Checks if the `TypeAssertion` is satisfied.
    ///
    pub fn evaluate(&self) -> Result<(), TypeAssertionError> {
        match self {
            TypeAssertion::Equality(equality) => equality.evaluate(),
            TypeAssertion::Membership(membership) => membership.evaluate(),
        }
    }
}

/// A predicate that evaluates to true if the given type is equal to a member in the set vector of types.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TypeMembership {
    given: Type,
    set: Vec<Type>,
    span: Span,
}

impl TypeMembership {
    ///
    /// Returns a `TypeMembership` predicate from given and set `Type`s.
    ///
    pub fn new(given: Type, set: Vec<Type>, span: &Span) -> Self {
        Self {
            given,
            set,
            span: span.to_owned(),
        }
    }

    ///
    /// Substitutes the given `TypeVariable` for each `Type` in the `TypeMembership`.
    ///
    pub fn substitute(&mut self, variable: &TypeVariable, type_: &Type) {
        self.given.substitute(variable, type_)
    }

    ///
    /// Returns true if the given type is equal to a member of the set.
    ///
    pub fn evaluate(&self) -> Result<(), TypeAssertionError> {
        if self.set.contains(&self.given) {
            Ok(())
        } else {
            Err(TypeAssertionError::membership_failed(
                &self.given,
                &self.set,
                &self.span,
            ))
        }
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
        unimplemented!()
    }
}

/// A predicate that evaluates equality between two `Type`s.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TypeEquality {
    left: Type,
    right: Type,
    span: Span,
}

impl TypeEquality {
    ///
    /// Returns a `TypeEquality` predicate from given left and right `Types`s
    ///
    pub fn new(left: Type, right: Type, span: &Span) -> Self {
        Self {
            left,
            right,
            span: span.to_owned(),
        }
    }

    ///
    /// Substitutes the given `TypeVariable` for each `Types` in the `TypeEquality`.
    ///
    pub fn substitute(&mut self, variable: &TypeVariable, type_: &Type) {
        self.left.substitute(variable, type_);
        self.right.substitute(variable, type_);
    }

    ///
    /// Checks if the `self.left` == `self.right`.
    ///
    pub fn evaluate(&self) -> Result<(), TypeAssertionError> {
        if self.left.eq(&self.right) {
            Ok(())
        } else {
            Err(TypeAssertionError::equality_failed(&self.left, &self.right, &self.span))
        }
    }

    ///
    /// Returns the (type variable, type) pair from this assertion.
    ///
    pub fn pairs(&self) -> Result<TypeVariablePairs, TypeAssertionError> {
        TypeVariablePairs::new(&self.left, &self.right, &self.span)
    }
}

/// A type variable -> type pair.
pub struct TypeVariablePair(TypeVariable, Type);

/// A vector of `TypeVariablePair`s.
pub struct TypeVariablePairs(Vec<TypeVariablePair>);

impl Default for TypeVariablePairs {
    fn default() -> Self {
        Self(Vec::new())
    }
}

impl TypeVariablePairs {
    ///
    /// Returns a new `TypeVariablePairs` struct from the given left and right types.
    ///
    pub fn new(left: &Type, right: &Type, span: &Span) -> Result<Self, TypeAssertionError> {
        let mut pairs = Self::default();

        // Push all `TypeVariablePair`s.
        pairs.push_pairs(left, right, span)?;

        Ok(pairs)
    }

    ///
    /// Returns true if the self vector has no pairs.
    ///
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    ///
    /// Returns the self vector of pairs.
    ///
    pub fn get_pairs(&self) -> &Vec<TypeVariablePair> {
        &self.0
    }

    ///
    /// Pushes a new `TypeVariablePair` struct to self.
    ///
    pub fn push(&mut self, variable: &TypeVariable, type_: &Type) {
        // Create a new type variable -> type pair.
        let pair = TypeVariablePair(variable.clone(), type_.clone());

        // Push the pair to the self vector.
        self.0.push(pair);
    }

    ///
    /// Checks if the given left or right type contains a `TypeVariable`.
    /// If a `TypeVariable` is found, create a new `TypeVariablePair` between the given left
    /// and right type.
    ///
    pub fn push_pairs(&mut self, left: &Type, right: &Type, span: &Span) -> Result<(), TypeAssertionError> {
        match (left, right) {
            (Type::TypeVariable(variable), type_) => Ok(self.push(variable, type_)),
            (type_, Type::TypeVariable(variable)) => Ok(self.push(variable, type_)),
            (Type::Array(left_type, left_dimensions), Type::Array(right_type, right_dimensions)) => {
                self.push_pairs_array(left_type, left_dimensions, right_type, right_dimensions, span)
            }
            (Type::Tuple(left_types), Type::Tuple(right_types)) => self.push_pairs_tuple(left_types, right_types, span),
            (_, _) => Ok(()), // No `TypeVariable` found so we do not push any pairs.
        }
    }

    ///
    /// Checks if the given left or right array type contains a `TypeVariable`.
    /// If a `TypeVariable` is found, create a new `TypeVariablePair` between the given left
    /// and right type.
    ///
    fn push_pairs_array(
        &mut self,
        left_type: &Type,
        left_dimensions: &Vec<usize>,
        right_type: &Type,
        right_dimensions: &Vec<usize>,
        span: &Span,
    ) -> Result<(), TypeAssertionError> {
        // Flatten the array types to get the element types.
        let (left_type_flat, left_dimensions_flat) = flatten_array_type(left_type, left_dimensions.to_owned());
        let (right_type_flat, right_dimensions_flat) = flatten_array_type(right_type, right_dimensions.to_owned());

        // If the dimensions do not match, then throw an error.
        if left_dimensions_flat.ne(&right_dimensions_flat) {
            return Err(TypeAssertionError::array_dimensions(
                left_dimensions_flat,
                right_dimensions_flat,
                span,
            ));
        }

        // Compare the array element types.
        self.push_pairs(left_type_flat, right_type_flat, span);

        Ok(())
    }

    ///
    /// Checks if any given left or right tuple type contains a `TypeVariable`.
    /// If a `TypeVariable` is found, create a new `TypeVariablePair` between the given left
    /// and right type.
    ///
    fn push_pairs_tuple(
        &mut self,
        left_types: &Vec<Type>,
        right_types: &Vec<Type>,
        span: &Span,
    ) -> Result<(), TypeAssertionError> {
        // Iterate over each left == right pair of types.
        for (left, right) in left_types.iter().zip(right_types) {
            // Check for `TypeVariablePair`s.
            self.push_pairs(left, right, span)?;
        }

        Ok(())
    }
}
