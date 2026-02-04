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

use leo_ast::{Composite, Expression, Function, Location, NodeBuilder, NodeID, Path, Type};
use leo_errors::{AstError, Color, Label, LeoError, Result};
use leo_span::{Span, Symbol};

use indexmap::{IndexMap, IndexSet};
use itertools::Itertools;
use std::{cell::RefCell, collections::HashMap, rc::Rc};

mod symbols;
pub use symbols::*;

/// Maps global and local symbols to information about them.
///
/// Scopes are indexed by the NodeID of the function, block, or iteration.
#[derive(Debug, Default)]
pub struct SymbolTable {
    /// Maps a program to the list of programs it imports
    imports: IndexMap<Symbol, IndexSet<Symbol>>,

    /// Functions indexed by location.
    functions: IndexMap<Location, FunctionSymbol>,

    /// Records indexed by location.
    records: IndexMap<Location, Composite>,

    /// Structs indexed by a path.
    structs: IndexMap<Location, Composite>,

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

    /// The children of `NodeID` of this scope
    children: Vec<NodeID>,

    /// The consts we've evaluated in this scope.
    consts: HashMap<Symbol, Expression>,

    /// Variables in this scope, indexed by name.
    variables: HashMap<Symbol, VariableSymbol>,
}

impl LocalTable {
    fn new(symbol_table: &mut SymbolTable, id: NodeID, parent: Option<NodeID>) -> Self {
        // If parent exists, register this scope as its child
        if let Some(parent_id) = parent
            && let Some(parent_table) = symbol_table.all_locals.get_mut(&parent_id)
        {
            parent_table.inner.borrow_mut().children.push(id);
        }

        LocalTable {
            inner: Rc::new(RefCell::new(LocalTableInner {
                id,
                parent,
                consts: HashMap::new(),
                variables: HashMap::new(),
                children: vec![], // Must still initialize our own children list
            })),
        }
    }

    fn clear_but_consts(&mut self) {
        self.inner.borrow_mut().variables.clear();
    }

    /// Recursively duplicates this table and all children.
    /// `new_parent` is the NodeID of the parent in the new tree (None for root).
    pub fn dup(
        &self,
        node_builder: &NodeBuilder,
        all_locals: &mut HashMap<NodeID, LocalTable>,
        new_parent: Option<NodeID>,
    ) -> LocalTable {
        let inner = self.inner.borrow();

        // Generate a new ID for this table
        let new_id = node_builder.next_id();

        // Recursively duplicate children with new_id as their parent
        let new_children: Vec<NodeID> = inner
            .children
            .iter()
            .map(|child_id| {
                let child_table = all_locals.get(child_id).expect("Child must exist").clone();
                let duped_child = child_table.dup(node_builder, all_locals, Some(new_id));
                let child_new_id = duped_child.inner.borrow().id;
                all_locals.insert(child_new_id, duped_child);
                child_new_id
            })
            .collect();

        // Duplicate this table with correct parent
        let new_table = LocalTable {
            inner: Rc::new(RefCell::new(LocalTableInner {
                id: new_id,
                parent: new_parent,
                children: new_children,
                consts: inner.consts.clone(),
                variables: inner.variables.clone(),
            })),
        };

        // Register in all_locals
        all_locals.insert(new_id, new_table.clone());

        new_table
    }
}

impl SymbolTable {
    /// Record that `importer` imports `imported`.
    pub fn add_import(&mut self, importer: Symbol, imported: Symbol) {
        self.imports.entry(importer).or_default().insert(imported);
    }

    /// Record that multiple importers import the same `imported` program.
    pub fn add_imported_by(&mut self, imported: Symbol, importers: &IndexSet<Symbol>) {
        for importer in importers {
            self.add_import(*importer, imported);
        }
    }

    /// Returns all programs imported by a given program.
    pub fn get_imports(&self, program: &Symbol) -> Option<&IndexSet<Symbol>> {
        self.imports.get(program)
    }

    /// Returns a mutable reference to the set of imports for a given program.
    pub fn get_imports_mut(&mut self, program: &Symbol) -> Option<&mut IndexSet<Symbol>> {
        self.imports.get_mut(program)
    }

    /// Returns an iterator over all import relationships.
    pub fn iter_imports(&self) -> impl Iterator<Item = (&Symbol, &IndexSet<Symbol>)> {
        self.imports.iter()
    }

    /// Check if `target` program is visible from `current` program.
    fn is_visible(&self, current: Symbol, target: &Symbol) -> bool {
        current == *target || self.imports.get(&current).map(|imports| imports.contains(target)).unwrap_or(false)
    }

