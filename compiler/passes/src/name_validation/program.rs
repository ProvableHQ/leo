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

        input.structs.iter().for_each(|(_, function)| self.visit_struct(function));
        input.functions.iter().for_each(|(_, function)| self.visit_function(function));
    }

    fn visit_module(&mut self, input: &Module) {
        input.structs.iter().for_each(|(_, function)| self.visit_struct(function));
        input.functions.iter().for_each(|(_, function)| self.visit_function(function));
    }

    fn visit_stub(&mut self, input: &Stub) {
        input.structs.iter().for_each(|(_, function)| self.visit_struct_stub(function));
        input.functions.iter().for_each(|(_, function)| self.visit_function_stub(function));
    }

    fn visit_struct(&mut self, input: &Composite) {
        let struct_name = input.identifier;
        let item_type = if input.is_record { "record" } else { "struct" };
        self.is_not_keyword(struct_name, item_type, &[]);
        if input.is_record {
            self.does_not_contain_aleo(struct_name, item_type);
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
            Transition | AsyncTransition => self.is_not_keyword(function.identifier, "transition", &[]),
            Function => self.is_not_keyword(function.identifier, "function", &[]),
            Inline | AsyncFunction | Script => {}
        }
    }

    fn visit_function_stub(&mut self, input: &FunctionStub) {
        use Variant::*;
        match input.variant {
            Transition | AsyncTransition => self.is_not_keyword(input.identifier, "transition", &[]),
            Function => self.is_not_keyword(input.identifier, "function", &[]),
            Inline | AsyncFunction | Script => {}
        }
    }

    fn visit_struct_stub(&mut self, input: &Composite) {
        self.visit_struct(input);
    }
}
