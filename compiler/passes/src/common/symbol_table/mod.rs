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

pub mod variable_symbol;

pub use variable_symbol::*;

use std::cell::RefCell;

use leo_ast::{normalize_json_value, remove_key_from_json, Composite, Function, Location};
use leo_errors::{AstError, Result};
use leo_span::{Span, Symbol};

use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use serde_json;

// TODO (@d0cd) Consider a safe interface for the symbol table.
// TODO (@d0cd) Cleanup API
#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
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
    pub(crate) variables: IndexMap<Location, VariableSymbol>,
    /// The index of the current scope.
    pub(crate) scope_index: usize,
    /// The sub-scopes of this scope.
    pub(crate) scopes: Vec<RefCell<SymbolTable>>,
}

impl SymbolTable {
    /// Recursively checks if the symbol table contains an entry for the given symbol.
    /// Leo does not allow any variable shadowing or overlap between different symbols.
    pub fn check_shadowing(&self, location: &Location, is_struct: bool, span: Span) -> Result<()> {
        if self.functions.contains_key(location) {
            return Err(AstError::shadowed_function(location.name, span).into());
        } else if self.structs.get(location).is_some() && !(location.program.is_none() && is_struct) {
            // The second half of the conditional makes sure that structs are only caught for shadowing local records during ST creation, not for redefinition of external structs.
            return Err(AstError::shadowed_record(location.name, span).into());
        } else if self.structs.get(&Location::new(None, location.name)).is_some() && !is_struct {
            // Struct redefinition is allowed. If there are more than one occurrences, the error will be caught in the creator pass.
            return Err(AstError::shadowed_struct(location.name, span).into());
        } else if self.variables.contains_key(location) {
            return Err(AstError::shadowed_variable(location.name, span).into());
        }
        if let Some(parent) = self.parent.as_ref() { parent.check_shadowing(location, is_struct, span) } else { Ok(()) }
    }

    /// Returns the current scope index.
    /// Increments the scope index.
    pub fn scope_index(&mut self) -> usize {
        let index = self.scope_index;
        self.scope_index += 1;
        index
    }

    /// Inserts a function into the symbol table.
    pub fn insert_fn(&mut self, location: Location, insert: &Function) -> Result<()> {
        let id = self.scope_index();
        self.check_shadowing(&location, false, insert.span)?;
        self.functions.insert(location, Self::new_function_symbol(id, insert));
        self.scopes.push(Default::default());
        Ok(())
    }

    /// Inserts a struct into the symbol table.
    pub fn insert_struct(&mut self, location: Location, insert: &Composite) -> Result<()> {
        // Check shadowing.
        self.check_shadowing(&location, !insert.is_record, insert.span)?;

        if insert.is_record {
            // Insert the record into the symbol table.
            self.structs.insert(location, insert.clone());
        } else {
            if let Some(struct_) = self.structs.get(&Location::new(None, location.name)) {
                // Allow redefinition of external structs so long as the definitions match.
                if !self.check_eq_struct(insert, struct_) {
                    return Err(AstError::redefining_external_struct(location.name, insert.span).into());
                }
            }
            // Insert with program location set to `None` to reflect that in snarkVM structs are not attached to programs (unlike records).
            self.structs.insert(Location::new(None, location.name), insert.clone());
        }

        Ok(())
    }

    /// Checks if two structs are equal.
    fn check_eq_struct(&self, new: &Composite, old: &Composite) -> bool {
        if new.is_record || old.is_record {
            return false;
        }
        if new.members.len() != old.members.len() {
            return false;
        }
        for (member1, member2) in new.members.iter().zip(old.members.iter()) {
            if member1.name() != member2.name() || !member1.type_.eq_flat_relax_composite(&member2.type_) {
                return false;
            }
        }
        true
    }

    /// Attach a finalize to a function.
    pub fn attach_finalize(&mut self, caller: Location, callee: Location) -> Result<()> {
        if let Some(func) = self.functions.get_mut(&caller) {
            func.finalize = Some(callee);
            Ok(())
        } else if let Some(parent) = self.parent.as_mut() {
            parent.attach_finalize(caller, callee)
        } else {
            Err(AstError::function_not_found(caller.name).into())
        }
    }

