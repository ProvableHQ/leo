// Copyright (C) 2019-2021 Aleo Systems Inc.
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

use assert_cmd::Command;
use std::path::PathBuf;
use test_dir::{DirBuilder, FileType, TestDir};

/// Create Command from given arguments and CWD.
fn command(args: &str, cwd: Option<PathBuf>) -> Command {
    let args = args.split(' ').collect::<Vec<&str>>();
    let mut cmd = Command::cargo_bin("leo").unwrap();

    if let Some(cwd) = cwd {
        cmd.current_dir(cwd);
    }

    cmd.args(args);
    cmd
}

fn expect_success(args: &str, cwd: Option<PathBuf>) {
    command(args, cwd).unwrap();
}

fn expect_fail(args: &str, cwd: Option<PathBuf>) {
    command(args, cwd).unwrap_err();
}

#[test]
fn test_global_options() {
    expect_success("--path examples/pedersen-hash build", None);
    expect_success("--path examples/pedersen-hash -q build", None);
    expect_success("--path examples/pedersen-hash -d build", None);

    expect_fail("--path examples/no-directory-there build", None);
    expect_fail("-v build", None); // no such option
}

#[test]
fn init() {
    let dir = TestDir::temp().create("init", FileType::Dir);
    let dir = Some(dir.path("init"));

    expect_success("init", dir.clone());
    expect_fail("init", dir); // can't do twice
}

#[test]
fn init_fail() {
    let dir = TestDir::temp().create("incorrect_name", FileType::Dir);
    let dir = Some(dir.path("incorrect_name"));

    expect_fail("init", Some("directory-doesnt-exist".into()));
    expect_fail("init", dir);
}

#[test]
fn new() {
    let dir = TestDir::temp().create("new", FileType::Dir);
    let dir = Some(dir.path("new"));

    expect_success("new test", dir.clone());
    expect_fail("new test", dir.clone()); // duplicate
    expect_fail("new wrong_name123123", dir);
}

#[test]
fn unimplemented() {
    expect_fail("lint", None);
    expect_fail("deploy", None);
}

#[test]
fn clean() {
    expect_success("--path examples/pedersen-hash build", None);
    expect_success("--path examples/pedersen-hash clean", None);
}

#[test]
fn setup_prove_run_clean() {
    let dir = TestDir::temp().create("test", FileType::Dir);
    let dir = dir.path("test");

    expect_success("new setup", Some(dir.clone()));

    // 'cd' into newly created setup directory
    let new_dir = Some(dir.join("setup"));

    expect_success("setup", new_dir.clone());
    expect_success("setup", new_dir.clone());
    expect_success("setup --skip-key-check", new_dir.clone());
    expect_success("prove --skip-key-check", new_dir.clone());
    expect_success("run --skip-key-check", new_dir.clone());
    expect_success("clean", new_dir);
}

#[test]
fn test_sudoku() {
    let path = "examples/silly-sudoku";

    expect_success("build", Some(path.into()));
    expect_success("test", Some(path.into()));
    expect_success("test -f src/lib.leo", Some(path.into()));
    expect_success("test -f src/main.leo", Some(path.into()));
}

#[test]
fn test_missing_file() {
    let path = TestDir::temp().create("test", FileType::Dir);
    let path = path.path("test");

    expect_success("new missing-file-test", Some(path.clone()));
    std::fs::remove_file(&path.join("missing-file-test/src/main.leo")).unwrap();
    expect_fail("test", Some(path.join("missing-file")));
}
