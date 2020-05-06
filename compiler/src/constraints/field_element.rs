//! Methods to enforce constraints on field elements in a resolved aleo program.

use crate::constraints::{ResolvedProgram, ResolvedValue};
use crate::{new_variable_from_variable, FieldElement, ParameterModel, ParameterValue, Variable};

use snarkos_errors::gadgets::SynthesisError;
use snarkos_models::curves::{Field, PrimeField};
use snarkos_models::gadgets::{
    r1cs::{ConstraintSystem, LinearCombination, Variable as R1CSVariable},
    utilities::uint32::UInt32,
};

impl<F: Field + PrimeField, CS: ConstraintSystem<F>> ResolvedProgram<F, CS> {
    pub(crate) fn field_element_from_parameter(
        &mut self,
        cs: &mut CS,
        scope: String,
        parameter_model: ParameterModel<F>,
        parameter_value: Option<ParameterValue<F>>,
    ) -> Variable<F> {
        // Check that the parameter value is the correct type
        let field_option = parameter_value.map(|parameter| match parameter {
            ParameterValue::Field(f) => f,
            value => unimplemented!("expected field parameter, got {}", value),
        });

        // Check visibility of parameter
        let name = parameter_model.variable.name.clone();
        let field_value = if parameter_model.private {
            cs.alloc(
                || name,
                || field_option.ok_or(SynthesisError::AssignmentMissing),
            )
            .unwrap()
        } else {
            cs.alloc_input(
                || name,
                || field_option.ok_or(SynthesisError::AssignmentMissing),
            )
            .unwrap()
        };

        let parameter_variable = new_variable_from_variable(scope, &parameter_model.variable);

        // Store parameter as variable in resolved program
        self.store_variable(
            parameter_variable.clone(),
            ResolvedValue::FieldElement(FieldElement::Allocated(field_option, field_value)),
        );

        parameter_variable
    }

    pub(crate) fn field_element_array_from_parameter(
        &mut self,
        _cs: &mut CS,
        _scope: String,
        _parameter_model: ParameterModel<F>,
        _parameter_value: Option<ParameterValue<F>>,
    ) -> Variable<F> {
        unimplemented!("Cannot enforce field element array as parameter")
        // // Check visibility of parameter
        // let mut array_value = vec![];
        // let name = parameter.variable.name.clone();
        // for argument in argument_array {
        //     if parameter.private {
        //         cs.alloc(|| name, || Ok(argument.clone())).unwrap();
        //     } else {
        //         cs.alloc_input(|| name, || Ok(argument.clone())).unwrap();
        //     };
        // }
        //
        //
        // let parameter_variable = new_variable_from_variable(scope, &parameter.variable);
        //
        // // store array as variable in resolved program
        // self.store_variable(parameter_variable.clone(), ResolvedValue::FieldElementArray(argument_array));
        //
        // parameter_variable
    }

    pub(crate) fn get_field_element_constant(fe: F) -> ResolvedValue<F> {
        ResolvedValue::FieldElement(FieldElement::Constant(fe))
    }

    // pub(crate) fn field_eq(fe1: F, fe2: F) -> ResolvedValue<F> {
    //     ResolvedValue::Boolean(Boolean::Constant(fe1.eq(&fe2)))
    // }
    //
    // pub(crate) fn field_geq(fe1: F, fe2: F) -> ResolvedValue<F> {
    //     ResolvedValue::Boolean(Boolean::Constant(fe1.ge(&fe2)))
    // }
    //
    // pub(crate) fn field_gt(fe1: F, fe2: F) -> ResolvedValue<F> {
    //     ResolvedValue::Boolean(Boolean::Constant(fe1.gt(&fe2)))
    // }
    //
    // pub(crate) fn field_leq(fe1: F, fe2: F) -> ResolvedValue<F> {
    //     ResolvedValue::Boolean(Boolean::Constant(fe1.le(&fe2)))
    // }
    //
    // pub(crate) fn field_lt(fe1: F, fe2: F) -> ResolvedValue<F> {
    //     ResolvedValue::Boolean(Boolean::Constant(fe1.lt(&fe2)))
    // }

