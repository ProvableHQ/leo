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

use super::RestError;

use axum::{
    extract::{FromRequestParts, path::ErrorKind, rejection::PathRejection},
    http::request::Parts,
};
use serde::de::DeserializeOwned;

struct PathError {
    message: String,
    path: String,
    cause: anyhow::Error,
    location: Option<String>,
}

/// Convert Path errors into the unified REST error type.
impl From<PathError> for RestError {
    fn from(val: PathError) -> Self {
        let err = if let Some(loc) = val.location {
            val.cause.context(format!("Invalid argument \"{loc}\" in path \"{}\": {}", val.path, val.message))
        } else {
            val.cause.context(format!("Invalid path \"{}\": {}", val.path, val.message))
        };

        RestError::bad_request(err)
    }
}

/// Custom Path extractor to improve errors in invalid URLs.
/// Adapted from axum's [customize-path-rejection](https://github.com/tokio-rs/axum/blob/main/examples/customize-path-rejection/src/main.rs)
pub struct Path<T>(pub T);

impl<S, T> FromRequestParts<S> for Path<T>
where
    T: DeserializeOwned + Send,
    S: Send + Sync,
{
    type Rejection = RestError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        match axum::extract::Path::<T>::from_request_parts(parts, state).await {
            Ok(value) => Ok(Self(value.0)),
            Err(rejection) => {
                let err = match rejection {
                    PathRejection::FailedToDeserializePathParams(inner) => {
                        let kind = inner.kind();

                        let (message, location) = match &kind {
                            ErrorKind::WrongNumberOfParameters { .. } => {
                                ("wrong number of parameters".to_string(), None)
                            }
                            ErrorKind::ParseErrorAtKey { key, value, expected_type } => (
                                format!("value `{value}` is not of expected type `{expected_type}`"),
                                Some(key.clone()),
                            ),
                            ErrorKind::ParseErrorAtIndex { index, value, expected_type } => (
                                format!("value `{value}` at index {index} is not of expected type `{expected_type}`"),
                                None,
                            ),
                            ErrorKind::ParseError { value, expected_type } => {
                                (format!("value `{value}` is not of expected type `{expected_type}`"), None)
                            }
                            ErrorKind::InvalidUtf8InPathParam { key } => {
                                ("invalid UTF-8 in parameter".to_string(), Some(key.clone()))
                            }
                            ErrorKind::Message(msg) => (format!("unknown error: {msg}"), None),
                            _ => ("unknown error".to_string(), None),
                        };

                        PathError { message, path: parts.uri.path().to_string(), location, cause: inner.into() }
                    }
                    PathRejection::MissingPathParams(error) => PathError {
                        message: "missing path parameter".to_string(),
                        path: parts.uri.path().to_string(),
                        location: None,
                        cause: error.into(),
                    },
                    _ => PathError {
                        message: "unknown path error".to_string(),
                        path: parts.uri.path().to_string(),
                        location: None,
                        cause: rejection.into(),
                    },
                };

                Err(err.into())
            }
        }
    }
}
