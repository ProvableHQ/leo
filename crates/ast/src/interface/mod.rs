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

use crate::{Identifier, Location, Node, NodeID, TypeKind, indent_display::Indent};
use leo_span::{Span, Symbol};
use serde::Serialize;
use std::fmt;

pub use prototypes::{FunctionPrototype, MappingPrototype, RecordPrototype, StorageVariablePrototype};

mod prototypes;

/// An interface definition.
#[derive(Clone, Default, Serialize)]
pub struct Interface {
    /// Whether the `export` keyword was written on this interface. `None` when
    /// visibility doesn't apply (program-block interfaces, which are always
    /// reachable).
    pub is_exported: Option<bool>,
    /// The interface identifier, e.g., `Foo` in `interface Foo { ... }`.
    pub identifier: Identifier,
    /// The interfaces this interface inherits from (supports multiple inheritance)
    pub parents: Vec<(Span, TypeKind)>,
    /// The entire span of the interface definition.
    pub span: Span,
    /// The ID of the node.
    pub id: NodeID,
    /// A vector of function prototypes.
    pub functions: Vec<(Symbol, FunctionPrototype)>,
    /// A vector of record prototypes.
    pub records: Vec<(Symbol, RecordPrototype)>,
    /// A vector of mapping prototypes.
    pub mappings: Vec<MappingPrototype>,
    /// A vector of storage variable prototypes.
    pub storages: Vec<StorageVariablePrototype>,
}

impl Interface {
    pub fn name(&self) -> Symbol {
        self.identifier.name
    }

    /// Returns `true` if `ty` resolves to a record declared directly in this interface.
    ///
    /// `interface_location` must be the canonical location used to retrieve this interface.
    /// This context is required because an `Interface` value does not carry its defining
    /// program or containing module path.
    /// Record identity includes the defining program, module path, and record name.
    /// Inherited record prototypes are not included.
    pub fn is_record_type(&self, ty: &TypeKind, interface_location: &Location) -> bool {
        let TypeKind::Composite(composite) = ty else {
            return false;
        };
        let Some(record_location) = composite.path.try_global_location() else {
            return false;
        };
        let Some(&record_name) = record_location.path.last() else {
            return false;
        };

        record_location.program == interface_location.program
            && record_location.module_path() == interface_location.module_path()
            && self.records.iter().any(|(name, _)| *name == record_name)
    }
}

impl PartialEq for Interface {
    fn eq(&self, other: &Self) -> bool {
        self.identifier == other.identifier
    }
}

impl Eq for Interface {}

impl fmt::Debug for Interface {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{self}")
    }
}

impl fmt::Display for Interface {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_exported == Some(true) {
            write!(f, "export ")?;
        }
        writeln!(
            f,
            "interface {}{} {{",
            self.identifier,
            if self.parents.is_empty() {
                String::new()
            } else {
                format!(" : {}", self.parents.iter().map(|(_, p)| p.to_string()).collect::<Vec<_>>().join(" + "))
            }
        )?;
        for (_, fun_prot) in &self.functions {
            writeln!(f, "{}", Indent(fun_prot))?;
        }
        for (_, rec_prot) in &self.records {
            writeln!(f, "{}", Indent(rec_prot))?;
        }
        write!(f, "}}")
    }
}

crate::simple_node_impl!(Interface);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{CompositeType, Path};
    use leo_span::create_session_if_not_set_then;

    fn composite_type(program: Symbol, path: &[Symbol]) -> TypeKind {
        let identifier = Identifier::new(*path.last().expect("test paths are non-empty"), NodeID::default());
        CompositeType {
            path: Path::new(None, Vec::new(), identifier, Default::default(), NodeID::default())
                .to_global(Location::new(program, path.to_vec())),
            const_arguments: Vec::new(),
        }
        .into()
    }

    #[test]
    fn record_type_matching_uses_complete_location_identity() {
        create_session_if_not_set_then(|_| {
            let library = Symbol::intern("ops_lib");
            let other_library = Symbol::intern("other_lib");
            let algorithms = Symbol::intern("algorithms");
            let nested = Symbol::intern("nested");
            let sibling = Symbol::intern("sibling");
            let processor = Symbol::intern("Processor");
            let token = Symbol::intern("Token");
            let undeclared = Symbol::intern("Undeclared");
            let interface = Interface { records: vec![(token, RecordPrototype::default())], ..Default::default() };

            let root_interface = Location::new(library, vec![processor]);
            let module_interface = Location::new(library, vec![algorithms, processor]);
            let nested_interface = Location::new(library, vec![algorithms, nested, processor]);

            assert!(interface.is_record_type(&composite_type(library, &[token]), &root_interface));
            assert!(interface.is_record_type(&composite_type(library, &[algorithms, token]), &module_interface));
            assert!(
                interface.is_record_type(&composite_type(library, &[algorithms, nested, token]), &nested_interface)
            );
            assert!(!interface.is_record_type(&composite_type(other_library, &[algorithms, token]), &module_interface));
            assert!(!interface.is_record_type(&composite_type(library, &[sibling, token]), &module_interface));
            assert!(!interface.is_record_type(&composite_type(library, &[algorithms, undeclared]), &module_interface));
            assert!(!interface.is_record_type(&TypeKind::Boolean, &module_interface));
        });
    }
}
