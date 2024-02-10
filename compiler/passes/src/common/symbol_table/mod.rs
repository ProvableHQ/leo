// Copyright (C) 2019-2023 Aleo Systems Inc.
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

pub mod function_symbol;
pub use function_symbol::*;

pub mod location;
pub use location::*;

pub mod variable_symbol;

pub use variable_symbol::*;

use std::cell::RefCell;

use leo_ast::{normalize_json_value, remove_key_from_json, Composite, Function};
use leo_errors::{AstError, Result};
use leo_span::{Span, Symbol};

use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use serde_json;

// TODO (@d0cd) Consider a safe interface for the symbol table.
// TODO (@d0cd) Cleanup API
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct SymbolTable {
    /// The parent scope if it exists.
    /// For example, the parent scope of a then-block is the scope containing the associated ConditionalStatement.
    pub(crate) parent: Option<Box<SymbolTable>>,
    /// Maps parent program name and  function name to the AST's function definition.
    /// This field is populated at a first pass.
    pub functions: IndexMap<Location, FunctionSymbol>,
    /// Maps parent program name and composite name to composite definitions.
    /// This field is populated at a first pass.
    pub structs: IndexMap<Location, Composite>,
    /// The variables defined in a scope.
    /// This field is populated as necessary.
    pub(crate) variables: IndexMap<Symbol, VariableSymbol>,
    /// The index of the current scope.
    pub(crate) scope_index: usize,
    /// The sub-scopes of this scope.
    pub(crate) scopes: Vec<RefCell<SymbolTable>>,
}

impl SymbolTable {
    /// Recursively checks if the symbol table contains an entry for the given symbol.
    /// Leo does not allow any variable shadowing or overlap between different symbols.
    pub fn check_shadowing(&self, program: Option<Symbol>, symbol: Symbol, span: Span) -> Result<()> {
        if let Some(program) = program {
            if self.functions.contains_key(&Location::new(program, symbol)) {
                return Err(AstError::shadowed_function(symbol, span).into());
            } else if let Some(existing) = self.structs.get(&Location::new(program, symbol)) {
                return match existing.is_record {
                    true => Err(AstError::shadowed_record(symbol, span).into()),
                    false => Err(AstError::shadowed_struct(symbol, span).into()),
                };
            }
        }
        if self.variables.contains_key(&symbol) {
            Err(AstError::shadowed_variable(symbol, span).into())
        } else if let Some(parent) = self.parent.as_ref() {
            parent.check_shadowing(program, symbol, span)
        } else {
            Ok(())
        }
    }

    /// Returns the current scope index.
    /// Increments the scope index.
    pub fn scope_index(&mut self) -> usize {
        let index = self.scope_index;
        self.scope_index += 1;
        index
    }

    /// Inserts a function into the symbol table.
    pub fn insert_fn(&mut self, program: Symbol, symbol: Symbol, insert: &Function) -> Result<()> {
        let id = self.scope_index();
        self.check_shadowing(Some(program), symbol, insert.span)?;
        self.functions.insert(Location::new(program, symbol), Self::new_function_symbol(id, insert));
        self.scopes.push(Default::default());
        Ok(())
    }

    /// Inserts a struct into the symbol table.
    pub fn insert_struct(&mut self, program: Symbol, symbol: Symbol, insert: &Composite) -> Result<()> {
        match self.check_shadowing(Some(program), symbol, insert.span) {
            Ok(_) => {
                self.structs.insert(Location::new(program, symbol), insert.clone());
                Ok(())
            }
            Err(e) => Err(e),
        }
    }

    /// Inserts a variable into the symbol table.
    pub fn insert_variable(&mut self, symbol: Symbol, insert: VariableSymbol) -> Result<()> {
        self.check_shadowing(None, symbol, insert.span)?;
        self.variables.insert(symbol, insert);
        Ok(())
    }

    /// Removes a variable from the symbol table.
    pub fn remove_variable_from_current_scope(&mut self, symbol: Symbol) {
        self.variables.remove(&symbol);
    }

    /// Creates a new scope for the block and stores it in the symbol table.
    pub fn insert_block(&mut self) -> usize {
        self.scopes.push(RefCell::new(Default::default()));
        self.scope_index()
    }

    /// Attempts to lookup a function in the symbol table.
    pub fn lookup_fn_symbol(&self, program: Symbol, symbol: Symbol) -> Option<&FunctionSymbol> {
        if let Some(func) = self.functions.get(&Location::new(program, symbol)) {
            Some(func)
        } else if let Some(parent) = self.parent.as_ref() {
            parent.lookup_fn_symbol(program, symbol)
        } else {
            None
        }
    }

    /// Attempts to lookup a struct in the symbol table.
    pub fn lookup_struct(&self, program: Symbol, symbol: Symbol) -> Option<&Composite> {
        if let Some(struct_) = self.structs.get(&Location::new(program, symbol)) {
            Some(struct_)
        } else if let Some(parent) = self.parent.as_ref() {
            parent.lookup_struct(program, symbol)
        } else {
            None
        }
    }

