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

use std::collections::BTreeMap;

#[derive(serde::Serialize, serde::Deserialize, PartialEq, Debug, Clone)]
pub enum TestExpectationMode {
    Pass,
    Fail,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct TestConfig {
    pub namespace: String,
    pub expectation: TestExpectationMode,
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_yaml::Value>,
}

pub fn extract_test_config(source: &str) -> Option<TestConfig> {
    let first_comment_start = source.find("/*")?;
    let end_first_comment = source[first_comment_start + 2..].find("*/")?;
    let comment_inner = &source[first_comment_start + 2..first_comment_start + 2 + end_first_comment];
    Some(serde_yaml::from_str(comment_inner).expect("invalid test configuration"))
}
