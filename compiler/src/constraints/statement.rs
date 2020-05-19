//! Methods to enforce constraints on statements in a resolved Leo program.

use crate::{
    constraints::{new_scope_from_variable, ConstrainedProgram, ConstrainedValue},
    errors::StatementError,
    new_scope,
    types::{
        Assignee, ConditionalNestedOrEnd, ConditionalStatement, Expression, Identifier, Integer,
        RangeOrExpression, Statement, Type,
    },
    Variable,
};

use snarkos_models::{
    curves::{Field, Group, PrimeField},
    gadgets::{r1cs::ConstraintSystem, utilities::boolean::Boolean, utilities::uint32::UInt32},
};

impl<F: Field + PrimeField, G: Group, CS: ConstraintSystem<F>> ConstrainedProgram<F, G, CS> {
    fn resolve_assignee(&mut self, scope: String, assignee: Assignee<F, G>) -> String {
        match assignee {
            Assignee::Identifier(name) => new_scope_from_variable(scope, &name),
            Assignee::Array(array, _index) => self.resolve_assignee(scope, *array),
            Assignee::CircuitField(circuit_name, _member) => {
                self.resolve_assignee(scope, *circuit_name)
            }
        }
    }

    fn get_mutable_assignee(
        &mut self,
        name: String,
    ) -> Result<&mut ConstrainedValue<F, G>, StatementError> {
        // Check that assignee exists and is mutable
        Ok(match self.get_mut(&name) {
            Some(value) => match value {
                ConstrainedValue::Mutable(mutable_value) => mutable_value,
                _ => return Err(StatementError::ImmutableAssign(name)),
            },
            None => return Err(StatementError::UndefinedVariable(name)),
        })
    }

    fn mutate_array(
        &mut self,
        cs: &mut CS,
        file_scope: String,
        function_scope: String,
        name: String,
        range_or_expression: RangeOrExpression<F, G>,
        new_value: ConstrainedValue<F, G>,
    ) -> Result<(), StatementError> {
        // Resolve index so we know if we are assigning to a single value or a range of values
        match range_or_expression {
            RangeOrExpression::Expression(index) => {
                let index =
                    self.enforce_index(cs, file_scope.clone(), function_scope.clone(), index)?;

                // Modify the single value of the array in place
                match self.get_mutable_assignee(name)? {
                    ConstrainedValue::Array(old) => {
                        old[index] = new_value;
                    }
                    _ => return Err(StatementError::ArrayAssignIndex),
                }
            }
            RangeOrExpression::Range(from, to) => {
                let from_index = match from {
                    Some(integer) => integer.to_usize(),
                    None => 0usize,
                };
                let to_index_option = match to {
                    Some(integer) => Some(integer.to_usize()),
                    None => None,
                };

                // Modify the range of values of the array in place
                match (self.get_mutable_assignee(name)?, new_value) {
                    (ConstrainedValue::Array(old), ConstrainedValue::Array(ref new)) => {
                        let to_index = to_index_option.unwrap_or(old.len());
                        old.splice(from_index..to_index, new.iter().cloned());
                    }
                    _ => return Err(StatementError::ArrayAssignRange),
                }
            }
        }

        Ok(())
    }

    fn mutute_circuit_field(
        &mut self,
        circuit_name: String,
        object_name: Identifier<F, G>,
        new_value: ConstrainedValue<F, G>,
    ) -> Result<(), StatementError> {
        match self.get_mutable_assignee(circuit_name)? {
            ConstrainedValue::CircuitExpression(_variable, members) => {
                // Modify the circuit field in place
                let matched_field = members.into_iter().find(|object| object.0 == object_name);

                match matched_field {
                    Some(object) => match &object.1 {
                        ConstrainedValue::Function(_circuit_identifier, function) => {
                            return Err(StatementError::ImmutableCircuitFunction(
                                function.function_name.to_string(),
                            ))
                        }
                        ConstrainedValue::Static(_value) => {
                            return Err(StatementError::ImmutableCircuitFunction("static".into()))
                        }
                        _ => object.1 = new_value.to_owned(),
                    },
                    None => {
                        return Err(StatementError::UndefinedCircuitObject(
                            object_name.to_string(),
                        ))
                    }
                }
            }
            _ => return Err(StatementError::UndefinedCircuit(object_name.to_string())),
        }

        Ok(())
    }

