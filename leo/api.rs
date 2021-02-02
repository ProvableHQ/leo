// Copyright (C) 2019-2020 Aleo Systems Inc.
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

use anyhow::Error;
use reqwest::{
    self,
    blocking::{Client, Response},
};
use serde::Serialize;

#[derive(Clone)]
pub struct Api {
    host: String,
    client: Client,
}

/// Body for POST /v1/package/fetch query
/// Fetch package and install it locally
#[derive(Serialize)]
struct Fetch {
    author: String,
    package_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    version: Option<String>,
}

/// Body for POST /v1/account/authenticate query
/// Used for logging in and writing auth token
#[derive(Serialize)]
struct Login {
    email_username: String,
    password: String,
}

impl Api {
    /// Create new instance of an API
    pub fn new(host: String) -> Api {
        Api {
            client: Client::new(),
            host,
        }
    }

    pub fn fetch(
        self,
        author: String,
        package_name: String,
        version: Option<String>,
        token_auth: Option<String>,
    ) -> Result<Response, Error> {
        let data = Fetch {
            author,
            package_name,
            version,
        };
        let req = self
            .client
            .post(&format!("{}{}", self.host, "v1/package/fetch"))
            .json(&data);

        match token_auth {
            Some(token) => req.bearer_auth(token),
            None => req,
        }
        .send()
        .map_err(|err| err.into())
    }
}
