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

//! Unit tests for ABI generation and compatibility checking.

use super::find_composite_at_location;
use crate::compatibility::check_compatibility;

use abi::{
    Function,
    FunctionInput,
    FunctionOutput,
    Mapping,
    Mode,
    Plaintext,
    Primitive,
    Program,
    Record,
    RecordField,
    RecordRef,
    StorageType,
    StorageVariable,
    StructRef,
    UInt,
};
use indexmap::{IndexMap, IndexSet};
use leo_abi_types as abi;
use leo_ast::{self as ast, Identifier, NodeID};
use leo_span::Symbol;

fn u64t() -> Plaintext {
    Plaintext::Primitive(Primitive::UInt(UInt::U64))
}

fn u32t() -> Plaintext {
    Plaintext::Primitive(Primitive::UInt(UInt::U32))
}

fn addr() -> Plaintext {
    Plaintext::Primitive(Primitive::Address)
}

fn input(ty: Plaintext, mode: Mode) -> FunctionInput {
    FunctionInput::Plaintext { ty, mode }
}

fn output(ty: Plaintext, mode: Mode) -> FunctionOutput {
    FunctionOutput::Plaintext { ty, mode }
}

fn func(name: &str, inputs: Vec<FunctionInput>, outputs: Vec<FunctionOutput>) -> Function {
    Function { name: name.to_string(), inputs, outputs }
}

fn program(name: &str, functions: Vec<Function>) -> Program {
    Program {
        program: name.to_string(),
        structs: Vec::new(),
        records: Vec::new(),
        mappings: Vec::new(),
        storage_variables: Vec::new(),
        functions,
        views: Vec::new(),
    }
}

/// `transfer(address.private, address.private, u64.public) -> ()`.
fn transfer() -> Function {
    func(
        "transfer",
        vec![input(addr(), Mode::Private), input(addr(), Mode::Private), input(u64t(), Mode::Public)],
        vec![],
    )
}

/// A view `total_supply() -> u64.public`. Views are represented as [`Function`]s.
fn total_supply() -> Function {
    func("total_supply", vec![], vec![output(u64t(), Mode::Public)])
}

fn storage(name: &str, ty: Plaintext) -> StorageVariable {
    StorageVariable { name: name.into(), ty: StorageType::Plaintext(ty) }
}

/// A plaintext reference to struct `path` defined in `program`.
fn struct_ref(path: &str, program: &str) -> Plaintext {
    Plaintext::Struct(StructRef { path: vec![path.into()], program: Some(program.into()) })
}

/// A function input that is a record `path` defined in `program`.
fn record_input(path: &str, program: &str) -> FunctionInput {
    FunctionInput::Record(RecordRef { path: vec![path.into()], program: Some(program.into()) })
}

fn ast_composite(name: Symbol, is_record: bool, field: Symbol) -> ast::Composite {
    ast::Composite {
        is_exported: None,
        identifier: Identifier::new(name, NodeID::default()),
        const_parameters: Vec::new(),
        members: vec![ast::Member {
            mode: ast::Mode::None,
            identifier: Identifier::new(field, NodeID::default()),
            type_: ast::TypeNode::unchecked(ast::TypeKind::Boolean, Default::default()),
            span: Default::default(),
            id: NodeID::default(),
        }],
        is_record,
        span: Default::default(),
        id: NodeID::default(),
    }
}

fn ast_module(unit_name: Symbol, path: &[Symbol], composites: Vec<(Symbol, ast::Composite)>) -> ast::Module {
    ast::Module {
        unit_name,
        path: path.to_vec(),
        consts: Vec::new(),
        composites,
        functions: Vec::new(),
        interfaces: Vec::new(),
    }
}

fn ast_program_scope(program_id: ast::ProgramId, composites: Vec<(Symbol, ast::Composite)>) -> ast::ProgramScope {
    ast::ProgramScope {
        program_id,
        parents: Vec::new(),
        consts: Vec::new(),
        composites,
        mappings: Vec::new(),
        storage_variables: Vec::new(),
        functions: Vec::new(),
        interfaces: Vec::new(),
        constructor: None,
        span: Default::default(),
    }
}

