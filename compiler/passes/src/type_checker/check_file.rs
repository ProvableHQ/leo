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

use std::cell::RefCell;

use leo_ast::*;
use leo_errors::TypeCheckerError;

use crate::{Declaration, TypeChecker, VariableSymbol};

impl<'a> ProgramVisitor<'a> for TypeChecker<'a> {
    fn visit_function(&mut self, input: &'a Function) {
        let prev_st = std::mem::take(&mut self.symbol_table);
        self.symbol_table
            .swap(prev_st.borrow().get_fn_scope(&input.name()).unwrap());
        self.symbol_table.borrow_mut().parent = Some(Box::new(prev_st.into_inner()));

        self.has_return = false;
        self.parent = Some(input.name());
        input.input.iter().for_each(|i| {
            let input_var = i.get_variable();
            self.validate_ident_type(&Some(input_var.type_));

            let var = VariableSymbol {
                type_: input_var.type_,
                span: input_var.span,
                declaration: Declaration::Input(input_var.type_, input_var.mode()),
            };
            if let Err(err) = self
                .symbol_table
                .borrow_mut()
                .insert_variable(input_var.identifier.name, var)
            {
                self.handler.emit_err(err);
            }
        });
        self.visit_block(&input.block);

        if !self.has_return {
            self.handler
                .emit_err(TypeCheckerError::function_has_no_return(input.name(), input.span()));
        }

        let prev_st = *self.symbol_table.borrow_mut().parent.take().unwrap();
        self.symbol_table.swap(prev_st.get_fn_scope(&input.name()).unwrap());
        self.symbol_table = RefCell::new(prev_st);
    }
}
