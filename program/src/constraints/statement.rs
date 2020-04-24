//! Methods to enforce constraints on statements in a resolved aleo program.

use crate::constraints::{new_scope_from_variable, ResolvedProgram, ResolvedValue};
use crate::{Assignee, Expression, Integer, RangeOrExpression, Statement, Type, Variable};

use snarkos_models::curves::{Field, PrimeField};
use snarkos_models::gadgets::{r1cs::ConstraintSystem, utilities::uint32::UInt32};

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

    fn enforce_definition(
        &mut self,
        cs: &mut CS,
        scope: String,
        assignee: Assignee<F>,
        return_value: &mut ResolvedValue<F>,
    ) {
        match assignee {
            Assignee::Variable(name) => {
                // Store the variable in the current scope
                let definition_name = new_scope_from_variable(scope.clone(), &name);

                self.store(definition_name, return_value.to_owned());
            }
            Assignee::Array(array, index_expression) => {
                // Check that array exists
                let expected_array_name = self.resolve_assignee(scope.clone(), *array);

                // Resolve index so we know if we are assigning to a single value or a range of values
                match index_expression {
                    RangeOrExpression::Expression(index) => {
                        let index = self.enforce_index(cs, scope.clone(), index);

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
                let expected_struct_name = self.resolve_assignee(scope.clone(), *struct_variable);

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

    pub(crate) fn enforce_definition_statement(
        &mut self,
        cs: &mut CS,
        scope: String,
        assignee: Assignee<F>,
        expression: Expression<F>,
    ) {
        let result_value = &mut self.enforce_expression(cs, scope.clone(), expression);

        self.enforce_definition(cs, scope, assignee, result_value);
    }

    pub(crate) fn enforce_multiple_definition_statement(
        &mut self,
        cs: &mut CS,
        scope: String,
        assignees: Vec<Assignee<F>>,
        function: Expression<F>,
    ) {
        // Expect return values from function
        let return_values = match self.enforce_expression(cs, scope.clone(), function) {
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
                self.enforce_definition(cs, scope.clone(), assignee, &mut return_value);
            });
    }

    pub(crate) fn enforce_return_statement(
        &mut self,
        cs: &mut CS,
        scope: String,
        statements: Vec<Expression<F>>,
        return_types: Vec<Type<F>>,
    ) -> ResolvedValue<F> {
        ResolvedValue::Return(
            statements
                .into_iter()
                .zip(return_types.into_iter())
                .map(|(expression, ty)| {
                    let result = self.enforce_expression(cs, scope.clone(), expression);
                    if !result.match_type(&ty) {
                        unimplemented!("expected return type {}, got {}", ty, result)
                    } else {
                        result
                    }
                })
                .collect::<Vec<ResolvedValue<F>>>(),
        )
    }

    fn enforce_statement(
        &mut self,
        cs: &mut CS,
        scope: String,
        statement: Statement<F>,
        return_types: Vec<Type<F>>,
    ) {
        match statement {
            Statement::Return(statements) => {
                // TODO: add support for early termination
                let _res = self.enforce_return_statement(cs, scope, statements, return_types);
            }
            Statement::For(index, start, stop, statements) => {
                self.enforce_for_statement(cs, scope, index, start, stop, statements);
            }
            Statement::MultipleDefinition(assignees, function) => {
                self.enforce_multiple_definition_statement(cs, scope, assignees, function);
            }
            Statement::Definition(variable, expression) => {
                self.enforce_definition_statement(cs, scope, variable, expression);
            }
        };
    }

    pub(crate) fn enforce_for_statement(
        &mut self,
        cs: &mut CS,
        scope: String,
        index: Variable<F>,
        start: Integer,
        stop: Integer,
        statements: Vec<Statement<F>>,
    ) {
        for i in start.to_usize()..stop.to_usize() {
            // Store index in current function scope.
            // For loop scope is not implemented.
            let index_name = new_scope_from_variable(scope.clone(), &index);
            self.store(index_name, ResolvedValue::U32(UInt32::constant(i as u32)));

            // Evaluate statements (for loop statements should not have a return type)
            statements
                .clone()
                .into_iter()
                .for_each(|statement| self.enforce_statement(cs, scope.clone(), statement, vec![]));
        }
    }
}
