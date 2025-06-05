// Copyright (C) 2019-2025 Provable Inc.
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

use super::*;
use interpreter::Interpreter;

use leo_ast::{
    CoreFunction,
    Expression,
    Node as _,
    halt,
    interpreter_value::{ExpectTc as _, Plaintext},
};
use leo_errors::{InterpreterHalt, Result};
use leo_span::{Symbol, sym};

use rand::Rng as _;

impl<'a> Interpreter<'a> {
    pub fn step_expression(&mut self) -> Result<()> {
        let Some(function_call) = self.cursor.function_call_stack.last_mut() else {
            panic!("Precondition violated: function call must exist.");
        };

        let cursor::FunctionCall::Leo(leo_function_call) = function_call else {
            panic!("Precondition violated: function call must be a Leo call.")
        };

        let Some(statement_frame) = leo_function_call.statement_frames.last_mut() else {
            panic!("Precondition violated: a statement frame must exist.");
        };

        let Some(expression_frame) = statement_frame.expression_frames.last_mut() else {
            panic!("Precondition violated: an expression frame must exist.");
        };

        let step = expression_frame.step;
        expression_frame.step += 1;

        let span = expression_frame.expression.span();

        match expression_frame.expression {
            Expression::ArrayAccess(access) if step == 0 => {
                // Step 0: Push the array and index expressions.
                statement_frame.expression_frames.push((&access.index).into());
                statement_frame.expression_frames.push((&access.array).into());
            }
            Expression::ArrayAccess(_access) if step == 1 => {
                // Step 1: Pop the values and access the array.
                assert!(statement_frame.values.len() >= 2);
                let index = statement_frame.values.pop().unwrap();
                let array = statement_frame.values.pop().unwrap();
                let index = index.try_as_usize().expect_tc(span)?;
                match array {
                    LeoValue::Value(snarkvm::prelude::Value::Plaintext(snarkvm::prelude::Plaintext::Array(
                        mut array,
                        _,
                    ))) => {
                        let plaintext = array.swap_remove(index);
                        statement_frame.expression_frames.pop();
                        statement_frame.values.push(plaintext.into());
                    }
                    _ => halt!(span, "type error"),
                }
            }
            Expression::ArrayAccess(_access) => panic!("Can't happen"),

            Expression::AssociatedConstant(_constant) if step == 0 => {
                // Step 0: Generate the value and push it. (Only one associated constant right now.)
                statement_frame.expression_frames.pop();
                let value = LeoValue::generator();
                statement_frame.values.push(value);
            }
            Expression::AssociatedConstant(_constant) => panic!("Can't happen"),

            Expression::AssociatedFunction(associated_function) if step == 0 => {
                // Step 0: Push all the arguments to be evaluated.
                let Some(core_function) =
                    CoreFunction::from_symbols(associated_function.variant.name, associated_function.name.name)
                else {
                    halt!(span, "invalid core function");
                };
                if associated_function.arguments.len() != core_function.num_args() {
                    halt!(span, "Incorrect number of arguments");
                }

                let mut push = |expr: &'a Expression| statement_frame.expression_frames.push(expr.into());

                // We want to push expressions for each of the arguments... except for mappings,
                // because we don't look them up as Values.
                match core_function {
                    CoreFunction::MappingGet | CoreFunction::MappingRemove | CoreFunction::MappingContains => {
                        push(&associated_function.arguments[1]);
                    }
                    CoreFunction::MappingGetOrUse | CoreFunction::MappingSet => {
                        push(&associated_function.arguments[2]);
                        push(&associated_function.arguments[1]);
                    }
                    CoreFunction::CheatCodePrintMapping => {
                        // Do nothing, as we don't need to evaluate the mapping.
                    }
                    _ => associated_function.arguments.iter().rev().for_each(push),
                }
            }
            Expression::AssociatedFunction(associated_function) if step == 1 => {
                // Step 1: Evaluate.
                statement_frame.expression_frames.pop();
                let Some(core_function) =
                    CoreFunction::from_symbols(associated_function.variant.name, associated_function.name.name)
                else {
                    halt!(span, "invalid core function");
                };
                if let CoreFunction::FutureAwait = core_function {
                    todo!()
                } else {
                    let value = leo_ast::interpreter_value::evaluate_core_function(
                        &mut self.cursor,
                        core_function,
                        &associated_function.arguments,
                        associated_function.span(),
                    )?;
                    assert!(value.is_some());

                    // We had to release our borrow of `leo_function_call` in order to pass `&mut self.cursor`,
                    // so get it back.
                    let Some(cursor::FunctionCall::Leo(leo_function_call)) = self.cursor.function_call_stack.last_mut()
                    else {
                        panic!("Can't happen.");
                    };
                    let Some(statement_frame) = leo_function_call.statement_frames.last_mut() else {
                        panic!("Can't happen.");
                    };
                    statement_frame.values.push(value.unwrap());
                }
            }
            Expression::AssociatedFunction(_associated_function) => panic!("Can't happen"),

            Expression::Array(array_expression) if step == 0 => {
                // Step 0: Push the elements.
                statement_frame
                    .expression_frames
                    .extend(array_expression.elements.iter().rev().map(|element| element.into()));
            }
            Expression::Array(array_expression) if step == 1 => {
                // Step 1: Pop the values, put them into an array, push the array.
                statement_frame.expression_frames.pop();
                let count = array_expression.elements.len();
                let start = statement_frame.values.len() - count;
                let Some(array) = LeoValue::try_make_array(statement_frame.values.drain(start..)) else {
                    halt!(span, "type error");
                };
                statement_frame.values.push(array);
            }
            Expression::Array(_array_expression) => panic!("Can't happen"),

            Expression::Binary(binary_expression) if step == 0 => {
                // Step 0: Push the operands.
                statement_frame.expression_frames.push((&binary_expression.right).into());
                statement_frame.expression_frames.push((&binary_expression.left).into());
            }
            Expression::Binary(binary_expression) if step == 1 => {
                // Step 1: Pop the operands, evaluate, push the result.
                statement_frame.expression_frames.pop();
                let rhs = statement_frame.values.pop().unwrap();
                let lhs = statement_frame.values.pop().unwrap();
                let value = leo_ast::interpreter_value::evaluate_binary(
                    binary_expression.span(),
                    binary_expression.op,
                    &lhs,
                    &rhs,
                )?;
                statement_frame.values.push(value);
            }
            Expression::Binary(_binary_expression) => panic!("Can't happen"),

            Expression::Call(call_expression) if step == 0 => {
                // Step 0: Push the arguments.
                statement_frame
                    .expression_frames
                    .extend(call_expression.arguments.iter().rev().map(|expr| expr.into()));
            }
            Expression::Call(call_expression) if step == 1 => {
                // Step 1: Pop the arguments and initiate the call.
                let count = call_expression.arguments.len();
                let start = statement_frame.values.len() - count;
                let program = call_expression.program.unwrap_or(leo_function_call.program);
                let Expression::Identifier(name) = call_expression.function else {
                    halt!(span, "type error");
                };
                let location = Location::new(program, name.name);
                let arguments: Vec<LeoValue> = statement_frame.values.drain(start..).collect();
                self.initiate_function(location, arguments)?;
            }
            Expression::Call(_call_expression) if step == 2 => {
                // Step 2: Push the returned value.
                statement_frame.expression_frames.pop();
                let Some(value) = self.cursor.last_return_value.take() else {
                    panic!("Can't happen");
                };
                statement_frame.values.push(value);
            }
            Expression::Call(_call_expression) => panic!("Can't happen"),

            Expression::Cast(cast_expression) if step == 0 => {
                // Step 0: Push the expression.
                statement_frame.expression_frames.push((&cast_expression.expression).into());
            }
            Expression::Cast(cast_expression) if step == 1 => {
                // Step 1: Pop the expression, do the cast, push the result.
                statement_frame.expression_frames.pop();
                let value = statement_frame.values.pop().unwrap();
                let Some(casted) = value.cast(&cast_expression.type_) else {
                    halt!(span, "cast failure");
                };
                statement_frame.values.push(casted);
            }
            Expression::Cast(_cast_expression) => panic!("Can't happen"),

            Expression::Err(_err) => panic!("Can't happen"),

            Expression::Identifier(identifier) if step == 0 => {
                // Step 0: Find the value and push it.
                statement_frame.expression_frames.pop();
                let Some(value) = leo_function_call.names.get(&identifier.name).or_else(|| {
                    let location = Location::new(leo_function_call.program, identifier.name);
                    self.cursor.globals.get(&location)
                }) else {
                    halt!(span, "Unknown identifier");
                };
                statement_frame.values.push(value.clone());
            }
            Expression::Identifier(..) => panic!("Can't happen"),

            Expression::Literal(literal) if step == 0 => {
                // Step 0: Translate the value and push it.
                statement_frame.expression_frames.pop();
                let ty = self.type_table.get(&literal.id());
                let value = leo_ast::interpreter_value::literal_to_value(literal, &ty)?;
                statement_frame.values.push(value);
            }
            Expression::Literal(..) => panic!("Can't happend"),

            Expression::Locator(_locator_expression) => todo!(),

            Expression::MemberAccess(access) if step == 0 => {
                // Step 0.
                'block: {
                    // If it's self.signer, self.caller or block.height, just get that.
                    if let Expression::Identifier(id) = &access.inner {
                        let value = match (id.name, access.name.name) {
                            (sym::SelfLower, sym::signer) => self.cursor.signer.into(),
                            (sym::SelfLower, sym::caller) => leo_function_call.caller.into(),
                            (sym::block, sym::height) => self.cursor.block_height.into(),
                            _ => break 'block,
                        };

                        statement_frame.expression_frames.pop();
                        statement_frame.values.push(value);
                        return Ok(());
                    }
                }

                // Otherwise, push the inner expression.
                statement_frame.expression_frames.push((&access.inner).into());
            }
            Expression::MemberAccess(member_access) if step == 1 => {
                // Step 1: Pop the struct, access the member, push the value.
                statement_frame.expression_frames.pop();
                let struct_ = statement_frame.values.pop().unwrap();
                let Some(plaintext) = struct_.member_get(member_access.name.name) else {
                    halt!(span, "Unknown struct member");
                };
                statement_frame.values.push(plaintext.into());
            }
            Expression::MemberAccess(_member_access) => panic!("Can't happen"),

            Expression::Struct(struct_expression) if step == 0 => {
                // Step 0: Push all the initializers.
                let initializer_expressions = struct_expression
                    .members
                    .iter()
                    .rev()
                    // Skip the ones with only Identifiers - we'll get those next step.
                    .filter_map(|initializer| initializer.expression.as_ref().map(|expr| expr.into()));
                statement_frame.expression_frames.extend(initializer_expressions);
            }
            Expression::Struct(struct_expression) if step == 1 => {
                // Step 1: Retrieve the values, build the struct, push it.
                statement_frame.expression_frames.pop();
                let mut values = IndexMap::<Symbol, LeoValue>::new();
                for member in struct_expression.members.iter().rev() {
                    let value = if member.expression.is_some() {
                        // We pushed this in the last step.
                        statement_frame.values.pop().unwrap()
                    } else {
                        // We just have the `Identifier`, so look it up.
                        let Some(value) = leo_function_call.names.get(&member.identifier.name).or_else(|| {
                            let location = Location::new(leo_function_call.program, member.identifier.name);
                            self.cursor.globals.get(&location)
                        }) else {
                            halt!(span, "Unknown identifier");
                        };
                        value.clone()
                    };

                    values.insert(member.identifier.name, value);
                }

                let Some(composite) = self.symbol_table.lookup_struct(struct_expression.name.name).or_else(|| {
                    self.symbol_table
                        .lookup_record(Location::new(leo_function_call.program, struct_expression.name.name))
                }) else {
                    halt!(span, "Unknown struct or record");
                };

                // The values must be inserted in the order in which they appear
                // in the definition.
                let mut ordered_values: IndexMap<Symbol, Plaintext> = composite
                    .members
                    .iter()
                    .filter_map(|member| {
                        let symbol = member.identifier.name;
                        let value = values.swap_remove(&symbol)?;
                        let plaintext: Plaintext = value.try_into().ok()?;
                        Some((symbol, plaintext))
                    })
                    .collect();

                if ordered_values.len() != composite.members.len() {
                    halt!(span, "Incorrect number of members");
                }

                let value = if composite.is_record {
                    let Some(owner) = ordered_values.shift_remove(&sym::owner) else {
                        halt!(span, "Missing owner member");
                    };
                    let owner_value: LeoValue = owner.into();
                    let Ok(owner_address) = owner_value.try_into() else {
                        halt!(span, "type error");
                    };
                    LeoValue::record(self.cursor.rng.r#gen(), owner_address, ordered_values.into_iter()).expect("NO")
                } else {
                    LeoValue::struct_plaintext(ordered_values.into_iter()).expect("NO")
                };

                statement_frame.values.push(value);
            }
            Expression::Struct(_struct_expression) => panic!("Can't happen"),

            Expression::Ternary(ternary_expression) if step == 0 => {
                // Step 0: Push the condition.
                statement_frame.expression_frames.push((&ternary_expression.condition).into());
            }
            Expression::Ternary(ternary_expression) if step == 1 => {
                // Step 1: Check the condition and push one of the branch expressions.
                statement_frame.expression_frames.pop();
                let value = statement_frame.values.pop().unwrap();
                let condition = value.try_into().expect("NO");
                let branch = if condition { &ternary_expression.if_true } else { &ternary_expression.if_false };
                statement_frame.expression_frames.push(branch.into());
            }
            Expression::Ternary(_ternary_expression) => panic!("Can't happen"),

            Expression::Tuple(tuple_expression) if step == 0 => {
                // Step 0: Push the elements.
                statement_frame
                    .expression_frames
                    .extend(tuple_expression.elements.iter().rev().map(|element| element.into()));
            }
            Expression::Tuple(tuple_expression) if step == 1 => {
                // Step 1: Pop the elements, put them in a tuple, push it.
                statement_frame.expression_frames.pop();
                let count = tuple_expression.elements.len();
                let start = statement_frame.values.len() - count;
                let Some(value) = LeoValue::try_make_tuple(statement_frame.values.drain(start..)) else {
                    panic!("NO");
                };
                statement_frame.values.push(value);
            }
            Expression::Tuple(_tuple_expression) => panic!("Can't happen"),

            Expression::TupleAccess(access) if step == 0 => {
                // Step 0: Push the tuple.
                statement_frame.expression_frames.push((&access.tuple).into());
            }
            Expression::TupleAccess(access) if step == 1 => {
                // Step 1: Pop the tuple, extract the element, push it.
                statement_frame.expression_frames.pop();
                let value = statement_frame.values.pop().unwrap();
                let LeoValue::Tuple(mut tuple) = value else {
                    panic!("NO");
                };
                let member = tuple.swap_remove(access.index.value());
                statement_frame.values.push(member.into());
                access.index.value();
            }
            Expression::TupleAccess(_access) => panic!("Can't happend"),

            Expression::Unary(unary_expression) if step == 0 => {
                // Step 0: Push the receiver.
                statement_frame.expression_frames.push((&unary_expression.receiver).into());
            }
            Expression::Unary(unary_expression) if step == 1 => {
                // Step 1: Pop the receiver value, apply the operation, and push the result.
                statement_frame.expression_frames.pop();
                let receiver = statement_frame.values.pop().unwrap();
                let value = leo_ast::interpreter_value::evaluate_unary(
                    unary_expression.span(),
                    unary_expression.op,
                    &receiver,
                )?;
                statement_frame.values.push(value);
            }
            Expression::Unary(_unary_expression) => panic!("Can't happen"),

            Expression::Unit(..) if step == 0 => {
                // Step 0: Just push a unit.
                statement_frame.expression_frames.pop();
                statement_frame.values.push(LeoValue::Unit);
            }
            Expression::Unit(..) => panic!("Can't happen"),
        }

        Ok(())
    }
}