    /// Returns the transitive closure of imports for a given program.
    pub fn get_transitive_imports(&self, program: &Symbol) -> IndexSet<Symbol> {
        let mut ordered = IndexSet::new();
        let mut seen = IndexSet::new();

        if let Some(direct_imports) = self.imports.get(program) {
            for imported in direct_imports {
                self.collect_imports_postorder(imported, &mut ordered, &mut seen);
            }
        }

        ordered
    }

    /// Performs a depth-first traversal of the import graph and records programs in dependency order, ensuring that
    /// each program is added only after all of its transitive imports have been processed.
    fn collect_imports_postorder(&self, program: &Symbol, ordered: &mut IndexSet<Symbol>, seen: &mut IndexSet<Symbol>) {
        // Prevent cycles
        if !seen.insert(*program) {
            return;
        }

        if let Some(direct_imports) = self.imports.get(program) {
            for imported in direct_imports {
                self.collect_imports_postorder(imported, ordered, seen);
            }
        }

        ordered.insert(*program);
    }

    /// Reset everything except leave consts that have been evaluated.
    pub fn reset_but_consts(&mut self) {
        self.functions.clear();
        self.records.clear();
        self.structs.clear();
        self.globals.clear();

        // clear all non-const locals
        for local_table in self.all_locals.values_mut() {
            local_table.clear_but_consts();
        }

        self.local = None;
    }

    /// Are we currently in the global scope?
    pub fn global_scope(&self) -> bool {
        self.local.is_none()
    }

    /// Iterator over all the structs (not records) in this program.
    pub fn iter_structs(&self) -> impl Iterator<Item = (&Location, &Composite)> {
        self.structs.iter()
    }

    /// Iterator over all the records in this program.
    pub fn iter_records(&self) -> impl Iterator<Item = (&Location, &Composite)> {
        self.records.iter()
    }

    /// Iterator over all the functions in this program.
    pub fn iter_functions(&self) -> impl Iterator<Item = (&Location, &FunctionSymbol)> {
        self.functions.iter()
    }

    /// Access a struct by this location if it exists and is accessible from program named `current_program`.
    pub fn lookup_struct(&self, current_program: Symbol, loc: &Location) -> Option<&Composite> {
        if self.is_visible(current_program, &loc.program) { self.structs.get(loc) } else { None }
    }

    /// Access a record at this location if it exists and is accessible from program named `current_program`.
    pub fn lookup_record(&self, current_program: Symbol, location: &Location) -> Option<&Composite> {
        if self.is_visible(current_program, &location.program) { self.records.get(location) } else { None }
    }

    /// Access a function by this name if it exists and is accessible from program named `current_program`.
    pub fn lookup_function(&self, current_program: Symbol, location: &Location) -> Option<&FunctionSymbol> {
        if self.is_visible(current_program, &location.program) { self.functions.get(location) } else { None }
    }

    /// Attempts to look up a variable by a path from program named `current_program`.
    ///
    /// First, it tries to resolve the symbol as a global using the full path under the given program.
    /// If that fails and the path is non-empty, it falls back to resolving the last component
    /// of the path as a local symbol.
    ///
    /// # Arguments
    ///
    /// * `program` - The root symbol representing the program or module context.
    /// * `path` - A `Path`.
    ///
    /// # Returns
    ///
    /// An `Option<VariableSymbol>` containing the resolved symbol if found, otherwise `None`.
    pub fn lookup_path(&self, current_program: Symbol, path: &Path) -> Option<VariableSymbol> {
        if let Some(loc) = path.try_global_location() {
            self.lookup_global(current_program, loc).cloned()
        } else if let Some(sym) = path.try_local_symbol() {
            self.lookup_local(sym)
        } else {
            None
        }
    }

    /// Access the variable accessible by this name in the current scope.
    pub fn lookup_local(&self, name: Symbol) -> Option<VariableSymbol> {
        let mut current = self.local.as_ref();

        while let Some(table) = current {
            let borrowed = table.inner.borrow();
            let value = borrowed.variables.get(&name);
            if value.is_some() {
                return value.cloned();
            }

            current = borrowed.parent.and_then(|id| self.all_locals.get(&id));
        }

        None
    }

    /// Enter the scope of this `NodeID`, creating a table if it doesn't exist yet.
    ///
    /// Passing `None` means to enter the global scope.
    pub fn enter_scope(&mut self, id: Option<NodeID>) {
        self.local = id.map(|id| {
            let parent = self.local.as_ref().map(|table| table.inner.borrow().id);
            let new_local_table = if let Some(existing) = self.all_locals.get(&id) {
                existing.clone()
            } else {
                let new_table = LocalTable::new(self, id, parent);
                self.all_locals.insert(id, new_table.clone());
                new_table
            };

            assert_eq!(parent, new_local_table.inner.borrow().parent, "Entered scopes out of order.");
            new_local_table.clone()
        });
    }

