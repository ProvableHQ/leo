// Copyright (C) 2019-2025 Provable Inc.
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

use crate::{Interpreter, InterpreterAction};

use leo_span::symbol::create_session_if_not_set_then;

use snarkvm::prelude::{Address, PrivateKey, TestnetV0};

use serial_test::serial;
use std::{fs, path::PathBuf, str::FromStr as _};

pub static TEST_PRIVATE_KEY: &str = "APrivateKey1zkp8CZNn3yeCseEtxuVPbDCwSyhGW6yZKUYKfgXmcpoGPWH";

fn runner_leo_test(test: &str) -> String {
    create_session_if_not_set_then(|_| {
        let tempdir = tempfile::tempdir().expect("tempdir");
        let mut filename = PathBuf::from(tempdir.path());
        filename.push("main.leo");
        fs::write(&filename, test).expect("write failed");

        let private_key: PrivateKey<TestnetV0> =
            PrivateKey::from_str(TEST_PRIVATE_KEY).expect("should be able to parse private key");
        let address = Address::try_from(&private_key).expect("should be able to create address");
        let empty: [&PathBuf; 0] = [];
        let mut interpreter = Interpreter::new([filename].iter(), empty, address, 0).expect("creating interpreter");
        match interpreter.action(InterpreterAction::LeoInterpretOver("test.aleo/main()".into())) {
            Err(e) => format!("{e}\n"),
            Ok(None) => "no value received\n".to_string(),
            Ok(Some(v)) => format!("{v}\n"),
        }
    })
}

#[test]
#[serial]
fn test_interpreter() {
    leo_test_framework::run_tests("interpreter-leo", runner_leo_test);
}
