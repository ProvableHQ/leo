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
use anyhow::{Error as AnyhowError, anyhow};
use tracing::info;

use axum::{
    extract::rejection::JsonRejection,
    http::{StatusCode, header::CONTENT_TYPE},
    response::{IntoResponse, Response},
};
use serde::{Deserialize, Serialize};

/// An enum of error handlers for the REST API server.
#[derive(Debug)]
pub enum RestError {
    /// 400 Bad Request - Invalid input, malformed parameters, validation errors
    BadRequest(AnyhowError),
    /// 404 Not Found - Resource not found
    NotFound(AnyhowError),
    /// 422 Unprocessable Entity - Business logic validation errors
    UnprocessableEntity(AnyhowError),
    /// 429 Too Many Requests - Rate limiting
    TooManyRequests(AnyhowError),
    // /// 503 Service Unavailable - Temporary service issues (node syncing, feature unavailable)
    // ServiceUnavailable(AnyhowError),
    /// 500 Internal Server Error - Actual server errors, unexpected failures
    InternalServerError(AnyhowError),
}

/// The serialized REST error sent over the network.
#[derive(Debug, Serialize, Deserialize)]
pub struct SerializedRestError {
    pub message: String,
    pub error_type: String,
    /// Does not include error chain in message if it is empty, and generates an empty error chain if none is given.
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub chain: Vec<String>,
}

impl RestError {
    /// Create a BadRequest error
    pub fn bad_request(inner: anyhow::Error) -> Self {
        Self::BadRequest(inner)
    }

    /// Create a NotFound error
    pub fn not_found(inner: anyhow::Error) -> Self {
        Self::NotFound(inner)
    }

    /// Create an UnprocessableEntity error
    pub fn unprocessable_entity(inner: anyhow::Error) -> Self {
        Self::UnprocessableEntity(inner)
    }

    /// Create a TooManyRequests error
    pub fn too_many_requests(inner: anyhow::Error) -> Self {
        Self::TooManyRequests(inner)
    }

    /// Create an InternalServerError error
    pub fn internal_server_error(inner: anyhow::Error) -> Self {
        Self::InternalServerError(inner)
    }

    /// Extract the full chain of errors from the `anyhow::Error`.
    /// (excludes the top-level error)
    fn error_chain(error: &AnyhowError) -> Vec<String> {
        let mut chain = vec![];
        let mut source = error.source();
        while let Some(err) = source {
            chain.push(err.to_string());
            source = err.source();
        }
        chain
    }
}

impl IntoResponse for RestError {
    fn into_response(self) -> Response {
        let (status, error_type, error) = match self {
            RestError::BadRequest(err) => (StatusCode::BAD_REQUEST, "bad_request", err),
            RestError::NotFound(err) => (StatusCode::NOT_FOUND, "not_found", err),
            RestError::UnprocessableEntity(err) => (StatusCode::UNPROCESSABLE_ENTITY, "unprocessable_entity", err),
            RestError::TooManyRequests(err) => (StatusCode::TOO_MANY_REQUESTS, "too_many_requests", err),
            RestError::InternalServerError(err) => (StatusCode::INTERNAL_SERVER_ERROR, "internal_server_error", err),
        };

        // Convert to JSON and include the chain of causes (if any).
        let json_body = serde_json::to_string(&SerializedRestError {
            message: error.to_string(),
            error_type: error_type.to_string(),
            chain: Self::error_chain(&error),
        })
        .unwrap_or_else(|err| format!("Failed to serialize error: {err}"));

        info!("Returning REST error: {json_body}");

        let mut response = Response::new(json_body.into());
        *response.status_mut() = status;
        response.headers_mut().insert(CONTENT_TYPE, "application/json".parse().unwrap());
        response
    }
}

impl From<anyhow::Error> for RestError {
    fn from(err: anyhow::Error) -> Self {
        // Default to 500 Internal Server Error
        Self::InternalServerError(err)
    }
}

impl From<String> for RestError {
    fn from(msg: String) -> Self {
        // Default to 500 Internal Server Error
        Self::InternalServerError(anyhow::anyhow!(msg))
    }
}

impl From<&str> for RestError {
    fn from(msg: &str) -> Self {
        // Default to 500 Internal Server Error
        Self::InternalServerError(anyhow::anyhow!("{msg}"))
    }
}

/// Implement `From<JsonRejection>` for `RestError` to enable automatic conversion
impl From<JsonRejection> for RestError {
    fn from(rejection: JsonRejection) -> Self {
        match rejection {
            JsonRejection::JsonDataError(err) => {
                RestError::bad_request(anyhow!(err).context("Invalid JSON data in request body"))
            }
            JsonRejection::JsonSyntaxError(err) => {
                RestError::bad_request(anyhow!(err).context("Invalid JSON syntax in request body"))
            }
            JsonRejection::MissingJsonContentType(_) => {
                RestError::bad_request(anyhow!("Content-Type must be `application/json`"))
            }
            JsonRejection::BytesRejection(err) => {
                RestError::bad_request(anyhow!(err).context("Failed to read request body"))
            }
            _ => RestError::bad_request(anyhow!("Invalid JSON request")),
        }
    }
}
