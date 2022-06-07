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

use leo_ast::{
    AssignStatement, Block, ConditionalStatement, ConsoleStatement, DefinitionStatement, IterationStatement,
    ReturnStatement, Statement,
};

use itertools::Itertools;

impl<'a> CodeGenerator<'a> {
    fn visit_statement(&mut self, input: &'a Statement) -> String {
        match input {
            Statement::Return(stmt) => self.visit_return(stmt),
            Statement::Definition(stmt) => self.visit_definition(stmt),
            Statement::Assign(stmt) => self.visit_assign(stmt),
            Statement::Conditional(stmt) => self.visit_conditional(stmt),
            Statement::Iteration(stmt) => self.visit_iteration(stmt),
            Statement::Console(stmt) => self.visit_console(stmt),
            Statement::Block(stmt) => self.visit_block(stmt),
        }
    }

    fn visit_return(&mut self, input: &'a ReturnStatement) -> String {
        let (operand, mut expression_instructions) = self.visit_expression(&input.expression);
        let output_type = self.visit_type(&self.current_function.unwrap().output);
        // TODO: Bytecode functions have an associated output mode. Currently defaulting to private since we do not yet support this at the Leo level.
        let output_statement = format!("    output {} as {}.private;\n", operand, output_type);
        expression_instructions.push_str(&output_statement);

        expression_instructions
    }

    fn visit_definition(&mut self, input: &'a DefinitionStatement) -> String {
        // Note: `DefinitionStatement`s should not exist in SSA form. However, for the purposes of this
        // prototype, we will need to support them.
        match input.variable_names.len() == 1 {
            // TODO: Is this a valid assumption?
            false => unreachable!("DefinitionStatements should contain only one variable once put into SSA form"),
            true => {
                let (operand, expression_instructions) = self.visit_expression(&input.value);
                self.variable_mapping
                    .insert(&input.variable_names[0].identifier.name, operand.clone());
                expression_instructions
            }
        }
    }

    fn visit_assign(&mut self, _input: &'a AssignStatement) -> String {
        // Note: SSA form requires that a variable is assigned to only once.
        // Since we do not have a compiler pass that transforms the AST into SSA form,
        // we will disallow `AssignStatement`s. This will only be the case for this prototype.
        unimplemented!("Code generation is not implemented for `AssignStatement`s.")
    }

    fn visit_conditional(&mut self, _input: &'a ConditionalStatement) -> String {
        // Note: This requires that the AST is in static-single assignment form.
        // It is not possible to provide an input program with a conditional statement in SSA form as
        // complete SSA has different semantics from source Leo programs.
        unimplemented!("Code generation is not implemented for conditional statements.")
    }

    fn visit_iteration(&mut self, _input: &'a IterationStatement) -> String {
        unreachable!("`IterationStatement`s should not be in the AST at this phase of compilation.");
    }

    fn visit_console(&mut self, _input: &'a ConsoleStatement) -> String {
        // `ConsoleStatement`s do not need to be included in the bytecode.
        String::new()
    }

    pub(crate) fn visit_block(&mut self, input: &'a Block) -> String {
        // For each statement in the block, visit it and add its instructions to the list.
        input.statements.iter().map(|stmt| self.visit_statement(stmt)).join("")
    }
}