    pub fn enter_existing_scope(&mut self, id: Option<NodeID>) {
        self.local = id.map(|id| {
            let parent = self.local.as_ref().map(|table| table.inner.borrow().id);
            let new_local_table = if let Some(existing) = self.all_locals.get(&id) {
                existing.clone()
            } else {
                panic!("local scope must exist");
            };

            assert_eq!(parent, new_local_table.inner.borrow().parent, "Entered scopes out of order.");
            new_local_table.clone()
        });
    }

    /// Enter a new scope by duplicating the local table at `old_id` recursively.
    ///
    /// Each scope in the subtree receives a fresh NodeID from `node_builder`.
    /// The new root scope's parent is set to the current scope (if any).
    /// Returns the NodeID of the new duplicated root scope.
    pub fn enter_scope_duped(&mut self, old_id: NodeID, node_builder: &NodeBuilder) -> usize {
        let old_local_table = self.all_locals.get(&old_id).expect("Must have an old scope to dup from.").clone();

        // Recursively duplicate the table and all its children
        let new_local_table =
            old_local_table.dup(node_builder, &mut self.all_locals, self.local.as_ref().map(|t| t.inner.borrow().id));

        let new_id = new_local_table.inner.borrow().id;

        // Update current scope
        self.local = Some(new_local_table);

        new_id
    }

    /// Enter the parent scope of the current scope (or the global scope if there is no local parent scope).
    pub fn enter_parent(&mut self) {
        let parent: Option<NodeID> = self.local.as_ref().and_then(|table| table.inner.borrow().parent);
        self.local = parent.map(|id| self.all_locals.get(&id).expect("Parent should exist.")).cloned();
    }

    /// Checks if a `symbol` is local to `scope`.
    pub fn is_local_to(&self, scope: NodeID, symbol: Symbol) -> bool {
        self.all_locals.get(&scope).map(|locals| locals.inner.borrow().variables.contains_key(&symbol)).unwrap_or(false)
    }

    /// Checks whether `symbol` is defined in the current scope (self.local) or any of its
    /// ancestor scopes, up to and including `scope`.
    ///
    /// Returns `false` if the current scope is not a descendant of `scope`.
    pub fn is_defined_in_scope_or_ancestor_until(&self, scope: NodeID, symbol: Symbol) -> bool {
        let mut current = self.local.as_ref();

        while let Some(table) = current {
            let inner = table.inner.borrow();

            // Check if symbol is defined in this scope
            if inner.variables.contains_key(&symbol) {
                return true;
            }

            // Stop when we reach the given upper-bound scope
            if inner.id == scope {
                break;
            }

            // Move to parent
            current = inner.parent.and_then(|parent_id| self.all_locals.get(&parent_id));
        }

        false
    }

    /// Checks if a `symbol` is local to `scope` or any of its child scopes.
    pub fn is_local_to_or_in_child_scope(&self, scope: NodeID, symbol: Symbol) -> bool {
        let mut stack = vec![scope];

        while let Some(current_id) = stack.pop() {
            if let Some(table) = self.all_locals.get(&current_id) {
                let inner = table.inner.borrow();

                if inner.variables.contains_key(&symbol) {
                    return true;
                }

                stack.extend(&inner.children);
            }
        }

        false
    }

    /// Insert an evaluated local const into the current scope.
    /// This function does nothing if we're in a global scope.
    pub fn insert_local_const(&mut self, name: Symbol, value: Expression) {
        if let Some(table) = self.local.as_mut() {
            table.inner.borrow_mut().consts.insert(name, value);
        }
    }

    /// Insert an evaluated global const into the global scope
    /// This function does nothing if we're in a local scope.
    pub fn insert_global_const(&mut self, location: Location, value: Expression) {
        if self.global_scope() {
            self.global_consts.insert(location, value);
        }
    }

    /// Find the evaluated const accessible by the given name in the current scope.
    pub fn lookup_local_const(&self, name: Symbol) -> Option<Expression> {
        let mut current = self.local.as_ref();

        while let Some(table) = current {
            let borrowed = table.inner.borrow();
            let value = borrowed.consts.get(&name);
            if value.is_some() {
                return value.cloned();
            }

            current = borrowed.parent.and_then(|id| self.all_locals.get(&id));
        }

        None
    }

    pub fn lookup_global_const(&self, current_program: Symbol, location: &Location) -> Option<Expression> {
        if self.is_visible(current_program, &location.program) {
            self.global_consts.get(location).cloned()
        } else {
            None
        }
    }

