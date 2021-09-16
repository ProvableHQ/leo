// Copyright (C) 2019-2021 Aleo Systems Inc.
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

//! Enforces constraints on a function in a compiled Leo program.

use crate::program::Program;

use leo_asg::{Expression, Function, FunctionQualifier};
use leo_errors::CompilerError;
use leo_errors::Result;
use snarkvm_ir::{CallCoreData, CallData, Instruction, Integer, PredicateData, QueryData, Value};
use std::cell::Cell;

impl<'a> Program<'a> {
    pub(crate) fn enforce_function_definition(&mut self, function: &'a Function<'a>) -> Result<()> {
        self.begin_function(function);
        let statement = function
            .body
            .get()
            .expect("attempted to build body for function header");
        self.enforce_statement(statement)?;
        // we are a mut self function with no explicit return
        if function.qualifier == FunctionQualifier::MutSelfRef && function.output.is_unit() {
            let self_var = function
                .scope
                .resolve_variable("self")
                .expect("missing self in mut self function");
            let self_var_register = self.resolve_var(self_var);
            let output = Value::Tuple(vec![Value::Ref(self_var_register), Value::Tuple(vec![])]);
            self.emit(Instruction::Return(PredicateData { values: vec![output] }));
        }
        Ok(())
    }

    pub(crate) fn enforce_function_call(
        &mut self,
        function: &'a Function<'a>,
        target: Option<&'a Expression<'a>>,
        arguments: &[Cell<&'a Expression<'a>>],
    ) -> Result<Value> {
        let target_value = target.map(|target| self.enforce_expression(target)).transpose()?;

        let mut ir_arguments = vec![];

        if let Some(target) = target_value {
            ir_arguments.push(target);
        }

        if function.arguments.len() != arguments.len() {
            return Err(CompilerError::function_input_not_found(
                &function.name.borrow().name.to_string(),
                "arguments length invalid",
                &function.span.clone().unwrap_or_default(),
            )
            .into());
        }

        // Store input values as new variables in resolved program
        for input_expression in arguments.iter() {
            let input_value = self.enforce_expression(input_expression.get())?;

            ir_arguments.push(input_value);
        }

        let output = self.alloc();

        let core_mapping = if let Some(circuit) = function.circuit.get() {
            let core_mapping = circuit.core_mapping.borrow();
            if let Some(core_mapping) = core_mapping.as_deref() {
                Some(core_mapping.to_string())
            } else {
                None
            }
        } else {
            None
        };

        if let Some(core_mapping) = core_mapping {
            self.emit(Instruction::CallCore(CallCoreData {
                destination: output,
                identifier: core_mapping,
                arguments: ir_arguments,
            }));
        } else {
            self.emit(Instruction::Call(CallData {
                destination: output,
                index: self.resolve_function(function),
                arguments: ir_arguments,
            }));
        }

        if function.qualifier == FunctionQualifier::MutSelfRef {
            let target = target.expect("missing target for mut self");
            let out_target = self.alloc();
            self.emit(Instruction::TupleIndexGet(QueryData {
                destination: out_target,
                values: vec![Value::Ref(output), Value::Integer(Integer::U32(0))],
            }));
            self.resolve_mut_ref(target, Value::Ref(out_target))?;
            self.emit(Instruction::TupleIndexGet(QueryData {
                destination: output,
                values: vec![Value::Ref(output), Value::Integer(Integer::U32(1))],
            }));
        }

        Ok(Value::Ref(output))
    }
}
