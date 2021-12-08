// Copyright (C) 2019-2022 Aleo Systems Inc.
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

//! Enforces constraints on the main function of a compiled Leo program.

use crate::program::Program;
use leo_errors::CompilerError;

use leo_asg::{Expression, Function, FunctionQualifier, InputCategory};
use leo_errors::Result;
use std::cell::Cell;

impl<'a> Program<'a> {
    pub fn enforce_main_function(&mut self, function: &'a Function<'a>, input: &leo_ast::Input) -> Result<()> {
        // Iterate over main function input variables and allocate new values
        let asg_input = function.scope.resolve_input();

        if let Some(asg_input) = asg_input {
            let output_value =
                self.allocate_input_keyword(&function.name.borrow().span, asg_input.container_circuit, input)?;
            self.alloc_var(asg_input.container);
            self.store(asg_input.container, output_value);
        }

        match function.qualifier {
            FunctionQualifier::SelfRef | FunctionQualifier::ConstSelfRef | FunctionQualifier::MutSelfRef => {
                unimplemented!("cannot access self variable in main function")
            }
            FunctionQualifier::Static => (),
        }

        let mut arguments = vec![];

        for (_, input_variable) in function.arguments.iter() {
            let name = input_variable.get().borrow().name.name;
            if matches!((input.get(name), input.get_constant(name)), (Some(_), Some(_))) {
                return Err(
                    CompilerError::double_input_declaration(name, &input_variable.get().borrow().name.span).into(),
                );
            }

            let category = if input_variable.get().borrow().const_ {
                InputCategory::ConstInput
            } else {
                InputCategory::MainInput
            };
            self.alloc_input_var(category, input_variable.get());

            arguments.push(Cell::new(&*function.scope.context.alloc_expression(
                Expression::VariableRef(leo_asg::VariableRef {
                    id: function.scope.context.get_id(),
                    parent: Cell::new(None),
                    span: Some(input_variable.get().borrow().name.span.clone()),
                    variable: input_variable.get(),
                }),
            )));
        }

        let statement = function
            .body
            .get()
            .expect("attempted to build body for function header");
        self.enforce_statement(statement)?;

        Ok(())
    }
}