fn leo_stub(
    program_name: Symbol,
    types: Symbol,
    sibling: Symbol,
    types_field: Symbol,
    sibling_field: Symbol,
) -> (Symbol, ast::Stub) {
    let program_id = ast::ProgramId::from(Identifier::new(program_name, NodeID::default()));
    let program_symbol = program_id.as_symbol();
    let token = Symbol::intern("Token");
    let root_field = Symbol::intern("root_field");

    let mut program = ast::Program::default();
    program
        .program_scopes
        .insert(program_symbol, ast_program_scope(program_id, vec![(token, ast_composite(token, true, root_field))]));
    program.modules.insert(
        vec![types],
        ast_module(program_name, &[types], vec![(token, ast_composite(token, false, types_field))]),
    );
    program.modules.insert(
        vec![sibling],
        ast_module(program_name, &[sibling], vec![(token, ast_composite(token, false, sibling_field))]),
    );

    (program_symbol, ast::Stub::FromLeo { program, parents: IndexSet::new() })
}

fn library_stub(
    library_name: Symbol,
    types: Symbol,
    sibling: Symbol,
    types_field: Symbol,
    sibling_field: Symbol,
) -> ast::Stub {
    let token = Symbol::intern("Token");
    let root_field = Symbol::intern("root_field");
    let mut modules = IndexMap::new();
    modules.insert(
        vec![types],
        ast_module(library_name, &[types], vec![(token, ast_composite(token, false, types_field))]),
    );
    modules.insert(
        vec![sibling],
        ast_module(library_name, &[sibling], vec![(token, ast_composite(token, false, sibling_field))]),
    );

    ast::Stub::FromLibrary {
        library: ast::Library {
            name: library_name,
            modules,
            consts: Vec::new(),
            structs: vec![(token, ast_composite(token, false, root_field))],
            functions: Vec::new(),
            interfaces: Vec::new(),
            stubs: IndexMap::new(),
        },
        parents: IndexSet::new(),
    }
}

fn aleo_stub() -> ast::Stub {
    let token = Symbol::intern("Token");
    let root_field = Symbol::intern("root_field");
    let mut aleo_program = ast::AleoProgram::default();
    aleo_program.composites.push((token, ast_composite(token, true, root_field)));
    ast::Stub::FromAleo { program: aleo_program, parents: IndexSet::new() }
}

fn lookup_composite(
    location: ast::Location,
    current_program: Symbol,
    composites: &[(Symbol, ast::Composite)],
    modules: &IndexMap<Vec<Symbol>, ast::Module>,
    stubs: &IndexMap<Symbol, ast::Stub>,
) -> Option<Symbol> {
    find_composite_at_location(&location, current_program, composites, modules, stubs)
        .and_then(|composite| composite.members.first())
        .map(|member| member.identifier.name)
}

