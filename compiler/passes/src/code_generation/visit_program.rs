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

use crate::CodeGenerator;

use leo_ast::{Function, Program};

use itertools::Itertools;
use std::collections::HashMap;

impl<'a> CodeGenerator<'a> {
    pub(crate) fn visit_program(&mut self, input: &'a Program) -> String {
        // Visit each `Function` in the Leo AST and produce a bytecode function.
        input
            .functions
            .values()
            .map(|function| self.visit_function(function))
            .join("\n")
    }

    fn visit_function(&mut self, function: &'a Function) -> String {
        // Initialize the state of `self` with the appropriate values before visiting `function`.
        self.next_register = 0;
        self.variable_mapping = HashMap::new();
        self.current_function = Some(function);

        // Construct the header of the function.
        let mut function_string = format!("function {}: ", function.identifier);

        // Construct and append the input declarations of the function.
        for input in function.input.iter() {
            let register_string = format!("r{}", self.next_register);
            self.variable_mapping
                .insert(&input.get_variable().identifier, register_string.clone());

            let type_string = self.visit_type(&input.get_variable().type_);
            function_string.push_str(&format!(
                "input {} as {}.{};",
                register_string,
                type_string,
                input.get_variable().mode()
            ))
        }

        //  Construct and append the function body.
        let block_string = self.visit_block(&function.block);
        function_string.push_str(&block_string);

        function_string
    }
}
