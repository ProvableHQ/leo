// Copyright (C) 2019-2024 Aleo Systems Inc.
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

use leo_ast::{Composite, Expression, Function, Location, NodeID, Type};
use leo_errors::{AstError, Result};
use leo_span::{Span, Symbol};

use indexmap::IndexMap;
use std::{cell::RefCell, collections::HashMap, rc::Rc};

mod symbols;
pub use symbols::*;

/// Maps global and local symbols to information about them.
///
/// Scopes are indexed by the NodeID of the function, block, or iteration.
#[derive(Debug, Default)]
pub struct SymbolTable {
    /// Functions indexed by location.
    functions: IndexMap<Location, FunctionSymbol>,

    /// Records indexed by location.
    records: IndexMap<Location, Composite>,

    /// Structs indexed by location.
    structs: IndexMap<Symbol, Composite>,

    /// Consts that have been successfully evaluated.
    global_consts: IndexMap<Location, Expression>,

    /// Global variables indexed by location.
    globals: IndexMap<Location, VariableSymbol>,

    /// Local tables index by the NodeID of the function, iteration, or block they're contained in.
    all_locals: HashMap<NodeID, LocalTable>,

    /// The current LocalTable we're looking at.
    local: Option<LocalTable>,
}

#[derive(Clone, Default, Debug)]
struct LocalTable {
    inner: Rc<RefCell<LocalTableInner>>,
}

#[derive(Clone, Default, Debug)]
struct LocalTableInner {
    /// The `NodeID` of the function, iteration, or block this table indexes.
    id: NodeID,

    /// The parent `NodeID` of this scope, if it exists.
    parent: Option<NodeID>,

    /// The consts we've evaluated in this scope.
    consts: HashMap<Symbol, Expression>,

    /// Variables in this scope, indexed by name.
    variables: HashMap<Symbol, VariableSymbol>,
}

impl LocalTable {
    fn new(id: NodeID, parent: Option<NodeID>) -> Self {
        LocalTable {
            inner: Rc::new(RefCell::new(LocalTableInner {
                id,
                parent,
                consts: HashMap::new(),
                variables: HashMap::new(),
            })),
        }
    }
}

impl SymbolTable {
    /// Iterator over all the structs (not records) in this program.
    pub fn iter_structs(&self) -> impl Iterator<Item = (Symbol, &Composite)> {
        self.structs.iter().map(|(name, comp)| (*name, comp))
    }

    /// Iterator over all the records in this program.
    pub fn iter_records(&self) -> impl Iterator<Item = (Location, &Composite)> {
        self.records.iter().map(|(loc, comp)| (*loc, comp))
    }

    /// Iterator over all the functions in this program.
    pub fn iter_functions(&self) -> impl Iterator<Item = (Location, &FunctionSymbol)> {
        self.functions.iter().map(|(loc, func_symbol)| (*loc, func_symbol))
    }

    /// Access the struct by this name if it exists.
    pub fn lookup_struct(&self, name: Symbol) -> Option<&Composite> {
        self.structs.get(&name)
    }

    /// Access the record at this location if it exists.
    pub fn lookup_record(&self, location: Location) -> Option<&Composite> {
        self.records.get(&location)
    }

    /// Access the function at this location if it exists.
    pub fn lookup_function(&self, location: Location) -> Option<&FunctionSymbol> {
        self.functions.get(&location)
    }

    /// Access the variable accessible by this name in the current scope.
    pub fn lookup_variable(&self, program: Symbol, name: Symbol) -> Option<VariableSymbol> {
        let mut current = self.local.as_ref();

        while let Some(table) = current {
            let borrowed = table.inner.borrow();
            let value = borrowed.variables.get(&name);
            if value.is_some() {
                return value.cloned();
            }

            current = borrowed.parent.and_then(|id| self.all_locals.get(&id));
        }

        self.globals.get(&Location::new(program, name)).cloned()
    }

    /// Enter the scope of this `NodeID`, creating a table if it doesn't exist yet.
    ///
    /// Passing `None` means to enter the global scope.
    pub fn enter_scope(&mut self, id: Option<NodeID>) {
        self.local = id.map(|id| {
            let parent = self.local.as_ref().map(|table| table.inner.borrow().id);
            let new_local_table = self.all_locals.entry(id).or_insert_with(|| LocalTable::new(id, parent));
            new_local_table.clone()
        });
    }

    /// Enther the parent scope of the current scope (or the global scope if there is no local parent scope).
    pub fn enter_parent(&mut self) {
        let parent: Option<NodeID> = self.local.as_ref().and_then(|table| table.inner.borrow().parent);
        self.local = parent.map(|id| self.all_locals.get(&id).expect("Parent should exist.")).cloned();
    }