#[test]
fn composite_lookup_uses_exact_locations_for_all_sources() {
    leo_span::create_session_if_not_set_then(|_| {
        let caller = Symbol::intern("caller.aleo");
        let types = Symbol::intern("types");
        let sibling = Symbol::intern("sibling");
        let token = Symbol::intern("Token");
        let undeclared = Symbol::intern("Undeclared");
        let root_field = Symbol::intern("root_field");
        let types_field = Symbol::intern("types_field");
        let sibling_field = Symbol::intern("sibling_field");
        let other_types_field = Symbol::intern("other_types_field");

        let current_composites = vec![(token, ast_composite(token, true, root_field))];
        let mut current_modules = IndexMap::new();
        current_modules
            .insert(vec![types], ast_module(caller, &[types], vec![(token, ast_composite(token, false, types_field))]));
        current_modules.insert(
            vec![sibling],
            ast_module(caller, &[sibling], vec![(token, ast_composite(token, false, sibling_field))]),
        );

        let mut stubs = IndexMap::new();
        let (leo_program, leo) = leo_stub(Symbol::intern("leo_pkg"), types, sibling, types_field, sibling_field);
        stubs.insert(leo_program, leo);
        let (other_program, other) =
            leo_stub(Symbol::intern("other_pkg"), types, sibling, other_types_field, sibling_field);
        stubs.insert(other_program, other);
        let library_program = Symbol::intern("shapes_lib");
        stubs.insert(library_program, library_stub(library_program, types, sibling, types_field, sibling_field));
        let aleo_program = Symbol::intern("credits.aleo");
        stubs.insert(aleo_program, aleo_stub());

        let lookup = |location| lookup_composite(location, caller, &current_composites, &current_modules, &stubs);

        assert_eq!(lookup(ast::Location::new(caller, vec![types, token])), Some(types_field));
        assert_eq!(lookup(ast::Location::new(caller, vec![token])), Some(root_field));
        assert_eq!(lookup(ast::Location::new(caller, vec![sibling, token])), Some(sibling_field));

        assert_eq!(lookup(ast::Location::new(leo_program, vec![token])), Some(root_field));
        assert_eq!(lookup(ast::Location::new(leo_program, vec![types, token])), Some(types_field));
        assert_eq!(lookup(ast::Location::new(other_program, vec![types, token])), Some(other_types_field));

        assert_eq!(lookup(ast::Location::new(library_program, vec![types, token])), Some(types_field));
        assert_eq!(lookup(ast::Location::new(library_program, vec![token])), Some(root_field));
        assert_eq!(lookup(ast::Location::new(library_program, vec![sibling, token])), Some(sibling_field));

        assert_eq!(lookup(ast::Location::new(aleo_program, vec![token])), Some(root_field));
        assert_eq!(lookup(ast::Location::new(aleo_program, vec![types, token])), None);
        assert_eq!(lookup(ast::Location::new(leo_program, vec![types, undeclared])), None);
        assert_eq!(lookup(ast::Location::new(Symbol::intern("missing.aleo"), vec![token])), None);
    });
}

#[test]
fn identical_interfaces_are_compatible() {
    let problems =
        check_compatibility(&program("token.aleo", vec![transfer()]), &program("std.aleo", vec![transfer()]));
    assert!(problems.is_empty(), "{problems:?}");
}

#[test]
fn candidate_superset_is_compatible() {
    let extra = func("mint", vec![input(addr(), Mode::Private), input(u64t(), Mode::Public)], vec![]);
    let candidate = program("token.aleo", vec![transfer(), extra]);
    assert!(check_compatibility(&candidate, &program("std.aleo", vec![transfer()])).is_empty());
}

#[test]
fn missing_function_is_reported() {
    let problems = check_compatibility(&program("token.aleo", vec![]), &program("std.aleo", vec![transfer()]));
    assert_eq!(problems, vec!["missing function `transfer`".to_string()]);
}

#[test]
fn input_type_mismatch_is_reported() {
    // Same name, but `amount` is u32 instead of u64.
    let candidate = program("token.aleo", vec![func(
        "transfer",
        vec![input(addr(), Mode::Private), input(addr(), Mode::Private), input(u32t(), Mode::Public)],
        vec![],
    )]);
    let problems = check_compatibility(&candidate, &program("std.aleo", vec![transfer()]));
    assert_eq!(problems, vec!["function `transfer` differs".to_string()]);
}

#[test]
fn input_mode_mismatch_is_reported() {
    // `amount` is private instead of public.
    let candidate = program("token.aleo", vec![func(
        "transfer",
        vec![input(addr(), Mode::Private), input(addr(), Mode::Private), input(u64t(), Mode::Private)],
        vec![],
    )]);
    assert!(!check_compatibility(&candidate, &program("std.aleo", vec![transfer()])).is_empty());
}

#[test]
fn output_mismatch_is_reported() {
    let standard = program("std.aleo", vec![func("balance_of", vec![input(addr(), Mode::Private)], vec![output(
        u64t(),
        Mode::Private,
    )])]);
    let candidate = program("token.aleo", vec![func("balance_of", vec![input(addr(), Mode::Private)], vec![output(
        u32t(),
        Mode::Private,
    )])]);
    assert!(!check_compatibility(&candidate, &standard).is_empty());
}

