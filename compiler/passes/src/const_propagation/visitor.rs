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

use crate::CompilerState;

use leo_ast::{Expression, Location, Node, NodeID, interpreter_value::Value};
use leo_errors::StaticAnalyzerError;
use leo_span::{Span, Symbol};

pub struct ConstPropagationVisitor<'a> {
    pub state: &'a mut CompilerState,
    /// The program name.
    pub program: Symbol,
    /// The module name.
    pub module: Vec<Symbol>,
    /// Have we actually modified the program at all?
    pub changed: bool,
    /// The RHS of a const declaration we were not able to evaluate.
    pub const_not_evaluated: Option<Span>,
    /// An array index which was not able to be evaluated.
    pub array_index_not_evaluated: Option<Span>,
    /// An array length which was not able to be evaluated.
    pub array_length_not_evaluated: Option<Span>,
    /// A repeat expression count which was not able to be evaluated.
    pub repeat_count_not_evaluated: Option<Span>,
}

impl ConstPropagationVisitor<'_> {
    /// Enter the symbol table's scope `id`, execute `func`, and then return to the parent scope.
    pub fn in_scope<T>(&mut self, id: NodeID, func: impl FnOnce(&mut Self) -> T) -> T {
        self.state.symbol_table.enter_existing_scope(Some(id));
        let result = func(self);
        self.state.symbol_table.enter_parent();
        result
    }

    /// Enter module scope with path `module`, execute `func`, and then return to the parent module.
    pub fn in_module_scope<T>(&mut self, module: &[Symbol], func: impl FnOnce(&mut Self) -> T) -> T {
        let parent_module = self.module.clone();
        self.module = module.to_vec();
        let result = func(self);
        self.module = parent_module;
        result
    }

    /// Emit a `StaticAnalyzerError`.
    pub fn emit_err(&self, err: StaticAnalyzerError) {
        self.state.handler.emit_err(err);
    }

    pub fn value_to_expression(&self, value: &Value, span: Span, id: NodeID) -> Option<Expression> {
        let ty = self.state.type_table.get(&id)?;
        let symbol_table = &self.state.symbol_table;
        let struct_lookup = |loc: &Location| {
            symbol_table
                .lookup_struct(self.program, loc)
                .unwrap()
                .members
                .iter()
                .map(|mem| (mem.identifier.name, mem.type_.clone()))
                .collect()
        };
        value.to_expression(span, &self.state.node_builder, &ty, &struct_lookup)
    }

    pub fn value_to_expression_node(&self, value: &Value, previous: &impl Node) -> Option<Expression> {
        self.value_to_expression(value, previous.span(), previous.id())
    }
}