    /// Insert an evaluated const into the current scope.
    pub fn insert_const(&mut self, program: Symbol, name: Symbol, value: Expression) {
        if let Some(table) = self.local.as_mut() {
            table.inner.borrow_mut().consts.insert(name, value);
        } else {
            self.global_consts.insert(Location::new(program, name), value);
        }
    }

    /// Find the evaluated const accessible by the given name in the current scope.
    pub fn lookup_const(&self, program: Symbol, name: Symbol) -> Option<Expression> {
        let mut current = self.local.as_ref();

        while let Some(table) = current {
            let borrowed = table.inner.borrow();
            let value = borrowed.consts.get(&name);
            if value.is_some() {
                return value.cloned();
            }

            current = borrowed.parent.and_then(|id| self.all_locals.get(&id));
        }

        self.global_consts.get(&Location::new(program, name)).cloned()
    }

    /// Insert a struct at this name.
    ///
    /// Since structs are indexed only by name, the program is used only to check shadowing.
    pub fn insert_struct(&mut self, program: Symbol, name: Symbol, composite: Composite) -> Result<()> {
        if let Some(old_composite) = self.structs.get(&name) {
            if eq_struct(&composite, old_composite) {
                Ok(())
            } else {
                Err(AstError::redefining_external_struct(name, composite.span).into())
            }
        } else {
            let location = Location::new(program, name);
            self.check_shadow_global(location, composite.span)?;
            self.structs.insert(name, composite);
            Ok(())
        }
    }

    /// Insert a record at this location.
    pub fn insert_record(&mut self, location: Location, composite: Composite) -> Result<()> {
        self.check_shadow_global(location, composite.span)?;
        self.records.insert(location, composite);
        Ok(())
    }

    /// Insert a function at this location.
    pub fn insert_function(&mut self, location: Location, function: Function) -> Result<()> {
        self.check_shadow_global(location, function.span)?;
        self.functions.insert(location, FunctionSymbol { function, finalizer: None });
        Ok(())
    }

    /// Insert a global at this location.
    pub fn insert_global(&mut self, location: Location, var: VariableSymbol) -> Result<()> {
        self.check_shadow_global(location, var.span)?;
        self.globals.insert(location, var);
        Ok(())
    }

    /// Access the global at this location if it exists.
    pub fn lookup_global(&self, location: Location) -> Option<&VariableSymbol> {
        self.globals.get(&location)
    }

    fn check_shadow_global(&self, location: Location, span: Span) -> Result<()> {
        if self.functions.contains_key(&location) {
            Err(AstError::shadowed_function(location.name, span).into())
        } else if self.records.contains_key(&location) {
            Err(AstError::shadowed_record(location.name, span).into())
        } else if self.structs.contains_key(&location.name) {
            Err(AstError::shadowed_struct(location.name, span).into())
        } else if self.globals.contains_key(&location) {
            Err(AstError::shadowed_variable(location.name, span).into())
        } else {
            Ok(())
        }
    }

    fn check_shadow_variable(&self, program: Symbol, name: Symbol, span: Span) -> Result<()> {
        let mut current = self.local.as_ref();

        while let Some(table) = current {
            if table.inner.borrow().variables.contains_key(&name) {
                return Err(AstError::shadowed_variable(name, span).into());
            }
            current = table.inner.borrow().parent.map(|id| self.all_locals.get(&id).expect("Parent should exist."));
        }

        self.check_shadow_global(Location::new(program, name), span)?;

        Ok(())
    }

    /// Insert a variable into the current scope.
    pub fn insert_variable(&mut self, program: Symbol, name: Symbol, var: VariableSymbol) -> Result<()> {
        self.check_shadow_variable(program, name, var.span)?;

        if let Some(table) = self.local.as_mut() {
            table.inner.borrow_mut().variables.insert(name, var);
        } else {
            self.globals.insert(Location::new(program, name), var);
        }

        Ok(())
    }

    /// Attach a finalizer to a function.
    pub fn attach_finalizer(
        &mut self,
        caller: Location,
        callee: Location,
        future_inputs: Vec<Location>,
        inferred_inputs: Vec<Type>,
    ) -> Result<()> {
        let callee_location = Location::new(callee.program, callee.name);

        if let Some(func) = self.functions.get_mut(&caller) {
            func.finalizer = Some(Finalizer { location: callee_location, future_inputs, inferred_inputs });
            Ok(())
        } else {
            Err(AstError::function_not_found(caller.name).into())
        }
    }
}

fn eq_struct(new: &Composite, old: &Composite) -> bool {
    if new.members.len() != old.members.len() {
        return false;
    }

    new.members
        .iter()
        .zip(old.members.iter())
        .all(|(member1, member2)| member1.name() == member2.name() && member1.type_.eq_flat_relaxed(&member2.type_))
}
