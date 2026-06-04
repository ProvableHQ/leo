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

//! Interface-compatibility checking between two program ABIs.
//!
//! A candidate program is *compatible* with an interface standard when every item the standard
//! declares is present in the candidate with an identical definition — that is, when the
//! standard's public interface is a subset of the candidate's. The candidate may declare
//! additional items beyond the standard.

use leo_abi_types as abi;

/// Returns the list of reasons `candidate` is not compatible with the interface standard
/// `standard`. An empty list means the candidate is compatible. Each item the standard declares
/// must appear in the candidate (by name, or by path for records and structs) with a matching
/// definition.
///
/// Type references are compared relative to each side's owning program: a reference to a type
/// defined in the standard's own program matches a reference to the corresponding type in the
/// candidate's own program (see [`ref_owner`]). References to a third, external program must name
/// that same program on both sides.
pub fn check_compatibility(candidate: &abi::Program, standard: &abi::Program) -> Vec<String> {
    let mut problems = Vec::new();
    let (ch, sh) = (candidate.program.as_str(), standard.program.as_str());

    for s in &standard.functions {
        match candidate.functions.iter().find(|c| c.name == s.name) {
            None => problems.push(format!("missing function `{}`", s.name)),
            Some(c) if function_compatible(c, ch, s, sh) => {}
            Some(_) => problems.push(format!("function `{}` differs", s.name)),
        }
    }

    for s in &standard.views {
        match candidate.views.iter().find(|c| c.name == s.name) {
            None => problems.push(format!("missing view `{}`", s.name)),
            Some(c) if function_compatible(c, ch, s, sh) => {}
            Some(_) => problems.push(format!("view `{}` differs", s.name)),
        }
    }

    for s in &standard.mappings {
        match candidate.mappings.iter().find(|c| c.name == s.name) {
            None => problems.push(format!("missing mapping `{}`", s.name)),
            Some(c) if mapping_compatible(c, ch, s, sh) => {}
            Some(_) => problems.push(format!("mapping `{}` differs", s.name)),
        }
    }

    for s in &standard.storage_variables {
        match candidate.storage_variables.iter().find(|c| c.name == s.name) {
            None => problems.push(format!("missing storage variable `{}`", s.name)),
            Some(c) if storage_type_compatible(&c.ty, ch, &s.ty, sh) => {}
            Some(_) => problems.push(format!("storage variable `{}` differs", s.name)),
        }
    }

    for s in &standard.records {
        match candidate.records.iter().find(|c| c.path == s.path) {
            None => problems.push(format!("missing record `{}`", s.path.join("::"))),
            Some(c) if record_compatible(c, ch, s, sh) => {}
            Some(_) => problems.push(format!("record `{}` differs", s.path.join("::"))),
        }
    }

    for s in &standard.structs {
        match candidate.structs.iter().find(|c| c.path == s.path) {
            None => problems.push(format!("missing struct `{}`", s.path.join("::"))),
            Some(c) if struct_compatible(c, ch, s, sh) => {}
            Some(_) => problems.push(format!("struct `{}` differs", s.path.join("::"))),
        }
    }

    problems
}

/// Classifies a reference's `program` qualifier relative to its holding program `holder`:
/// `None` for a local reference (unqualified, or naming `holder` itself) and `Some(name)` for an
/// external one. Comparing two references by their owner lets a self-reference in the standard
/// match a self-reference in the candidate even though the two programs have different names.
fn ref_owner<'a>(program: &'a Option<String>, holder: &str) -> Option<&'a str> {
    match program.as_deref() {
        Some(p) if p != holder => Some(p),
        _ => None,
    }
}

fn function_compatible(c: &abi::Function, ch: &str, s: &abi::Function, sh: &str) -> bool {
    c.inputs.len() == s.inputs.len()
        && c.outputs.len() == s.outputs.len()
        && c.inputs.iter().zip(&s.inputs).all(|(a, b)| input_compatible(a, ch, b, sh))
        && c.outputs.iter().zip(&s.outputs).all(|(a, b)| output_compatible(a, ch, b, sh))
}

fn input_compatible(c: &abi::FunctionInput, ch: &str, s: &abi::FunctionInput, sh: &str) -> bool {
    use abi::FunctionInput::{DynamicRecord, Plaintext, Record};
    match (c, s) {
        (Plaintext { ty: a, mode: ma }, Plaintext { ty: b, mode: mb }) => {
            ma == mb && plaintext_compatible(a, ch, b, sh)
        }
        (Record(a), Record(b)) => a.path == b.path && ref_owner(&a.program, ch) == ref_owner(&b.program, sh),
        (DynamicRecord, DynamicRecord) => true,
        _ => false,
    }
}

fn output_compatible(c: &abi::FunctionOutput, ch: &str, s: &abi::FunctionOutput, sh: &str) -> bool {
    use abi::FunctionOutput::{DynamicRecord, Final, Plaintext, Record};
    match (c, s) {
        (Plaintext { ty: a, mode: ma }, Plaintext { ty: b, mode: mb }) => {
            ma == mb && plaintext_compatible(a, ch, b, sh)
        }
        (Record(a), Record(b)) => a.path == b.path && ref_owner(&a.program, ch) == ref_owner(&b.program, sh),
        (Final, Final) | (DynamicRecord, DynamicRecord) => true,
        _ => false,
    }
}

fn mapping_compatible(c: &abi::Mapping, ch: &str, s: &abi::Mapping, sh: &str) -> bool {
    plaintext_compatible(&c.key, ch, &s.key, sh) && plaintext_compatible(&c.value, ch, &s.value, sh)
}

fn record_compatible(c: &abi::Record, ch: &str, s: &abi::Record, sh: &str) -> bool {
    c.fields.len() == s.fields.len()
        && c.fields
            .iter()
            .zip(&s.fields)
            .all(|(a, b)| a.name == b.name && a.mode == b.mode && plaintext_compatible(&a.ty, ch, &b.ty, sh))
}

fn struct_compatible(c: &abi::Struct, ch: &str, s: &abi::Struct, sh: &str) -> bool {
    c.fields.len() == s.fields.len()
        && c.fields.iter().zip(&s.fields).all(|(a, b)| a.name == b.name && plaintext_compatible(&a.ty, ch, &b.ty, sh))
}

fn storage_type_compatible(c: &abi::StorageType, ch: &str, s: &abi::StorageType, sh: &str) -> bool {
    use abi::StorageType::{Plaintext, Vector};
    match (c, s) {
        (Plaintext(a), Plaintext(b)) => plaintext_compatible(a, ch, b, sh),
        (Vector(a), Vector(b)) => storage_type_compatible(a, ch, b, sh),
        _ => false,
    }
}

fn plaintext_compatible(c: &abi::Plaintext, ch: &str, s: &abi::Plaintext, sh: &str) -> bool {
    use abi::Plaintext::{Array, Optional, Primitive, Struct};
    match (c, s) {
        (Primitive(a), Primitive(b)) => a == b,
        (Array(a), Array(b)) => a.length == b.length && plaintext_compatible(&a.element, ch, &b.element, sh),
        (Optional(a), Optional(b)) => plaintext_compatible(&a.0, ch, &b.0, sh),
        (Struct(a), Struct(b)) => a.path == b.path && ref_owner(&a.program, ch) == ref_owner(&b.program, sh),
        _ => false,
    }
}
