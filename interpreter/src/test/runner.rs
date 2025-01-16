// Copyright (C) 2019-2025 Aleo Systems Inc.
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

use crate::*;

use snarkvm::prelude::{Address, PrivateKey};

use leo_span::symbol::create_session_if_not_set_then;
use leo_test_framework::runner::{Namespace, ParseType, Runner, Test};

use std::{fs, path::PathBuf, str::FromStr as _};

pub struct LeoNamespace;

pub static TEST_PRIVATE_KEY: &str = "APrivateKey1zkp8CZNn3yeCseEtxuVPbDCwSyhGW6yZKUYKfgXmcpoGPWH";

impl Namespace for LeoNamespace {
    fn parse_type(&self) -> ParseType {
        ParseType::Whole
    }

    fn run_test(&self, test: Test) -> Result<toml::Value, String> {
        create_session_if_not_set_then(|_| run_leo_test(test).map(|v| toml::Value::String(format!("{v}"))))
    }
}

pub struct AleoNamespace;

impl Namespace for AleoNamespace {
    fn parse_type(&self) -> ParseType {
        ParseType::Whole
    }

    fn run_test(&self, test: Test) -> Result<toml::Value, String> {
        create_session_if_not_set_then(|_| run_aleo_test(test).map(|v| toml::Value::String(format!("{v}"))))
    }
}

pub struct InterpreterRunner;

impl Runner for InterpreterRunner {
    fn resolve_namespace(&self, name: &str) -> Option<Box<dyn Namespace>> {
        match name {
            "Leo" => Some(Box::new(LeoNamespace)),
            "Aleo" => Some(Box::new(AleoNamespace)),
            _ => None,
        }
    }
}

fn run_leo_test(test: Test) -> Result<Value, String> {
    let tempdir = tempfile::tempdir().map_err(|e| format!("{e}"))?;
    let mut filename = PathBuf::from(tempdir.path());
    filename.push("main.leo");
    fs::write(&filename, &test.content).map_err(|e| format!("{e}"))?;

    let private_key: PrivateKey<TestnetV0> =
        PrivateKey::from_str(TEST_PRIVATE_KEY).expect("should be able to parse private key");
    let address = Address::try_from(&private_key).expect("should be able to create address");
    let empty: [&PathBuf; 0] = [];
    let mut interpreter = Interpreter::new([filename].iter(), empty, address, 0).map_err(|e| format!("{e}"))?;
    let v = interpreter.action(InterpreterAction::LeoInterpretOver("test.aleo/main()".into()));
    println!("got {v:?}");
    match v {
        Err(e) => Err(format!("{e}")),
        Ok(None) => Err("no value received".to_string()),
        Ok(Some(v)) => Ok(v),
    }
}

fn run_aleo_test(test: Test) -> Result<Value, String> {
    let tempdir = tempfile::tempdir().map_err(|e| format!("{e}"))?;
    let mut filename = PathBuf::from(tempdir.path());
    filename.push("main.aleo");
    fs::write(&filename, &test.content).map_err(|e| format!("{e}"))?;

    let private_key: PrivateKey<TestnetV0> =
        PrivateKey::from_str(TEST_PRIVATE_KEY).expect("should be able to parse private key");
    let address = Address::try_from(&private_key).expect("should be able to create address");
    let empty: [&PathBuf; 0] = [];
    let mut interpreter = Interpreter::new(empty, [filename].iter(), address, 0).map_err(|e| format!("{e}"))?;
    match interpreter.action(InterpreterAction::LeoInterpretOver("test.aleo/main()".into())) {
        Err(e) => Err(format!("{e}")),
        Ok(None) => Err("no value received".to_string()),
        Ok(Some(v)) => Ok(v),
    }
}