    pub(crate) fn enforce_field_eq(
        &mut self,
        cs: &mut CS,
        fe_1: FieldElement<F>,
        fe_2: FieldElement<F>,
    ) {
        let mut lc = LinearCombination::zero();

        match (fe_1, fe_2) {
            (FieldElement::Constant(fe_1_constant), FieldElement::Constant(fe_2_constant)) => {
                // lc = lc + (fe_1_constant * 1) - (fe_2_constant * 1)
                // lc = lc + fe_1 - fe_2
                lc = lc + (fe_1_constant, CS::one()) - (fe_2_constant, CS::one());
            }
            // else, return an allocated result
            (
                FieldElement::Allocated(_fe_1_value, fe_1_variable),
                FieldElement::Constant(fe_2_constant),
            ) => {
                // lc = lc + fe_1 - (fe_2_constant * 1)
                // lc = lc + fe_1 - fe_2
                lc = lc + fe_1_variable - (fe_2_constant, CS::one())
            }
            (
                FieldElement::Constant(fe_1_constant),
                FieldElement::Allocated(_fe_2_value, fe_2_variable),
            ) => {
                // lc = lc + (fe_1_constant * 1) - fe_2
                // lc = lc + fe_1 - fe_2
                lc = lc + (fe_1_constant, CS::one()) - fe_2_variable
            }
            (
                FieldElement::Allocated(_fe_1_value, fe_1_variable),
                FieldElement::Allocated(_fe_2_value, fe_2_variable),
            ) => {
                // lc = lc + fe_1 - fe_2
                lc = lc + fe_1_variable - fe_2_variable
            }
        }

        // enforce that the linear combination is zero
        cs.enforce(|| "field equality", |lc| lc, |lc| lc, |_| lc);
    }

    pub(crate) fn enforce_field_add(
        &mut self,
        cs: &mut CS,
        fe_1: FieldElement<F>,
        fe_2: FieldElement<F>,
    ) -> ResolvedValue<F> {
        match (fe_1, fe_2) {
            // if both constants, then return a constant result
            (FieldElement::Constant(fe_1_constant), FieldElement::Constant(fe_2_constant)) => {
                ResolvedValue::FieldElement(FieldElement::Constant(
                    fe_1_constant.add(&fe_2_constant),
                ))
            }
            // else, return an allocated result
            (
                FieldElement::Allocated(fe_1_value, fe_1_variable),
                FieldElement::Constant(fe_2_constant),
            ) => {
                let sum_value: Option<F> = fe_1_value.map(|v| v.add(&fe_2_constant));
                let sum_variable: R1CSVariable = cs
                    .alloc(
                        || "field addition",
                        || sum_value.ok_or(SynthesisError::AssignmentMissing),
                    )
                    .unwrap();

                cs.enforce(
                    || "sum = 1 * (fe_1 + fe2)",
                    |lc| lc + CS::one(),
                    |lc| lc + fe_1_variable + (fe_2_constant, CS::one()),
                    |lc| lc + sum_variable.clone(),
                );

                ResolvedValue::FieldElement(FieldElement::Allocated(sum_value, sum_variable))
            }
            (
                FieldElement::Constant(fe_1_constant),
                FieldElement::Allocated(fe_2_value, fe_2_variable),
            ) => {
                let sum_value: Option<F> = fe_2_value.map(|v| fe_1_constant.add(&v));
                let sum_variable: R1CSVariable = cs
                    .alloc(
                        || "field addition",
                        || sum_value.ok_or(SynthesisError::AssignmentMissing),
                    )
                    .unwrap();

                cs.enforce(
                    || "sum = 1 * (fe_1 + fe_2)",
                    |lc| lc + CS::one(),
                    |lc| lc + (fe_1_constant, CS::one()) + fe_2_variable,
                    |lc| lc + sum_variable.clone(),
                );

                ResolvedValue::FieldElement(FieldElement::Allocated(sum_value, sum_variable))
            }
            (
                FieldElement::Allocated(fe_1_value, fe_1_variable),
                FieldElement::Allocated(fe_2_value, fe_2_variable),
            ) => {
                let sum_value: Option<F> = match (fe_1_value, fe_2_value) {
                    (Some(fe_1_value), Some(fe_2_value)) => Some(fe_1_value.add(&fe_2_value)),
                    (_, _) => None,
                };
                let sum_variable: R1CSVariable = cs
                    .alloc(
                        || "field addition",
                        || sum_value.ok_or(SynthesisError::AssignmentMissing),
                    )
                    .unwrap();

                cs.enforce(
                    || "sum = 1 * (fe_1 + fe_2)",
                    |lc| lc + CS::one(),
                    |lc| lc + fe_1_variable + fe_2_variable,
                    |lc| lc + sum_variable.clone(),
                );

                ResolvedValue::FieldElement(FieldElement::Allocated(sum_value, sum_variable))
            }
        }
    }

