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

//! Tests for dependency resolution, trusted network bytecode, lock files, manifests, and workspaces.

use crate::{
    GitReference,
    LOCK_FILENAME,
    Lock,
    MANIFEST_FILENAME,
    Package,
    WORKSPACE_MANIFEST_FILENAME,
    git::resolve,
    test_util::{
        file_url,
        fixture_repo,
        git_available,
        init_repo,
        manifest_json,
        read_manifest,
        run_git,
        unique_dir,
        write_consumer,
        write_file,
        write_library,
        write_program,
    },
};

use leo_span::Symbol;

use snarkvm::prelude::{CanaryV0, MainnetV0, Program, TestnetV0};
use std::{
    io::{Read, Write},
    net::TcpListener,
    path::Path,
    thread::{self, JoinHandle},
    time::{Duration, Instant},
};

const TRUSTED_TOKEN: &str =
    "program token.aleo;\nfunction reveal:\n    input r0 as u32.private;\n    output r0 as u32.private;\n";

fn network_pin(bytecode: &str, edition: u16) -> serde_json::Value {
    let program: Program<TestnetV0> = bytecode.parse().expect("fixture bytecode must parse");
    serde_json::json!({
        "name": program.id().to_string(),
        "network": "testnet",
        "edition": edition,
        "checksum": program.to_checksum().map(|byte| *byte),
    })
}

fn write_network_lock(directory: &Path, pins: &[serde_json::Value]) -> Lock {
    let contents = serde_json::json!({"version": 2, "git": [], "network": pins});
    write_file(&directory.join(LOCK_FILENAME), &contents.to_string());
    Lock::read(directory).expect("fixture lock must be valid")
}

fn network_response(responses: &[(&str, &str, &str)]) -> (String, JoinHandle<Vec<String>>) {
    let listener = TcpListener::bind("127.0.0.1:0").expect("fixture listener must bind");
    listener.set_nonblocking(true).expect("fixture listener must be nonblocking");
    let endpoint = format!("http://{}", listener.local_addr().expect("fixture address must exist"));
    let responses: Vec<_> = responses.iter().map(|(path, status, body)| {
        (format!("GET {path} HTTP/1.1\r\n"), format!(
            "HTTP/1.1 {status}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
            body.len()
        ))
    }).collect();
    let handle = thread::spawn(move || {
        let mut requests = Vec::new();
        let mut deadline = Instant::now() + Duration::from_secs(2);
        while Instant::now() < deadline {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    stream.set_nonblocking(false).expect("fixture stream must use blocking I/O");
                    stream.set_read_timeout(Some(Duration::from_secs(1))).expect("read timeout must be set");
                    stream.set_write_timeout(Some(Duration::from_secs(1))).expect("write timeout must be set");
                    let mut request = Vec::new();
                    let mut buffer = [0; 1024];
                    while !request.windows(4).any(|bytes| bytes == b"\r\n\r\n") {
                        let count = stream.read(&mut buffer).expect("fixture request must arrive");
                        assert!(count > 0 && request.len() < 8192, "fixture request must contain bounded headers");
                        request.extend_from_slice(&buffer[..count]);
                    }
                    let request = String::from_utf8(request).expect("fixture request must be UTF-8");
                    let (_, response) = responses
                        .iter()
                        .find(|(prefix, _)| prefix == "GET  HTTP/1.1\r\n" || request.starts_with(prefix))
                        .unwrap_or_else(|| panic!("unexpected request: {request}"));
                    requests.push(request);
                    stream.write_all(response.as_bytes()).expect("fixture response must be sent");
                    deadline = Instant::now() + Duration::from_millis(200);
                    assert!(requests.len() < 10, "fetch must not make unbounded requests");
                }
                Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => {
                    thread::sleep(Duration::from_millis(10));
                }
                Err(error) => panic!("fixture accept failed: {error}"),
            }
        }
        requests
    });
    (endpoint, handle)
}

#[test]
fn first_network_fetch_ignores_unverified_cache_and_records_checksum() {
    leo_span::create_session_if_not_set_then(|_| {
        let home = unique_dir("network-first-fetch");
        let cache = home.join("registry/testnet/token/0/token.aleo");
        write_file(&cache, &TRUSTED_TOKEN.replace("private", "public"));
        let (endpoint, server) = network_response(&[("", "200 OK", TRUSTED_TOKEN)]);
        let mut lock = Lock::default();
        let unit = crate::CompilationUnit::fetch(
            Symbol::intern("token.aleo"),
            Some(0),
            &home,
            leo_ast::NetworkName::TestnetV0,
            &endpoint,
            false,
            0,
            &mut lock,
        )
        .expect("the first download must create its checksum automatically");
        assert_eq!(unit.edition, Some(0));
        assert_eq!(server.join().expect("fixture must finish").len(), 1);
        assert_eq!(std::fs::read_to_string(cache).expect("cache must exist"), TRUSTED_TOKEN);
        assert_eq!(
            serde_json::json!(lock.network_pin("token.aleo", leo_ast::NetworkName::TestnetV0, Some(0))),
            network_pin(TRUSTED_TOKEN, 0),
        );
        std::fs::remove_dir_all(home).expect("test directory must be removed");
    });
}

#[test]
fn cached_network_program_checks_canonical_bytes_on_every_read() {
    leo_span::create_session_if_not_set_then(|_| {
        let home = unique_dir("network-cache-verification");
        let mut lock = write_network_lock(&home, &[network_pin(TRUSTED_TOKEN, 0)]);
        let cache = home.join("registry/testnet/token/0/token.aleo");
        let formatted = format!("// Independent formatting does not change canonical bytes.\n{TRUSTED_TOKEN}\n");
        write_file(&cache, &formatted);
        let unit = crate::CompilationUnit::fetch(
            Symbol::intern("token.aleo"),
            Some(0),
            &home,
            leo_ast::NetworkName::TestnetV0,
            "http://127.0.0.1:1",
            false,
            0,
            &mut lock,
        )
        .expect("approved canonical bytecode must load from cache");
        assert_eq!(unit.edition, Some(0));
        assert!(matches!(unit.data, crate::ProgramData::Bytecode(ref bytecode) if bytecode == &formatted));

        for modified in [
            TRUSTED_TOKEN.replace("u32.private", "u32.public"),
            TRUSTED_TOKEN.replace("output r0", "add r0 1u32 into r1;\n    output r1"),
        ] {
            write_file(&cache, &modified);
            let error = crate::CompilationUnit::fetch(
                Symbol::intern("token.aleo"),
                Some(0),
                &home,
                leo_ast::NetworkName::TestnetV0,
                "http://127.0.0.1:1",
                false,
                0,
                &mut lock,
            )
            .expect_err("changed same-ID bytecode must fail on the next read");
            assert!(error.to_string().contains("checksum"), "{error}");
            assert_eq!(std::fs::read_to_string(&cache).expect("cache must remain readable"), modified);
        }
        std::fs::remove_dir_all(home).expect("test directory must be removed");
    });
}

#[test]
fn inferred_network_edition_comes_from_pin_not_cache() {
    leo_span::create_session_if_not_set_then(|_| {
        let home = unique_dir("network-pinned-edition");
        let mut lock = write_network_lock(&home, &[network_pin(TRUSTED_TOKEN, 2)]);
        write_file(&home.join("registry/testnet/token/2/token.aleo"), TRUSTED_TOKEN);
        write_file(&home.join("registry/testnet/token/99/token.aleo"), &TRUSTED_TOKEN.replace("private", "public"));
        let unit = crate::CompilationUnit::fetch(
            Symbol::intern("token.aleo"),
            None,
            &home,
            leo_ast::NetworkName::TestnetV0,
            "http://127.0.0.1:1",
            false,
            0,
            &mut lock,
        )
        .expect("the trusted edition must load without a latest-edition request");
        assert_eq!(unit.edition, Some(2));
        assert!(matches!(unit.data, crate::ProgramData::Bytecode(ref bytecode) if bytecode == TRUSTED_TOKEN));
        std::fs::remove_dir_all(home).expect("test directory must be removed");
    });
}

#[test]
fn explicit_edition_change_records_fresh_checksum_and_preserves_other_networks() {
    leo_span::create_session_if_not_set_then(|_| {
        let home = unique_dir("network-edition-change");
        let mut mainnet = network_pin(TRUSTED_TOKEN, 2);
        mainnet["network"] = serde_json::json!("mainnet");
        let mut lock = write_network_lock(&home, &[network_pin(TRUSTED_TOKEN, 2), mainnet]);
        let old = lock.clone();
        let updated = TRUSTED_TOKEN.replace("private", "public");
        write_file(&home.join("registry/testnet/token/3/token.aleo"), TRUSTED_TOKEN);
        let (endpoint, server) = network_response(&[("/testnet/program/token.aleo/3", "200 OK", &updated)]);
        let unit = crate::CompilationUnit::fetch(
            Symbol::intern("token.aleo"),
            Some(3),
            &home,
            leo_ast::NetworkName::TestnetV0,
            &endpoint,
            false,
            0,
            &mut lock,
        )
        .expect("an explicit edition change must fetch and record the new edition");
        assert_eq!(unit.edition, Some(3));
        assert_eq!(server.join().expect("fixture must finish").len(), 1);
        lock.carry_over(&old, |_| true);
        lock.write(&home).expect("updated lock must write");
        let lock = Lock::read(&home).expect("updated lock must read");
        assert_eq!(
            serde_json::json!(lock.network_pin("token.aleo", leo_ast::NetworkName::TestnetV0, Some(3))),
            network_pin(&updated, 3),
        );
        assert!(lock.network_pin("token.aleo", leo_ast::NetworkName::TestnetV0, Some(2)).is_none());
        assert_eq!(
            lock.network_pin("token.aleo", leo_ast::NetworkName::MainnetV0, Some(2)),
            old.network_pin("token.aleo", leo_ast::NetworkName::MainnetV0, Some(2)),
        );
        std::fs::remove_dir_all(home).expect("test directory must be removed");
    });
}

#[test]
fn network_fetch_uses_exact_pinned_edition() {
    leo_span::create_session_if_not_set_then(|_| {
        let home = unique_dir("network-exact-edition");
        let mut lock = write_network_lock(&home, &[network_pin(TRUSTED_TOKEN, 7)]);
        let body = serde_json::to_string(TRUSTED_TOKEN).expect("fixture body must serialize");
        let (endpoint, server) = network_response(&[("", "200 OK", &body)]);
        let unit = crate::CompilationUnit::fetch(
            Symbol::intern("token.aleo"),
            None,
            &home,
            leo_ast::NetworkName::TestnetV0,
            &endpoint,
            false,
            0,
            &mut lock,
        )
        .expect("approved response must load");
        let requests = server.join().expect("fixture must finish");
        assert_eq!(requests.len(), 1);
        assert!(requests[0].starts_with("GET /testnet/program/token.aleo/7 HTTP/1.1\r\n"), "{:?}", requests);
        assert_eq!(unit.edition, Some(7));
        assert_eq!(
            std::fs::read_to_string(home.join("registry/testnet/token/7/token.aleo"))
                .expect("approved cache must exist"),
            TRUSTED_TOKEN
        );
        std::fs::remove_dir_all(home).expect("test directory must be removed");
    });
}