#[test]
fn mapping_value_mismatch_is_reported() {
    let mut standard = program("std.aleo", vec![]);
    standard.mappings.push(Mapping { name: "balances".into(), key: addr(), value: u64t() });
    let mut candidate = program("token.aleo", vec![]);
    candidate.mappings.push(Mapping { name: "balances".into(), key: addr(), value: u32t() });
    assert_eq!(check_compatibility(&candidate, &standard), vec!["mapping `balances` differs".to_string()]);
}

#[test]
fn record_field_layout_mismatch_is_reported() {
    let field = |name: &str, ty: Plaintext| RecordField { name: name.into(), ty, mode: Mode::Private };
    let mut standard = program("std.aleo", vec![]);
    standard
        .records
        .push(Record { path: vec!["Token".into()], fields: vec![field("owner", addr()), field("amount", u64t())] });
    let mut candidate = program("token.aleo", vec![]);
    candidate
        .records
        .push(Record { path: vec!["Token".into()], fields: vec![field("owner", addr()), field("amount", u32t())] });
    assert_eq!(check_compatibility(&candidate, &standard), vec!["record `Token` differs".to_string()]);
}

#[test]
fn missing_record_is_reported() {
    let field = RecordField { name: "owner".into(), ty: addr(), mode: Mode::Private };
    let mut standard = program("std.aleo", vec![]);
    standard.records.push(Record { path: vec!["Token".into()], fields: vec![field] });
    let problems = check_compatibility(&program("token.aleo", vec![]), &standard);
    assert_eq!(problems, vec!["missing record `Token`".to_string()]);
}

#[test]
fn identical_view_is_compatible() {
    let mut standard = program("std.aleo", vec![]);
    standard.views.push(total_supply());
    let mut candidate = program("token.aleo", vec![]);
    candidate.views.push(total_supply());
    assert!(check_compatibility(&candidate, &standard).is_empty());
}

#[test]
fn missing_view_is_reported() {
    let mut standard = program("std.aleo", vec![]);
    standard.views.push(total_supply());
    let problems = check_compatibility(&program("token.aleo", vec![]), &standard);
    assert_eq!(problems, vec!["missing view `total_supply`".to_string()]);
}

#[test]
fn view_mismatch_is_reported() {
    let mut standard = program("std.aleo", vec![]);
    standard.views.push(total_supply());
    let mut candidate = program("token.aleo", vec![]);
    // Same name, but returns u32 instead of u64.
    candidate.views.push(func("total_supply", vec![], vec![output(u32t(), Mode::Public)]));
    assert_eq!(check_compatibility(&candidate, &standard), vec!["view `total_supply` differs".to_string()]);
}

#[test]
fn identical_storage_variable_is_compatible() {
    let mut standard = program("std.aleo", vec![]);
    standard.storage_variables.push(storage("total", u64t()));
    let mut candidate = program("token.aleo", vec![]);
    candidate.storage_variables.push(storage("total", u64t()));
    assert!(check_compatibility(&candidate, &standard).is_empty());
}

#[test]
fn missing_storage_variable_is_reported() {
    let mut standard = program("std.aleo", vec![]);
    standard.storage_variables.push(storage("total", u64t()));
    let problems = check_compatibility(&program("token.aleo", vec![]), &standard);
    assert_eq!(problems, vec!["missing storage variable `total`".to_string()]);
}

#[test]
fn storage_variable_mismatch_is_reported() {
    let mut standard = program("std.aleo", vec![]);
    standard.storage_variables.push(storage("total", u64t()));
    let mut candidate = program("token.aleo", vec![]);
    candidate.storage_variables.push(storage("total", u32t()));
    assert_eq!(check_compatibility(&candidate, &standard), vec!["storage variable `total` differs".to_string()]);
}

#[test]
fn self_record_reference_is_program_relative() {
    // Each program declares a local `Token` record qualified with its own program id. A
    // self-reference in the standard must match a self-reference in the candidate.
    let standard = program("std.aleo", vec![func("wrap", vec![record_input("Token", "std.aleo")], vec![])]);
    let candidate = program("token.aleo", vec![func("wrap", vec![record_input("Token", "token.aleo")], vec![])]);
    assert!(check_compatibility(&candidate, &standard).is_empty());
}

