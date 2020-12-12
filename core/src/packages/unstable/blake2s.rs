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

use crate::{CoreCircuit, CoreCircuitError, Value};

use leo_ast::{
    ArrayDimensions,
    Block,
    CallExpression,
    Circuit,
    CircuitMember,
    Expression,
    Function,
    FunctionInput,
    FunctionInputVariable,
    Identifier,
    IntegerType,
    PositiveNumber,
    ReturnStatement,
    Span,
    Statement,
    Type,
};
use snarkos_gadgets::algorithms::prf::Blake2sGadget;
use snarkos_models::{
    curves::{Field, PrimeField},
    gadgets::{
        algorithms::PRFGadget,
        r1cs::ConstraintSystem,
        utilities::{uint::UInt8, ToBytesGadget},
    },
};

// internal identifier
pub const CORE_UNSTABLE_BLAKE2S_NAME: &str = "#blake2s";
pub const CORE_UNSTABLE_BLAKE2S_PACKAGE_NAME: &str = "Blake2s";

#[derive(Clone, PartialEq, Eq)]
pub struct Blake2sCircuit {}

impl CoreCircuit for Blake2sCircuit {
    fn name() -> String {
        CORE_UNSTABLE_BLAKE2S_NAME.to_owned()
    }

    /* Blake2s circuit ast
     * circuit Blake2s {
     *     static function hash(seed: [u8; 32], message: [u8; 32]) -> [u8; 32] {
     *         // call `check_eval_gadget` in snarkOS
     *         return check_eval_gadget(seed, message)
     *     }
     */
    fn ast(circuit_name: Identifier, span: Span) -> Circuit {
        Circuit {
            circuit_name,
            members: vec![CircuitMember::CircuitFunction(Function {
                identifier: Identifier {
                    name: "hash".to_owned(),
                    span: span.clone(),
                },
                input: vec![
                    FunctionInput::Variable(FunctionInputVariable {
                        identifier: Identifier {
                            name: "seed".to_owned(),
                            span: span.clone(),
                        },
                        mutable: false,
                        type_: Type::Array(
                            Box::new(Type::IntegerType(IntegerType::U8)),
                            ArrayDimensions(vec![PositiveNumber {
                                value: 32usize.to_string(),
                            }]),
                        ),
                        span: span.clone(),
                    }),
                    FunctionInput::Variable(FunctionInputVariable {
                        identifier: Identifier {
                            name: "message".to_owned(),
                            span: span.clone(),
                        },
                        mutable: false,
                        type_: Type::Array(
                            Box::new(Type::IntegerType(IntegerType::U8)),
                            ArrayDimensions(vec![PositiveNumber {
                                value: 32usize.to_string(),
                            }]),
                        ),
                        span: span.clone(),
                    }),
                ],
                output: Some(Type::Array(
                    Box::new(Type::IntegerType(IntegerType::U8)),
                    ArrayDimensions(vec![PositiveNumber {
                        value: 32usize.to_string(),
                    }]),
                )),
                block: Block {
                    statements: vec![Statement::Return(ReturnStatement {
                        expression: Expression::Call(CallExpression {
                            function: Box::new(Expression::Identifier(Identifier::new_with_span(&Self::name(), &span))),
                            arguments: vec![
                                Expression::Identifier(Identifier::new_with_span("seed", &span)),
                                Expression::Identifier(Identifier::new_with_span("message", &span)),
                            ],
                            span: span.clone(),
                        }),
                        span: span.clone(),
                    })],
                    span: span.clone(),
                },
                span,
            })],
        }
    }

