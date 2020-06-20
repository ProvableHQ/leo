//! Methods to enforce constraints on statements in a resolved Leo program.

use crate::{
    constraints::{ConstrainedProgram, ConstrainedValue},
    errors::StatementError,
    new_scope,
    GroupType,
};
use leo_types::{
    Assignee,
    ConditionalNestedOrEndStatement,
    ConditionalStatement,
    Declare,
    Expression,
    Identifier,
    Integer,
    RangeOrExpression,
    Span,
    Statement,
    Type,
    Variable,
};

use snarkos_models::{
    curves::{Field, PrimeField},
    gadgets::{
        r1cs::ConstraintSystem,
        utilities::{boolean::Boolean, eq::ConditionalEqGadget, select::CondSelectGadget, uint::UInt32},
    },
};

impl<F: Field + PrimeField, G: GroupType<F>> ConstrainedProgram<F, G> {
    fn resolve_assignee(&mut self, scope: String, assignee: Assignee) -> String {
        match assignee {
            Assignee::Identifier(name) => new_scope(scope, name.to_string()),
            Assignee::Array(array, _index) => self.resolve_assignee(scope, *array),
            Assignee::CircuitField(circuit_name, _member) => self.resolve_assignee(scope, *circuit_name),
        }
    }

    fn get_mutable_assignee(
        &mut self,
        name: String,
        span: Span,
    ) -> Result<&mut ConstrainedValue<F, G>, StatementError> {
        // Check that assignee exists and is mutable
        Ok(match self.get_mut(&name) {
            Some(value) => match value {
                ConstrainedValue::Mutable(mutable_value) => mutable_value,
                _ => return Err(StatementError::immutable_assign(name, span)),
            },
            None => return Err(StatementError::undefined_variable(name, span)),
        })
    }