    fn enforce_assign_statement(
        &mut self,
        cs: &mut CS,
        file_scope: String,
        function_scope: String,
        assignee: Assignee<F, G>,
        expression: Expression<F, G>,
    ) -> Result<(), StatementError> {
        // Get the name of the variable we are assigning to
        let variable_name = self.resolve_assignee(function_scope.clone(), assignee.clone());

        // Evaluate new value
        let new_value = self.enforce_expression(
            cs,
            file_scope.clone(),
            function_scope.clone(),
            &vec![],
            expression,
        )?;

        // Mutate the old value into the new value
        match assignee {
            Assignee::Identifier(_identifier) => {
                let old_value = self.get_mutable_assignee(variable_name.clone())?;

                *old_value = new_value;

                Ok(())
            }
            Assignee::Array(_assignee, range_or_expression) => self.mutate_array(
                cs,
                file_scope,
                function_scope,
                variable_name,
                range_or_expression,
                new_value,
            ),
            Assignee::CircuitField(_assignee, object_name) => {
                self.mutute_circuit_field(variable_name, object_name, new_value)
            }
        }
    }

    fn store_definition(
        &mut self,
        function_scope: String,
        variable: Variable<F, G>,
        mut value: ConstrainedValue<F, G>,
    ) -> Result<(), StatementError> {
        // Store with given mutability
        if variable.mutable {
            value = ConstrainedValue::Mutable(Box::new(value));
        }

        let variable_program_identifier = new_scope(function_scope, variable.identifier.name);

        self.store(variable_program_identifier, value);

        Ok(())
    }

    fn enforce_definition_statement(
        &mut self,
        cs: &mut CS,
        file_scope: String,
        function_scope: String,
        variable: Variable<F, G>,
        expression: Expression<F, G>,
    ) -> Result<(), StatementError> {
        let mut expected_types = vec![];
        if let Some(ref _type) = variable._type {
            expected_types.push(_type.clone());
        }
        let value = self.enforce_expression(
            cs,
            file_scope.clone(),
            function_scope.clone(),
            &expected_types,
            expression,
        )?;

        self.store_definition(function_scope, variable, value)
    }

    fn enforce_multiple_definition_statement(
        &mut self,
        cs: &mut CS,
        file_scope: String,
        function_scope: String,
        variables: Vec<Variable<F, G>>,
        function: Expression<F, G>,
    ) -> Result<(), StatementError> {
        let mut expected_types = vec![];
        for variable in variables.iter() {
            if let Some(ref _type) = variable._type {
                expected_types.push(_type.clone());
            }
        }

        // Expect return values from function
        let return_values = match self.enforce_expression(
            cs,
            file_scope.clone(),
            function_scope.clone(),
            &expected_types,
            function,
        )? {
            ConstrainedValue::Return(values) => values,
            value => unimplemented!(
                "multiple assignment only implemented for functions, got {}",
                value
            ),
        };

        if variables.len() != return_values.len() {
            return Err(StatementError::InvalidNumberOfDefinitions(
                variables.len(),
                return_values.len(),
            ));
        }

        for (variable, value) in variables.into_iter().zip(return_values.into_iter()) {
            self.store_definition(function_scope.clone(), variable, value)?;
        }
        Ok(())
    }