    /// Calls the native `Blake2sGadget` on the given constraint system with the given arguments
    fn call<F: Field + PrimeField, CS: ConstraintSystem<F>>(
        mut cs: CS,
        arguments: Vec<Value>,
        span: Span,
    ) -> Result<Vec<Value>, CoreCircuitError> {
        // The blake2s check evaluation gadget has two arguments: seed and input
        let expected_length = 2usize;
        let actual_length = arguments.len();

        if expected_length != actual_length {
            return Err(CoreCircuitError::arguments_length(expected_length, actual_length, span));
        }

        let seed_value = arguments[0].to_owned();
        let input_value = arguments[1].to_owned();

        let seed = check_array_bytes(seed_value, 32, span.clone())?;
        let input = check_array_bytes(input_value, 32, span.clone())?;

        // Call blake2s gadget
        let digest =
            Blake2sGadget::check_evaluation_gadget(cs.ns(|| "blake2s hash"), &seed[..], &input[..]).map_err(|e| {
                CoreCircuitError::cannot_enforce("Blake2s check evaluation gadget".to_owned(), e, span.clone())
            })?;

        // Convert digest to bytes
        let bytes = digest
            .to_bytes(cs)
            .map_err(|e| CoreCircuitError::cannot_enforce("Vec<UInt8> ToBytes".to_owned(), e, span.clone()))?;

        let return_value = bytes.into_iter().map(Value::U8).collect();

        // Return one array digest value
        Ok(vec![Value::Array(return_value)])
    }
}

fn check_array_bytes(value: Value, size: usize, span: Span) -> Result<Vec<UInt8>, CoreCircuitError> {
    let array_value = match value {
        Value::Array(array) => array,
        value => return Err(CoreCircuitError::invalid_array(value, span)),
    };

    if size != array_value.len() {
        return Err(CoreCircuitError::array_length(size, array_value.len(), span));
    }

    let mut array_bytes = Vec::with_capacity(array_value.len());

    for value in array_value {
        let byte = match value {
            Value::U8(u8) => u8,
            value => return Err(CoreCircuitError::invalid_array_bytes(value, span)),
        };

        array_bytes.push(byte)
    }

    Ok(array_bytes)
}

#[cfg(test)]
mod tests {
    use super::*;
    use snarkos_curves::bls12_377::Fr;
    use snarkos_models::gadgets::{
        r1cs::TestConstraintSystem,
        utilities::{boolean::Boolean, uint::UInt8},
    };

    #[test]
    fn test_call_arguments_length_fail() {
        let cs = TestConstraintSystem::<Fr>::new();

        let seed = Value::Array(vec![]);
        let dummy_span = Span {
            text: "".to_string(),
            line: 0,
            start: 0,
            end: 0,
        };

        let err = Blake2sCircuit::call(cs, vec![seed], dummy_span.clone()).err();

        assert!(err.is_some());

        let expected = CoreCircuitError::arguments_length(2, 1, dummy_span);
        let actual = err.unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_array_length_fail() {
        let cs = TestConstraintSystem::<Fr>::new();

        let seed = Value::Array(vec![]);
        let input = Value::Array(vec![]);
        let dummy_span = Span {
            text: "".to_string(),
            line: 0,
            start: 0,
            end: 0,
        };

        let err = Blake2sCircuit::call(cs, vec![seed, input], dummy_span.clone()).err();

        assert!(err.is_some());

        let expected = CoreCircuitError::array_length(32, 0, dummy_span);
        let actual = err.unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_invalid_array() {
        let cs = TestConstraintSystem::<Fr>::new();

        let seed = Value::U8(UInt8::constant(0));
        let input = Value::Array(vec![]);
        let dummy_span = Span {
            text: "".to_string(),
            line: 0,
            start: 0,
            end: 0,
        };

        let err = Blake2sCircuit::call(cs, vec![seed.clone(), input], dummy_span.clone()).err();

        assert!(err.is_some());

        let expected = CoreCircuitError::invalid_array(seed, dummy_span);
        let actual = err.unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_invalid_array_bytes() {
        let cs = TestConstraintSystem::<Fr>::new();

        let invalid_byte = Value::Boolean(Boolean::Constant(true));
        let seed = Value::Array(vec![invalid_byte.clone(); 32]);
        let input = Value::Array(vec![Value::U8(UInt8::constant(0)); 32]);
        let dummy_span = Span {
            text: "".to_string(),
            line: 0,
            start: 0,
            end: 0,
        };

        let err = Blake2sCircuit::call(cs, vec![seed, input], dummy_span.clone()).err();

        assert!(err.is_some());

        let expected = CoreCircuitError::invalid_array_bytes(invalid_byte, dummy_span);
        let actual = err.unwrap();

        assert_eq!(expected, actual);
    }
}
