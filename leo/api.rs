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

use serde::Serialize;

use anyhow::{anyhow, Error, Result};
use reqwest::{
    blocking::{Client, Response},
    Method,
    StatusCode,
};

pub trait Route {
    /// Whether to use bearer auth or not. Some routes may have additional
    /// features for logged-in users, so authorization token should be sent
    /// if it is created of course
    const AUTH: bool;

    /// HTTP method to use when requesting
    const METHOD: Method;

    /// URL path without first forward slash (e.g. v1/package/fetch)
    const PATH: &'static str;

    /// Output type for this route. For login it is simple - String
    /// But for other routes may be more complex.
    type Output;

    /// Process reqwest Response and turn it into Output
    fn process(&self, res: Response) -> Result<Self::Output>;

    /// Transform specific status codes into correct errors for this route.
    /// For example 404 on package fetch should mean that 'Package is not found'
    fn status_to_err(&self, status: StatusCode) -> Error {
        match status {
            _ => anyhow!("Unidentified API error"),
        }
    }
}

/// REST API handler with reqwest::blocking inside
#[derive(Clone, Debug)]
pub struct Api {
    host: String,
    client: Client,
}

impl Api {
    /// Create new instance of API, set host and Client is going to be
    /// created and set automatically
    pub fn new(host: String) -> Api {
        Api {
            client: Client::new(),
            host,
        }
    }

    /// Run specific route struct. Turn struct into request body
    /// and use type constants and Route implementation to get request params
    pub fn run_route<T>(&self, route: T) -> Result<T::Output>
    where
        T: Route,
        T: Serialize,
    {
        let res = self
            .client
            .request(T::METHOD, &format!("{}{}", self.host, T::PATH))
            .json(&route)
            .send()
            .map_err(|_| anyhow!("Unable to connect to Aleo PM"))?;

        route.process(res)
    }
}

// --------------------------------------------------
// |                Defining routes                 |
// --------------------------------------------------

/// Handler for 'fetch' route - fetch packages from Aleo PM
/// Route: POST /v1/package/fetch
#[derive(Serialize, Debug)]
pub struct Fetch {
    pub author: String,
    pub package_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

impl Route for Fetch {
    type Output = Response;

    const AUTH: bool = true;
    const METHOD: Method = Method::POST;
    const PATH: &'static str = "api/package/fetch";

    fn process(&self, res: Response) -> Result<Self::Output, Error> {
        // check status code first
        if res.status() != 200 {
            return Err(self.status_to_err(res.status()));
        };

        Ok(res)
    }

    fn status_to_err(&self, status: StatusCode) -> Error {
        match status {
            StatusCode::BAD_REQUEST => anyhow!("Package is not found - check author and/or package name"),
            // This one is ILLOGICAL - we should return 404 on incorrect author/package
            // and return BAD_REQUEST if data format is incorrect
            StatusCode::NOT_FOUND => anyhow!("Package is hidden"),
            _ => anyhow!("Unknown API error: {}", status),
        }
    }
}

/// Handler for 'login' route - send username and password and receive JWT
/// Route: POST /v1/account/authenticate
#[derive(Serialize)]
pub struct Login {
    pub email_username: String,
    pub password: String,
}

impl Route for Login {
    type Output = Response;

    const AUTH: bool = false;
    const METHOD: Method = Method::POST;
    const PATH: &'static str = "api/account/authenticate";

    fn process(&self, res: Response) -> Result<Self::Output, Error> {
        if res.status() != 200 {
            return Err(self.status_to_err(res.status()));
        }

        Ok(res)
    }

    fn status_to_err(&self, status: StatusCode) -> Error {
        match status {
            StatusCode::BAD_REQUEST => anyhow!("This username is not yet registered or the password is incorrect"),
            // NOT_FOUND here should be replaced, this error code has no relation
            // to what this route is doing
            StatusCode::NOT_FOUND => anyhow!("Incorrect password"),
            _ => anyhow!("Unknown API error: {}", status),
        }
    }
}