    fn enforce_return_statement(
        &mut self,
        cs: &mut CS,
        file_scope: String,
        function_scope: String,
        expressions: Vec<Expression<F, G>>,
        return_types: Vec<Type<F, G>>,
    ) -> Result<ConstrainedValue<F, G>, StatementError> {
        // Make sure we return the correct number of values
        if return_types.len() != expressions.len() {
            return Err(StatementError::InvalidNumberOfReturns(
                return_types.len(),
                expressions.len(),
            ));
        }

        let mut returns = vec![];
        for (expression, ty) in expressions.into_iter().zip(return_types.into_iter()) {
            let expected_types = vec![ty.clone()];
            let result = self.enforce_branch(
                cs,
                file_scope.clone(),
                function_scope.clone(),
                &expected_types,
                expression,
            )?;

            returns.push(result);
        }

        Ok(ConstrainedValue::Return(returns))
    }

    fn iterate_or_early_return(
        &mut self,
        cs: &mut CS,
        file_scope: String,
        function_scope: String,
        statements: Vec<Statement<F, G>>,
        return_types: Vec<Type<F, G>>,
    ) -> Result<Option<ConstrainedValue<F, G>>, StatementError> {
        let mut res = None;
        // Evaluate statements and possibly return early
        for statement in statements.iter() {
            if let Some(early_return) = self.enforce_statement(
                cs,
                file_scope.clone(),
                function_scope.clone(),
                statement.clone(),
                return_types.clone(),
            )? {
                res = Some(early_return);
                break;
            }
        }

        Ok(res)
    }

    fn enforce_conditional_statement(
        &mut self,
        cs: &mut CS,
        file_scope: String,
        function_scope: String,
        statement: ConditionalStatement<F, G>,
        return_types: Vec<Type<F, G>>,
    ) -> Result<Option<ConstrainedValue<F, G>>, StatementError> {
        let expected_types = vec![Type::Boolean];
        let condition = match self.enforce_expression(
            cs,
            file_scope.clone(),
            function_scope.clone(),
            &expected_types,
            statement.condition.clone(),
        )? {
            ConstrainedValue::Boolean(resolved) => resolved,
            value => return Err(StatementError::IfElseConditional(value.to_string())),
        };

        // use gadget impl
        if condition.eq(&Boolean::Constant(true)) {
            self.iterate_or_early_return(
                cs,
                file_scope,
                function_scope,
                statement.statements,
                return_types,
            )
        } else {
            match statement.next {
                Some(next) => match next {
                    ConditionalNestedOrEnd::Nested(nested) => self.enforce_conditional_statement(
                        cs,
                        file_scope,
                        function_scope,
                        *nested,
                        return_types,
                    ),
                    ConditionalNestedOrEnd::End(statements) => self.iterate_or_early_return(
                        cs,
                        file_scope,
                        function_scope,
                        statements,
                        return_types,
                    ),
                },
                None => Ok(None),
            }
        }
    }

    fn enforce_for_statement(
        &mut self,
        cs: &mut CS,
        file_scope: String,
        function_scope: String,
        index: Identifier<F, G>,
        start: Integer,
        stop: Integer,
        statements: Vec<Statement<F, G>>,
        return_types: Vec<Type<F, G>>,
    ) -> Result<Option<ConstrainedValue<F, G>>, StatementError> {
        let mut res = None;

        for i in start.to_usize()..stop.to_usize() {
            // Store index in current function scope.
            // For loop scope is not implemented.
            let index_name = new_scope_from_variable(function_scope.clone(), &index);
            self.store(
                index_name,
                ConstrainedValue::Integer(Integer::U32(UInt32::constant(i as u32))),
            );

            // Evaluate statements and possibly return early
            if let Some(early_return) = self.iterate_or_early_return(
                cs,
                file_scope.clone(),
                function_scope.clone(),
                statements.clone(),
                return_types.clone(),
            )? {
                res = Some(early_return);
                break;
            }
        }

        Ok(res)
    }

