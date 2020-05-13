//! Methods to enforce constraints on expressions in a resolved Leo program.

use crate::{
    constraints::{
        new_scope_from_variable, new_variable_from_variable, ConstrainedCircuitMember,
        ConstrainedProgram, ConstrainedValue,
    },
    errors::ExpressionError,
    types::{CircuitMember, Expression, Identifier, RangeOrExpression, SpreadOrExpression},
};

use snarkos_models::{
    curves::{Field, Group, PrimeField},
    gadgets::{r1cs::ConstraintSystem, utilities::boolean::Boolean},
};

impl<F: Field + PrimeField, G: Group, CS: ConstraintSystem<F>> ConstrainedProgram<F, G, CS> {
    /// Enforce a variable expression by getting the resolved value
    pub(crate) fn evaluate_identifier(
        &mut self,
        scope: String,
        unresolved_variable: Identifier<F, G>,
    ) -> Result<ConstrainedValue<F, G>, ExpressionError> {
        // Evaluate the variable name in the current function scope
        let variable_name = new_scope_from_variable(scope, &unresolved_variable);

        if self.contains_name(&variable_name) {
            // Reassigning variable to another variable
            Ok(self.get_mut(&variable_name).unwrap().clone())
        } else if self.contains_variable(&unresolved_variable) {
            // Check global scope (function and circuit names)
            Ok(self.get_mut_variable(&unresolved_variable).unwrap().clone())
        } else {
            Err(ExpressionError::UndefinedVariable(
                unresolved_variable.to_string(),
            ))
        }
    }

    /// Enforce numerical operations
    fn enforce_add_expression(
        &mut self,
        cs: &mut CS,
        left: ConstrainedValue<F, G>,
        right: ConstrainedValue<F, G>,
    ) -> Result<ConstrainedValue<F, G>, ExpressionError> {
        Ok(match (left, right) {
            (ConstrainedValue::Integer(num_1), ConstrainedValue::Integer(num_2)) => {
                Self::enforce_integer_add(cs, num_1, num_2)?
            }
            (ConstrainedValue::FieldElement(fe_1), ConstrainedValue::FieldElement(fe_2)) => {
                self.enforce_field_add(cs, fe_1, fe_2)?
            }
            (ConstrainedValue::GroupElement(ge_1), ConstrainedValue::GroupElement(ge_2)) => {
                Self::evaluate_group_add(ge_1, ge_2)
            }
            (val_1, val_2) => {
                return Err(ExpressionError::IncompatibleTypes(format!(
                    "{} + {}",
                    val_1, val_2,
                )))
            }
        })
    }

    fn enforce_sub_expression(
        &mut self,
        cs: &mut CS,
        left: ConstrainedValue<F, G>,
        right: ConstrainedValue<F, G>,
    ) -> Result<ConstrainedValue<F, G>, ExpressionError> {
        Ok(match (left, right) {
            (ConstrainedValue::Integer(num_1), ConstrainedValue::Integer(num_2)) => {
                Self::enforce_integer_sub(cs, num_1, num_2)?
            }
            (ConstrainedValue::FieldElement(fe_1), ConstrainedValue::FieldElement(fe_2)) => {
                self.enforce_field_sub(cs, fe_1, fe_2)?
            }
            (ConstrainedValue::GroupElement(ge_1), ConstrainedValue::GroupElement(ge_2)) => {
                Self::evaluate_group_sub(ge_1, ge_2)
            }
            (val_1, val_2) => {
                return Err(ExpressionError::IncompatibleTypes(format!(
                    "{} - {}",
                    val_1, val_2,
                )))
            }
        })
    }