#[test]
fn rejected_network_responses_do_not_replace_cache_or_fall_back() {
    leo_span::create_session_if_not_set_then(|_| {
        for (status, bytecode, checksum_source, reason) in [
            ("200 OK", TRUSTED_TOKEN.replace("private", "public"), TRUSTED_TOKEN.to_owned(), "checksum"),
            (
                "200 OK",
                TRUSTED_TOKEN.replace("output r0", "add r0 1u32 into r1;\n    output r1"),
                TRUSTED_TOKEN.to_owned(),
                "checksum",
            ),
            (
                "200 OK",
                TRUSTED_TOKEN.replace("token.aleo", "other.aleo"),
                TRUSTED_TOKEN.replace("token.aleo", "other.aleo"),
                "program ID",
            ),
            ("404 Not Found", "missing edition".to_owned(), TRUSTED_TOKEN.to_owned(), ""),
        ] {
            let home = unique_dir("network-rejected-response");
            let mut pin = network_pin(&checksum_source, 7);
            pin["name"] = serde_json::json!("token.aleo");
            let mut lock = write_network_lock(&home, &[pin]);
            let cache = home.join("registry/testnet/token/7/token.aleo");
            write_file(&cache, TRUSTED_TOKEN);
            let (endpoint, server) = network_response(&[("", status, &bytecode)]);
            let error = crate::CompilationUnit::fetch(
                Symbol::intern("token.aleo"),
                Some(7),
                &home,
                leo_ast::NetworkName::TestnetV0,
                &endpoint,
                true,
                0,
                &mut lock,
            )
            .expect_err("an unapproved response must fail");
            assert!(error.to_string().contains(reason), "{error}");
            let requests = server.join().expect("fixture must finish");
            assert_eq!(requests.len(), 1, "no latest-edition request or unversioned fallback is permitted");
            assert!(requests[0].starts_with("GET /testnet/program/token.aleo/7 HTTP/1.1\r\n"));
            assert_eq!(std::fs::read_to_string(cache).expect("old cache must exist"), TRUSTED_TOKEN);
            std::fs::remove_dir_all(home).expect("test directory must be removed");
        }
    });
}

#[test]
fn rejected_network_response_does_not_create_a_cache() {
    leo_span::create_session_if_not_set_then(|_| {
        let home = unique_dir("network-rejected-new-cache");
        let mut lock = write_network_lock(&home, &[network_pin(TRUSTED_TOKEN, 0)]);
        let (endpoint, server) = network_response(&[("", "200 OK", &TRUSTED_TOKEN.replace("private", "public"))]);
        let error = crate::CompilationUnit::fetch(
            Symbol::intern("token.aleo"),
            Some(0),
            &home,
            leo_ast::NetworkName::TestnetV0,
            &endpoint,
            false,
            0,
            &mut lock,
        )
        .expect_err("an unapproved response must not enter a new cache");
        assert!(error.to_string().contains("checksum"), "{error}");
        assert_eq!(server.join().expect("fixture must finish").len(), 1);
        assert!(!home.join("registry").exists());
        std::fs::remove_dir_all(home).expect("test directory must be removed");
    });
}

#[test]
fn bundled_credits_ignores_hostile_cache() {
    leo_span::create_session_if_not_set_then(|_| {
        let home = unique_dir("network-bundled-credits");
        let hostile = TRUSTED_TOKEN.replace("token.aleo", "credits.aleo");
        for network in
            [leo_ast::NetworkName::MainnetV0, leo_ast::NetworkName::TestnetV0, leo_ast::NetworkName::CanaryV0]
        {
            let expected = match network {
                leo_ast::NetworkName::MainnetV0 => {
                    Program::<MainnetV0>::credits().expect("bundled credits must exist").to_string()
                }
                leo_ast::NetworkName::TestnetV0 => {
                    Program::<TestnetV0>::credits().expect("bundled credits must exist").to_string()
                }
                leo_ast::NetworkName::CanaryV0 => {
                    Program::<CanaryV0>::credits().expect("bundled credits must exist").to_string()
                }
            };
            let cache = home.join(format!("registry/{network}/credits/0/credits.aleo"));
            write_file(&cache, &hostile);
            for no_cache in [false, true] {
                for edition in [None, Some(0), Some(1), Some(u16::MAX)] {
                    let unit = crate::CompilationUnit::fetch(
                        Symbol::intern("credits.aleo"),
                        edition,
                        &home,
                        network,
                        "http://127.0.0.1:1",
                        no_cache,
                        0,
                        &mut Lock::default(),
                    )
                    .expect("bundled credits must work without a pin or endpoint");
                    assert!(matches!(unit.data, crate::ProgramData::Bytecode(ref bytecode) if bytecode == &expected));
                    assert_eq!(unit.edition, Some(0));
                    assert_eq!(std::fs::read_to_string(&cache).expect("hostile cache must remain unchanged"), hostile);
                }
            }
        }
        std::fs::remove_dir_all(home).expect("test directory must be removed");
    });
}

#[test]
fn network_lock_rejects_malformed_and_ambiguous_trust() {
    let directory = unique_dir("network-lock-invalid");
    let pin = network_pin(TRUSTED_TOKEN, 0);
    let mut other_edition = pin.clone();
    other_edition["edition"] = serde_json::json!(1);
    let mut short_checksum = pin.clone();
    short_checksum["checksum"] = serde_json::json!([1, 2, 3]);
    for contents in [
        "{".to_owned(),
        serde_json::json!({"version": 99, "git": []}).to_string(),
        serde_json::json!({"version": 2, "network": [pin.clone(), pin.clone()]}).to_string(),
        serde_json::json!({"version": 2, "network": [pin, other_edition]}).to_string(),
        serde_json::json!({"version": 2, "network": [short_checksum]}).to_string(),
        serde_json::json!({"version": 2, "netwrok": []}).to_string(),
    ] {
        write_file(&directory.join(LOCK_FILENAME), &contents);
        assert!(Lock::read(&directory).is_err(), "invalid trust file accepted: {contents}");
        assert_eq!(std::fs::read_to_string(directory.join(LOCK_FILENAME)).expect("invalid file must remain"), contents);
    }
    std::fs::remove_file(directory.join(LOCK_FILENAME)).expect("invalid fixture must be removed");
    assert!(Lock::read(&directory).expect("missing implicit lock is empty").is_empty());
    std::fs::remove_dir_all(directory).expect("test directory must be removed");
}

#[test]
fn legacy_git_lock_is_readable_and_network_pins_survive_git_updates() {
    let directory = unique_dir("network-lock-preservation");
    write_file(
        &directory.join(LOCK_FILENAME),
        r#"{"version":1,"git":[{"name":"legacy","git":"url","reference":"default","commit":"abc"}]}"#,
    );
    let legacy = Lock::read(&directory).expect("version one Git lock must load");
    assert_eq!(legacy.commit_for("legacy", "url", "default"), Some("abc"));
    let mut old = write_network_lock(&directory, &[network_pin(TRUSTED_TOKEN, 2)]);
    old.record("token.aleo".into(), "url".into(), "default".into(), "old".into());
    let mut updated = Lock::default();
    updated.record("other".into(), "url2".into(), "default".into(), "new".into());
    updated.carry_over(&old, |_| false);
    updated.remove_name("token.aleo");
    updated.write(&directory).expect("updated lock must write");
    let reloaded = Lock::read(&directory).expect("updated lock must load");
    let pin = reloaded
        .network_pin("token.aleo", leo_ast::NetworkName::TestnetV0, Some(2))
        .expect("network pin must survive Git changes");
    assert_eq!(serde_json::json!(pin), network_pin(TRUSTED_TOKEN, 2));
    assert!(reloaded.commit_for("token.aleo", "url", "default").is_none());
    assert_eq!(reloaded.commit_for("other", "url2", "default"), Some("new"));
    updated.remove_name("other");
    updated.write(&directory).expect("a lock with only network pins must write");
    assert!(!Lock::read(&directory).expect("network-only lock must remain").is_empty());
    std::fs::remove_dir_all(directory).expect("test directory must be removed");
}

