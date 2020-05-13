//! Methods to enforce constraints on statements in a resolved Leo program.

use crate::{
    constraints::{new_scope_from_variable, ConstrainedProgram, ConstrainedValue},
    errors::StatementError,
    types::{
        Assignee, ConditionalNestedOrEnd, ConditionalStatement, Expression, Integer,
        RangeOrExpression, Statement, Type, Variable,
    },
};

use snarkos_models::{
    curves::{Group, Field, PrimeField},
    gadgets::{r1cs::ConstraintSystem, utilities::boolean::Boolean, utilities::uint32::UInt32},
};

impl<G: Group, F: Field + PrimeField, CS: ConstraintSystem<F>> ConstrainedProgram<G, F, CS> {
    fn resolve_assignee(&mut self, scope: String, assignee: Assignee<F, G>) -> String {
        match assignee {
            Assignee::Variable(name) => new_scope_from_variable(scope, &name),
            Assignee::Array(array, _index) => self.resolve_assignee(scope, *array),
            Assignee::StructMember(struct_variable, _member) => {
                self.resolve_assignee(scope, *struct_variable)
            }
        }
    }

    fn store_assignment(
        &mut self,
        cs: &mut CS,
        file_scope: String,
        function_scope: String,
        assignee: Assignee<F, G>,
        return_value: &mut ConstrainedValue<F, G>,
    ) -> Result<(), StatementError> {
        match assignee {
            Assignee::Variable(name) => {
                // Store the variable in the current scope
                let definition_name = new_scope_from_variable(function_scope.clone(), &name);

                self.store(definition_name, return_value.to_owned());
            }
            Assignee::Array(array, index_expression) => {
                // Check that array exists
                let expected_array_name = self.resolve_assignee(function_scope.clone(), *array);

                // Resolve index so we know if we are assigning to a single value or a range of values
                match index_expression {
                    RangeOrExpression::Expression(index) => {
                        let index = self.enforce_index(
                            cs,
                            file_scope.clone(),
                            function_scope.clone(),
                            index,
                        )?;

                        // Modify the single value of the array in place
                        match self.get_mut(&expected_array_name) {
                            Some(value) => match value {
                                ConstrainedValue::Array(old) => {
                                    old[index] = return_value.to_owned();
                                }
                                _ => return Err(StatementError::ArrayAssignIndex),
                            },
                            None => {
                                return Err(StatementError::UndefinedArray(expected_array_name))
                            }
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
                        match self.get_mut(&expected_array_name) {
                            Some(value) => match (value, return_value) {
                                (ConstrainedValue::Array(old), ConstrainedValue::Array(new)) => {
                                    let to_index = to_index_option.unwrap_or(old.len());
                                    old.splice(from_index..to_index, new.iter().cloned());
                                }
                                _ => return Err(StatementError::ArrayAssignRange),
                            },
                            None => {
                                return Err(StatementError::UndefinedArray(expected_array_name))
                            }
                        }
                    }
                }
            }
            Assignee::StructMember(struct_variable, struct_member) => {
                // Check that struct exists
                let expected_struct_name =
                    self.resolve_assignee(function_scope.clone(), *struct_variable);

                match self.get_mut(&expected_struct_name) {
                    Some(ConstrainedValue::StructExpression(_variable, members)) => {
                        // Modify the struct member in place
                        let matched_member =
                            members.into_iter().find(|member| member.0 == struct_member);
                        match matched_member {
                            Some(mut member) => member.1 = return_value.to_owned(),
                            None => {
                                return Err(StatementError::UndefinedStructField(
                                    struct_member.to_string(),
                                ))
                            }
                        }
                    }
                    _ => return Err(StatementError::UndefinedStruct(expected_struct_name)),
                }
            }
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
        // Check that assignee exists
        let name = self.resolve_assignee(function_scope.clone(), assignee.clone());

        match self.get(&name) {
            Some(_assignee) => {
                let result_value = &mut self.enforce_expression(
                    cs,
                    file_scope.clone(),
                    function_scope.clone(),
                    expression,
                )?;

                self.store_assignment(cs, file_scope, function_scope, assignee, result_value)
            }
            None => Err(StatementError::UndefinedVariable(assignee.to_string())),
        }
    }

    fn enforce_definition_statement(
        &mut self,
        cs: &mut CS,
        file_scope: String,
        function_scope: String,
        assignee: Assignee<F, G>,
        ty: Option<Type<F, G>>,
        expression: Expression<F, G>,
    ) -> Result<(), StatementError> {
        let result_value = &mut self.enforce_expression(
            cs,
            file_scope.clone(),
            function_scope.clone(),
            expression,
        )?;

        match ty {
            // Explicit type
            Some(ty) => {
                result_value.expect_type(&ty)?;
                self.store_assignment(cs, file_scope, function_scope, assignee, result_value)
            }
            // Implicit type
            None => self.store_assignment(cs, file_scope, function_scope, assignee, result_value),
        }
    }

    fn enforce_multiple_definition_statement(
        &mut self,
        cs: &mut CS,
        file_scope: String,
        function_scope: String,
        assignees: Vec<Assignee<F, G>>,
        function: Expression<F, G>,
    ) -> Result<(), StatementError> {
        // Expect return values from function
        let return_values = match self.enforce_expression(
            cs,
            file_scope.clone(),
            function_scope.clone(),
            function,
        )? {
            ConstrainedValue::Return(values) => values,
            value => unimplemented!(
                "multiple assignment only implemented for functions, got {}",
                value
            ),
        };

        assignees
            .into_iter()
            .zip(return_values.into_iter())
            .map(|(assignee, mut return_value)| {
                self.store_assignment(
                    cs,
                    file_scope.clone(),
                    function_scope.clone(),
                    assignee,
                    &mut return_value,
                )
            })
            .collect::<Result<Vec<_>, _>>()?;

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
            let result = self.enforce_expression(
                cs,
                file_scope.clone(),
                function_scope.clone(),
                expression,
            )?;
            result.expect_type(&ty)?;

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
        let condition = match self.enforce_expression(
            cs,
            file_scope.clone(),
            function_scope.clone(),
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
        index: Variable<F, G>,
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
            Statement::Definition(assignee, ty, expression) => {
                self.enforce_definition_statement(
                    cs,
                    file_scope,
                    function_scope,
                    assignee,
                    ty,
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
            Statement::MultipleAssign(assignees, function) => {
                self.enforce_multiple_definition_statement(
                    cs,
                    file_scope,
                    function_scope,
                    assignees,
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
                let resolved_left =
                    self.enforce_expression(cs, file_scope.clone(), function_scope.clone(), left)?;
                let resolved_right =
                    self.enforce_expression(cs, file_scope.clone(), function_scope.clone(), right)?;

                self.enforce_assert_eq_statement(cs, resolved_left, resolved_right)?;
            }
            Statement::Expression(expression) => {
                match self.enforce_expression(cs, file_scope, function_scope, expression.clone())? {
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