    fn enforce_mul_expression(
        &mut self,
        cs: &mut CS,
        left: ConstrainedValue<F, G>,
        right: ConstrainedValue<F, G>,
    ) -> Result<ConstrainedValue<F, G>, ExpressionError> {
        Ok(match (left, right) {
            (ConstrainedValue::Integer(num_1), ConstrainedValue::Integer(num_2)) => {
                Self::enforce_integer_mul(cs, num_1, num_2)?
            }
            (ConstrainedValue::FieldElement(fe_1), ConstrainedValue::FieldElement(fe_2)) => {
                self.enforce_field_mul(cs, fe_1, fe_2)?
            }
            (val_1, val_2) => {
                return Err(ExpressionError::IncompatibleTypes(format!(
                    "{} * {}",
                    val_1, val_2,
                )))
            }
        })
    }

    fn enforce_div_expression(
        &mut self,
        cs: &mut CS,
        left: ConstrainedValue<F, G>,
        right: ConstrainedValue<F, G>,
    ) -> Result<ConstrainedValue<F, G>, ExpressionError> {
        Ok(match (left, right) {
            (ConstrainedValue::Integer(num_1), ConstrainedValue::Integer(num_2)) => {
                Self::enforce_integer_div(cs, num_1, num_2)?
            }
            (ConstrainedValue::FieldElement(fe_1), ConstrainedValue::FieldElement(fe_2)) => {
                self.enforce_field_div(cs, fe_1, fe_2)?
            }
            (val_1, val_2) => {
                return Err(ExpressionError::IncompatibleTypes(format!(
                    "{} / {}",
                    val_1, val_2,
                )))
            }
        })
    }
    fn enforce_pow_expression(
        &mut self,
        cs: &mut CS,
        left: ConstrainedValue<F, G>,
        right: ConstrainedValue<F, G>,
    ) -> Result<ConstrainedValue<F, G>, ExpressionError> {
        Ok(match (left, right) {
            (ConstrainedValue::Integer(num_1), ConstrainedValue::Integer(num_2)) => {
                Self::enforce_integer_pow(cs, num_1, num_2)?
            }
            (ConstrainedValue::FieldElement(fe_1), ConstrainedValue::Integer(num_2)) => {
                self.enforce_field_pow(cs, fe_1, num_2)?
            }
            (_, ConstrainedValue::FieldElement(num_2)) => {
                return Err(ExpressionError::InvalidExponent(num_2.to_string()))
            }
            (val_1, val_2) => {
                return Err(ExpressionError::IncompatibleTypes(format!(
                    "{} * {}",
                    val_1, val_2,
                )))
            }
        })
    }

    /// Evaluate Boolean operations
    fn evaluate_eq_expression(
        &mut self,
        left: ConstrainedValue<F, G>,
        right: ConstrainedValue<F, G>,
    ) -> Result<ConstrainedValue<F, G>, ExpressionError> {
        Ok(match (left, right) {
            (ConstrainedValue::Boolean(bool_1), ConstrainedValue::Boolean(bool_2)) => {
                Self::boolean_eq(bool_1, bool_2)
            }
            (ConstrainedValue::Integer(num_1), ConstrainedValue::Integer(num_2)) => {
                Self::evaluate_integer_eq(num_1, num_2)?
            }
            // (ResolvedValue::FieldElement(fe_1), ResolvedValue::FieldElement(fe_2)) => {
            //     Self::field_eq(fe_1, fe_2)
            // }
            (ConstrainedValue::GroupElement(ge_1), ConstrainedValue::GroupElement(ge_2)) => {
                Self::evaluate_group_eq(ge_1, ge_2)
            }
            (val_1, val_2) => {
                return Err(ExpressionError::IncompatibleTypes(format!(
                    "{} == {}",
                    val_1, val_2,
                )))
            }
        })
    }

    fn evaluate_geq_expression(
        &mut self,
        left: ConstrainedValue<F, G>,
        right: ConstrainedValue<F, G>,
    ) -> Result<ConstrainedValue<F, G>, ExpressionError> {
        match (left, right) {
            // (ResolvedValue::FieldElement(fe_1), ResolvedValue::FieldElement(fe_2)) => {
            //     Self::field_geq(fe_1, fe_2)
            // }
            (val_1, val_2) => Err(ExpressionError::IncompatibleTypes(format!(
                "{} >= {}, values must be fields",
                val_1, val_2
            ))),
        }
    }

