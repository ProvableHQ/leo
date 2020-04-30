//! Methods to enforce constraints on statements in a resolved aleo program.

use crate::constraints::{new_scope_from_variable, ResolvedProgram, ResolvedValue};
use crate::{
    Assignee, ConditionalNestedOrEnd, ConditionalStatement, Expression, Integer, RangeOrExpression,
    Statement, Type, Variable,
};

use snarkos_models::curves::{Field, PrimeField};
use snarkos_models::gadgets::{
    r1cs::ConstraintSystem, utilities::boolean::Boolean, utilities::uint32::UInt32,
};

impl<F: Field + PrimeField, CS: ConstraintSystem<F>> ResolvedProgram<F, CS> {
    fn resolve_assignee(&mut self, scope: String, assignee: Assignee<F>) -> String {
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
        assignee: Assignee<F>,
        return_value: &mut ResolvedValue<F>,
    ) {
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
                        );

                        // Modify the single value of the array in place
                        match self.get_mut(&expected_array_name) {
                            Some(value) => match value {
                                ResolvedValue::Array(old) => {
                                    old[index] = return_value.to_owned();
                                }
                                _ => {
                                    unimplemented!("Cannot assign single index to array of values ")
                                }
                            },
                            None => unimplemented!(
                                "tried to assign to unknown array {}",
                                expected_array_name
                            ),
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
                                (ResolvedValue::Array(old), ResolvedValue::Array(new)) => {
                                    let to_index = to_index_option.unwrap_or(old.len());
                                    old.splice(from_index..to_index, new.iter().cloned());
                                }
                                _ => unimplemented!(
                                    "Cannot assign a range of array values to single value"
                                ),
                            },
                            None => unimplemented!(
                                "tried to assign to unknown array {}",
                                expected_array_name
                            ),
                        }
                    }
                }
            }
            Assignee::StructMember(struct_variable, struct_member) => {
                // Check that struct exists
                let expected_struct_name =
                    self.resolve_assignee(function_scope.clone(), *struct_variable);

                match self.get_mut(&expected_struct_name) {
                    Some(value) => match value {
                        ResolvedValue::StructExpression(_variable, members) => {
                            // Modify the struct member in place
                            let matched_member =
                                members.into_iter().find(|member| member.0 == struct_member);
                            match matched_member {
                                Some(mut member) => member.1 = return_value.to_owned(),
                                None => unimplemented!(
                                    "struct member {} does not exist in {}",
                                    struct_member,
                                    expected_struct_name
                                ),
                            }
                        }
                        _ => unimplemented!(
                            "tried to assign to unknown struct {}",
                            expected_struct_name
                        ),
                    },
                    None => {
                        unimplemented!("tried to assign to unknown struct {}", expected_struct_name)
                    }
                }
            }
        }
    }

    fn enforce_assign_statement(
        &mut self,
        cs: &mut CS,
        file_scope: String,
        function_scope: String,
        assignee: Assignee<F>,
        expression: Expression<F>,
    ) {
        let result_value = &mut self.enforce_expression(
            cs,
            file_scope.clone(),
            function_scope.clone(),
            expression,
        );

        self.store_assignment(cs, file_scope, function_scope, assignee, result_value);
    }

    fn enforce_definition_statement(
        &mut self,
        cs: &mut CS,
        file_scope: String,
        function_scope: String,
        ty: Type<F>,
        assignee: Assignee<F>,
        expression: Expression<F>,
    ) {
        let result_value = &mut self.enforce_expression(
            cs,
            file_scope.clone(),
            function_scope.clone(),
            expression,
        );

        if result_value.match_type(&ty) {
            self.store_assignment(cs, file_scope, function_scope, assignee, result_value);
        } else {
            unimplemented!("incompatible types {} = {}", assignee, result_value)
        }
    }

    fn enforce_multiple_definition_statement(
        &mut self,
        cs: &mut CS,
        file_scope: String,
        function_scope: String,
        assignees: Vec<Assignee<F>>,
        function: Expression<F>,
    ) {
        // Expect return values from function
        let return_values =
            match self.enforce_expression(cs, file_scope.clone(), function_scope.clone(), function)
            {
                ResolvedValue::Return(values) => values,
                value => unimplemented!(
                    "multiple assignment only implemented for functions, got {}",
                    value
                ),
            };

        assignees
            .into_iter()
            .zip(return_values.into_iter())
            .for_each(|(assignee, mut return_value)| {
                self.store_assignment(
                    cs,
                    file_scope.clone(),
                    function_scope.clone(),
                    assignee,
                    &mut return_value,
                );
            });
    }

    fn enforce_return_statement(
        &mut self,
        cs: &mut CS,
        file_scope: String,
        function_scope: String,
        expressions: Vec<Expression<F>>,
        return_types: Vec<Type<F>>,
    ) -> ResolvedValue<F> {
        ResolvedValue::Return(
            expressions
                .into_iter()
                .zip(return_types.into_iter())
                .map(|(expression, ty)| {
                    let result = self.enforce_expression(
                        cs,
                        file_scope.clone(),
                        function_scope.clone(),
                        expression,
                    );
                    if !result.match_type(&ty) {
                        unimplemented!("expected return type {}, got {}", ty, result)
                    } else {
                        result
                    }
                })
                .collect::<Vec<ResolvedValue<F>>>(),
        )
    }

    fn iterate_or_early_return(
        &mut self,
        cs: &mut CS,
        file_scope: String,
        function_scope: String,
        statements: Vec<Statement<F>>,
        return_types: Vec<Type<F>>,
    ) -> Option<ResolvedValue<F>> {
        let mut res = None;
        // Evaluate statements and possibly return early
        for statement in statements.iter() {
            if let Some(early_return) = self.enforce_statement(
                cs,
                file_scope.clone(),
                function_scope.clone(),
                statement.clone(),
                return_types.clone(),
            ) {
                res = Some(early_return);
                break;
            }
        }

        res
    }

    fn enforce_conditional_statement(
        &mut self,
        cs: &mut CS,
        file_scope: String,
        function_scope: String,
        statement: ConditionalStatement<F>,
        return_types: Vec<Type<F>>,
    ) -> Option<ResolvedValue<F>> {
        let condition = match self.enforce_expression(
            cs,
            file_scope.clone(),
            function_scope.clone(),
            statement.condition.clone(),
        ) {
            ResolvedValue::Boolean(resolved) => resolved,
            value => unimplemented!("if else conditional must resolve to boolean, got {}", value),
        };

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
                None => None,
            }
        }
    }

    fn enforce_for_statement(
        &mut self,
        cs: &mut CS,
        file_scope: String,
        function_scope: String,
        index: Variable<F>,
        start: Integer,
        stop: Integer,
        statements: Vec<Statement<F>>,
        return_types: Vec<Type<F>>,
    ) -> Option<ResolvedValue<F>> {
        let mut res = None;

        for i in start.to_usize()..stop.to_usize() {
            // Store index in current function scope.
            // For loop scope is not implemented.
            let index_name = new_scope_from_variable(function_scope.clone(), &index);
            self.store(index_name, ResolvedValue::U32(UInt32::constant(i as u32)));

            // Evaluate statements and possibly return early
            if let Some(early_return) = self.iterate_or_early_return(
                cs,
                file_scope.clone(),
                function_scope.clone(),
                statements.clone(),
                return_types.clone(),
            ) {
                res = Some(early_return);
                break;
            }
        }

        res
    }

    fn enforce_assert_eq_statement(
        &mut self,
        cs: &mut CS,
        left: ResolvedValue<F>,
        right: ResolvedValue<F>,
    ) {
        match (left, right) {
            (ResolvedValue::Boolean(bool1), ResolvedValue::Boolean(bool2)) => {
                self.enforce_boolean_eq(cs, bool1, bool2)
            }
            (ResolvedValue::U32(num1), ResolvedValue::U32(num2)) => {
                Self::enforce_u32_eq(cs, num1, num2)
            }
            (ResolvedValue::FieldElement(fe1), ResolvedValue::FieldElement(fe2)) => {
                self.enforce_field_eq(cs, fe1, fe2)
            }
            (val1, val2) => unimplemented!("cannot enforce equality between {} == {}", val1, val2),
        }
    }

    pub(crate) fn enforce_statement(
        &mut self,
        cs: &mut CS,
        file_scope: String,
        function_scope: String,
        statement: Statement<F>,
        return_types: Vec<Type<F>>,
    ) -> Option<ResolvedValue<F>> {
        let mut res = None;
        match statement {
            Statement::Return(expressions) => {
                // TODO: add support for early termination
                res = Some(self.enforce_return_statement(
                    cs,
                    file_scope,
                    function_scope,
                    expressions,
                    return_types,
                ));
            }
            Statement::Definition(ty, assignee, expression) => {
                self.enforce_definition_statement(
                    cs,
                    file_scope,
                    function_scope,
                    ty,
                    assignee,
                    expression,
                );
            }
            Statement::Assign(variable, expression) => {
                self.enforce_assign_statement(cs, file_scope, function_scope, variable, expression);
            }
            Statement::MultipleAssign(assignees, function) => {
                self.enforce_multiple_definition_statement(
                    cs,
                    file_scope,
                    function_scope,
                    assignees,
                    function,
                );
            }
            Statement::Conditional(statement) => {
                if let Some(early_return) = self.enforce_conditional_statement(
                    cs,
                    file_scope,
                    function_scope,
                    statement,
                    return_types,
                ) {
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
                ) {
                    res = Some(early_return)
                }
            }
            Statement::AssertEq(left, right) => {
                let resolved_left =
                    self.enforce_expression(cs, file_scope.clone(), function_scope.clone(), left);
                let resolved_right =
                    self.enforce_expression(cs, file_scope.clone(), function_scope.clone(), right);

                self.enforce_assert_eq_statement(cs, resolved_left, resolved_right);
            }
        };

        res
    }
}