    /// Insert a struct at this location.
    pub fn insert_struct(&mut self, location: Location, r#struct: Composite) -> Result<()> {
        self.check_shadow_global(&location, r#struct.identifier.span)?;
        self.structs.insert(location, r#struct);
        Ok(())
    }

    /// Insert a record at this location.
    pub fn insert_record(&mut self, location: Location, record: Composite) -> Result<()> {
        self.check_shadow_global(&location, record.identifier.span)?;
        self.records.insert(location, record);
        Ok(())
    }

    /// Insert a function at this location.
    pub fn insert_function(&mut self, location: Location, function: Function) -> Result<()> {
        self.check_shadow_global(&location, function.identifier.span)?;
        self.functions.insert(location, FunctionSymbol { function, finalizer: None });
        Ok(())
    }

    /// Insert a global at this location.
    pub fn insert_global(&mut self, location: Location, var: VariableSymbol) -> Result<()> {
        self.check_shadow_global(&location, var.span)?;
        self.globals.insert(location, var);
        Ok(())
    }

    /// Access the global variable at this location if it exists and is visible
    /// from the given `current_program`.
    pub fn lookup_global(&self, current_program: Symbol, location: &Location) -> Option<&VariableSymbol> {
        if self.is_visible(current_program, &location.program) { self.globals.get(location) } else { None }
    }

    /// Sets the type of a local using its name. Returns `false` if the local is not found.
    pub fn set_local_type(&mut self, name: Symbol, ty: Type) -> bool {
        let mut current = self.local.as_ref();

        while let Some(table) = current {
            let mut inner = table.inner.borrow_mut();

            if let Some(sym) = inner.variables.get_mut(&name) {
                sym.type_ = Some(ty);
                return true;
            }

            current = inner.parent.and_then(|id| self.all_locals.get(&id));
        }

        false
    }

    /// Sets the type of a global using its location. Returns `false` if the global is not found.
    pub fn set_global_type(&mut self, location: &Location, ty: Type) -> bool {
        if let Some(sym) = self.globals.get_mut(location) {
            sym.type_ = Some(ty);
            return true;
        }
        false
    }

    pub fn emit_shadow_error(name: Symbol, span: Span, prev_span: Span) -> LeoError {
        AstError::name_defined_multiple_times(name, span)
            .with_labels(vec![
                Label::new(format!("previous definition of `{name}` here"), prev_span).with_color(Color::Blue),
                Label::new(format!("`{name}` redefined here"), span),
            ])
            .into()
    }

    fn check_shadow_global(&self, location: &Location, span: Span) -> Result<()> {
        let name = location.path.last().expect("location path must have at least one segment.");

        self.functions
            .get(location)
            .map(|f| f.function.identifier.span)
            .or_else(|| self.records.get(location).map(|r| r.identifier.span))
            .or_else(|| self.structs.get(location).map(|s| s.identifier.span))
            .or_else(|| self.globals.get(location).map(|g| g.span))
            .map_or_else(|| Ok(()), |prev_span| Err(Self::emit_shadow_error(*name, span, prev_span)))
    }

    fn check_shadow_variable(&self, program: Symbol, path: &[Symbol], span: Span) -> Result<()> {
        let mut current = self.local.as_ref();

        while let Some(table) = current {
            if let [name] = &path
                && let Some(var_symbol) = table.inner.borrow().variables.get(name)
            {
                return Err(Self::emit_shadow_error(*name, span, var_symbol.span));
            }
            current = table.inner.borrow().parent.map(|id| self.all_locals.get(&id).expect("Parent should exist."));
        }

        self.check_shadow_global(&Location::new(program, path.to_vec()), span)?;

        Ok(())
    }

    /// Insert a variable into the current scope.
    pub fn insert_variable(&mut self, program: Symbol, path: &[Symbol], var: VariableSymbol) -> Result<()> {
        self.check_shadow_variable(program, path, var.span)?;

        if let Some(table) = self.local.as_mut() {
            let [name] = &path else { panic!("Local variables cannot have paths with more than 1 segment.") };
            table.inner.borrow_mut().variables.insert(*name, var);
        } else {
            self.globals.insert(Location::new(program, path.to_vec()), var);
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
        let callee_location = Location::new(callee.program, callee.path);

        if let Some(func) = self.functions.get_mut(&caller) {
            func.finalizer = Some(Finalizer { location: callee_location, future_inputs, inferred_inputs });
            Ok(())
        } else {
            Err(AstError::function_not_found(caller.path.iter().format("::")).into())
        }
    }
}