    fn evaluate_gt_expression(
        &mut self,
        left: ConstrainedValue<F, G>,
        right: ConstrainedValue<F, G>,
    ) -> Result<ConstrainedValue<F, G>, ExpressionError> {
        match (left, right) {
            // (ResolvedValue::FieldElement(fe_1), ResolvedValue::FieldElement(fe_2)) => {
            //     Self::field_gt(fe_1, fe_2)
            // }
            (val_1, val_2) => Err(ExpressionError::IncompatibleTypes(format!(
                "{} > {}, values must be fields",
                val_1, val_2
            ))),
        }
    }

    fn evaluate_leq_expression(
        &mut self,
        left: ConstrainedValue<F, G>,
        right: ConstrainedValue<F, G>,
    ) -> Result<ConstrainedValue<F, G>, ExpressionError> {
        match (left, right) {
            // (ResolvedValue::FieldElement(fe_1), ResolvedValue::FieldElement(fe_2)) => {
            //     Self::field_leq(fe_1, fe_2)
            // }
            (val_1, val_2) => Err(ExpressionError::IncompatibleTypes(format!(
                "{} <= {}, values must be fields",
                val_1, val_2
            ))),
        }
    }

    fn evaluate_lt_expression(
        &mut self,
        left: ConstrainedValue<F, G>,
        right: ConstrainedValue<F, G>,
    ) -> Result<ConstrainedValue<F, G>, ExpressionError> {
        match (left, right) {
            // (ResolvedValue::FieldElement(fe_1), ResolvedValue::FieldElement(fe_2)) => {
            //     Self::field_lt(fe_1, fe_2)
            // }
            (val_1, val_2) => Err(ExpressionError::IncompatibleTypes(format!(
                "{} < {}, values must be fields",
                val_1, val_2,
            ))),
        }
    }

    /// Enforce array expressions
    fn enforce_array_expression(
        &mut self,
        cs: &mut CS,
        file_scope: String,
        function_scope: String,
        array: Vec<Box<SpreadOrExpression<F, G>>>,
    ) -> Result<ConstrainedValue<F, G>, ExpressionError> {
        let mut result = vec![];
        for element in array.into_iter() {
            match *element {
                SpreadOrExpression::Spread(spread) => match spread {
                    Expression::Identifier(variable) => {
                        let array_name = new_scope_from_variable(function_scope.clone(), &variable);
                        match self.get(&array_name) {
                            Some(value) => match value {
                                ConstrainedValue::Array(array) => result.extend(array.clone()),
                                value => {
                                    return Err(ExpressionError::InvalidSpread(value.to_string()));
                                }
                            },
                            None => return Err(ExpressionError::UndefinedArray(variable.name)),
                        }
                    }
                    value => return Err(ExpressionError::InvalidSpread(value.to_string())),
                },
                SpreadOrExpression::Expression(expression) => {
                    result.push(self.enforce_expression(
                        cs,
                        file_scope.clone(),
                        function_scope.clone(),
                        expression,
                    )?);
                }
            }
        }
        Ok(ConstrainedValue::Array(result))
    }

    pub(crate) fn enforce_index(
        &mut self,
        cs: &mut CS,
        file_scope: String,
        function_scope: String,
        index: Expression<F, G>,
    ) -> Result<usize, ExpressionError> {
        match self.enforce_expression(cs, file_scope, function_scope, index)? {
            ConstrainedValue::Integer(number) => Ok(number.to_usize()),
            value => Err(ExpressionError::InvalidIndex(value.to_string())),
        }
    }

