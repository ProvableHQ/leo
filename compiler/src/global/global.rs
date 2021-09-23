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

//! Generates R1CS constraints for a compiled Leo program.

use crate::Program;
use leo_asg::CircuitMember;
use leo_errors::CompilerError;
use leo_errors::Result;

impl<'a> Program<'a> {
    pub fn enforce_program(&mut self, input: &leo_ast::Input) -> Result<()> {
        let asg = self.asg.clone();
        let main = *self
            .asg
            .functions
            .get("main")
            .ok_or_else(|| CompilerError::no_main_function())?;
        let secondary_functions: Vec<_> = asg
            .scope
            .get_functions()
            .iter()
            .filter(|(name, _)| *name != "main")
            .map(|(_, f)| *f)
            .chain(asg.scope.get_circuits().iter().flat_map(|(_, circuit)| {
                circuit
                    .members
                    .borrow()
                    .iter()
                    .flat_map(|(_, member)| match member {
                        CircuitMember::Function(function) => Some(*function),
                        CircuitMember::Variable(_) => None,
                    })
                    .collect::<Vec<_>>()
                    .into_iter()
            }))
            .collect();

        self.enforce_function(&asg, &main, &secondary_functions, input)
    }

    pub fn enforce_function(
        &mut self,
        asg: &leo_asg::Program<'a>,
        function: &'a leo_asg::Function<'a>,
        secondary_functions: &[&'a leo_asg::Function<'a>],
        input: &leo_ast::Input,
    ) -> Result<()> {
        self.register_function(function);
        for function in secondary_functions.iter() {
            self.register_function(*function);
        }

        self.current_function = Some(function);
        self.begin_main_function(function);

        for (_, global_const) in asg.scope.get_global_consts().iter() {
            self.enforce_definition_statement(global_const)?;
        }

        self.enforce_main_function(&function, input)?;
        for function in secondary_functions.iter() {
            self.enforce_function_definition(*function)?;
        }
        Ok(())
    }
}
