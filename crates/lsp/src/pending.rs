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

#![allow(clippy::mutable_key_type)]

//! Shared drain abstractions for routing-thread pending-request state.
//!
//! Five pending-request structs (`semantic_tokens`, `definitions`,
//! `references`, `rename`, `prepare_rename`) all surface the same lifecycle
//! events: `didClose`, package eviction, bucket invalidation, and worker
//! panic. Each event drains pending entries on one of three dimensions —
//! URI, `PackageAnalysisKey`, or `AnalysisBucket` — then replies to the
//! drained requests. Without a shared abstraction, every fan-out helper in
//! `server.rs` repeats five copies of the same `drain_* + send_*` sequence.
//!
//! `PendingFeature` and `PendingRequest` collapse that to one generic helper.
//! The trait is generic over the per-feature request type so each implementor
//! keeps its own struct shape (cancel flag for references/rename; bare
//! `RequestId` for semantic tokens; query-only for definitions and
//! prepare-rename) without least-common-denominator dispatch.
//!
//! Per-feature operations that are not a four-dimension drain (semantic
//! tokens' `take_key`, references' and rename's `mark_undispatched`) stay on
//! the concrete state types — collapsing those would force every implementor
//! to grow methods it does not need.

use crate::document_store::{AnalysisBucket, PackageAnalysisKey};
use anyhow::Result;
use lsp_server::{Connection, Message, RequestId, Response, ResponseError};
use lsp_types::Uri;
use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};

/// One drainable pending request, sufficient to reply on cancellation.
pub trait PendingRequest {
    /// Original JSON-RPC request ID to answer.
    fn id(&self) -> &RequestId;

    /// Cancellation flag observed by any response-pool job dispatched against
    /// this request. References and rename carry one; semantic tokens,
    /// definitions, and prepare-rename do not. Routing-thread cancel paths
    /// flip the flag before replying so a still-running pool job drops its
    /// completion on arrival.
    fn cancel_flag(&self) -> Option<&Arc<AtomicBool>> {
        None
    }
}

impl PendingRequest for RequestId {
    /// A bare request ID is its own identity; carries no cancel flag.
    fn id(&self) -> &RequestId {
        self
    }
}

/// Routing-thread pending-state container shared by all LSP features.
///
/// Each implementor exposes the same three drain dimensions so the fan-out
/// helpers in `server.rs` can iterate them generically. The per-feature
/// `Request` associated type stays concrete so cancel flags and other
/// per-feature payload survive the drain.
pub trait PendingFeature {
    /// Per-feature pending request payload (e.g. `PendingRenameRequest`).
    type Request: PendingRequest;

    /// Remove and return all pending requests targeting this URI.
    /// Called from `didClose`.
    fn drain_uri(&mut self, uri: &Uri) -> Vec<Self::Request>;

    /// Remove and return all pending requests for one package key.
    /// Called from package-eviction, package-cancellation, and worker-panic
    /// fan-outs.
    fn drain_package(&mut self, key: &PackageAnalysisKey) -> Vec<Self::Request>;

    /// Remove and return all pending requests for one analysis bucket.
    /// Called from bucket-bump invalidation.
    fn drain_bucket(&mut self, bucket: &AnalysisBucket) -> Vec<Self::Request>;
}

/// Flip every cancel flag and send the same error reply to every drained request.
///
/// Cancel flags are flipped before any response is sent so an in-flight pool
/// completion that races the cancel reply observes the cancellation and drops
/// its payload on arrival rather than emitting a stale value the client
/// already received an error for.
pub fn cancel_drained<R: PendingRequest>(
    connection: &Connection,
    requests: Vec<R>,
    code: i32,
    message: impl Into<String>,
) -> Result<()> {
    let message = message.into();
    for request in requests {
        if let Some(flag) = request.cancel_flag() {
            flag.store(true, Ordering::SeqCst);
        }
        let response = Response {
            id: request.id().clone(),
            result: None,
            error: Some(ResponseError { code, message: message.clone(), data: None }),
        };
        connection.sender.send(Message::Response(response))?;
    }
    Ok(())
}