    /// Inserts a variable into the symbol table.
    pub fn insert_variable(&mut self, location: Location, insert: VariableSymbol) -> Result<()> {
        self.check_shadowing(&location, false, insert.span)?;
        self.variables.insert(location, insert);
        Ok(())
    }

    /// Inserts futures into the function definition.
    pub fn insert_futures(&mut self, program: Symbol, function: Symbol, futures: Vec<Location>) -> Result<()> {
        if let Some(func) = self.functions.get_mut(&Location::new(Some(program), function)) {
            func.future_inputs = futures;
            Ok(())
        } else if let Some(parent) = self.parent.as_mut() {
            parent.insert_futures(program, function, futures)
        } else {
            Err(AstError::function_not_found(function).into())
        }
    }

    /// Removes a variable from the symbol table.
    pub fn remove_variable_from_current_scope(&mut self, location: Location) {
        self.variables.remove(&location);
    }

    /// Creates a new scope for the block and stores it in the symbol table.
    pub fn insert_block(&mut self) -> usize {
        self.scopes.push(RefCell::new(Default::default()));
        self.scope_index()
    }

    /// Attempts to lookup a function in the symbol table.
    pub fn lookup_fn_symbol(&self, location: Location) -> Option<&FunctionSymbol> {
        if let Some(func) = self.functions.get(&location) {
            Some(func)
        } else if let Some(parent) = self.parent.as_ref() {
            parent.lookup_fn_symbol(location)
        } else {
            None
        }
    }

    /// Attempts to lookup a struct in the symbol table.
    pub fn lookup_struct(&self, location: Location, main_program: Option<Symbol>) -> Option<&Composite> {
        if let Some(struct_) = self.structs.get(&location) {
            return Some(struct_);
        } else if location.program == main_program {
            if let Some(struct_) = self.structs.get(&Location::new(None, location.name)) {
                return Some(struct_);
            }
        }
        if let Some(parent) = self.parent.as_ref() { parent.lookup_struct(location, main_program) } else { None }
    }

    /// Attempts to lookup a variable in the symbol table.
    pub fn lookup_variable(&self, location: Location) -> Option<&VariableSymbol> {
        if let Some(var) = self.variables.get(&location) {
            Some(var)
        } else if let Some(parent) = self.parent.as_ref() {
            parent.lookup_variable(location)
        } else {
            None
        }
    }

    /// Attempts to lookup a variable in the current scope.
    pub fn lookup_variable_in_current_scope(&self, location: Location) -> Option<&VariableSymbol> {
        self.variables.get(&location)
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
    use leo_span::{symbol::create_session_if_not_set_then, Symbol};
    #[test]
    fn serialization_test() {
        create_session_if_not_set_then(|_| {
            let mut symbol_table = SymbolTable::default();
            let func_loc = Location::new(Some(Symbol::intern("credits")), Symbol::intern("transfer_public"));
            let insert = Function {
                annotations: Vec::new(),
                id: 0,
                output_type: Type::Address,
                variant: Variant::Inline,
                span: Default::default(),
                input: Vec::new(),
                identifier: Identifier::new(Symbol::intern("transfer_public"), Default::default()),
                output: vec![],
                block: Default::default(),
            };
            symbol_table.insert_fn(func_loc, &insert).unwrap();
            symbol_table
                .insert_variable(
                    Location::new(Some(Symbol::intern("credits")), Symbol::intern("accounts")),
                    VariableSymbol { type_: Type::Address, span: Default::default(), declaration: VariableType::Const },
                )
                .unwrap();
            symbol_table
                .insert_struct(Location::new(Some(Symbol::intern("credits")), Symbol::intern("token")), &Composite {
                    is_record: false,
                    span: Default::default(),
                    id: 0,
                    identifier: Identifier::new(Symbol::intern("token"), Default::default()),
                    members: Vec::new(),
                    external: None,
                })
                .unwrap();
            symbol_table
                .insert_variable(Location::new(None, Symbol::intern("foo")), VariableSymbol {
                    type_: Type::Address,
                    span: Default::default(),
                    declaration: VariableType::Const,
                })
                .unwrap();
            let json = symbol_table.to_json_string().unwrap();
            let deserialized = SymbolTable::from_json_string(&json).unwrap();
            assert_eq!(symbol_table, deserialized);
        });
    }
}
