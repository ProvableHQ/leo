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

use crate::{CoreCircuit, Value};

use leo_typed::{
    Circuit,
    CircuitMember,
    Expression,
    Function,
    FunctionInput,
    Identifier,
    InputVariable,
    IntegerType,
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

#[derive(Clone, PartialEq, Eq)]
pub struct Blake2sFunction {}

impl CoreCircuit for Blake2sFunction {
    /* Hardcode blake2s circuit ast
     * circuit Blake2s {
     *     static function hash(seed: [u8; 32], message: [u8; 32]) -> [u8; 32] {
     *         // call `check_eval_gadget` in snarkOS
     *         return check_eval_gadget(seed, message)
     *     }
     */
    fn ast(circuit_name: Identifier, span: Span) -> Circuit {
        Circuit {
            circuit_name,
            members: vec![CircuitMember::CircuitFunction(
                true, // static function
                Function {
                    identifier: Identifier {
                        name: "hash".to_owned(),
                        span: span.clone(),
                    },
                    input: vec![
                        InputVariable::FunctionInput(FunctionInput {
                            identifier: Identifier {
                                name: "seed".to_owned(),
                                span: span.clone(),
                            },
                            mutable: false,
                            type_: Type::Array(Box::new(Type::IntegerType(IntegerType::U8)), vec![32usize]),
                            span: span.clone(),
                        }),
                        InputVariable::FunctionInput(FunctionInput {
                            identifier: Identifier {
                                name: "message".to_owned(),
                                span: span.clone(),
                            },
                            mutable: false,
                            type_: Type::Array(Box::new(Type::IntegerType(IntegerType::U8)), vec![32usize]),
                            span: span.clone(),
                        }),
                    ],
                    returns: Some(Type::Array(Box::new(Type::IntegerType(IntegerType::U8)), vec![32usize])),
                    statements: vec![Statement::Return(
                        Expression::CoreFunctionCall(
                            "core_blake2s_unstable".to_owned(),
                            vec![
                                Expression::Identifier(Identifier {
                                    name: "seed".to_owned(),
                                    span: span.clone(),
                                }),
                                Expression::Identifier(Identifier {
                                    name: "message".to_owned(),
                                    span: span.clone(),
                                }),
                            ],
                            span.clone(),
                        ),
                        span.clone(),
                    )],
                    span: span.clone(),
                },
            )],
        }
    }

    fn call<F: Field + PrimeField, CS: ConstraintSystem<F>>(
        mut cs: CS,
        arguments: Vec<Value>,
        _span: Span, // todo: return errors using `leo-typed` span
    ) -> Vec<Value> {
        // The check evaluation gadget should have two arguments: seed and input
        if arguments.len() != 2 {
            unimplemented!("incorrect number of arguments")
        }

        let seed_value = arguments[0].to_owned();
        let input_value = arguments[1].to_owned();

        let seed = check_array_bytes(seed_value, 32);
        let input = check_array_bytes(input_value, 32);

        let res = Blake2sGadget::check_evaluation_gadget(cs.ns(|| "blake2s hash"), &seed[..], &input[..]).unwrap();
        let bytes = res.to_bytes(cs).unwrap();
        let return_value = bytes.into_iter().map(|byte| Value::U8(byte)).collect();

        // Return one array digest value
        vec![Value::Array(return_value)]
    }
}

fn check_array_bytes(value: Value, size: usize) -> Vec<UInt8> {
    let array_value = match value {
        Value::Array(array) => array,
        _ => unimplemented!("expected array value"),
    };

    if array_value.len() != size {
        unimplemented!("expected array size of {}", size)
    }

    let mut array_bytes = vec![];

    for value in array_value {
        let byte = match value {
            Value::U8(u8) => u8,
            _ => unimplemented!("expected u8 byte"),
        };

        array_bytes.push(byte)
    }

    array_bytes
}