#[test]
fn package_automatically_locks_transitive_network_imports() {
    leo_span::create_session_if_not_set_then(|_| {
        let base = unique_dir("network-transitive-pins");
        let home = base.join("home");
        let consumer = base.join("consumer");
        let parent = format!("import token.aleo;\n{}", TRUSTED_TOKEN.replace("token.aleo", "parent.aleo"));
        write_consumer(&consumer, r#"{"name":"parent.aleo","location":"network"}"#);
        write_file(&home.join("registry/testnet/parent/99/parent.aleo"), &parent);
        write_file(&home.join("registry/testnet/token/99/token.aleo"), TRUSTED_TOKEN);
        let (endpoint, server) = network_response(&[
            ("/testnet/program/parent.aleo/latest_edition", "200 OK", "2"),
            ("/testnet/program/parent.aleo/2", "200 OK", &parent),
            ("/testnet/program/token.aleo/latest_edition", "200 OK", "1"),
            ("/testnet/program/token.aleo/1", "200 OK", TRUSTED_TOKEN),
        ]);
        let package = Package::from_directory(
            &consumer,
            &home,
            false,
            false,
            false,
            Some(leo_ast::NetworkName::TestnetV0),
            Some(&endpoint),
            0,
        )
        .expect("first build must fetch and lock every transitive dependency");
        assert_eq!(server.join().expect("fixture must finish").len(), 4);
        let names: Vec<_> = package.compilation_units.iter().map(|unit| unit.name.to_string()).collect();
        assert_eq!(names, ["token.aleo", "parent.aleo", "consumer.aleo"]);
        let lock = Lock::read(&consumer).expect("automatic lock must exist");
        assert_eq!(
            serde_json::json!(lock.network_pin("parent.aleo", leo_ast::NetworkName::TestnetV0, Some(2))),
            network_pin(&parent, 2),
        );
        assert_eq!(
            serde_json::json!(lock.network_pin("token.aleo", leo_ast::NetworkName::TestnetV0, Some(1))),
            network_pin(TRUSTED_TOKEN, 1),
        );
        let original_lock = std::fs::read(consumer.join(LOCK_FILENAME)).expect("lock must exist");
        Package::from_directory(
            &consumer,
            &home,
            false,
            false,
            false,
            Some(leo_ast::NetworkName::TestnetV0),
            Some("http://127.0.0.1:1"),
            0,
        )
        .expect("repeat build must use locked editions without consulting the endpoint");
        let (endpoint, server) = network_response(&[
            ("/testnet/program/parent.aleo/2", "200 OK", &parent),
            ("/testnet/program/token.aleo/1", "200 OK", TRUSTED_TOKEN),
        ]);
        Package::from_directory(
            &consumer,
            &home,
            true,
            false,
            false,
            Some(leo_ast::NetworkName::TestnetV0),
            Some(&endpoint),
            0,
        )
        .expect("no-cache must fetch locked editions and retain their checksums");
        assert_eq!(server.join().expect("fixture must finish").len(), 2);
        assert_eq!(std::fs::read(consumer.join(LOCK_FILENAME)).expect("lock must remain"), original_lock);
        let local = base.join("local");
        write_program(&local, "local.aleo", "null");
        let package = Package::from_directory(&local, &home, false, false, false, None, None, 0)
            .expect("a local-only package must not require network pins");
        assert_eq!(package.compilation_units.len(), 1);
        assert!(!local.join(LOCK_FILENAME).exists());
        std::fs::remove_dir_all(base).expect("test directory must be removed");
    });
}

#[test]
fn package_keeps_workspace_and_development_network_pins() {
    leo_span::create_session_if_not_set_then(|_| {
        let base = unique_dir("network-workspace-pins");
        let home = base.join("home");
        let workspace = base.join("workspace");
        let consumer = workspace.join("consumer");
        write_file(&workspace.join(WORKSPACE_MANIFEST_FILENAME), r#"{"members":["consumer"]}"#);
        write_consumer(&consumer, r#"{"name":"token.aleo","location":"network","edition":0}"#);
        std::fs::create_dir_all(&home).expect("home must exist");
        let sibling = TRUSTED_TOKEN.replace("token.aleo", "sibling.aleo");
        let dev = TRUSTED_TOKEN.replace("token.aleo", "dev.aleo");
        write_network_lock(&workspace, &[network_pin(&sibling, 1), network_pin(&dev, 2)]);
        write_file(&consumer.join(LOCK_FILENAME), "ignored member lock");
        let (endpoint, server) = network_response(&[("/testnet/program/token.aleo/0", "200 OK", TRUSTED_TOKEN)]);
        Package::from_directory(
            &consumer,
            &home,
            false,
            false,
            false,
            Some(leo_ast::NetworkName::TestnetV0),
            Some(&endpoint),
            0,
        )
        .expect("workspace root must receive newly resolved network dependencies");
        assert_eq!(server.join().expect("fixture must finish").len(), 1);
        let lock = Lock::read(&workspace).expect("workspace lock must exist");
        for (name, edition) in [("token.aleo", 0), ("sibling.aleo", 1), ("dev.aleo", 2)] {
            assert!(lock.network_pin(name, leo_ast::NetworkName::TestnetV0, Some(edition)).is_some());
        }
        assert_eq!(
            std::fs::read_to_string(consumer.join(LOCK_FILENAME)).expect("member file must remain"),
            "ignored member lock"
        );
        let original = std::fs::read(workspace.join(LOCK_FILENAME)).expect("workspace lock must exist");
        write_file(&home.join("registry/testnet/token/0/token.aleo"), &TRUSTED_TOKEN.replace("private", "public"));
        let error = Package::from_directory(
            &consumer,
            &home,
            false,
            false,
            false,
            Some(leo_ast::NetworkName::TestnetV0),
            Some("http://127.0.0.1:1"),
            0,
        )
        .expect_err("a changed cached dependency must stop the build");
        assert!(error.to_string().contains("checksum"), "{error}");
        assert_eq!(std::fs::read(workspace.join(LOCK_FILENAME)).expect("workspace lock must remain"), original);
        std::fs::remove_dir_all(base).expect("test directory must be removed");
    });
}

#[test]
fn invalid_first_response_does_not_create_a_pin_or_cache() {
    leo_span::create_session_if_not_set_then(|_| {
        for bytecode in [
            TRUSTED_TOKEN.replace("token.aleo", "other.aleo"),
            "invalid Aleo".to_owned(),
            " ".repeat(crate::MAX_PROGRAM_SIZE + 1),
        ] {
            let home = unique_dir("network-invalid-first-response");
            let mut lock = Lock::default();
            let (endpoint, server) = network_response(&[("/testnet/program/token.aleo/0", "200 OK", &bytecode)]);
            assert!(
                crate::CompilationUnit::fetch(
                    Symbol::intern("token.aleo"),
                    Some(0),
                    &home,
                    leo_ast::NetworkName::TestnetV0,
                    &endpoint,
                    false,
                    0,
                    &mut lock,
                )
                .is_err()
            );
            assert_eq!(server.join().expect("fixture must finish").len(), 1);
            assert!(lock.is_empty(), "invalid first response must not establish a checksum");
            assert!(!home.join("registry").exists());
            std::fs::remove_dir_all(home).expect("test directory must be removed");
        }
    });
}

#[test]
fn failed_network_graph_does_not_commit_new_pins() {
    leo_span::create_session_if_not_set_then(|_| {
        for existing_lock in [false, true] {
            let base = unique_dir("network-failed-graph");
            let home = base.join("home");
            let consumer = base.join("consumer");
            std::fs::create_dir_all(&home).expect("home must exist");
            write_consumer(&consumer, r#"{"name":"parent.aleo","location":"network"}"#);
            let original = if existing_lock {
                write_network_lock(&consumer, &[network_pin(&TRUSTED_TOKEN.replace("token.aleo", "other.aleo"), 1)]);
                Some(std::fs::read(consumer.join(LOCK_FILENAME)).expect("old lock must exist"))
            } else {
                None
            };
            let parent = format!("import token.aleo;\n{}", TRUSTED_TOKEN.replace("token.aleo", "parent.aleo"));
            let token = format!("import parent.aleo;\n{TRUSTED_TOKEN}");
            let (endpoint, server) = network_response(&[
                ("/testnet/program/parent.aleo/latest_edition", "200 OK", "0"),
                ("/testnet/program/parent.aleo/0", "200 OK", &parent),
                ("/testnet/program/token.aleo/latest_edition", "200 OK", "0"),
                ("/testnet/program/token.aleo/0", "200 OK", &token),
            ]);
            let error = Package::from_directory(
                &consumer,
                &home,
                false,
                false,
                false,
                Some(leo_ast::NetworkName::TestnetV0),
                Some(&endpoint),
                0,
            )
            .expect_err("a circular dependency graph must fail after resolution");
            assert!(error.to_string().contains("circular"), "{error}");
            assert_eq!(server.join().expect("fixture must finish").len(), 4);
            assert_eq!(std::fs::read(consumer.join(LOCK_FILENAME)).ok(), original);
            std::fs::remove_dir_all(base).expect("test directory must be removed");
        }
    });
}

#[test]
fn standalone_bytecode_resolves_imports_without_creating_a_lock_file() {
    leo_span::create_session_if_not_set_then(|_| {
        let base = unique_dir("network-standalone");
        let home = base.join("home");
        std::fs::create_dir_all(&home).expect("home must exist");
        let path = base.join("parent.aleo");
        write_file(&path, &format!("import token.aleo;\n{}", TRUSTED_TOKEN.replace("token.aleo", "parent.aleo")));
        let (endpoint, server) = network_response(&[
            ("/testnet/program/token.aleo/latest_edition", "200 OK", "0"),
            ("/testnet/program/token.aleo/0", "200 OK", TRUSTED_TOKEN),
        ]);
        let package = Package::from_aleo_file(
            &path,
            &home,
            None,
            false,
            false,
            Some(leo_ast::NetworkName::TestnetV0),
            Some(&endpoint),
            0,
        )
        .expect("standalone bytecode imports must resolve without manual pins");
        assert_eq!(package.compilation_units.len(), 2);
        assert_eq!(server.join().expect("fixture must finish").len(), 2);
        assert!(!base.join(LOCK_FILENAME).exists(), "standalone loading must not create a project lock");
        write_program(&base, "parent.aleo", "null");
        write_network_lock(&base, &[network_pin(TRUSTED_TOKEN, 0)]);
        let original = std::fs::read(base.join(LOCK_FILENAME)).expect("project lock must exist");
        let nested_path = base.join("build/parent/parent.aleo");
        write_file(&nested_path, &std::fs::read_to_string(&path).expect("bytecode fixture must exist"));
        Package::from_aleo_file(
            &nested_path,
            &home,
            None,
            false,
            false,
            Some(leo_ast::NetworkName::TestnetV0),
            Some("http://127.0.0.1:1"),
            0,
        )
        .expect("standalone imports must use the enclosing project lock");
        assert_eq!(std::fs::read(base.join(LOCK_FILENAME)).expect("project lock must remain"), original);
        assert!(!base.join("build/parent/leo.lock").exists());
        std::fs::remove_dir_all(base).expect("test directory must be removed");
    });
}

#[cfg(unix)]
#[test]
fn lock_write_preserves_existing_permissions() {
    use std::os::unix::fs::PermissionsExt;

    let dir = unique_dir("lock-permissions");
    let path = dir.join(LOCK_FILENAME);
    let mut lock = Lock::default();
    lock.record("foo".into(), "url".into(), "default".into(), "abc123".into());
    lock.write(&dir).expect("write initial lock");
    for mode in [0o600, 0o640] {
        std::fs::set_permissions(&path, std::fs::Permissions::from_mode(mode)).expect("set lock permissions");
        lock.write(&dir).expect("replace lock");
        assert_eq!(std::fs::metadata(&path).expect("read lock metadata").permissions().mode() & 0o777, mode);
    }
    std::fs::remove_dir_all(dir).expect("remove fixture");
}

#[cfg(unix)]
#[test]
fn lock_write_failure_preserves_existing_lock() {
    const CHILD: &str = "LEO_TEST_LOCK_WRITE_LIMIT";
    if std::env::var_os(CHILD).is_none() {
        let output = std::process::Command::new("bash")
            .args([
                "-c",
                "trap '' XFSZ; ulimit -f 1; exec \"$1\" --exact tests::lock_write_failure_preserves_existing_lock --nocapture",
                "bash",
            ])
            .arg(std::env::current_exe().expect("The test executable must exist."))
            .env(CHILD, "1")
            .output()
            .expect("The limited child process must start.");
        assert!(
            output.status.success(),
            "The limited child test failed: {}\n{}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
        return;
    }
    let dir = unique_dir("lock-write-failure");
    let path = dir.join(LOCK_FILENAME);
    let original = r#"{"version":1,"git":[]}"#;
    write_file(&path, original);
    let mut lock = Lock::default();
    lock.record("foo".into(), "x".repeat(16 * 1024), "default".into(), "abc123".into());
    let error = lock.write(&dir).expect_err("The file-size limit must reject the temporary write.");
    assert!(error.to_string().contains("failed to write lock file"), "{error}");
    assert_eq!(std::fs::read_to_string(&path).expect("The previous lock must remain readable."), original);
    let mut entries = std::fs::read_dir(&dir).expect("The lock directory must remain readable.");
    assert_eq!(entries.next().expect("The lock must remain.").expect("The entry must be readable.").path(), path);
    assert!(entries.next().is_none(), "The failed write must remove its temporary file.");
    std::fs::remove_dir_all(dir).expect("The test directory must be removed.");
}

#[test]
fn dependency_update_creates_lock_and_dry_run_preserves_project_files() {
    leo_span::create_session_if_not_set_then(|_| {
        let base = unique_dir("update-first-lock");
        let home = base.join("home");
        let consumer = base.join("consumer");
        std::fs::create_dir_all(&home).expect("home must exist");
        write_consumer(&consumer, r#"{"name":"token.aleo","location":"network","edition":0}"#);
        let manifest = std::fs::read(consumer.join(MANIFEST_FILENAME)).expect("manifest must exist");
        for dry_run in [true, false] {
            let (endpoint, server) = network_response(&[("/testnet/program/token.aleo/0", "200 OK", TRUSTED_TOKEN)]);
            let (old, new) = Package::update_dependencies(
                &consumer,
                &home,
                None,
                dry_run,
                leo_ast::NetworkName::TestnetV0,
                &endpoint,
                0,
            )
            .expect("an update must resolve a missing lock");
            assert!(old.is_empty());
            assert!(new.network_pin("token.aleo", leo_ast::NetworkName::TestnetV0, Some(0)).is_some());
            assert_eq!(consumer.join(LOCK_FILENAME).exists(), !dry_run);
            assert_eq!(server.join().expect("fixture must finish").len(), 1);
            assert_eq!(std::fs::read(consumer.join(MANIFEST_FILENAME)).expect("manifest must remain"), manifest);
            assert!(!consumer.join("build").exists());
        }
        std::fs::remove_dir_all(base).expect("test directory must be removed");
    });
}

#[test]
fn dependency_update_is_selective_and_keeps_transitive_pins() {
    leo_span::create_session_if_not_set_then(|_| {
        let base = unique_dir("update-selected");
        let home = base.join("home");
        let consumer = base.join("consumer");
        let parent = format!("import token.aleo;\n{}", TRUSTED_TOKEN.replace("token.aleo", "parent.aleo"));
        let other = TRUSTED_TOKEN.replace("token.aleo", "other.aleo");
        write_consumer(
            &consumer,
            r#"{"name":"parent.aleo","location":"network"},{"name":"other.aleo","location":"network"}"#,
        );
        write_network_lock(&consumer, &[
            network_pin(&parent, 0),
            network_pin(TRUSTED_TOKEN, 0),
            network_pin(&other, 0),
        ]);
        write_file(&home.join("registry/testnet/token/0/token.aleo"), TRUSTED_TOKEN);
        write_file(&home.join("registry/testnet/other/0/other.aleo"), &other);
        let original = std::fs::read(consumer.join(LOCK_FILENAME)).expect("lock must exist");
        for dry_run in [true, false] {
            let (endpoint, server) = network_response(&[
                ("/testnet/program/parent.aleo/latest_edition", "200 OK", "1"),
                ("/testnet/program/parent.aleo/1", "200 OK", &parent),
            ]);
            let (_, updated) = Package::update_dependencies(
                &consumer,
                &home,
                Some("parent"),
                dry_run,
                leo_ast::NetworkName::TestnetV0,
                &endpoint,
                0,
            )
            .expect("selected update must leave unrelated dependencies locked");
            assert_eq!(server.join().expect("fixture must finish").len(), 2);
            assert!(updated.network_pin("parent.aleo", leo_ast::NetworkName::TestnetV0, Some(1)).is_some());
            for name in ["token.aleo", "other.aleo"] {
                assert!(updated.network_pin(name, leo_ast::NetworkName::TestnetV0, Some(0)).is_some());
            }
            if dry_run {
                assert_eq!(std::fs::read(consumer.join(LOCK_FILENAME)).expect("lock must remain"), original);
            }
        }
        let original = std::fs::read(consumer.join(LOCK_FILENAME)).expect("lock must exist");
        let changed = parent.replace("private", "public");
        let (endpoint, server) = network_response(&[
            ("/testnet/program/parent.aleo/latest_edition", "200 OK", "1"),
            ("/testnet/program/parent.aleo/1", "200 OK", &changed),
        ]);
        let error = Package::update_dependencies(
            &consumer,
            &home,
            Some("parent.aleo"),
            false,
            leo_ast::NetworkName::TestnetV0,
            &endpoint,
            0,
        )
        .expect_err("same-edition updates must not replace the locked checksum");
        assert!(error.to_string().contains("checksum"), "{error}");
        assert_eq!(server.join().expect("fixture must finish").len(), 2);
        assert_eq!(std::fs::read(consumer.join(LOCK_FILENAME)).expect("lock must remain"), original);
        std::fs::remove_dir_all(base).expect("test directory must be removed");
    });
}

#[test]
fn dependency_update_respects_workspace_development_edition_constraints() {
    leo_span::create_session_if_not_set_then(|_| {
        let base = unique_dir("update-workspace-constraints");
        let home = base.join("home");
        let workspace = base.join("workspace");
        let flexible = workspace.join("flexible");
        let fixed = workspace.join("fixed");
        std::fs::create_dir_all(&home).expect("home must exist");
        write_file(&workspace.join(WORKSPACE_MANIFEST_FILENAME), r#"{"members":["flexible","fixed"]}"#);
        write_program(&flexible, "flexible.aleo", r#"[{"name":"token.aleo","location":"network"}]"#);
        write_program(&fixed, "fixed.aleo", "null");
        let mut manifest: serde_json::Value =
            serde_json::from_str(&std::fs::read_to_string(fixed.join(MANIFEST_FILENAME)).expect("manifest must exist"))
                .expect("manifest must parse");
        manifest["dev_dependencies"] = serde_json::json!([{"name":"token.aleo","location":"network","edition":2}]);
        write_file(&fixed.join(MANIFEST_FILENAME), &manifest.to_string());
        let mut flexible_manifest: serde_json::Value = serde_json::from_str(
            &std::fs::read_to_string(flexible.join(MANIFEST_FILENAME)).expect("manifest must exist"),
        )
        .expect("manifest must parse");
        flexible_manifest["dev_dependencies"] = manifest["dev_dependencies"].clone();
        write_file(&flexible.join(MANIFEST_FILENAME), &flexible_manifest.to_string());
        write_network_lock(&workspace, &[network_pin(TRUSTED_TOKEN, 2)]);
        let (endpoint, server) = network_response(&[("/testnet/program/token.aleo/2", "200 OK", TRUSTED_TOKEN)]);
        let (_, updated) =
            Package::update_dependencies(&flexible, &home, None, false, leo_ast::NetworkName::TestnetV0, &endpoint, 0)
                .expect("workspace constraints must apply before flexible dependencies select latest editions");
        assert_eq!(server.join().expect("fixture must finish").len(), 2);
        assert!(updated.network_pin("token.aleo", leo_ast::NetworkName::TestnetV0, Some(2)).is_some());
        Package::from_directory_with_tests(
            &flexible,
            &home,
            false,
            false,
            false,
            Some(leo_ast::NetworkName::TestnetV0),
            Some("http://127.0.0.1:1"),
            0,
        )
        .expect("a normal test build must reuse compatible fixed and flexible declarations");
        assert!(!flexible.join(LOCK_FILENAME).exists());
        assert!(!fixed.join(LOCK_FILENAME).exists());
        assert!(!workspace.join("build").exists());
        std::fs::remove_dir_all(base).expect("test directory must be removed");
    });
}

#[test]
fn dependency_update_rejects_unknown_local_and_failed_graph_without_writing() {
    leo_span::create_session_if_not_set_then(|_| {
        let base = unique_dir("update-failure");
        let home = base.join("home");
        let consumer = base.join("consumer");
        let local = base.join("local");
        write_library(&local, "local", "null");
        write_consumer(&consumer, r#"{"name":"local","location":"local","path":"../local"}"#);
        let original = r#"{"version":1,"git":[]}"#;
        write_file(&consumer.join(LOCK_FILENAME), original);
        for name in ["unknown", "local"] {
            let error = Package::update_dependencies(
                &consumer,
                &home,
                Some(name),
                false,
                leo_ast::NetworkName::TestnetV0,
                "http://127.0.0.1:1",
                0,
            )
            .expect_err("only a present network or Git dependency can be selected");
            assert!(error.to_string().contains("No network or Git dependency"), "{error}");
            assert_eq!(std::fs::read_to_string(consumer.join(LOCK_FILENAME)).expect("lock must remain"), original);
        }
        write_consumer(&consumer, r#"{"name":"parent.aleo","location":"network","edition":0}"#);
        let parent = format!("import token.aleo;\n{}", TRUSTED_TOKEN.replace("token.aleo", "parent.aleo"));
        let (endpoint, server) = network_response(&[
            ("/testnet/program/parent.aleo/0", "200 OK", &parent),
            ("/testnet/program/token.aleo/latest_edition", "404 Not Found", "missing dependency"),
        ]);
        assert!(
            Package::update_dependencies(&consumer, &home, None, false, leo_ast::NetworkName::TestnetV0, &endpoint, 0,)
                .is_err()
        );
        assert_eq!(server.join().expect("fixture must finish").len(), 2);
        assert_eq!(std::fs::read_to_string(consumer.join(LOCK_FILENAME)).expect("lock must remain"), original);
        assert!(!consumer.join("build").exists());
        std::fs::remove_dir_all(base).expect("test directory must be removed");
    });
}

#[test]
fn dependency_update_refreshes_git_branches_but_keeps_tags_and_revisions() {
    if !git_available() {
        return;
    }
    leo_span::create_session_if_not_set_then(|_| {
        let base = unique_dir("update-git");
        let home = base.join("home");
        let source = base.join("source");
        let consumer = base.join("consumer");
        std::fs::create_dir_all(&home).expect("home must exist");
        for name in ["floating", "tagged", "pinned", "other"] {
            write_library(&source.join(name), name, "null");
        }
        init_repo(&source, None);
        let first = run_git(&source, &["rev-parse", "HEAD"]);
        run_git(&source, &["tag", "stable"]);
        run_git(&source, &["branch", "other"]);
        let url = file_url(&source);
        write_consumer(
            &consumer,
            &format!(
                r#"{{"name":"floating","location":"git","git":{{"url":"{url}"}}}},{{"name":"tagged","location":"git","git":{{"url":"{url}","tag":"stable"}}}},{{"name":"pinned","location":"git","git":{{"url":"{url}","rev":"{first}"}}}},{{"name":"other","location":"git","git":{{"url":"{url}","branch":"other"}}}}"#
            ),
        );
        Package::from_directory(&consumer, &home, false, false, false, None, None, 0)
            .expect("initial graph must resolve");
        let original = std::fs::read(consumer.join(LOCK_FILENAME)).expect("lock must exist");
        write_file(&source.join("floating/src/lib.leo"), "// updated\n");
        run_git(&source, &["commit", "-qam", "second"]);
        let second = run_git(&source, &["rev-parse", "HEAD"]);
        run_git(&source, &["tag", "-f", "stable"]);
        run_git(&source, &["branch", "-f", "other"]);
        Package::from_directory(&consumer, &home, false, false, false, None, None, 0)
            .expect("normal build must retain pins");
        assert_eq!(std::fs::read(consumer.join(LOCK_FILENAME)).expect("lock must remain"), original);
        for dry_run in [true, false] {
            let (_, updated) = Package::update_dependencies(
                &consumer,
                &home,
                Some("floating"),
                dry_run,
                leo_ast::NetworkName::TestnetV0,
                "http://127.0.0.1:1",
                0,
            )
            .expect("explicit update must advance only the selected branch");
            assert_eq!(updated.commit_for("floating", &url, "default"), Some(second.as_str()));
            assert_eq!(updated.commit_for("tagged", &url, "tag=stable"), Some(first.as_str()));
            assert_eq!(updated.commit_for("pinned", &url, &format!("rev={first}")), Some(first.as_str()));
            assert_eq!(updated.commit_for("other", &url, "branch=other"), Some(first.as_str()));
            if dry_run {
                assert_eq!(std::fs::read(consumer.join(LOCK_FILENAME)).expect("lock must remain"), original);
            }
        }
        let (_, updated) = Package::update_dependencies(
            &consumer,
            &home,
            None,
            false,
            leo_ast::NetworkName::TestnetV0,
            "http://127.0.0.1:1",
            0,
        )
        .expect("update all must refresh other mutable branches");
        assert_eq!(updated.commit_for("other", &url, "branch=other"), Some(second.as_str()));
        assert_eq!(updated.commit_for("tagged", &url, "tag=stable"), Some(first.as_str()));
        assert_eq!(updated.commit_for("pinned", &url, &format!("rev={first}")), Some(first.as_str()));
        assert!(!consumer.join("build").exists());
        std::fs::remove_dir_all(base).expect("test directory must be removed");
    });
}

// Reference resolution (`crate::git::resolve`).

#[test]
fn resolves_default_branch_tag_and_rev() {
    if !git_available() {
        eprintln!("skipping: `git` CLI not available");
        return;
    }
    let base = unique_dir("resolve");
    let home = base.join("home");
    let (url, c1, c2) = fixture_repo(&base);

    // Default branch -> latest commit on main.
    let (dir, commit) = resolve(&home, "dep", &url, &GitReference::DefaultBranch, None, false).unwrap();
    assert_eq!(commit, c2);
    assert_eq!(std::fs::read_to_string(dir.join("a.txt")).unwrap(), "two");

    // Tag -> the tagged (first) commit.
    let (dir, commit) = resolve(&home, "dep", &url, &GitReference::Tag("v1".into()), None, false).unwrap();
    assert_eq!(commit, c1);
    assert_eq!(std::fs::read_to_string(dir.join("a.txt")).unwrap(), "one");

    // Branch -> the feature branch's content.
    let (dir, _) = resolve(&home, "dep", &url, &GitReference::Branch("feature".into()), None, false).unwrap();
    assert_eq!(std::fs::read_to_string(dir.join("b.txt")).unwrap(), "feat");

    // Rev -> the exact commit.
    let (dir, commit) = resolve(&home, "dep", &url, &GitReference::Rev(c1.clone()), None, false).unwrap();
    assert_eq!(commit, c1);
    assert_eq!(std::fs::read_to_string(dir.join("a.txt")).unwrap(), "one");

    let _ = std::fs::remove_dir_all(&base);
}

#[test]
fn locked_commit_is_reused_without_network() {
    if !git_available() {
        eprintln!("skipping: `git` CLI not available");
        return;
    }
    let base = unique_dir("locked");
    let home = base.join("home");
    let (url, c1, _c2) = fixture_repo(&base);

    // Populate the cache for the tagged commit.
    let (_, commit) = resolve(&home, "dep", &url, &GitReference::Tag("v1".into()), None, false).unwrap();
    assert_eq!(commit, c1);

    // The bogus URL hashes to a different checkout dir, so the locked commit isn't reused: this
    // must fail, confirming the lock fast-path is keyed on the URL too.
    let bogus = "file:///nonexistent/repo";
    let bogus_locked = resolve(&home, "dep", bogus, &GitReference::Tag("v1".into()), Some(&c1), false);
    assert!(bogus_locked.is_err());

    // The real URL with the locked commit reuses the checkout (offline succeeds).
    let (dir, commit) = resolve(&home, "dep", &url, &GitReference::Tag("v1".into()), Some(&c1), true).unwrap();
    assert_eq!(commit, c1);
    assert_eq!(std::fs::read_to_string(dir.join("a.txt")).unwrap(), "one");

    let _ = std::fs::remove_dir_all(&base);
}

#[test]
fn mutable_reference_reuses_lock_until_explicit_update() {
    if !git_available() {
        eprintln!("skipping: `git` CLI not available");
        return;
    }
    let base = unique_dir("mutable");
    let home = base.join("home");
    let (url, _c1, c2) = fixture_repo(&base);
    let src = base.join("src");

    // Resolve the default branch; it pins to the current tip.
    let (_, first) = resolve(&home, "dep", &url, &GitReference::DefaultBranch, None, false).unwrap();
    assert_eq!(first, c2);

    // Advance the branch with a new commit.
    std::fs::write(src.join("a.txt"), "three").unwrap();
    run_git(&src, &["commit", "-qam", "c3"]);
    let c3 = run_git(&src, &["rev-parse", "HEAD"]);
    assert_ne!(c3, c2);

    // Online builds keep the locked commit even after the branch advances.
    let (dir, reused) = resolve(&home, "dep", &url, &GitReference::DefaultBranch, Some(&c2), false).unwrap();
    assert_eq!(reused, c2);
    assert_eq!(std::fs::read_to_string(dir.join("a.txt")).unwrap(), "two");
    std::fs::remove_dir_all(home.join("git")).unwrap();
    let (dir, restored) = resolve(&home, "dep", &url, &GitReference::DefaultBranch, Some(&c2), false).unwrap();
    assert_eq!(restored, c2, "a missing checkout must not move the locked revision");
    assert_eq!(std::fs::read_to_string(dir.join("a.txt")).unwrap(), "two");
    let (_, refreshed) = resolve(&home, "dep", &url, &GitReference::DefaultBranch, None, false).unwrap();
    assert_eq!(refreshed, c3);

    // Offline, the locked commit is reused even for a mutable reference (no network access).
    let (dir, offline) = resolve(&home, "dep", &url, &GitReference::DefaultBranch, Some(&c2), true).unwrap();
    assert_eq!(offline, c2);
    assert_eq!(std::fs::read_to_string(dir.join("a.txt")).unwrap(), "two");

    let _ = std::fs::remove_dir_all(&base);
}

#[test]
fn branch_reference_refreshes_only_without_a_lock() {
    if !git_available() {
        eprintln!("skipping: `git` CLI not available");
        return;
    }
    let base = unique_dir("branch");
    let home = base.join("home");
    let (url, _c1, _c2) = fixture_repo(&base);
    let src = base.join("src");

    // Pin the feature branch's current tip.
    let (_, first) = resolve(&home, "dep", &url, &GitReference::Branch("feature".into()), None, false).unwrap();

    // Advance the feature branch with a new commit.
    run_git(&src, &["checkout", "-q", "feature"]);
    std::fs::write(src.join("b.txt"), "feat2").unwrap();
    run_git(&src, &["commit", "-qam", "feat2"]);
    let advanced = run_git(&src, &["rev-parse", "HEAD"]);
    assert_ne!(advanced, first);

    let (_, pinned) =
        resolve(&home, "dep", &url, &GitReference::Branch("feature".into()), Some(&first), false).unwrap();
    assert_eq!(pinned, first);
    let (dir, refreshed) = resolve(&home, "dep", &url, &GitReference::Branch("feature".into()), None, false).unwrap();
    assert_eq!(refreshed, advanced);
    assert_eq!(std::fs::read_to_string(dir.join("b.txt")).unwrap(), "feat2");

    let _ = std::fs::remove_dir_all(&base);
}

#[test]
fn immutable_reference_reuses_locked_commit_online_without_fetching() {
    if !git_available() {
        eprintln!("skipping: `git` CLI not available");
        return;
    }
    let base = unique_dir("immutable");
    let home = base.join("home");
    let (url, c1, _c2) = fixture_repo(&base);

    // Cache the tagged commit.
    let (_, commit) = resolve(&home, "dep", &url, &GitReference::Tag("v1".into()), None, false).unwrap();
    assert_eq!(commit, c1);

    // Make the source repository unreachable; any fetch would now fail.
    std::fs::remove_dir_all(base.join("src")).unwrap();

    // Online, an immutable locked reference is served from the cache without contacting the remote.
    let (dir, reused) = resolve(&home, "dep", &url, &GitReference::Tag("v1".into()), Some(&c1), false).unwrap();
    assert_eq!(reused, c1);
    assert_eq!(std::fs::read_to_string(dir.join("a.txt")).unwrap(), "one");

    let _ = std::fs::remove_dir_all(&base);
}

#[test]
fn offline_without_cache_errors() {
    let base = unique_dir("offline");
    let home = base.join("home");
    let result = resolve(&home, "dep", "file:///nonexistent/repo", &GitReference::DefaultBranch, None, true);
    assert!(result.is_err());
    let _ = std::fs::remove_dir_all(&base);
}

#[test]
fn clone_failure_errors() {
    // Online resolution of an unreachable repository must error (not panic).
    let base = unique_dir("clonefail");
    let home = base.join("home");
    let result = resolve(&home, "dep", "file:///no/such/leo/repo", &GitReference::DefaultBranch, None, false);
    assert!(result.is_err());
    let _ = std::fs::remove_dir_all(&base);
}

/// A locked immutable reference is honored even when the checkout is gone (e.g. a fresh
/// machine): the repository is re-fetched but the LOCKED commit is checked out, so a tag moved
/// upstream cannot change what is built.
#[test]
fn locked_tag_wins_when_checkout_missing() {
    if !git_available() {
        eprintln!("skipping: `git` CLI not available");
        return;
    }
    let base = unique_dir("relock");
    let home = base.join("home");
    let (url, c1, c2) = fixture_repo(&base);
    let src = base.join("src");

    // Pin the tag, then clear the cache and move the tag upstream.
    let (_, commit) = resolve(&home, "dep", &url, &GitReference::Tag("v1".into()), None, false).unwrap();
    assert_eq!(commit, c1);
    std::fs::remove_dir_all(home.join("git")).unwrap();
    run_git(&src, &["tag", "-f", "v1", &c2]);

    // Re-resolution fetches again but checks out the locked commit, not the moved tag.
    let (dir, commit) = resolve(&home, "dep", &url, &GitReference::Tag("v1".into()), Some(&c1), false).unwrap();
    assert_eq!(commit, c1);
    assert_eq!(std::fs::read_to_string(dir.join("a.txt")).unwrap(), "one");

    let _ = std::fs::remove_dir_all(&base);
}

/// Checkouts are keyed by URL and commit, so all dependencies into one repository share them.
#[test]
fn checkouts_are_shared_across_dependency_names() {
    if !git_available() {
        eprintln!("skipping: `git` CLI not available");
        return;
    }
    let base = unique_dir("shared");
    let home = base.join("home");
    let (url, c1, _c2) = fixture_repo(&base);

    let (dir_a, _) = resolve(&home, "depa", &url, &GitReference::Tag("v1".into()), None, false).unwrap();
    let (dir_b, _) = resolve(&home, "depb", &url, &GitReference::Tag("v1".into()), Some(&c1), false).unwrap();
    assert_eq!(dir_a, dir_b, "same repository and commit must share one checkout");

    let _ = std::fs::remove_dir_all(&base);
}

#[test]
fn missing_reference_errors() {
    if !git_available() {
        eprintln!("skipping: `git` CLI not available");
        return;
    }
    let base = unique_dir("missingref");
    let home = base.join("home");
    let (url, _c1, _c2) = fixture_repo(&base);

    // A branch, tag, or revision that does not exist must error.
    assert!(resolve(&home, "dep", &url, &GitReference::Branch("nope".into()), None, false).is_err());
    assert!(resolve(&home, "dep", &url, &GitReference::Tag("v9.9.9".into()), None, false).is_err());
    let bad_rev = "0000000000000000000000000000000000000000".to_string();
    assert!(resolve(&home, "dep", &url, &GitReference::Rev(bad_rev), None, false).is_err());

    let _ = std::fs::remove_dir_all(&base);
}

/// Clones a real public repository over HTTPS, pinned to an immutable commit, and locates a
/// package within it. Ignored by default since it needs network access; run with
/// `cargo test -p leo-package -- --ignored`.
#[test]
#[ignore = "requires network access"]
fn resolves_real_github_repo() {
    let base = unique_dir("real");
    let home = base.join("home");
    let url = "https://github.com/ProvableHQ/leo-examples";
    let rev = "6728690fcf10261a4023cf4f64b9c7960296d4e0";

    let (dir, commit) = resolve(&home, "helloworld.aleo", url, &GitReference::Rev(rev.into()), None, false).unwrap();
    assert_eq!(commit, rev);

    // `helloworld` lives in a subdirectory; it is found as a package directory by name.
    let located = crate::find_in_checkout(&dir, "helloworld.aleo").unwrap();
    assert!(located.is_dir(), "expected a Leo package directory, found a bytecode file");
    assert!(located.join(MANIFEST_FILENAME).is_file());

    let _ = std::fs::remove_dir_all(&base);
}

#[test]
fn find_in_checkout_locates_each_kind_and_errors_when_absent() {
    let base = unique_dir("find");
    // A library in a subdirectory, a program in a subdirectory, and a bytecode file at the root.
    std::fs::create_dir_all(base.join("lib")).unwrap();
    std::fs::write(
        base.join("lib/program.json"),
        r#"{"program":"mylib","version":"0.1.0","description":"","license":"MIT"}"#,
    )
    .unwrap();
    std::fs::create_dir_all(base.join("prog")).unwrap();
    std::fs::write(
        base.join("prog/program.json"),
        r#"{"program":"myprog.aleo","version":"0.1.0","description":"","license":"MIT"}"#,
    )
    .unwrap();
    std::fs::write(base.join("mybytes.aleo"), "// bytecode\n").unwrap();

    // The library and program are located as package directories; the root `.aleo` as a file.
    assert!(crate::find_in_checkout(&base, "mylib").unwrap().is_dir());
    assert!(crate::find_in_checkout(&base, "myprog.aleo").unwrap().is_dir());
    let bytes = crate::find_in_checkout(&base, "mybytes.aleo").unwrap();
    assert!(bytes.is_file() && bytes.extension().and_then(|e| e.to_str()) == Some("aleo"));
    // A name present in neither a manifest nor as a root `.aleo` file errors.
    assert!(crate::find_in_checkout(&base, "absent").is_err());

    let _ = std::fs::remove_dir_all(&base);
}

/// A directory declaring exactly the requested name form wins over the alternate (`.aleo`) form.
#[test]
fn find_in_checkout_prefers_exact_name_form() {
    let base = unique_dir("exact");
    std::fs::create_dir_all(base.join("lib")).unwrap();
    std::fs::write(
        base.join("lib/program.json"),
        r#"{"program":"foo","version":"0.1.0","description":"","license":"MIT"}"#,
    )
    .unwrap();
    std::fs::create_dir_all(base.join("prog")).unwrap();
    std::fs::write(
        base.join("prog/program.json"),
        r#"{"program":"foo.aleo","version":"0.1.0","description":"","license":"MIT"}"#,
    )
    .unwrap();

    // `foo` matches the library, `foo.aleo` the program — regardless of directory sort order.
    assert!(crate::find_in_checkout(&base, "foo").unwrap().ends_with("lib"));
    assert!(crate::find_in_checkout(&base, "foo.aleo").unwrap().ends_with("prog"));

    let _ = std::fs::remove_dir_all(&base);
}

/// Multiple directories declaring the same program name are ambiguous, not first-match-wins.
#[test]
fn find_in_checkout_errors_on_ambiguous_name() {
    let base = unique_dir("ambiguous");
    for dir in ["examples/dup", "dup"] {
        std::fs::create_dir_all(base.join(dir)).unwrap();
        std::fs::write(
            base.join(dir).join("program.json"),
            r#"{"program":"dup","version":"0.1.0","description":"","license":"MIT"}"#,
        )
        .unwrap();
    }

    let err = crate::find_in_checkout(&base, "dup").unwrap_err();
    assert!(err.to_string().contains("ambiguous"), "expected ambiguity error: {err}");

    let _ = std::fs::remove_dir_all(&base);
}

/// `find_in_checkout` must not follow symlinks out of the checkout, or a malicious repo could match
/// (and compile) a package outside it.
#[cfg(unix)]
#[test]
fn find_in_checkout_does_not_follow_symlinks() {
    let base = unique_dir("symlink");
    // A package that lives OUTSIDE the checkout directory.
    let outside = base.join("outside");
    std::fs::create_dir_all(&outside).unwrap();
    std::fs::write(
        outside.join("program.json"),
        r#"{"program":"secret","version":"0.1.0","description":"","license":"MIT"}"#,
    )
    .unwrap();
    // The checkout contains only a symlink pointing at the outside package.
    let checkout = base.join("checkout");
    std::fs::create_dir_all(&checkout).unwrap();
    std::os::unix::fs::symlink(&outside, checkout.join("link")).unwrap();

    // The search must not traverse the symlink, so the outside package is not found.
    assert!(crate::find_in_checkout(&checkout, "secret").is_err());

    let _ = std::fs::remove_dir_all(&base);
}

// The `leo.lock` lock file (`crate::Lock`).

#[test]
fn round_trip_and_lookup() {
    let dir = unique_dir("lock");
    let mut lock = Lock::read(&dir).expect("lock must be valid");
    assert!(lock.is_empty());

    lock.record("foo.aleo".into(), "https://example.com/foo".into(), "tag=v1".into(), "abc123".into());
    lock.write(&dir).unwrap();

    let reloaded = Lock::read(&dir).expect("lock must be valid");
    assert_eq!(reloaded.commit_for("foo.aleo", "https://example.com/foo", "tag=v1"), Some("abc123"));
    // Reference mismatch forces re-resolution.
    assert_eq!(reloaded.commit_for("foo.aleo", "https://example.com/foo", "tag=v2"), None);
    // URL mismatch forces re-resolution.
    assert_eq!(reloaded.commit_for("foo.aleo", "https://example.com/other", "tag=v1"), None);

    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn record_replaces_existing_commit() {
    let mut lock = Lock::default();
    lock.record("foo".into(), "url".into(), "branch=main".into(), "c1".into());
    lock.record("foo".into(), "url".into(), "branch=main".into(), "c2".into());
    assert_eq!(lock.commit_for("foo", "url", "branch=main"), Some("c2"));
}

#[test]
fn record_keeps_entries_under_other_references() {
    let mut lock = Lock::default();
    lock.record("foo".into(), "url".into(), "default".into(), "c1".into());
    // In a shared workspace lock an entry under another reference may belong to another member,
    // so recording must not evict it; stale entries are pruned by `carry_over` or `leo remove`.
    lock.record("foo".into(), "url".into(), "tag=v1".into(), "c2".into());
    assert_eq!(lock.commit_for("foo", "url", "default"), Some("c1"));
    assert_eq!(lock.commit_for("foo", "url", "tag=v1"), Some("c2"));
}

#[test]
fn carry_over_keeps_only_accepted_unrecorded_entries() {
    let mut old = Lock::default();
    old.record("foo".into(), "url".into(), "tag=v1".into(), "c1".into());
    old.record("bar".into(), "url2".into(), "default".into(), "c2".into());

    let mut new = Lock::default();
    new.record("foo".into(), "url".into(), "tag=v1".into(), "c9".into());
    new.carry_over(&old, |entry| entry.name == "bar");

    // The re-recorded entry is not overwritten, and only accepted old entries are carried.
    assert_eq!(new.commit_for("foo", "url", "tag=v1"), Some("c9"));
    assert_eq!(new.commit_for("bar", "url2", "default"), Some("c2"));
}

#[test]
fn remove_name_drops_all_entries_for_dependency() {
    let mut lock = Lock::default();
    lock.record("foo".into(), "url".into(), "tag=v1".into(), "c1".into());
    lock.record("foo".into(), "url2".into(), "default".into(), "c2".into());
    lock.record("bar".into(), "url".into(), "default".into(), "c3".into());
    lock.remove_name("foo");
    assert_eq!(lock.commit_for("foo", "url", "tag=v1"), None);
    assert_eq!(lock.commit_for("foo", "url2", "default"), None);
    assert_eq!(lock.commit_for("bar", "url", "default"), Some("c3"));
}

#[test]
fn empty_lock_is_not_written() {
    let dir = unique_dir("lock-empty");
    Lock::default().write(&dir).unwrap();
    assert!(!dir.join(LOCK_FILENAME).exists());
    let _ = std::fs::remove_dir_all(&dir);
}

// Manifest validation of git dependencies (`crate::Manifest`).

#[test]
fn manifest_rejects_git_dependency_without_git_url() {
    let err = read_manifest(&manifest_json(r#"[{"name":"foo.aleo","location":"git"}]"#, "null")).unwrap_err();

    assert!(err.to_string().contains("invalid dependency `foo.aleo`"));
    assert!(err.to_string().contains("`git` dependencies must specify `git`"));
}

#[test]
fn manifest_rejects_git_dependency_with_path() {
    let err = read_manifest(&manifest_json(
        r#"[{"name":"foo.aleo","location":"git","git":{"url":"https://example.com/foo"},"path":"../foo"}]"#,
        "null",
    ))
    .unwrap_err();

    assert!(err.to_string().contains("`git` dependencies cannot specify `path`"));
}

#[test]
fn manifest_rejects_git_dependency_with_edition() {
    let err = read_manifest(&manifest_json(
        r#"[{"name":"foo.aleo","location":"git","git":{"url":"https://example.com/foo"},"edition":1}]"#,
        "null",
    ))
    .unwrap_err();

    assert!(err.to_string().contains("`git` dependencies cannot specify `edition`"));
}

#[test]
fn manifest_rejects_git_dependency_with_multiple_references() {
    let err = read_manifest(&manifest_json(
        r#"[{"name":"foo.aleo","location":"git","git":{"url":"https://example.com/foo","branch":"main","tag":"v1"}}]"#,
        "null",
    ))
    .unwrap_err();

    assert!(err.to_string().contains("at most one of `branch`, `tag`, or `rev`"));
}

#[test]
fn manifest_rejects_git_field_on_non_git_dependency() {
    // A stray `git` object on a non-git dependency must error, not be silently ignored.
    let err = read_manifest(&manifest_json(
        r#"[{"name":"foo.aleo","location":"local","path":"../foo","git":{"url":"https://example.com/foo"}}]"#,
        "null",
    ))
    .unwrap_err();
    assert!(err.to_string().contains("`local` dependencies cannot specify `git`"));

    let err = read_manifest(&manifest_json(
        r#"[{"name":"foo.aleo","location":"network","edition":1,"git":{"url":"https://example.com/foo"}}]"#,
        "null",
    ))
    .unwrap_err();
    assert!(err.to_string().contains("`network` dependencies cannot specify `git`"));
}

#[test]
fn manifest_rejects_invalid_git_dependency_name() {
    // The name is matched against checkout manifests, so it must be a valid package name.
    let err = read_manifest(&manifest_json(
        r#"[{"name":"not a name","location":"git","git":{"url":"https://example.com/foo"}}]"#,
        "null",
    ))
    .unwrap_err();
    assert!(err.to_string().contains("must be a valid program or library name"));
}

#[test]
fn manifest_accepts_git_dependency_variants() {
    let manifest = read_manifest(&manifest_json(
        r#"[
  {"name":"git_default.aleo","location":"git","git":{"url":"https://example.com/a"}},
  {"name":"git_branch.aleo","location":"git","git":{"url":"https://example.com/b","branch":"main"}},
  {"name":"git_tag.aleo","location":"git","git":{"url":"https://example.com/c","tag":"v0.1.0"}},
  {"name":"git_rev.aleo","location":"git","git":{"url":"https://example.com/d","rev":"abc123"}}
]"#,
        "null",
    ))
    .unwrap();

    assert_eq!(manifest.dependencies.unwrap().len(), 4);
}

#[test]
fn manifest_accepts_git_dev_dependency() {
    // The same validation applies to `dev_dependencies`, so a git dev-dependency is accepted there.
    let manifest = read_manifest(&manifest_json(
        "null",
        r#"[{"name":"mylib","location":"git","git":{"url":"https://example.com/a","tag":"v0.1.0"}}]"#,
    ))
    .unwrap();

    assert_eq!(manifest.dev_dependencies.unwrap().len(), 1);
}

// End-to-end resolution through `Package::from_directory`.

/// A consumer package with a git dependency on a Leo library resolves the library through a
/// `file://` clone and records the commit in `leo.lock`.
#[test]
fn git_dependency_resolves_and_locks() {
    if !git_available() {
        eprintln!("skipping: `git` CLI not available");
        return;
    }
    let root = unique_dir("e2e");
    let home = root.join("home");
    std::fs::create_dir_all(&home).unwrap();

    let lib = root.join("mylib_repo");
    write_library(&lib, "mylib", "null");
    init_repo(&lib, None);

    let consumer = root.join("consumer");
    let url = file_url(&lib);
    write_consumer(&consumer, &format!(r#"{{"name":"mylib","location":"git","git":{{"url":"{url}"}}}}"#));

    leo_span::create_session_if_not_set_then(|_| {
        let package = Package::from_directory(&consumer, &home, false, false, false, None, None, 3).unwrap();

        // The library was resolved as a Library compilation unit.
        let lib_unit = package
            .compilation_units
            .iter()
            .find(|u| u.name == Symbol::intern("mylib"))
            .expect("mylib compilation unit present");
        assert!(lib_unit.kind.is_library());

        // The lock file was written and pins the library to a commit.
        let lock = Lock::read(&consumer).expect("lock must be valid");
        let commit = lock.commit_for("mylib", &url, "default").expect("lock pins mylib");
        assert_eq!(commit.len(), 40);

        // A second resolution re-resolves the mutable default branch and still succeeds (the lock
        // is consulted, but a default-branch reference is re-fetched online; see
        // `immutable_reference_reuses_locked_commit_online_without_fetching` for the no-fetch case).
        let _ = Package::from_directory(&consumer, &home, false, false, false, None, None, 3).unwrap();
    });

    let _ = std::fs::remove_dir_all(&root);
}

/// A git dependency listed under `dev_dependencies` is resolved and locked when the package is
/// built with tests (the path `leo test` / `leo build --tests` takes).
#[test]
fn git_dev_dependency_resolves_with_tests() {
    if !git_available() {
        eprintln!("skipping: `git` CLI not available");
        return;
    }
    let root = unique_dir("dev_dep");
    let home = root.join("home");
    std::fs::create_dir_all(&home).unwrap();

    let lib = root.join("mylib_repo");
    write_library(&lib, "mylib", "null");
    init_repo(&lib, None);
    let url = file_url(&lib);

    let consumer = root.join("consumer");
    write_file(
        &consumer.join(MANIFEST_FILENAME),
        &format!(
            r#"{{"program":"consumer.aleo","version":"0.1.0","description":"","license":"MIT","dependencies":null,"dev_dependencies":[{{"name":"mylib","location":"git","git":{{"url":"{url}"}}}}]}}"#
        ),
    );
    write_file(&consumer.join("src/main.leo"), "// main\n");

    leo_span::create_session_if_not_set_then(|_| {
        // A plain build ignores dev-dependencies; building with tests resolves them.
        let package = Package::from_directory_with_tests(&consumer, &home, false, false, false, None, None, 3).unwrap();
        assert!(
            package.compilation_units.iter().any(|u| u.name == Symbol::intern("mylib")),
            "git dev-dependency resolved",
        );
        assert!(
            Lock::read(&consumer).expect("lock must be valid").commit_for("mylib", &url, "default").is_some(),
            "dev-dependency locked"
        );
    });

    let _ = std::fs::remove_dir_all(&root);
}

/// With `offline`, a build whose git dependency is locked and cached succeeds without any
/// network access, even for a mutable (default branch) reference.
#[test]
fn offline_build_uses_locked_commit_and_cache() {
    if !git_available() {
        eprintln!("skipping: `git` CLI not available");
        return;
    }
    let root = unique_dir("offline_e2e");
    let home = root.join("home");
    std::fs::create_dir_all(&home).unwrap();

    let lib = root.join("mylib_repo");
    write_library(&lib, "mylib", "null");
    init_repo(&lib, None);
    let url = file_url(&lib);

    let consumer = root.join("consumer");
    write_consumer(&consumer, &format!(r#"{{"name":"mylib","location":"git","git":{{"url":"{url}"}}}}"#));

    leo_span::create_session_if_not_set_then(|_| {
        // Build online once to populate the lock and the checkout cache.
        Package::from_directory(&consumer, &home, false, false, false, None, None, 3).unwrap();
        let commit = Lock::read(&consumer)
            .expect("lock must be valid")
            .commit_for("mylib", &url, "default")
            .expect("locked")
            .to_string();

        // Make the source repository unreachable; any fetch would now fail.
        std::fs::remove_dir_all(&lib).unwrap();

        // The offline build reuses the locked commit from the cache.
        Package::from_directory(&consumer, &home, false, false, true, None, None, 3).unwrap();
        assert_eq!(
            Lock::read(&consumer).expect("lock must be valid").commit_for("mylib", &url, "default"),
            Some(commit.as_str())
        );
    });

    let _ = std::fs::remove_dir_all(&root);
}

/// A regular `dependency` must be visible to in-package tests, and a local library in both
/// `dependencies` and `dev_dependencies` must dedup rather than conflict. Regression test for #29592.
#[test]
fn library_visible_to_src_and_tests() {
    let run = |dev_dependencies: &str| {
        let root = unique_dir("lib_visibility");
        let home = root.join("home");
        std::fs::create_dir_all(&home).unwrap();

        let lib = root.join("mylib");
        write_library(&lib, "mylib", "null");

        let app = root.join("app");
        write_file(
            &app.join(MANIFEST_FILENAME),
            &format!(
                r#"{{"program":"app.aleo","version":"0.1.0","description":"","license":"MIT","dependencies":[{{"name":"mylib","location":"local","path":"../mylib"}}],"dev_dependencies":{dev_dependencies}}}"#
            ),
        );
        write_file(&app.join("src/main.leo"), "// main\n");
        write_file(&app.join("tests/test_app.leo"), "// test\n");

        leo_span::create_session_if_not_set_then(|_| {
            // Relative `../mylib` and the canonicalized absolute path are the same library.
            let package = Package::from_directory_with_tests(&app, &home, false, false, false, None, None, 3).unwrap();
            let mylib = Symbol::intern("mylib");
            let test_unit = package
                .compilation_units
                .iter()
                .find(|u| u.kind.is_test())
                .expect("the in-package test program is a compilation unit");
            assert!(
                test_unit.dependencies.iter().any(|d| d.name == "mylib"),
                "a regular `dependencies` library is visible to the test program",
            );
            assert_eq!(
                package.compilation_units.iter().filter(|u| u.name == mylib).count(),
                1,
                "the shared local library is resolved exactly once",
            );
        });

        let _ = std::fs::remove_dir_all(&root);
    };

    // Case 2: library only in `dependencies` — must still be visible to the test program.
    run("null");
    // Case 1: library in both lists — must dedup, not conflict.
    run(r#"[{"name":"mylib","location":"local","path":"../mylib"}]"#);
}

/// A plain (non-test) build must not drop a git dev-dependency's pin from `leo.lock`.
#[test]
fn plain_build_keeps_dev_dependency_pin() {
    if !git_available() {
        eprintln!("skipping: `git` CLI not available");
        return;
    }
    let root = unique_dir("dev_pin");
    let home = root.join("home");
    std::fs::create_dir_all(&home).unwrap();

    let lib = root.join("mylib_repo");
    write_library(&lib, "mylib", "null");
    init_repo(&lib, None);
    let url = file_url(&lib);

    let consumer = root.join("consumer");
    write_file(
        &consumer.join(MANIFEST_FILENAME),
        &format!(
            r#"{{"program":"consumer.aleo","version":"0.1.0","description":"","license":"MIT","dependencies":null,"dev_dependencies":[{{"name":"mylib","location":"git","git":{{"url":"{url}"}}}}]}}"#
        ),
    );
    write_file(&consumer.join("src/main.leo"), "// main\n");

    leo_span::create_session_if_not_set_then(|_| {
        // A test build records the dev pin; a subsequent plain build must carry it over.
        Package::from_directory_with_tests(&consumer, &home, false, false, false, None, None, 3).unwrap();
        let commit = Lock::read(&consumer)
            .expect("lock must be valid")
            .commit_for("mylib", &url, "default")
            .expect("locked")
            .to_string();
        Package::from_directory(&consumer, &home, false, false, false, None, None, 3).unwrap();
        assert_eq!(
            Lock::read(&consumer).expect("lock must be valid").commit_for("mylib", &url, "default"),
            Some(commit.as_str())
        );

        // Once the dev dependency is gone from the manifest, the plain build prunes its pin.
        write_file(
            &consumer.join(MANIFEST_FILENAME),
            r#"{"program":"consumer.aleo","version":"0.1.0","description":"","license":"MIT","dependencies":null,"dev_dependencies":null}"#,
        );
        Package::from_directory(&consumer, &home, false, false, false, None, None, 3).unwrap();
        assert!(!consumer.join(LOCK_FILENAME).exists(), "stale lock file removed");
    });

    let _ = std::fs::remove_dir_all(&root);
}

/// Changing a git dependency's reference re-resolves it and prunes the stale lock entry.
#[test]
fn git_dependency_ref_change_updates_lock() {
    if !git_available() {
        eprintln!("skipping: `git` CLI not available");
        return;
    }
    let root = unique_dir("refchange");
    let home = root.join("home");
    std::fs::create_dir_all(&home).unwrap();

    let lib = root.join("mylib_repo");
    write_library(&lib, "mylib", "null");
    init_repo(&lib, Some("v1"));
    let url = file_url(&lib);
    let consumer = root.join("consumer");

    leo_span::create_session_if_not_set_then(|_| {
        // Track the default branch first.
        write_consumer(&consumer, &format!(r#"{{"name":"mylib","location":"git","git":{{"url":"{url}"}}}}"#));
        Package::from_directory(&consumer, &home, false, false, false, None, None, 3).unwrap();
        assert!(Lock::read(&consumer).expect("lock must be valid").commit_for("mylib", &url, "default").is_some());

        // Re-pin to the tag: the lock gains the tag entry and drops the stale default one.
        write_consumer(
            &consumer,
            &format!(r#"{{"name":"mylib","location":"git","git":{{"url":"{url}","tag":"v1"}}}}"#),
        );
        Package::from_directory(&consumer, &home, false, false, false, None, None, 3).unwrap();
        let lock = Lock::read(&consumer).expect("lock must be valid");
        assert!(lock.commit_for("mylib", &url, "tag=v1").is_some(), "tag entry recorded");
        assert!(lock.commit_for("mylib", &url, "default").is_none(), "stale default entry pruned");
    });

    let _ = std::fs::remove_dir_all(&root);
}

/// A git dependency whose own manifest declares another git dependency is resolved transitively.
#[test]
fn transitive_git_dependency() {
    if !git_available() {
        eprintln!("skipping: `git` CLI not available");
        return;
    }
    let root = unique_dir("transitive");
    let home = root.join("home");
    std::fs::create_dir_all(&home).unwrap();

    // libb (leaf), then liba which depends on libb via git.
    let libb = root.join("libb_repo");
    write_library(&libb, "libb", "null");
    init_repo(&libb, None);
    let url_b = file_url(&libb);

    let liba = root.join("liba_repo");
    write_library(&liba, "liba", &format!(r#"[{{"name":"libb","location":"git","git":{{"url":"{url_b}"}}}}]"#));
    init_repo(&liba, None);
    let url_a = file_url(&liba);

    let consumer = root.join("consumer");
    write_consumer(&consumer, &format!(r#"{{"name":"liba","location":"git","git":{{"url":"{url_a}"}}}}"#));

    leo_span::create_session_if_not_set_then(|_| {
        let package = Package::from_directory(&consumer, &home, false, false, false, None, None, 3).unwrap();
        let names: Vec<String> = package.compilation_units.iter().map(|u| u.name.to_string()).collect();
        assert!(names.iter().any(|n| n == "liba"), "liba resolved: {names:?}");
        assert!(names.iter().any(|n| n == "libb"), "transitive libb resolved: {names:?}");

        let lock = Lock::read(&consumer).expect("lock must be valid");
        assert!(lock.commit_for("liba", &url_a, "default").is_some());
        assert!(lock.commit_for("libb", &url_b, "default").is_some());
    });

    let _ = std::fs::remove_dir_all(&root);
}

/// A git program dependency that itself declares a git library dependency resolves the whole
/// chain, mixing package kinds (program and library) across the transitive git edges.
#[test]
fn transitive_git_program_depends_on_library() {
    if !git_available() {
        eprintln!("skipping: `git` CLI not available");
        return;
    }
    let root = unique_dir("transitive_mixed");
    let home = root.join("home");
    std::fs::create_dir_all(&home).unwrap();

    // deeplib (library leaf), then midprog (a program) which depends on deeplib via git.
    let deeplib = root.join("deeplib_repo");
    write_library(&deeplib, "deeplib", "null");
    init_repo(&deeplib, None);
    let url_lib = file_url(&deeplib);

    let midprog = root.join("midprog_repo");
    write_program(
        &midprog,
        "midprog.aleo",
        &format!(r#"[{{"name":"deeplib","location":"git","git":{{"url":"{url_lib}"}}}}]"#),
    );
    init_repo(&midprog, None);
    let url_prog = file_url(&midprog);

    let consumer = root.join("consumer");
    write_consumer(&consumer, &format!(r#"{{"name":"midprog.aleo","location":"git","git":{{"url":"{url_prog}"}}}}"#));

    leo_span::create_session_if_not_set_then(|_| {
        let package = Package::from_directory(&consumer, &home, false, false, false, None, None, 3).unwrap();

        // The git program resolved as a program, and its own git library resolved transitively.
        let prog = package
            .compilation_units
            .iter()
            .find(|u| u.name == Symbol::intern("midprog.aleo"))
            .expect("git program resolved");
        assert!(prog.kind.is_program());
        let lib = package
            .compilation_units
            .iter()
            .find(|u| u.name == Symbol::intern("deeplib"))
            .expect("transitive git library resolved");
        assert!(lib.kind.is_library());

        let lock = Lock::read(&consumer).expect("lock must be valid");
        assert!(lock.commit_for("midprog.aleo", &url_prog, "default").is_some());
        assert!(lock.commit_for("deeplib", &url_lib, "default").is_some());
    });

    let _ = std::fs::remove_dir_all(&root);
}

/// A git dependency may point at a repository that is itself a Leo workspace. The member is
/// located by name (the repository's `workspace.json` is ignored for location), and a member's
/// intra-workspace `workspace` dependency on a sibling resolves within the same checkout.
#[test]
fn git_dependency_into_workspace_repo_resolves_member_and_sibling() {
    if !git_available() {
        eprintln!("skipping: `git` CLI not available");
        return;
    }
    let root = unique_dir("git_ws");
    let home = root.join("home");
    std::fs::create_dir_all(&home).unwrap();

    // A single git repository that is a workspace of two libraries, where `libb` depends on
    // `liba` through a `workspace` dependency.
    let repo = root.join("ws_repo");
    write_file(&repo.join(WORKSPACE_MANIFEST_FILENAME), r#"{"members":["liba","libb"]}"#);
    write_library(&repo.join("liba"), "liba", "null");
    write_library(&repo.join("libb"), "libb", r#"[{"name":"liba","location":"workspace"}]"#);
    init_repo(&repo, None);
    let url = file_url(&repo);

    let consumer = root.join("consumer");
    write_consumer(&consumer, &format!(r#"{{"name":"libb","location":"git","git":{{"url":"{url}"}}}}"#));

    leo_span::create_session_if_not_set_then(|_| {
        let package = Package::from_directory(&consumer, &home, false, false, false, None, None, 3).unwrap();
        let names: Vec<String> = package.compilation_units.iter().map(|u| u.name.to_string()).collect();
        // The requested member and its in-repo workspace sibling both resolve.
        assert!(names.iter().any(|n| n == "libb"), "git workspace member resolved: {names:?}");
        assert!(names.iter().any(|n| n == "liba"), "sibling workspace member resolved: {names:?}");

        // The sibling is rewritten to a git dependency on the same source, so both are locked
        // (to the same commit, since the repository is resolved once per build).
        let lock = Lock::read(&consumer).expect("lock must be valid");
        let libb_commit = lock.commit_for("libb", &url, "default").expect("libb locked");
        assert_eq!(lock.commit_for("liba", &url, "default"), Some(libb_commit));
    });

    let _ = std::fs::remove_dir_all(&root);
}

/// A workspace member reached both as a direct git dependency and through a sibling's
/// intra-workspace dependency is the same dependency, not a conflict.
#[test]
fn direct_and_sibling_route_to_same_git_member_do_not_conflict() {
    if !git_available() {
        eprintln!("skipping: `git` CLI not available");
        return;
    }
    let root = unique_dir("git_ws_both");
    let home = root.join("home");
    std::fs::create_dir_all(&home).unwrap();

    let repo = root.join("ws_repo");
    write_file(&repo.join(WORKSPACE_MANIFEST_FILENAME), r#"{"members":["liba","libb"]}"#);
    write_library(&repo.join("liba"), "liba", "null");
    write_library(&repo.join("libb"), "libb", r#"[{"name":"liba","location":"workspace"}]"#);
    init_repo(&repo, None);
    let url = file_url(&repo);

    // The consumer depends on BOTH members directly, and libb also reaches liba internally.
    let consumer = root.join("consumer");
    write_consumer(
        &consumer,
        &format!(
            r#"{{"name":"liba","location":"git","git":{{"url":"{url}"}}}},{{"name":"libb","location":"git","git":{{"url":"{url}"}}}}"#
        ),
    );

    leo_span::create_session_if_not_set_then(|_| {
        let package = Package::from_directory(&consumer, &home, false, false, false, None, None, 3).unwrap();
        let names: Vec<String> = package.compilation_units.iter().map(|u| u.name.to_string()).collect();
        assert!(names.iter().any(|n| n == "liba"), "liba resolved: {names:?}");
        assert!(names.iter().any(|n| n == "libb"), "libb resolved: {names:?}");
    });

    let _ = std::fs::remove_dir_all(&root);
}

/// A package fetched via git may not reference local paths outside its own checkout.
#[test]
fn git_dependency_path_escape_errors() {
    if !git_available() {
        eprintln!("skipping: `git` CLI not available");
        return;
    }
    let root = unique_dir("escape");
    let home = root.join("home");
    std::fs::create_dir_all(&home).unwrap();

    // A library OUTSIDE any checkout that the malicious repo tries to pull in by absolute path.
    let outside = root.join("outside");
    write_library(&outside, "outside", "null");
    let outside_path = outside.canonicalize().unwrap().display().to_string().replace('\\', "/");

    // The repository's package declares a `local` dependency with an absolute path.
    let evil = root.join("evil_repo");
    write_library(&evil, "evil", &format!(r#"[{{"name":"outside","location":"local","path":"{outside_path}"}}]"#));
    init_repo(&evil, None);
    let url = file_url(&evil);

    let consumer = root.join("consumer");
    write_consumer(&consumer, &format!(r#"{{"name":"evil","location":"git","git":{{"url":"{url}"}}}}"#));

    leo_span::create_session_if_not_set_then(|_| {
        let err = Package::from_directory(&consumer, &home, false, false, false, None, None, 3).unwrap_err();
        assert!(err.to_string().contains("inside its own repository checkout"), "path escape must be rejected: {err}");
    });

    let _ = std::fs::remove_dir_all(&root);
}

/// Two dependencies on the same program with different git references conflict.
#[test]
fn conflicting_git_dependency_errors() {
    if !git_available() {
        eprintln!("skipping: `git` CLI not available");
        return;
    }
    let root = unique_dir("conflict");
    let home = root.join("home");
    std::fs::create_dir_all(&home).unwrap();

    let lib = root.join("shared_repo");
    write_library(&lib, "shared", "null");
    init_repo(&lib, None);
    let url = file_url(&lib);

    // The same dependency name pinned two different ways.
    let consumer = root.join("consumer");
    write_consumer(
        &consumer,
        &format!(
            r#"{{"name":"shared","location":"git","git":{{"url":"{url}","branch":"main"}}}},{{"name":"shared","location":"git","git":{{"url":"{url}"}}}}"#
        ),
    );

    leo_span::create_session_if_not_set_then(|_| {
        let result = Package::from_directory(&consumer, &home, false, false, false, None, None, 3);
        assert!(result.is_err(), "conflicting git references must error");
    });

    let _ = std::fs::remove_dir_all(&root);
}

// Workspace lock sharing across independently-built members.

/// In a workspace, members are built independently but share one `leo.lock` at the root.
/// Building a second member must merge into, not clobber, the first member's git entry.
#[test]
fn workspace_members_share_lock_without_clobbering() {
    if !git_available() {
        eprintln!("skipping: `git` CLI not available");
        return;
    }
    let root = unique_dir("ws_lock");
    let home = root.join("home");
    std::fs::create_dir_all(&home).unwrap();

    // Two independent git libraries, one per workspace member.
    let liba = root.join("liba_repo");
    write_library(&liba, "liba", "null");
    init_repo(&liba, None);
    let url_a = file_url(&liba);

    let libb = root.join("libb_repo");
    write_library(&libb, "libb", "null");
    init_repo(&libb, None);
    let url_b = file_url(&libb);

    // A workspace whose two members each depend on a different git library.
    let ws = root.join("ws");
    write_file(&ws.join(WORKSPACE_MANIFEST_FILENAME), r#"{"members":["mema","memb"]}"#);
    let mema = ws.join("mema");
    write_file(
        &mema.join(MANIFEST_FILENAME),
        &format!(
            r#"{{"program":"mema.aleo","version":"0.1.0","description":"","license":"MIT","dependencies":[{{"name":"liba","location":"git","git":{{"url":"{url_a}"}}}}]}}"#
        ),
    );
    write_file(&mema.join("src/main.leo"), "// main\n");
    let memb = ws.join("memb");
    write_file(
        &memb.join(MANIFEST_FILENAME),
        &format!(
            r#"{{"program":"memb.aleo","version":"0.1.0","description":"","license":"MIT","dependencies":[{{"name":"libb","location":"git","git":{{"url":"{url_b}"}}}}]}}"#
        ),
    );
    write_file(&memb.join("src/main.leo"), "// main\n");

    leo_span::create_session_if_not_set_then(|_| {
        // Build each member independently, as `leo build` does for a workspace.
        Package::from_directory(&mema, &home, false, false, false, None, None, 3).unwrap();
        Package::from_directory(&memb, &home, false, false, false, None, None, 3).unwrap();

        // The shared lock at the workspace root retains both members' entries.
        let lock = Lock::read(&ws).expect("lock must be valid");
        assert!(lock.commit_for("liba", &url_a, "default").is_some(), "first member's entry retained");
        assert!(lock.commit_for("libb", &url_b, "default").is_some(), "second member's entry recorded");
    });

    let _ = std::fs::remove_dir_all(&root);
}

/// Two members pinning the same dependency name and URL at different references must not evict
/// each other's entries from the shared workspace lock.
#[test]
fn workspace_members_keep_different_references_to_same_repo() {
    if !git_available() {
        eprintln!("skipping: `git` CLI not available");
        return;
    }
    let root = unique_dir("ws_refs");
    let home = root.join("home");
    std::fs::create_dir_all(&home).unwrap();

    let lib = root.join("shared_repo");
    write_library(&lib, "shared", "null");
    init_repo(&lib, Some("v1"));
    let url = file_url(&lib);

    let ws = root.join("ws");
    write_file(&ws.join(WORKSPACE_MANIFEST_FILENAME), r#"{"members":["mema","memb"]}"#);
    let mema = ws.join("mema");
    write_file(
        &mema.join(MANIFEST_FILENAME),
        &format!(
            r#"{{"program":"mema.aleo","version":"0.1.0","description":"","license":"MIT","dependencies":[{{"name":"shared","location":"git","git":{{"url":"{url}","tag":"v1"}}}}]}}"#
        ),
    );
    write_file(&mema.join("src/main.leo"), "// main\n");
    let memb = ws.join("memb");
    write_file(
        &memb.join(MANIFEST_FILENAME),
        &format!(
            r#"{{"program":"memb.aleo","version":"0.1.0","description":"","license":"MIT","dependencies":[{{"name":"shared","location":"git","git":{{"url":"{url}"}}}}]}}"#
        ),
    );
    write_file(&memb.join("src/main.leo"), "// main\n");

    leo_span::create_session_if_not_set_then(|_| {
        // Build each member twice; the entries must not thrash.
        for member in [&mema, &memb, &mema, &memb] {
            Package::from_directory(member, &home, false, false, false, None, None, 3).unwrap();
        }
        let lock = Lock::read(&ws).expect("lock must be valid");
        assert!(lock.commit_for("shared", &url, "tag=v1").is_some(), "tag entry retained");
        assert!(lock.commit_for("shared", &url, "default").is_some(), "default entry retained");
    });

    let _ = std::fs::remove_dir_all(&root);
}
