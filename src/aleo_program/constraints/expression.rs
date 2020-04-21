//! Methods to enforce constraints on expressions in a resolved aleo program.
//!
//! @file expression.rs
//! @author Collin Chin <collin@aleo.org>
//! @date 2020

use crate::aleo_program::constraints::{new_scope_from_variable, ResolvedProgram, ResolvedValue};
use crate::aleo_program::{
    Expression, IntegerExpression, IntegerRangeOrExpression, StructMember, Variable,
};

use snarkos_models::curves::{Field, PrimeField};
use snarkos_models::gadgets::r1cs::ConstraintSystem;

impl<F: Field + PrimeField, CS: ConstraintSystem<F>> ResolvedProgram<F, CS> {
    fn enforce_struct_expression(
        &mut self,
        cs: &mut CS,
        scope: String,
        variable: Variable<F>,
        members: Vec<StructMember<F>>,
    ) -> ResolvedValue<F> {
        if let Some(resolved_value) = self.get_mut_variable(&variable) {
            match resolved_value {
                ResolvedValue::StructDefinition(struct_definition) => {
                    struct_definition
                        .fields
                        .clone()
                        .iter()
                        .zip(members.clone().into_iter())
                        .for_each(|(field, member)| {
                            if field.variable != member.variable {
                                unimplemented!("struct field variables do not match")
                            }
                            // Resolve and possibly enforce struct fields
                            // do we need to store the results here?
                            let _result =
                                self.enforce_expression(cs, scope.clone(), member.expression);
                        });

                    ResolvedValue::StructExpression(variable, members)
                }
                _ => unimplemented!("Inline struct type is not defined as a struct"),
            }
        } else {
            unimplemented!("Struct must be declared before it is used in an inline expression")
        }
    }

    pub(crate) fn enforce_index(
        &mut self,
        cs: &mut CS,
        scope: String,
        index: IntegerExpression<F>,
    ) -> usize {
        match self.enforce_integer_expression(cs, scope.clone(), index) {
            ResolvedValue::U32(number) => number.value.unwrap() as usize,
            value => unimplemented!("From index must resolve to a uint32, got {}", value),
        }
    }

    fn enforce_array_access_expression(
        &mut self,
        cs: &mut CS,
        scope: String,
        array: Box<Expression<F>>,
        index: IntegerRangeOrExpression<F>,
    ) -> ResolvedValue<F> {
        match self.enforce_expression(cs, scope.clone(), *array) {
            ResolvedValue::U32Array(field_array) => {
                match index {
                    IntegerRangeOrExpression::Range(from, to) => {
                        let from_resolved = match from {
                            Some(from_index) => self.enforce_index(cs, scope.clone(), from_index),
                            None => 0usize, // Array slice starts at index 0
                        };
                        let to_resolved = match to {
                            Some(to_index) => self.enforce_index(cs, scope.clone(), to_index),
                            None => field_array.len(), // Array slice ends at array length
                        };
                        ResolvedValue::U32Array(field_array[from_resolved..to_resolved].to_owned())
                    }
                    IntegerRangeOrExpression::Expression(index) => {
                        let index_resolved = self.enforce_index(cs, scope.clone(), index);
                        ResolvedValue::U32(field_array[index_resolved].to_owned())
                    }
                }
            }
            ResolvedValue::BooleanArray(bool_array) => {
                match index {
                    IntegerRangeOrExpression::Range(from, to) => {
                        let from_resolved = match from {
                            Some(from_index) => self.enforce_index(cs, scope.clone(), from_index),
                            None => 0usize, // Array slice starts at index 0
                        };
                        let to_resolved = match to {
                            Some(to_index) => self.enforce_index(cs, scope.clone(), to_index),
                            None => bool_array.len(), // Array slice ends at array length
                        };
                        ResolvedValue::BooleanArray(
                            bool_array[from_resolved..to_resolved].to_owned(),
                        )
                    }
                    IntegerRangeOrExpression::Expression(index) => {
                        let index_resolved = self.enforce_index(cs, scope.clone(), index);
                        ResolvedValue::Boolean(bool_array[index_resolved].to_owned())
                    }
                }
            }
            value => unimplemented!("Cannot access element of untyped array {}", value),
        }
    }

    fn enforce_struct_access_expression(
        &mut self,
        cs: &mut CS,
        scope: String,
        struct_variable: Box<Expression<F>>,
        struct_member: Variable<F>,
    ) -> ResolvedValue<F> {
        match self.enforce_expression(cs, scope.clone(), *struct_variable) {
            ResolvedValue::StructExpression(_name, members) => {
                let matched_member = members
                    .into_iter()
                    .find(|member| member.variable == struct_member);
                match matched_member {
                    Some(member) => self.enforce_expression(cs, scope.clone(), member.expression),
                    None => unimplemented!("Cannot access struct member {}", struct_member.name),
                }
            }
            value => unimplemented!("Cannot access element of untyped struct {}", value),
        }
    }

    fn enforce_function_access_expression(
        &mut self,
        cs: &mut CS,
        scope: String,
        function: Box<Expression<F>>,
        arguments: Vec<Expression<F>>,
    ) -> ResolvedValue<F> {
        match self.enforce_expression(cs, scope, *function) {
            ResolvedValue::Function(function) => self.enforce_function(cs, function, arguments),
            value => unimplemented!("Cannot call unknown function {}", value),
        }
    }

    pub(crate) fn enforce_expression(
        &mut self,
        cs: &mut CS,
        scope: String,
        expression: Expression<F>,
    ) -> ResolvedValue<F> {
        match expression {
            Expression::Boolean(boolean_expression) => {
                self.enforce_boolean_expression(cs, scope, boolean_expression)
            }
            Expression::Integer(integer_expression) => {
                self.enforce_integer_expression(cs, scope, integer_expression)
            }
            Expression::FieldElement(field_expression) => {
                self.enforce_field_expression(cs, scope, field_expression)
            }
            Expression::Variable(unresolved_variable) => {
                let variable_name = new_scope_from_variable(scope, &unresolved_variable);

                // Evaluate the variable name in the current function scope
                if self.contains_name(&variable_name) {
                    // Reassigning variable to another variable
                    self.get_mut(&variable_name).unwrap().clone()
                } else if self.contains_variable(&unresolved_variable) {
                    // Check global scope (function and struct names)
                    self.get_mut_variable(&unresolved_variable).unwrap().clone()
                } else {
                    // The type of the unassigned variable depends on what is passed in
                    if std::env::args()
                        .nth(1)
                        .expect("variable declaration not passed in")
                        .parse::<bool>()
                        .is_ok()
                    {
                        ResolvedValue::Boolean(self.bool_from_variable(
                            cs,
                            variable_name,
                            unresolved_variable,
                        ))
                    } else {
                        self.integer_from_variable(cs, variable_name, unresolved_variable)
                    }
                }
            }
            Expression::Struct(struct_name, members) => {
                self.enforce_struct_expression(cs, scope, struct_name, members)
            }
            Expression::ArrayAccess(array, index) => {
                self.enforce_array_access_expression(cs, scope, array, index)
            }
            Expression::StructMemberAccess(struct_variable, struct_member) => {
                self.enforce_struct_access_expression(cs, scope, struct_variable, struct_member)
            }
            Expression::FunctionCall(function, arguments) => {
                self.enforce_function_access_expression(cs, scope, function, arguments)
            } // expression => unimplemented!("expression not impl {}", expression),
        }
    }
}
