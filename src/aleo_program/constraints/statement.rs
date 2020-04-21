use crate::aleo_program::constraints::{new_scope_from_variable, ResolvedProgram, ResolvedValue};
use crate::aleo_program::{
    Assignee, Expression, IntegerExpression, IntegerRangeOrExpression, Statement, Variable,
};

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

    pub(crate) fn enforce_definition_statement(
        &mut self,
        cs: &mut CS,
        scope: String,
        assignee: Assignee<F>,
        expression: Expression<F>,
    ) {
        // Create or modify the lhs variable in the current function scope
        match assignee {
            Assignee::Variable(name) => {
                // Store the variable in the current scope
                let definition_name = new_scope_from_variable(scope.clone(), &name);

                // Evaluate the rhs expression in the current function scope
                let result = self.enforce_expression(cs, scope, expression);

                self.store(definition_name, result);
            }
            Assignee::Array(array, index_expression) => {
                // Evaluate the rhs expression in the current function scope
                let result = &mut self.enforce_expression(cs, scope.clone(), expression);

                // Check that array exists
                let expected_array_name = self.resolve_assignee(scope.clone(), *array);

                // Resolve index so we know if we are assigning to a single value or a range of values
                match index_expression {
                    IntegerRangeOrExpression::Expression(index) => {
                        let index = self.enforce_index(cs, scope.clone(), index);

                        // Modify the single value of the array in place
                        match self.get_mut(&expected_array_name) {
                            Some(value) => match (value, result) {
                                (ResolvedValue::U32Array(old), ResolvedValue::U32(new)) => {
                                    old[index] = new.to_owned();
                                }
                                (ResolvedValue::BooleanArray(old), ResolvedValue::Boolean(new)) => {
                                    old[index] = new.to_owned();
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
                    IntegerRangeOrExpression::Range(from, to) => {
                        let from_index = match from {
                            Some(expression) => self.enforce_index(cs, scope.clone(), expression),
                            None => 0usize,
                        };
                        let to_index_option = match to {
                            Some(expression) => {
                                Some(self.enforce_index(cs, scope.clone(), expression))
                            }
                            None => None,
                        };

                        // Modify the range of values of the array in place
                        match self.get_mut(&expected_array_name) {
                            Some(value) => match (value, result) {
                                (ResolvedValue::U32Array(old), ResolvedValue::U32Array(new)) => {
                                    let to_index = to_index_option.unwrap_or(old.len());
                                    old.splice(from_index..to_index, new.iter().cloned());
                                }
                                (
                                    ResolvedValue::BooleanArray(old),
                                    ResolvedValue::BooleanArray(new),
                                ) => {
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
                            let matched_member = members
                                .into_iter()
                                .find(|member| member.variable == struct_member);
                            match matched_member {
                                Some(mut member) => member.expression = expression,
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
        };
    }

    pub(crate) fn enforce_return_statement(
        &mut self,
        cs: &mut CS,
        scope: String,
        statements: Vec<Expression<F>>,
    ) -> ResolvedValue<F> {
        ResolvedValue::Return(
            statements
                .into_iter()
                .map(|expression| self.enforce_expression(cs, scope.clone(), expression))
                .collect::<Vec<ResolvedValue<F>>>(),
        )
    }

    fn enforce_statement(&mut self, cs: &mut CS, scope: String, statement: Statement<F>) {
        match statement {
            Statement::Definition(variable, expression) => {
                self.enforce_definition_statement(cs, scope, variable, expression);
            }
            Statement::For(index, start, stop, statements) => {
                self.enforce_for_statement(cs, scope, index, start, stop, statements);
            }
            Statement::Return(statements) => {
                // TODO: add support for early termination
                let _res = self.enforce_return_statement(cs, scope, statements);
            }
        };
    }

    pub(crate) fn enforce_for_statement(
        &mut self,
        cs: &mut CS,
        scope: String,
        index: Variable<F>,
        start: IntegerExpression<F>,
        stop: IntegerExpression<F>,
        statements: Vec<Statement<F>>,
    ) {
        let start_index = self.enforce_index(cs, scope.clone(), start);
        let stop_index = self.enforce_index(cs, scope.clone(), stop);

        for i in start_index..stop_index {
            // Store index in current function scope.
            // For loop scope is not implemented.
            let index_name = new_scope_from_variable(scope.clone(), &index);
            self.store(index_name, ResolvedValue::U32(UInt32::constant(i as u32)));

            // Evaluate statements
            statements
                .clone()
                .into_iter()
                .for_each(|statement| self.enforce_statement(cs, scope.clone(), statement));
        }
    }
}