    pub(crate) fn enforce_field_sub(
        &mut self,
        cs: &mut CS,
        fe_1: FieldElement<F>,
        fe_2: FieldElement<F>,
    ) -> ResolvedValue<F> {
        match (fe_1, fe_2) {
            // if both constants, then return a constant result
            (FieldElement::Constant(fe_1_constant), FieldElement::Constant(fe_2_constant)) => {
                ResolvedValue::FieldElement(FieldElement::Constant(
                    fe_1_constant.sub(&fe_2_constant),
                ))
            }
            // else, return an allocated result
            (
                FieldElement::Allocated(fe_1_value, fe_1_variable),
                FieldElement::Constant(fe_2_constant),
            ) => {
                let sub_value: Option<F> = fe_1_value.map(|v| v.sub(&fe_2_constant));
                let sub_variable: R1CSVariable = cs
                    .alloc(
                        || "field subtraction",
                        || sub_value.ok_or(SynthesisError::AssignmentMissing),
                    )
                    .unwrap();

                cs.enforce(
                    || "sub = 1 * (fe_1 - fe2)",
                    |lc| lc + CS::one(),
                    |lc| lc + fe_1_variable - (fe_2_constant, CS::one()),
                    |lc| lc + sub_variable.clone(),
                );

                ResolvedValue::FieldElement(FieldElement::Allocated(sub_value, sub_variable))
            }
            (
                FieldElement::Constant(fe_1_constant),
                FieldElement::Allocated(fe_2_value, fe_2_variable),
            ) => {
                let sub_value: Option<F> = fe_2_value.map(|v| fe_1_constant.sub(&v));
                let sub_variable: R1CSVariable = cs
                    .alloc(
                        || "field subtraction",
                        || sub_value.ok_or(SynthesisError::AssignmentMissing),
                    )
                    .unwrap();

                cs.enforce(
                    || "sub = 1 * (fe_1 - fe_2)",
                    |lc| lc + CS::one(),
                    |lc| lc + (fe_1_constant, CS::one()) - fe_2_variable,
                    |lc| lc + sub_variable.clone(),
                );

                ResolvedValue::FieldElement(FieldElement::Allocated(sub_value, sub_variable))
            }
            (
                FieldElement::Allocated(fe_1_value, fe_1_variable),
                FieldElement::Allocated(fe_2_value, fe_2_variable),
            ) => {
                let sub_value: Option<F> = match (fe_1_value, fe_2_value) {
                    (Some(fe_1_value), Some(fe_2_value)) => Some(fe_1_value.sub(&fe_2_value)),
                    (_, _) => None,
                };
                let sub_variable: R1CSVariable = cs
                    .alloc(
                        || "field subtraction",
                        || sub_value.ok_or(SynthesisError::AssignmentMissing),
                    )
                    .unwrap();

                cs.enforce(
                    || "sub = 1 * (fe_1 - fe_2)",
                    |lc| lc + CS::one(),
                    |lc| lc + fe_1_variable - fe_2_variable,
                    |lc| lc + sub_variable.clone(),
                );

                ResolvedValue::FieldElement(FieldElement::Allocated(sub_value, sub_variable))
            }
        }
    }