    fn enforce_array_access_expression(
        &mut self,
        cs: &mut CS,
        file_scope: String,
        function_scope: String,
        array: Box<Expression<F, G>>,
        index: RangeOrExpression<F, G>,
    ) -> Result<ConstrainedValue<F, G>, ExpressionError> {
        match self.enforce_expression(cs, file_scope.clone(), function_scope.clone(), *array)? {
            ConstrainedValue::Array(array) => {
                match index {
                    RangeOrExpression::Range(from, to) => {
                        let from_resolved = match from {
                            Some(from_index) => from_index.to_usize(),
                            None => 0usize, // Array slice starts at index 0
                        };
                        let to_resolved = match to {
                            Some(to_index) => to_index.to_usize(),
                            None => array.len(), // Array slice ends at array length
                        };
                        Ok(ConstrainedValue::Array(
                            array[from_resolved..to_resolved].to_owned(),
                        ))
                    }
                    RangeOrExpression::Expression(index) => {
                        let index_resolved =
                            self.enforce_index(cs, file_scope, function_scope, index)?;
                        Ok(array[index_resolved].to_owned())
                    }
                }
            }
            value => Err(ExpressionError::InvalidArrayAccess(value.to_string())),
        }
    }

    fn enforce_circuit_expression(
        &mut self,
        cs: &mut CS,
        file_scope: String,
        function_scope: String,
        variable: Identifier<F, G>,
        members: Vec<CircuitMember<F, G>>,
    ) -> Result<ConstrainedValue<F, G>, ExpressionError> {
        let circuit_name = new_variable_from_variable(file_scope.clone(), &variable);

        if let Some(ConstrainedValue::CircuitDefinition(circuit_definition)) =
            self.get_mut_variable(&circuit_name)
        {
            let mut resolved_members = vec![];
            for (field, member) in circuit_definition
                .fields
                .clone()
                .into_iter()
                .zip(members.clone().into_iter())
            {
                if field.identifier != member.identifier {
                    return Err(ExpressionError::InvalidCircuitObject(
                        field.identifier.name,
                        member.identifier.name,
                    ));
                }
                // Resolve and enforce circuit fields
                let member_value = self.enforce_expression(
                    cs,
                    file_scope.clone(),
                    function_scope.clone(),
                    member.expression,
                )?;

                // Check member types
                member_value.expect_type(&field._type)?;

                resolved_members.push(ConstrainedCircuitMember(member.identifier, member_value))
            }

            Ok(ConstrainedValue::CircuitExpression(
                variable,
                resolved_members,
            ))
        } else {
            Err(ExpressionError::UndefinedCircuit(variable.to_string()))
        }
    }

    fn enforce_circuit_access_expression(
        &mut self,
        cs: &mut CS,
        file_scope: String,
        function_scope: String,
        circuit_variable: Box<Expression<F, G>>,
        circuit_member: Identifier<F, G>,
    ) -> Result<ConstrainedValue<F, G>, ExpressionError> {
        match self.enforce_expression(cs, file_scope, function_scope, *circuit_variable)? {
            ConstrainedValue::CircuitExpression(_name, members) => {
                let matched_member = members
                    .into_iter()
                    .find(|member| member.0 == circuit_member);
                match matched_member {
                    Some(member) => Ok(member.1),
                    None => Err(ExpressionError::UndefinedCircuitObject(
                        circuit_member.to_string(),
                    )),
                }
            }
            value => Err(ExpressionError::InvalidCircuitAccess(value.to_string())),
        }
    }

