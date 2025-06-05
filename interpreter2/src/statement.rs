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
use cursor::BlockOrStatement;
use interpreter::Interpreter;

use leo_ast::{
    AssertVariant,
    DefinitionPlace,
    Statement,
    interpreter_value::{LeoValue, Literal, SvmLiteralParam, TryAsRef as _},
};

use std::mem;

impl<'a> Interpreter<'a> {
    pub fn step_statement(&mut self) -> Result<()> {
        let Some(function_call) = self.cursor.function_call_stack.last_mut() else {
            panic!("NO");
        };

        let cursor::FunctionCall::Leo(leo_function_call) = function_call else {
            panic!("NO");
        };

        let Some(statement_frame) = leo_function_call.statement_frames.last_mut() else {
            panic!("NO");
        };

        macro_rules! do_block {
            ($block: expr) => {{
                let new_statement_frames = $block.statements.iter().rev().map(|stmt| stmt.into());
                leo_function_call.statement_frames.pop();
                leo_function_call.statement_frames.extend(new_statement_frames);
            }};
        }

        let step = statement_frame.step;
        statement_frame.step += 1;

        let finish = |slf: &mut Self| {
            if let Some(cursor::FunctionCall::Leo(leo_function_call)) = slf.cursor.function_call_stack.last_mut() {
                if leo_function_call.statement_frames.is_empty() {
                    // We ran out of statements and didn't hit a `return`.
                    slf.cursor.function_call_stack.pop();
                    slf.cursor.last_return_value = Some(LeoValue::Unit);
                }
            }
            Ok(())
        };

        let statement = match statement_frame.statement {
            BlockOrStatement::Block(block) if step == 0 => {
                do_block!(block);
                return finish(self);
            }
            BlockOrStatement::Block(_block) => panic!("NO"),
            BlockOrStatement::Statement(statement) => statement,
        };

        use Statement::*;

        match statement {
            Assert(stmt) if step == 0 => {
                // Step 0: Push the expressions.

                let mut push = |expression: &'a leo_ast::Expression| {
                    statement_frame.expression_frames.push(expression.into());
                };

                match &stmt.variant {
                    AssertVariant::Assert(expr) => push(expr),
                    AssertVariant::AssertEq(expr0, expr1) | AssertVariant::AssertNeq(expr0, expr1) => {
                        push(expr1);
                        push(expr0);
                    }
                }
            }
            Assert(stmt) if step == 1 => {
                // Step 1: Pop the values and check the condition.

                match &stmt.variant {
                    AssertVariant::Assert(..) => {
                        let val = statement_frame.values.pop().unwrap();
                        match val.try_into() {
                            Ok(true) => {}
                            Ok(false) => panic!("HALT"),
                            Err(..) => panic!("HALT"),
                        }
                    }
                    AssertVariant::AssertEq(..) => {
                        let val0 = statement_frame.values.pop().unwrap();
                        let val1 = statement_frame.values.pop().unwrap();
                        if val0.neq(&val1)? {
                            panic!("HALT");
                        }
                    }
                    AssertVariant::AssertNeq(..) => {
                        let val0 = statement_frame.values.pop().unwrap();
                        let val1 = statement_frame.values.pop().unwrap();
                        if val0.eq(&val1)? {
                            panic!("HALT");
                        }
                    }
                }

                // Get rid of the frame.
                leo_function_call.statement_frames.pop();
            }
            Assert(_stmt) => panic!("NO"),

            Assign(stmt) if step == 0 => {
                // Step 0: push the expression frame and any array index expression frames.
                assert!(statement_frame.expression_frames.is_empty());
                statement_frame.expression_frames.push((&stmt.value).into());
                let mut place = &stmt.place;
                loop {
                    match place {
                        leo_ast::Expression::ArrayAccess(access) => {
                            statement_frame.expression_frames.push((&access.index).into());
                            place = &access.array;
                        }
                        leo_ast::Expression::Identifier(..) => break,
                        leo_ast::Expression::MemberAccess(access) => {
                            place = &access.inner;
                        }
                        leo_ast::Expression::TupleAccess(access) => {
                            place = &access.tuple;
                        }
                        _ => panic!("NO"),
                    }
                }
            }
            Assign(_stmt) if step == 1 => {
                // Step 1: evaluate the expression frames.
                for _ in 0..statement_frame.expression_frames.len() {
                    self.finish_expression()?;
                }
            }
            Assign(stmt) if step == 2 => {
                // Step 2: set the variable (or place).

                assert!(statement_frame.expression_frames.is_empty());
                assert!(statement_frame.values.len() > 0);

                // All we need from this frame is the values.
                let mut values = mem::take(&mut statement_frame.values);

                // Get the value.
                let value = values.pop().unwrap();

                // Make the assignment
                leo_function_call.assign(value, &stmt.place, &mut values.into_iter())?;

                // Get rid of the frame.
                leo_function_call.statement_frames.pop();
            }
            Assign(_stmt) => panic!("NO"),

            Block(block) if step == 0 => do_block!(block),
            Block(_block) => panic!("NO"),

            Conditional(stmt) if step == 0 => {
                // Step 0: push the conditional expression frame.
                assert!(statement_frame.expression_frames.is_empty());
                statement_frame.expression_frames.push((&stmt.condition).into());
            }
            Conditional(_stmt) if step == 1 => {
                // Step 1: evaluate the expression frame.
                for _ in 0..statement_frame.expression_frames.len() {
                    self.finish_expression()?;
                }
            }
            Conditional(stmt) if step == 2 => {
                // Step 2: If appropriate, push `then` or `otherwise`, and we're done.
                assert!(statement_frame.expression_frames.is_empty());
                assert_eq!(statement_frame.values.len(), 1);
                let value = statement_frame.values.pop().unwrap();
                let Ok(b) = value.try_into() else {
                    panic!("NO");
                };

                leo_function_call.statement_frames.pop();

                if b {
                    // Condition was true - push `then`.
                    leo_function_call.statement_frames.push((&stmt.then).into());
                } else if let Some(otherwise) = &stmt.otherwise {
                    // Condition was false and we've got an `otherwise` - push it.
                    leo_function_call.statement_frames.push((&**otherwise).into());
                }
            }
            Conditional(_stmt) => panic!("NO"),

            Const(stmt) if step == 0 => {
                // Step 0: push the expression frame onto the stack.
                assert!(statement_frame.expression_frames.is_empty());
                statement_frame.expression_frames.push((&stmt.value).into());
            }
            Const(_stmt) if step == 1 => {
                // Step 1: evaluate the expression frame.
                for _ in 0..statement_frame.expression_frames.len() {
                    self.finish_expression()?;
                }
            }
            Const(stmt) if step == 0 => {
                // Step 2 - set the variable value.
                assert!(statement_frame.expression_frames.is_empty());
                assert_eq!(statement_frame.values.len(), 1);
                let value = statement_frame.values.pop().unwrap();
                leo_function_call.names.insert(stmt.place.name, value);
                leo_function_call.statement_frames.pop();
            }
            Const(..) => panic!("NO"),

            Definition(stmt) if step == 0 => {
                // Step 0: push the expression frame onto the stack.
                assert!(statement_frame.expression_frames.is_empty());
                statement_frame.expression_frames.push((&stmt.value).into());
            }
            Definition(_stmt) if step == 1 => {
                // Step 1: evaluate the expression frame.
                for _ in 0..statement_frame.expression_frames.len() {
                    self.finish_expression()?;
                }
            }
            Definition(stmt) if step == 2 => {
                // Step 2 - set the variable value(s).
                assert!(statement_frame.expression_frames.is_empty());
                assert_eq!(statement_frame.values.len(), 1);
                let value = statement_frame.values.pop().unwrap();
                match &stmt.place {
                    DefinitionPlace::Single(identifier) => {
                        leo_function_call.names.insert(identifier.name, value);
                    }
                    DefinitionPlace::Multiple(vec) => {
                        let LeoValue::Tuple(tuple) = value else {
                            panic!("NO");
                        };
                        if tuple.len() != vec.len() {
                            panic!("NO");
                        }
                        for (id, value) in vec.iter().zip(tuple) {
                            leo_function_call.names.insert(id.name, value.into());
                        }
                    }
                }
                leo_function_call.statement_frames.pop();
            }
            Definition(..) => panic!("NO"),

            Expression(stmt) if step == 0 => {
                // Step 0: push the expression frame.
                assert!(statement_frame.expression_frames.is_empty());
                statement_frame.expression_frames.push((&stmt.expression).into());
            }
            Expression(_stmt) if step == 1 => {
                // Step 1: evaluate the expression frame and end.
                for _ in 0..statement_frame.expression_frames.len() {
                    self.finish_expression()?;
                }
            }
            Expression(_stmt) if step == 2 => {
                // Step 2 - we're done.
                assert!(statement_frame.expression_frames.is_empty());
                assert_eq!(statement_frame.values.len(), 1);
                leo_function_call.statement_frames.pop();
            }
            Expression(..) => panic!("NO"),

            Iteration(stmt) if step == 0 => {
                // Step 0: push the expression frames for stop and start.
                assert!(statement_frame.expression_frames.is_empty());
                statement_frame.expression_frames.push((&stmt.stop).into());
                statement_frame.expression_frames.push((&stmt.start).into());
            }
            Iteration(_stmt) if step == 1 => {
                // Step 1: evaluate the expression frames.
                for _ in 0..statement_frame.expression_frames.len() {
                    self.finish_expression()?;
                }
            }
            Iteration(stmt) => {
                // Step n: end or set the loop variable and push one iteration frame.
                assert_eq!(statement_frame.values.len(), 2);
                let start = &statement_frame.values[0];
                let stop = &statement_frame.values[1];
                match start.gte(stop) {
                    Err(..) => panic!("NO"),
                    Ok(true) => panic!("NO"),
                    Ok(false) => {}
                }

                let i = step - 2;

                let Some(start_literal): Option<&Literal> = start.try_as_ref() else {
                    panic!("NO");
                };

                let Some(stop_literal): Option<&Literal> = stop.try_as_ref() else {
                    panic!("NO");
                };

                macro_rules! compute {
                    ($start: expr, $stop: expr, $ty: ty) => {{ (($start).wrapping_add(i as $ty).into(), (**$stop - **$start) as usize) }};
                }

                let (new_value, difference) = match (start_literal, stop_literal) {
                    (SvmLiteralParam::I8(start), SvmLiteralParam::I8(stop)) => {
                        compute!(start, stop, i8)
                    }
                    (SvmLiteralParam::I16(start), SvmLiteralParam::I16(stop)) => {
                        compute!(start, stop, i16)
                    }
                    (SvmLiteralParam::I32(start), SvmLiteralParam::I32(stop)) => {
                        compute!(start, stop, i32)
                    }
                    (SvmLiteralParam::I64(start), SvmLiteralParam::I64(stop)) => {
                        compute!(start, stop, i64)
                    }
                    (SvmLiteralParam::I128(start), SvmLiteralParam::I128(stop)) => {
                        compute!(start, stop, i128)
                    }
                    (SvmLiteralParam::U8(start), SvmLiteralParam::U8(stop)) => {
                        compute!(start, stop, u8)
                    }
                    (SvmLiteralParam::U16(start), SvmLiteralParam::U16(stop)) => {
                        compute!(start, stop, u16)
                    }
                    (SvmLiteralParam::U32(start), SvmLiteralParam::U32(stop)) => {
                        compute!(start, stop, u32)
                    }
                    (SvmLiteralParam::U64(start), SvmLiteralParam::U64(stop)) => {
                        compute!(start, stop, u64)
                    }
                    (SvmLiteralParam::U128(start), SvmLiteralParam::U128(stop)) => {
                        compute!(start, stop, u128)
                    }
                    _ => panic!("NO"),
                };

                if i > difference || (!stmt.inclusive && i == difference) {
                    // We're done.
                    leo_function_call.statement_frames.pop();
                } else {
                    // Set the value of the iterator.
                    leo_function_call.names.insert(stmt.variable.name, new_value);

                    // Push the block statement frame.
                    leo_function_call.statement_frames.push((&stmt.block).into());
                }
            }

            Return(stmt) if step == 0 => {
                // Step 0: push the expression frame.
                assert!(statement_frame.expression_frames.is_empty());
                statement_frame.expression_frames.push((&stmt.expression).into());
            }
            Return(..) if step == 1 => {
                // Step 1: evaluate the expression frame.
                for _ in 0..statement_frame.expression_frames.len() {
                    self.finish_expression()?;
                }
            }
            Return(..) if step == 2 => {
                // Step 2: Get the return value and exit the function.
                assert!(statement_frame.expression_frames.is_empty());
                assert_eq!(statement_frame.values.len(), 1);

                let value = statement_frame.values.pop().unwrap();
                self.cursor.function_call_stack.pop();
                self.cursor.last_return_value = Some(value);
            }
            Return(..) => panic!("NO"),
        }

        finish(self)
    }
}