    /// Attempts to lookup a variable in the symbol table.
    pub fn lookup_variable(&self, symbol: Symbol) -> Option<&VariableSymbol> {
        if let Some(var) = self.variables.get(&symbol) {
            Some(var)
        } else if let Some(parent) = self.parent.as_ref() {
            parent.lookup_variable(symbol)
        } else {
            None
        }
    }

    /// Returns true if the variable exists in the local scope
    pub fn variable_in_local_scope(&self, symbol: Symbol) -> bool {
        self.variables.contains_key(&symbol)
    }

    /// Returns true if the variable exists in any parent scope
    pub fn variable_in_parent_scope(&self, symbol: Symbol) -> bool {
        if let Some(parent) = self.parent.as_ref() {
            if parent.variables.contains_key(&symbol) { true } else { parent.variable_in_parent_scope(symbol) }
        } else {
            false
        }
    }

    /// Returns a mutable reference to the `VariableSymbol` if it exists in the symbol table.
    pub fn lookup_variable_mut(&mut self, symbol: Symbol) -> Option<&mut VariableSymbol> {
        if let Some(var) = self.variables.get_mut(&symbol) {
            Some(var)
        } else if let Some(parent) = self.parent.as_mut() {
            parent.lookup_variable_mut(symbol)
        } else {
            None
        }
    }

    /// Returns the scope associated with `index`, if it exists in the symbol table.
    pub fn lookup_scope_by_index(&self, index: usize) -> Option<&RefCell<Self>> {
        self.scopes.get(index)
    }

    /// Serializes the symbol table into a JSON string.
    pub fn to_json_string(&self) -> Result<String> {
        Ok(serde_json::to_string_pretty(&self)
            .map_err(|e| AstError::failed_to_convert_symbol_table_to_json_string(&e))?)
    }

    /// Converts the symbol table into a JSON value
    pub fn to_json_value(&self) -> Result<serde_json::Value> {
        Ok(serde_json::to_value(self).map_err(|e| AstError::failed_to_convert_symbol_table_to_json_value(&e))?)
    }

    // Serializes the symbol table into a JSON file.
    pub fn to_json_file(&self, mut path: std::path::PathBuf, file_name: &str) -> Result<()> {
        path.push(file_name);
        let file =
            std::fs::File::create(&path).map_err(|e| AstError::failed_to_create_symbol_table_json_file(&path, &e))?;
        let writer = std::io::BufWriter::new(file);
        Ok(serde_json::to_writer_pretty(writer, &self)
            .map_err(|e| AstError::failed_to_write_symbol_table_to_json_file(&path, &e))?)
    }

    /// Serializes the symbol table into a JSON value and removes keys from object mappings before writing to a file.
    pub fn to_json_file_without_keys(
        &self,
        mut path: std::path::PathBuf,
        file_name: &str,
        excluded_keys: &[&str],
    ) -> Result<()> {
        path.push(file_name);
        let file =
            std::fs::File::create(&path).map_err(|e| AstError::failed_to_create_symbol_table_json_file(&path, &e))?;
        let writer = std::io::BufWriter::new(file);

        let mut value = self.to_json_value().unwrap();
        for key in excluded_keys {
            value = remove_key_from_json(value, key);
        }
        value = normalize_json_value(value);

        Ok(serde_json::to_writer_pretty(writer, &value)
            .map_err(|e| AstError::failed_to_write_symbol_table_to_json_file(&path, &e))?)
    }

    /// Deserializes the JSON string into a symbol table.
    pub fn from_json_string(json: &str) -> Result<Self> {
        let symbol_table: SymbolTable =
            serde_json::from_str(json).map_err(|e| AstError::failed_to_read_json_string_to_symbol_table(&e))?;
        Ok(symbol_table)
    }

    /// Deserializes the JSON string into a symbol table from a file.
    pub fn from_json_file(path: std::path::PathBuf) -> Result<Self> {
        let data = std::fs::read_to_string(&path).map_err(|e| AstError::failed_to_read_json_file(&path, &e))?;
        Self::from_json_string(&data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use leo_ast::{Identifier, Type, Variant};
    use leo_span::symbol::create_session_if_not_set_then;
    #[test]
    #[ignore]
    fn serialization_test() {
        create_session_if_not_set_then(|_| {
            let mut symbol_table = SymbolTable::default();
            let program = Symbol::intern("credits");
            let function = Symbol::intern("transfer_public");
            let insert = Function {
                annotations: Vec::new(),
                id: 0,
                output_type: Type::Address,
                variant: Variant::Inline,
                span: Default::default(),
                input: Vec::new(),
                finalize: None,
                identifier: Identifier::new(Symbol::intern("transfer_public"), Default::default()),
                output: vec![],
                block: Default::default(),
            };
            symbol_table.insert_fn(program, function, &insert).unwrap();
            let json = symbol_table.to_json_string().unwrap();
            dbg!(json.clone());
            let deserialized = SymbolTable::from_json_string(&json).unwrap();
            dbg!(deserialized);
        });
    }
}