    fn enforce_function_call_expression(
        &mut self,
        cs: &mut CS,
        file_scope: String,
        function_scope: String,
        function: Identifier<F, G>,
        arguments: Vec<Expression<F, G>>,
    ) -> Result<ConstrainedValue<F, G>, ExpressionError> {
        let function_name = new_variable_from_variable(file_scope.clone(), &function);
        let function_call = match self.get(&function_name.to_string()) {
            Some(ConstrainedValue::Function(function)) => function.clone(),
            _ => return Err(ExpressionError::UndefinedFunction(function.to_string())),
        };

        match self.enforce_function(cs, file_scope, function_scope, function_call, arguments) {
            Ok(ConstrainedValue::Return(return_values)) => {
                if return_values.len() == 1 {
                    Ok(return_values[0].clone())
                } else {
                    Ok(ConstrainedValue::Return(return_values))
                }
            }
            Ok(_) => Err(ExpressionError::FunctionDidNotReturn(function.to_string())),
            Err(error) => Err(ExpressionError::from(Box::new(error))),
        }
    }

    pub(crate) fn enforce_expression(
        &mut self,
        cs: &mut CS,
        file_scope: String,
        function_scope: String,
        expression: Expression<F, G>,
    ) -> Result<ConstrainedValue<F, G>, ExpressionError> {
        match expression {
            // Variables
            Expression::Identifier(unresolved_variable) => {
                self.evaluate_identifier(function_scope, unresolved_variable)
            }

            // Values
            Expression::Integer(integer) => Ok(Self::get_integer_constant(integer)),
            Expression::FieldElement(fe) => Ok(Self::get_field_element_constant(fe)),
            Expression::GroupElement(gr) => Ok(ConstrainedValue::GroupElement(gr)),
            Expression::Boolean(bool) => Ok(Self::get_boolean_constant(bool)),

            // Binary operations
            Expression::Add(left, right) => {
                let resolved_left =
                    self.enforce_expression(cs, file_scope.clone(), function_scope.clone(), *left)?;
                let resolved_right = self.enforce_expression(
                    cs,
                    file_scope.clone(),
                    function_scope.clone(),
                    *right,
                )?;

                self.enforce_add_expression(cs, resolved_left, resolved_right)
            }
            Expression::Sub(left, right) => {
                let resolved_left =
                    self.enforce_expression(cs, file_scope.clone(), function_scope.clone(), *left)?;
                let resolved_right = self.enforce_expression(
                    cs,
                    file_scope.clone(),
                    function_scope.clone(),
                    *right,
                )?;

                self.enforce_sub_expression(cs, resolved_left, resolved_right)
            }
            Expression::Mul(left, right) => {
                let resolved_left =
                    self.enforce_expression(cs, file_scope.clone(), function_scope.clone(), *left)?;
                let resolved_right = self.enforce_expression(
                    cs,
                    file_scope.clone(),
                    function_scope.clone(),
                    *right,
                )?;

                self.enforce_mul_expression(cs, resolved_left, resolved_right)
            }
            Expression::Div(left, right) => {
                let resolved_left =
                    self.enforce_expression(cs, file_scope.clone(), function_scope.clone(), *left)?;
                let resolved_right = self.enforce_expression(
                    cs,
                    file_scope.clone(),
                    function_scope.clone(),
                    *right,
                )?;

                self.enforce_div_expression(cs, resolved_left, resolved_right)
            }
            Expression::Pow(left, right) => {
                let resolved_left =
                    self.enforce_expression(cs, file_scope.clone(), function_scope.clone(), *left)?;
                let resolved_right = self.enforce_expression(
                    cs,
                    file_scope.clone(),
                    function_scope.clone(),
                    *right,
                )?;

                self.enforce_pow_expression(cs, resolved_left, resolved_right)
            }

            // Boolean operations
            Expression::Not(expression) => Ok(Self::evaluate_not(self.enforce_expression(
                cs,
                file_scope,
                function_scope,
                *expression,
            )?)?),
            Expression::Or(left, right) => {
                let resolved_left =
                    self.enforce_expression(cs, file_scope.clone(), function_scope.clone(), *left)?;
                let resolved_right = self.enforce_expression(
                    cs,
                    file_scope.clone(),
                    function_scope.clone(),
                    *right,
                )?;

                Ok(self.enforce_or(cs, resolved_left, resolved_right)?)
            }
            Expression::And(left, right) => {
                let resolved_left =
                    self.enforce_expression(cs, file_scope.clone(), function_scope.clone(), *left)?;
                let resolved_right = self.enforce_expression(
                    cs,
                    file_scope.clone(),
                    function_scope.clone(),
                    *right,
                )?;

                Ok(self.enforce_and(cs, resolved_left, resolved_right)?)
            }
            Expression::Eq(left, right) => {
                let resolved_left =
                    self.enforce_expression(cs, file_scope.clone(), function_scope.clone(), *left)?;
                let resolved_right = self.enforce_expression(
                    cs,
                    file_scope.clone(),
                    function_scope.clone(),
                    *right,
                )?;

                Ok(self.evaluate_eq_expression(resolved_left, resolved_right)?)
            }
            Expression::Geq(left, right) => {
                let resolved_left =
                    self.enforce_expression(cs, file_scope.clone(), function_scope.clone(), *left)?;
                let resolved_right = self.enforce_expression(
                    cs,
                    file_scope.clone(),
                    function_scope.clone(),
                    *right,
                )?;

                Ok(self.evaluate_geq_expression(resolved_left, resolved_right)?)
            }
            Expression::Gt(left, right) => {
                let resolved_left =
                    self.enforce_expression(cs, file_scope.clone(), function_scope.clone(), *left)?;
                let resolved_right = self.enforce_expression(
                    cs,
                    file_scope.clone(),
                    function_scope.clone(),
                    *right,
                )?;

                Ok(self.evaluate_gt_expression(resolved_left, resolved_right)?)
            }
            Expression::Leq(left, right) => {
                let resolved_left =
                    self.enforce_expression(cs, file_scope.clone(), function_scope.clone(), *left)?;
                let resolved_right = self.enforce_expression(
                    cs,
                    file_scope.clone(),
                    function_scope.clone(),
                    *right,
                )?;

                Ok(self.evaluate_leq_expression(resolved_left, resolved_right)?)
            }
            Expression::Lt(left, right) => {
                let resolved_left =
                    self.enforce_expression(cs, file_scope.clone(), function_scope.clone(), *left)?;
                let resolved_right = self.enforce_expression(
                    cs,
                    file_scope.clone(),
                    function_scope.clone(),
                    *right,
                )?;

                Ok(self.evaluate_lt_expression(resolved_left, resolved_right)?)
            }

            // Conditionals
            Expression::IfElse(first, second, third) => {
                let resolved_first = match self.enforce_expression(
                    cs,
                    file_scope.clone(),
                    function_scope.clone(),
                    *first,
                )? {
                    ConstrainedValue::Boolean(resolved) => resolved,
                    value => return Err(ExpressionError::IfElseConditional(value.to_string())),
                };

                if resolved_first.eq(&Boolean::Constant(true)) {
                    self.enforce_expression(cs, file_scope, function_scope, *second)
                } else {
                    self.enforce_expression(cs, file_scope, function_scope, *third)
                }
            }

            // Arrays
            Expression::Array(array) => {
                self.enforce_array_expression(cs, file_scope, function_scope, array)
            }
            Expression::ArrayAccess(array, index) => {
                self.enforce_array_access_expression(cs, file_scope, function_scope, array, *index)
            }

            // Circuits
            Expression::Circuit(circuit_name, members) => self.enforce_circuit_expression(
                cs,
                file_scope,
                function_scope,
                circuit_name,
                members,
            ),
            Expression::CircuitMemberAccess(circuit_variable, circuit_member) => self
                .enforce_circuit_access_expression(
                    cs,
                    file_scope,
                    function_scope,
                    circuit_variable,
                    circuit_member,
                ),

            // Functions
            Expression::FunctionCall(function, arguments) => self.enforce_function_call_expression(
                cs,
                file_scope,
                function_scope,
                function,
                arguments,
            ),
        }
    }
}
