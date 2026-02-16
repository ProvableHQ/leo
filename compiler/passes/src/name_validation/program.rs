// Copyright (C) 2019-2026 Provable Inc.
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

use super::NameValidationVisitor;

use leo_ast::*;

impl ProgramVisitor for NameValidationVisitor<'_> {
    fn visit_program(&mut self, input: &Program) {
        input.stubs.iter().for_each(|(_symbol, stub)| self.visit_stub(stub));
        input.modules.values().for_each(|module| self.visit_module(module));
        input.program_scopes.values().for_each(|scope| self.visit_program_scope(scope));
    }

    fn visit_program_scope(&mut self, input: &ProgramScope) {
        let program_name = input.program_id.name;
        self.does_not_contain_aleo(program_name, "program");
        self.is_not_keyword(program_name, "program", &[]);

        input.composites.iter().for_each(|(_, function)| self.visit_composite(function));
        input.functions.iter().for_each(|(_, function)| self.visit_function(function));
    }

    fn visit_module(&mut self, input: &Module) {
        input.composites.iter().for_each(|(_, function)| self.visit_composite(function));
        input.functions.iter().for_each(|(_, function)| self.visit_function(function));
    }

    fn visit_aleo_program(&mut self, input: &AleoProgram) {
        input.composites.iter().for_each(|(_, function)| self.visit_composite_stub(function));
        input.functions.iter().for_each(|(_, function)| self.visit_function_stub(function));
    }

    fn visit_composite(&mut self, input: &Composite) {
        let composite_name = input.identifier;
        let item_type = if input.is_record { "record" } else { "struct" };
        self.is_not_keyword(composite_name, item_type, &[]);
        if input.is_record {
            self.does_not_contain_aleo(composite_name, item_type);
        }

        for Member { identifier: member_name, .. } in &input.members {
            if input.is_record {
                self.is_not_keyword(*member_name, "record member", &["owner"]);
                self.does_not_contain_aleo(*member_name, "record member");
            } else {
                self.is_not_keyword(*member_name, "struct member", &[]);
            }
        }
    }

    fn visit_function(&mut self, function: &Function) {
        use Variant::*;
        match function.variant {
            EntryPoint => self.is_not_keyword(function.identifier, "entry point fn", &[]),
            Fn => self.is_not_keyword(function.identifier, "regular fn", &[]),
            FinalFn | Finalize | Script => {}
        }
    }

    fn visit_function_stub(&mut self, input: &FunctionStub) {
        use Variant::*;
        match input.variant {
            EntryPoint => self.is_not_keyword(input.identifier, "entry point fn", &[]),
            Fn => self.is_not_keyword(input.identifier, "regular fn", &[]),
            FinalFn | Finalize | Script => {}
        }
    }

    fn visit_composite_stub(&mut self, input: &Composite) {
        self.visit_composite(input);
    }
}