#[test]
fn self_struct_reference_is_program_relative() {
    let standard =
        program("std.aleo", vec![func("echo", vec![input(struct_ref("Point", "std.aleo"), Mode::Private)], vec![])]);
    let candidate = program("token.aleo", vec![func(
        "echo",
        vec![input(struct_ref("Point", "token.aleo"), Mode::Private)],
        vec![],
    )]);
    assert!(check_compatibility(&candidate, &standard).is_empty());
}

#[test]
fn unqualified_and_self_qualified_refs_match() {
    // A local reference may arrive unqualified (`None`, e.g. from disassembled bytecode) or
    // qualified with the owning program (e.g. from Leo source). Both are local and must match.
    let standard = program("std.aleo", vec![func(
        "wrap",
        vec![FunctionInput::Record(RecordRef { path: vec!["Token".into()], program: None })],
        vec![],
    )]);
    let candidate = program("token.aleo", vec![func("wrap", vec![record_input("Token", "token.aleo")], vec![])]);
    assert!(check_compatibility(&candidate, &standard).is_empty());
}

#[test]
fn external_record_reference_must_match_program() {
    let standard = program("std.aleo", vec![func("wrap", vec![record_input("Coin", "credits.aleo")], vec![])]);

    // A reference to a different external program is incompatible.
    let mismatched = program("token.aleo", vec![func("wrap", vec![record_input("Coin", "other.aleo")], vec![])]);
    assert_eq!(check_compatibility(&mismatched, &standard), vec!["function `wrap` differs".to_string()]);

    // A reference to the same external program is compatible.
    let matched = program("token.aleo", vec![func("wrap", vec![record_input("Coin", "credits.aleo")], vec![])]);
    assert!(check_compatibility(&matched, &standard).is_empty());
}

/// End-to-end checks that disassemble real Aleo bytecode, generate ABIs, and run the
/// compatibility check on them. Gated on `aleo-bytecode` since they use `leo-disassembler`.
#[cfg(feature = "aleo-bytecode")]
mod bytecode {
    use super::*;

    use leo_span::create_session_if_not_set_then;

    /// A standard `transfer:(address, u64) -> address` that the candidate also declares (alongside an extra
    /// `mint`) must be reported as compatible — a superset satisfies the standard.
    const STANDARD_SRC: &str = "\
program std_iface.aleo;

function transfer:
    input r0 as address.private;
    input r1 as u64.public;
    output r0 as address.private;
";

    const CANDIDATE_SRC: &str = "\
program token.aleo;

function transfer:
    input r0 as address.private;
    input r1 as u64.public;
    output r0 as address.private;

function mint:
    input r0 as address.private;
    output r0 as address.private;
";

    /// Disassembles `src` (no imports) and generates its ABI.
    fn abi_of(name: &str, src: &str) -> Program {
        let aleo = leo_disassembler::disassemble_from_str_for_network(name, src, leo_ast::NetworkName::TestnetV0)
            .expect("expected valid Aleo bytecode");
        crate::aleo::generate(&aleo)
    }

    #[test]
    fn compat_superset_is_compatible() {
        create_session_if_not_set_then(|_| {
            let standard = abi_of("std_iface.aleo", STANDARD_SRC);
            let candidate = abi_of("token.aleo", CANDIDATE_SRC);
            let problems = check_compatibility(&candidate, &standard);
            assert!(problems.is_empty(), "expected compatible: {problems:?}");
        });
    }

    #[test]
    fn compat_missing_function_is_incompatible() {
        create_session_if_not_set_then(|_| {
            let standard_src = "\
program std_iface.aleo;

function burn:
    input r0 as u64.public;
    output r0 as u64.public;
";
            let standard = abi_of("std_iface.aleo", standard_src);
            let candidate = abi_of("token.aleo", CANDIDATE_SRC);
            let problems = check_compatibility(&candidate, &standard);
            assert_eq!(problems, vec!["missing function `burn`".to_string()]);
        });
    }
}