    pub(crate) fn enforce_field_mul(
        &mut self,
        cs: &mut CS,
        fe_1: FieldElement<F>,
        fe_2: FieldElement<F>,
    ) -> ResolvedValue<F> {
        match (fe_1, fe_2) {
            // if both constants, then return a constant result
            (FieldElement::Constant(fe_1_constant), FieldElement::Constant(fe_2_constant)) => {
                ResolvedValue::FieldElement(FieldElement::Constant(
                    fe_1_constant.mul(&fe_2_constant),
                ))
            }
            // else, return an allocated result
            (
                FieldElement::Allocated(fe_1_value, fe_1_variable),
                FieldElement::Constant(fe_2_constant),
            ) => {
                let mul_value: Option<F> = fe_1_value.map(|v| v.mul(&fe_2_constant));
                let mul_variable: R1CSVariable = cs
                    .alloc(
                        || "field multiplication",
                        || mul_value.ok_or(SynthesisError::AssignmentMissing),
                    )
                    .unwrap();

                cs.enforce(
                    || "mul = fe_1 * fe_2",
                    |lc| lc + fe_1_variable,
                    |lc| lc + (fe_2_constant, CS::one()),
                    |lc| lc + mul_variable.clone(),
                );

                ResolvedValue::FieldElement(FieldElement::Allocated(mul_value, mul_variable))
            }
            (
                FieldElement::Constant(fe_1_constant),
                FieldElement::Allocated(fe_2_value, fe_2_variable),
            ) => {
                let mul_value: Option<F> = fe_2_value.map(|v| fe_1_constant.mul(&v));
                let mul_variable: R1CSVariable = cs
                    .alloc(
                        || "field multiplication",
                        || mul_value.ok_or(SynthesisError::AssignmentMissing),
                    )
                    .unwrap();

                cs.enforce(
                    || "mul = fe_1 * fe_2",
                    |lc| lc + (fe_1_constant, CS::one()),
                    |lc| lc + fe_2_variable,
                    |lc| lc + mul_variable.clone(),
                );

                ResolvedValue::FieldElement(FieldElement::Allocated(mul_value, mul_variable))
            }
            (
                FieldElement::Allocated(fe_1_value, fe_1_variable),
                FieldElement::Allocated(fe_2_value, fe_2_variable),
            ) => {
                let mul_value: Option<F> = match (fe_1_value, fe_2_value) {
                    (Some(fe_1_value), Some(fe_2_value)) => Some(fe_1_value.mul(&fe_2_value)),
                    (_, _) => None,
                };
                let mul_variable: R1CSVariable = cs
                    .alloc(
                        || "field multiplication",
                        || mul_value.ok_or(SynthesisError::AssignmentMissing),
                    )
                    .unwrap();

                cs.enforce(
                    || "mul = fe_1 * fe_2",
                    |lc| lc + fe_1_variable,
                    |lc| lc + fe_2_variable,
                    |lc| lc + mul_variable.clone(),
                );

                ResolvedValue::FieldElement(FieldElement::Allocated(mul_value, mul_variable))
            }
        }
    }