    fn enforce_assert_eq_statement(
        &mut self,
        cs: &mut CS,
        left: ConstrainedValue<F, G>,
        right: ConstrainedValue<F, G>,
    ) -> Result<(), StatementError> {
        Ok(match (left, right) {
            (ConstrainedValue::Boolean(bool_1), ConstrainedValue::Boolean(bool_2)) => {
                self.enforce_boolean_eq(cs, bool_1, bool_2)?
            }
            (ConstrainedValue::Integer(num_1), ConstrainedValue::Integer(num_2)) => {
                Self::enforce_integer_eq(cs, num_1, num_2)?
            }
            (ConstrainedValue::FieldElement(fe_1), ConstrainedValue::FieldElement(fe_2)) => {
                self.enforce_field_eq(cs, fe_1, fe_2)
            }
            (ConstrainedValue::Array(arr_1), ConstrainedValue::Array(arr_2)) => {
                for (left, right) in arr_1.into_iter().zip(arr_2.into_iter()) {
                    self.enforce_assert_eq_statement(cs, left, right)?;
                }
            }
            (val_1, val_2) => {
                unimplemented!(
                    "assert eq not supported for given types {} == {}",
                    val_1,
                    val_2
                )
                // return Err(StatementError::AssertEq(
                //     val_1.to_string(),
                //     val_2.to_string(),
                // ))
            }
        })
    }

    pub(crate) fn enforce_statement(
        &mut self,
        cs: &mut CS,
        file_scope: String,
        function_scope: String,
        statement: Statement<F, G>,
        return_types: Vec<Type<F, G>>,
    ) -> Result<Option<ConstrainedValue<F, G>>, StatementError> {
        let mut res = None;
        match statement {
            Statement::Return(expressions) => {
                res = Some(self.enforce_return_statement(
                    cs,
                    file_scope,
                    function_scope,
                    expressions,
                    return_types,
                )?);
            }
            Statement::Definition(variable, expression) => {
                self.enforce_definition_statement(
                    cs,
                    file_scope,
                    function_scope,
                    variable,
                    expression,
                )?;
            }
            Statement::Assign(variable, expression) => {
                self.enforce_assign_statement(
                    cs,
                    file_scope,
                    function_scope,
                    variable,
                    expression,
                )?;
            }
            Statement::MultipleAssign(variables, function) => {
                self.enforce_multiple_definition_statement(
                    cs,
                    file_scope,
                    function_scope,
                    variables,
                    function,
                )?;
            }
            Statement::Conditional(statement) => {
                if let Some(early_return) = self.enforce_conditional_statement(
                    cs,
                    file_scope,
                    function_scope,
                    statement,
                    return_types,
                )? {
                    res = Some(early_return)
                }
            }
            Statement::For(index, start, stop, statements) => {
                if let Some(early_return) = self.enforce_for_statement(
                    cs,
                    file_scope,
                    function_scope,
                    index,
                    start,
                    stop,
                    statements,
                    return_types,
                )? {
                    res = Some(early_return)
                }
            }
            Statement::AssertEq(left, right) => {
                let resolved_left = self.enforce_expression(
                    cs,
                    file_scope.clone(),
                    function_scope.clone(),
                    &vec![],
                    left,
                )?;
                let resolved_right = self.enforce_expression(
                    cs,
                    file_scope.clone(),
                    function_scope.clone(),
                    &vec![],
                    right,
                )?;

                self.enforce_assert_eq_statement(cs, resolved_left, resolved_right)?;
            }
            Statement::Expression(expression) => {
                match self.enforce_expression(
                    cs,
                    file_scope,
                    function_scope,
                    &vec![],
                    expression.clone(),
                )? {
                    ConstrainedValue::Return(values) => {
                        if !values.is_empty() {
                            return Err(StatementError::Unassigned(expression.to_string()));
                        }
                    }
                    _ => return Err(StatementError::Unassigned(expression.to_string())),
                }
            }
        };

        Ok(res)
    }
}