    fn mutate_array<CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        file_scope: String,
        function_scope: String,
        indicator: Option<Boolean>,
        name: String,
        range_or_expression: RangeOrExpression,
        mut new_value: ConstrainedValue<F, G>,
        span: Span,
    ) -> Result<(), StatementError> {
        let condition = indicator.unwrap_or(Boolean::Constant(true));

        // Resolve index so we know if we are assigning to a single value or a range of values
        match range_or_expression {
            RangeOrExpression::Expression(index) => {
                let index = self.enforce_index(cs, file_scope.clone(), function_scope.clone(), index, span.clone())?;

                // Modify the single value of the array in place
                match self.get_mutable_assignee(name, span.clone())? {
                    ConstrainedValue::Array(old) => {
                        new_value.resolve_type(&vec![old[index].to_type()])?;

                        let selected_value =
                            ConstrainedValue::conditionally_select(cs, &condition, &new_value, &old[index]).map_err(
                                |_| StatementError::select_fail(new_value.to_string(), old[index].to_string(), span),
                            )?;

                        old[index] = selected_value;
                    }
                    _ => return Err(StatementError::array_assign_index(span)),
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

                // Modify the range of values of the array
                let old_array = self.get_mutable_assignee(name, span.clone())?;
                let new_array = match (old_array.clone(), new_value) {
                    (ConstrainedValue::Array(mut mutable), ConstrainedValue::Array(new)) => {
                        let to_index = to_index_option.unwrap_or(mutable.len());

                        mutable.splice(from_index..to_index, new.iter().cloned());
                        ConstrainedValue::Array(mutable)
                    }
                    _ => return Err(StatementError::array_assign_range(span)),
                };
                let selected_array = ConstrainedValue::conditionally_select(cs, &condition, &new_array, old_array)
                    .map_err(|_| StatementError::select_fail(new_array.to_string(), old_array.to_string(), span))?;
                *old_array = selected_array;
            }
        }

        Ok(())
    }

    fn mutute_circuit_field<CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        indicator: Option<Boolean>,
        circuit_name: String,
        object_name: Identifier,
        mut new_value: ConstrainedValue<F, G>,
        span: Span,
    ) -> Result<(), StatementError> {
        let condition = indicator.unwrap_or(Boolean::Constant(true));

        match self.get_mutable_assignee(circuit_name, span.clone())? {
            ConstrainedValue::CircuitExpression(_variable, members) => {
                // Modify the circuit field in place
                let matched_field = members.into_iter().find(|object| object.0 == object_name);

                match matched_field {
                    Some(object) => match &object.1 {
                        ConstrainedValue::Function(_circuit_identifier, function) => {
                            return Err(StatementError::immutable_circuit_function(
                                function.function_name.to_string(),
                                span,
                            ));
                        }
                        ConstrainedValue::Static(_value) => {
                            return Err(StatementError::immutable_circuit_function("static".into(), span));
                        }
                        _ => {
                            new_value.resolve_type(&vec![object.1.to_type()])?;

                            let selected_value =
                                ConstrainedValue::conditionally_select(cs, &condition, &new_value, &object.1).map_err(
                                    |_| StatementError::select_fail(new_value.to_string(), object.1.to_string(), span),
                                )?;

                            object.1 = selected_value.to_owned();
                        }
                    },
                    None => return Err(StatementError::undefined_circuit_object(object_name.to_string(), span)),
                }
            }
            _ => return Err(StatementError::undefined_circuit(object_name.to_string(), span)),
        }

        Ok(())
    }

    fn enforce_assign_statement<CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        file_scope: String,
        function_scope: String,
        indicator: Option<Boolean>,
        assignee: Assignee,
        expression: Expression,
        span: Span,
    ) -> Result<(), StatementError> {
        // Get the name of the variable we are assigning to
        let variable_name = self.resolve_assignee(function_scope.clone(), assignee.clone());

        // Evaluate new value
        let mut new_value =
            self.enforce_expression(cs, file_scope.clone(), function_scope.clone(), &vec![], expression)?;

        // Mutate the old value into the new value
        match assignee {
            Assignee::Identifier(_identifier) => {
                let condition = indicator.unwrap_or(Boolean::Constant(true));
                let old_value = self.get_mutable_assignee(variable_name.clone(), span.clone())?;
                new_value.resolve_type(&vec![old_value.to_type()])?;
                let selected_value = ConstrainedValue::conditionally_select(cs, &condition, &new_value, old_value)
                    .map_err(|_| StatementError::select_fail(new_value.to_string(), old_value.to_string(), span))?;

                *old_value = selected_value;

                Ok(())
            }
            Assignee::Array(_assignee, range_or_expression) => self.mutate_array(
                cs,
                file_scope,
                function_scope,
                indicator,
                variable_name,
                range_or_expression,
                new_value,
                span,
            ),
            Assignee::CircuitField(_assignee, object_name) => {
                self.mutute_circuit_field(cs, indicator, variable_name, object_name, new_value, span)
            }
        }
    }

    fn store_definition(
        &mut self,
        function_scope: String,
        variable: Variable,
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

    fn enforce_definition_statement<CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        file_scope: String,
        function_scope: String,
        declare: Declare,
        variable: Variable,
        expression: Expression,
        span: Span,
    ) -> Result<(), StatementError> {
        let mut expected_types = vec![];
        if let Some(ref _type) = variable._type {
            expected_types.push(_type.clone());
        }
        let mut value = self.enforce_expression(
            cs,
            file_scope.clone(),
            function_scope.clone(),
            &expected_types,
            expression,
        )?;

        if let Declare::Let = declare {
            value.allocate_value(cs, span)?;
        }

        self.store_definition(function_scope, variable, value)
    }

    fn enforce_multiple_definition_statement<CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        file_scope: String,
        function_scope: String,
        variables: Vec<Variable>,
        function: Expression,
        span: Span,
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
            value => unimplemented!("multiple assignment only implemented for functions, got {}", value),
        };

        if variables.len() != return_values.len() {
            return Err(StatementError::invalid_number_of_definitions(
                variables.len(),
                return_values.len(),
                span,
            ));
        }

        for (variable, value) in variables.into_iter().zip(return_values.into_iter()) {
            self.store_definition(function_scope.clone(), variable, value)?;
        }
        Ok(())
    }

    fn enforce_return_statement<CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        file_scope: String,
        function_scope: String,
        expressions: Vec<Expression>,
        return_types: Vec<Type>,
        span: Span,
    ) -> Result<ConstrainedValue<F, G>, StatementError> {
        // Make sure we return the correct number of values
        if return_types.len() != expressions.len() {
            return Err(StatementError::invalid_number_of_returns(
                return_types.len(),
                expressions.len(),
                span,
            ));
        }

        let mut returns = vec![];
        for (expression, ty) in expressions.into_iter().zip(return_types.into_iter()) {
            let expected_types = vec![ty.clone()];
            let result = self.enforce_expression_value(
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

    fn evaluate_branch<CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        file_scope: String,
        function_scope: String,
        indicator: Option<Boolean>,
        statements: Vec<Statement>,
        return_types: Vec<Type>,
    ) -> Result<Option<ConstrainedValue<F, G>>, StatementError> {
        let mut res = None;
        // Evaluate statements and possibly return early
        for statement in statements.iter() {
            if let Some(early_return) = self.enforce_statement(
                cs,
                file_scope.clone(),
                function_scope.clone(),
                indicator.clone(),
                statement.clone(),
                return_types.clone(),
            )? {
                res = Some(early_return);
                break;
            }
        }

        Ok(res)
    }

    /// Enforces a statements.conditional statement with one or more branches.
    /// Due to R1CS constraints, we must evaluate every branch to properly construct the circuit.
    /// At program execution, we will pass an `indicator bit` down to all child statements within each branch.
    /// The `indicator bit` will select that branch while keeping the constraint system satisfied.
    fn enforce_conditional_statement<CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        file_scope: String,
        function_scope: String,
        indicator: Option<Boolean>,
        statement: ConditionalStatement,
        return_types: Vec<Type>,
        span: Span,
    ) -> Result<Option<ConstrainedValue<F, G>>, StatementError> {
        let statement_string = statement.to_string();
        let outer_indicator = indicator.unwrap_or(Boolean::Constant(true));

        let expected_types = vec![Type::Boolean];
        let inner_indicator = match self.enforce_expression(
            cs,
            file_scope.clone(),
            function_scope.clone(),
            &expected_types,
            statement.condition.clone(),
        )? {
            ConstrainedValue::Boolean(resolved) => resolved,
            value => return Err(StatementError::conditional_boolean(value.to_string(), span)),
        };

        // Determine nested branch selection
        let branch_1_indicator = Boolean::and(
            &mut cs.ns(|| format!("statement branch 1 indicator {}", statement_string)),
            &outer_indicator,
            &inner_indicator,
        )?;

        // Execute branch 1
        self.evaluate_branch(
            cs,
            file_scope.clone(),
            function_scope.clone(),
            Some(branch_1_indicator),
            statement.statements,
            return_types.clone(),
        )?;

        // Execute branch 2
        let branch_2_indicator = Boolean::and(
            &mut cs.ns(|| format!("statement branch 2 indicator {}", statement_string)),
            &outer_indicator,
            &inner_indicator.not(),
        )?;

        match statement.next {
            Some(next) => match next {
                ConditionalNestedOrEndStatement::Nested(nested) => self.enforce_conditional_statement(
                    cs,
                    file_scope,
                    function_scope,
                    Some(branch_2_indicator),
                    *nested,
                    return_types,
                    span,
                ),
                ConditionalNestedOrEndStatement::End(statements) => self.evaluate_branch(
                    cs,
                    file_scope,
                    function_scope,
                    Some(branch_2_indicator),
                    statements,
                    return_types,
                ),
            },
            None => Ok(None), // this is an if with no else, have to pass statements.conditional down to next statements somehow
        }
    }

    fn enforce_for_statement<CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        file_scope: String,
        function_scope: String,
        indicator: Option<Boolean>,
        index: Identifier,
        start: Integer,
        stop: Integer,
        statements: Vec<Statement>,
        return_types: Vec<Type>,
        _span: Span,
    ) -> Result<Option<ConstrainedValue<F, G>>, StatementError> {
        let mut res = None;

        for i in start.to_usize()..stop.to_usize() {
            // Store index in current function scope.
            // For loop scope is not implemented.
            let index_name = new_scope(function_scope.clone(), index.to_string());
            self.store(
                index_name,
                ConstrainedValue::Integer(Integer::U32(UInt32::constant(i as u32))),
            );

            cs.ns(|| format!("loop {} = {}", index.to_string(), i));

            // Evaluate statements and possibly return early
            if let Some(early_return) = self.evaluate_branch(
                cs,
                file_scope.clone(),
                function_scope.clone(),
                indicator,
                statements.clone(),
                return_types.clone(),
            )? {
                res = Some(early_return);
                break;
            }
        }

        Ok(res)
    }

    fn enforce_assert_eq_statement<CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        indicator: Option<Boolean>,
        left: &ConstrainedValue<F, G>,
        right: &ConstrainedValue<F, G>,
        span: Span,
    ) -> Result<(), StatementError> {
        let condition = indicator.unwrap_or(Boolean::Constant(true));
        let result = left.conditional_enforce_equal(cs, right, &condition);

        Ok(result.map_err(|_| StatementError::assertion_failed(left.to_string(), right.to_string(), span))?)
    }

    pub(crate) fn enforce_statement<CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        file_scope: String,
        function_scope: String,
        indicator: Option<Boolean>,
        statement: Statement,
        return_types: Vec<Type>,
    ) -> Result<Option<ConstrainedValue<F, G>>, StatementError> {
        let mut res = None;
        match statement {
            Statement::Return(expressions, span) => {
                res = Some(self.enforce_return_statement(
                    cs,
                    file_scope,
                    function_scope,
                    expressions,
                    return_types,
                    span,
                )?);
            }
            Statement::Definition(declare, variable, expression, span) => {
                self.enforce_definition_statement(cs, file_scope, function_scope, declare, variable, expression, span)?;
            }
            Statement::Assign(variable, expression, span) => {
                self.enforce_assign_statement(cs, file_scope, function_scope, indicator, variable, expression, span)?;
            }
            Statement::MultipleAssign(variables, function, span) => {
                self.enforce_multiple_definition_statement(cs, file_scope, function_scope, variables, function, span)?;
            }
            Statement::Conditional(statement, span) => {
                if let Some(early_return) = self.enforce_conditional_statement(
                    cs,
                    file_scope,
                    function_scope,
                    indicator,
                    statement,
                    return_types,
                    span,
                )? {
                    res = Some(early_return)
                }
            }
            Statement::For(index, start, stop, statements, span) => {
                if let Some(early_return) = self.enforce_for_statement(
                    cs,
                    file_scope,
                    function_scope,
                    indicator,
                    index,
                    start,
                    stop,
                    statements,
                    return_types,
                    span,
                )? {
                    res = Some(early_return)
                }
            }
            Statement::AssertEq(left, right, span) => {
                let (resolved_left, resolved_right) =
                    self.enforce_binary_expression(cs, file_scope, function_scope, &vec![], left, right)?;

                self.enforce_assert_eq_statement(cs, indicator, &resolved_left, &resolved_right, span)?;
            }
            Statement::Expression(expression, span) => {
                match self.enforce_expression(cs, file_scope, function_scope, &vec![], expression.clone())? {
                    ConstrainedValue::Return(values) => {
                        if !values.is_empty() {
                            return Err(StatementError::unassigned(expression.to_string(), span));
                        }
                    }
                    _ => return Err(StatementError::unassigned(expression.to_string(), span)),
                }
            }
        };

        Ok(res)
    }
}