    pub(crate) fn enforce_field_div(
        &mut self,
        cs: &mut CS,
        fe_1: FieldElement<F>,
        fe_2: FieldElement<F>,
    ) -> ResolvedValue<F> {
        match (fe_1, fe_2) {
            // if both constants, then return a constant result
            (FieldElement::Constant(fe_1_constant), FieldElement::Constant(fe_2_constant)) => {
                ResolvedValue::FieldElement(FieldElement::Constant(
                    fe_1_constant.div(&fe_2_constant),
                ))
            }
            // else, return an allocated result
            (
                FieldElement::Allocated(fe_1_value, fe_1_variable),
                FieldElement::Constant(fe_2_constant),
            ) => {
                let div_value: Option<F> = fe_1_value.map(|v| v.div(&fe_2_constant));
                let div_variable: R1CSVariable = cs
                    .alloc(
                        || "field division",
                        || div_value.ok_or(SynthesisError::AssignmentMissing),
                    )
                    .unwrap();
                let fe_2_inverse_value = fe_2_constant.inverse().unwrap();

                cs.enforce(
                    || "div = fe_1 * fe_2^-1",
                    |lc| lc + fe_1_variable,
                    |lc| lc + (fe_2_inverse_value, CS::one()),
                    |lc| lc + div_variable.clone(),
                );

                ResolvedValue::FieldElement(FieldElement::Allocated(div_value, div_variable))
            }
            (
                FieldElement::Constant(fe_1_constant),
                FieldElement::Allocated(fe_2_value, _fe_2_variable),
            ) => {
                let div_value: Option<F> = fe_2_value.map(|v| fe_1_constant.div(&v));
                let div_variable: R1CSVariable = cs
                    .alloc(
                        || "field division",
                        || div_value.ok_or(SynthesisError::AssignmentMissing),
                    )
                    .unwrap();
                let fe_2_inverse_value = fe_2_value.map(|v| v.inverse().unwrap());
                let fe_2_inverse_variable = cs
                    .alloc(
                        || "field inverse",
                        || fe_2_inverse_value.ok_or(SynthesisError::AssignmentMissing),
                    )
                    .unwrap();

                cs.enforce(
                    || "div = fe_1 * fe_2^-1",
                    |lc| lc + (fe_1_constant, CS::one()),
                    |lc| lc + fe_2_inverse_variable,
                    |lc| lc + div_variable.clone(),
                );

                ResolvedValue::FieldElement(FieldElement::Allocated(div_value, div_variable))
            }
            (
                FieldElement::Allocated(fe_1_value, fe_1_variable),
                FieldElement::Allocated(fe_2_value, _fe_2_variable),
            ) => {
                let div_value: Option<F> = match (fe_1_value, fe_2_value) {
                    (Some(fe_1_value), Some(fe_2_value)) => Some(fe_1_value.div(&fe_2_value)),
                    (_, _) => None,
                };
                let div_variable: R1CSVariable = cs
                    .alloc(
                        || "field division",
                        || div_value.ok_or(SynthesisError::AssignmentMissing),
                    )
                    .unwrap();
                let fe_2_inverse_value = fe_2_value.map(|v| v.inverse().unwrap());
                let fe_2_inverse_variable = cs
                    .alloc(
                        || "field inverse",
                        || fe_2_inverse_value.ok_or(SynthesisError::AssignmentMissing),
                    )
                    .unwrap();

                cs.enforce(
                    || "div = fe_1 * fe_2^-1",
                    |lc| lc + fe_1_variable,
                    |lc| lc + fe_2_inverse_variable,
                    |lc| lc + div_variable.clone(),
                );

                ResolvedValue::FieldElement(FieldElement::Allocated(div_value, div_variable))
            }
        }
    }

    pub(crate) fn enforce_field_pow(
        &mut self,
        cs: &mut CS,
        fe_1: FieldElement<F>,
        num: UInt32,
    ) -> ResolvedValue<F> {
        match fe_1 {
            // if both constants, then return a constant result
            FieldElement::Constant(fe_1_constant) => ResolvedValue::FieldElement(
                FieldElement::Constant(fe_1_constant.pow(&[num.value.unwrap() as u64])),
            ),
            // else, return an allocated result
            FieldElement::Allocated(fe_1_value, _fe_1_variable) => {
                let pow_value: Option<F> = fe_1_value.map(|v| v.pow(&[num.value.unwrap() as u64]));
                let pow_variable: R1CSVariable = cs
                    .alloc(
                        || "field exponentiation",
                        || pow_value.ok_or(SynthesisError::AssignmentMissing),
                    )
                    .unwrap();

                // cs.enforce( //todo: find a linear combination for this
                //     || "pow = 1 + fe_1^num",
                //     |lc| lc + fe_1_variable,
                //     |lc| lc + (fe_2_inverse_value, CS::one()),
                //     |lc| lc + pow_variable.clone());

                ResolvedValue::FieldElement(FieldElement::Allocated(pow_value, pow_variable))
            }
        }
    }
}
