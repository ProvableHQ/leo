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

#![forbid(unsafe_code)]

mod helpers;
use helpers::*;

mod routes;

use snarkvm::{
    console::{program::ProgramID, types::Field},
    prelude::{Ledger, Network, Transaction, store::ConsensusStorage},
};

use super::*;

use anyhow::Context;
use axum::{
    body::Body,
    extract::{ConnectInfo, DefaultBodyLimit, Query, State},
    http::{Method, Request, StatusCode, header::CONTENT_TYPE},
    middleware,
    response::Response,
    routing::{get, post},
};
use axum_extra::response::ErasedJson;

use parking_lot::Mutex;
use std::{
    net::SocketAddr,
    sync::{Arc, atomic::AtomicUsize},
};
use tokio::{net::TcpListener, task::JoinHandle};
// use tower::util::ServiceExt;
use tower_governor::{GovernorLayer, governor::GovernorConfigBuilder};
use tower_http::{
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};
use tracing::{debug, info};

/// The API version prefixes.
pub const API_VERSION_V1: &str = "v1";
pub const API_VERSION_V2: &str = "v2";

/// A REST API server for the ledger.
#[derive(Clone)]
pub struct Rest<N: Network, C: ConsensusStorage<N>> {
    /// The ledger.
    ledger: Ledger<N, C>,
    /// A buffer for pending transactions to be added to the next block.
    buffer: Arc<Mutex<Vec<Transaction<N>>>>,
    /// The server handles.
    handles: Arc<Mutex<Vec<JoinHandle<()>>>>,
    /// The number of ongoing deploy transaction verifications via REST.
    num_verifying_deploys: Arc<AtomicUsize>,
    /// The number of ongoing execute transaction verifications via REST.
    num_verifying_executions: Arc<AtomicUsize>,
    /// Whether manual block creation is enabled.
    manual_block_creation: bool,
    /// The Private Key used for block creation.
    private_key: String,
}

impl<N: Network, C: 'static + ConsensusStorage<N>> Rest<N, C> {
    /// Initializes a new instance of the server.
    pub async fn start(
        rest_ip: SocketAddr,
        rest_rps: u32,
        ledger: Ledger<N, C>,
        manual_block_creation: bool,
        private_key: String,
    ) -> Result<Self> {
        // Initialize the server.
        let mut server = Self {
            ledger,
            buffer: Arc::new(Mutex::new(Vec::new())),
            handles: Default::default(),
            num_verifying_deploys: Default::default(),
            num_verifying_executions: Default::default(),
            manual_block_creation,
            private_key,
        };
        // Spawn the server.
        server.spawn_server(rest_ip, rest_rps).await?;
        // Return the server.
        Ok(server)
    }
}

impl<N: Network, C: ConsensusStorage<N>> Rest<N, C> {
    fn build_routes(&self, rest_rps: u32) -> axum::Router {
        let cors = CorsLayer::new()
            .allow_origin(Any)
            .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
            .allow_headers([CONTENT_TYPE]);

        // Prepare the rate limiting setup.
        let governor_config = GovernorConfigBuilder::default()
            .per_nanosecond((1_000_000_000 / rest_rps) as u64)
            .burst_size(rest_rps)
            .finish()
            .expect("Couldn't set up rate limiting for the REST server!");

        let governor_layer =
            GovernorLayer::new(governor_config).error_handler(|error: tower_governor::errors::GovernorError| {
                // Properly return a 429 Too Many Requests error
                let error_message = error.to_string();

                let mut response = axum::response::Response::new(error_message.clone().into());

                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;

                if error_message.contains("Too Many Requests") {
                    *response.status_mut() = StatusCode::TOO_MANY_REQUESTS;
                }

                response
            });

        let routes = axum::Router::new()
            // Get ../consensus_version
            .route("/consensus_version", get(Self::get_consensus_version))

            // GET ../block/..
            .route("/block/height/latest", get(Self::get_block_height_latest))
            .route("/block/hash/latest", get(Self::get_block_hash_latest))
            .route("/block/latest", get(Self::get_block_latest))
            .route("/block/{height_or_hash}", get(Self::get_block))
            // The path param here is actually only the height, but the name must match the route
            // above, otherwise there'll be a conflict at runtime.
            .route("/block/{height_or_hash}/header", get(Self::get_block_header))
            .route("/block/{height_or_hash}/transactions", get(Self::get_block_transactions))
            .route("/block/create", post(Self::create_block))

            // GET and POST ../transaction/..
            .route("/transaction/broadcast", post(Self::transaction_broadcast))
            .route("/transaction/confirmed/{id}", get(Self::get_confirmed_transaction))
            .route("/transaction/unconfirmed/{id}", get(Self::get_unconfirmed_transaction))
            .route("/transaction/{id}", get(Self::get_transaction))

            // GET ../find/..
            .route("/find/blockHash/{tx_id}", get(Self::find_block_hash))
            .route("/find/blockHeight/{state_root}", get(Self::find_block_height_from_state_root))
            .route("/find/transactionID/deployment/{program_id}", get(Self::find_latest_transaction_id_from_program_id))
            .route("/find/transactionID/deployment/{program_id}/{edition}", get(Self::find_transaction_id_from_program_id_and_edition))
            .route("/find/transactionID/{transition_id}", get(Self::find_transaction_id_from_transition_id))
            .route("/find/transitionID/{input_or_output_id}", get(Self::find_transition_id))

            // GET ../program/..
            .route("/program/{id}", get(Self::get_program))
            .route("/program/{id}/latest_edition", get(Self::get_latest_program_edition))
            .route("/program/{id}/{edition}", get(Self::get_program_for_edition))
            .route("/program/{id}/mappings", get(Self::get_mapping_names))
            .route("/program/{id}/mapping/{name}/{key}", get(Self::get_mapping_value))
            .route("/program/{id}/mapping/{name}", get(Self::get_mapping_values))

            // GET misc endpoints.
            .route("/blocks", get(Self::get_blocks))
            .route("/height/{hash}", get(Self::get_height))
            .route("/statePath/{commitment}", get(Self::get_state_path_for_commitment))
            .route("/statePaths", get(Self::get_state_paths_for_commitments))
            .route("/stateRoot/latest", get(Self::get_state_root_latest))
            .route("/stateRoot/{height}", get(Self::get_state_root));

        routes
            // Pass in `Rest` to make things convenient.
            .with_state(self.clone())
            // Enable tower-http tracing.
            .layer(TraceLayer::new_for_http())
            // Custom logging.
            .layer(middleware::map_request(log_middleware))
            // Enable CORS.
            .layer(cors)
            // Cap the request body size at 512KiB.
            .layer(DefaultBodyLimit::max(512 * 1024))
            .layer(governor_layer)
    }

    async fn spawn_server(&mut self, rest_ip: SocketAddr, rest_rps: u32) -> Result<()> {
        // Log the REST rate limit per IP.
        debug!("REST rate limit per IP - {rest_rps} RPS");

        // Add the v1 API as default and under "/v1".
        let default_router = axum::Router::new().nest(
            &format!("/{}", N::SHORT_NAME),
            self.build_routes(rest_rps).layer(middleware::map_response(v1_error_middleware)),
        );
        let v1_router = axum::Router::new().nest(
            &format!("/{API_VERSION_V1}/{}", N::SHORT_NAME),
            self.build_routes(rest_rps).layer(middleware::map_response(v1_error_middleware)),
        );

        // Add the v2 API under "/v2".
        let v2_router =
            axum::Router::new().nest(&format!("/{API_VERSION_V2}/{}", N::SHORT_NAME), self.build_routes(rest_rps));

        // Combine all routes.
        let router = default_router.merge(v1_router).merge(v2_router);

        let rest_listener =
            TcpListener::bind(rest_ip).await.with_context(|| "Failed to bind TCP port for REST endpoints")?;

        let handle = tokio::spawn(async move {
            if let Err(e) = axum::serve(rest_listener, router.into_make_service_with_connect_info::<SocketAddr>()).await
            {
                eprintln!("REST server crashed: {e:?}");
            }
        });

        self.handles.lock().push(handle);
        Ok(())
    }
}

/// Creates a log message for every HTTP request.
async fn log_middleware(ConnectInfo(addr): ConnectInfo<SocketAddr>, request: Request<Body>) -> Request<Body> {
    info!("Received {:?} {:?} from {:?}", request.method(), request.uri(), addr);
    request
}

/// Converts errors to the old style for the v1 API.
/// The error code will always be 500 and the content a simple string.
async fn v1_error_middleware(response: Response) -> Response {
    // The status code used by all v1 errors
    const V1_STATUS_CODE: StatusCode = StatusCode::INTERNAL_SERVER_ERROR;

    if response.status().is_success() {
        return response;
    }

    // Returns a opaque error instead of panicking.
    let fallback = || {
        let mut response = Response::new(Body::from("Failed to convert error"));
        *response.status_mut() = V1_STATUS_CODE;
        response
    };

    let Ok(bytes) = axum::body::to_bytes(response.into_body(), usize::MAX).await else {
        return fallback();
    };

    // Deserialize REST error so we can convert it to a string.
    let Ok(json_err) = serde_json::from_slice::<SerializedRestError>(&bytes) else {
        return fallback();
    };

    let mut message = json_err.message;
    for next in json_err.chain.into_iter() {
        message = format!("{message} — {next}");
    }

    let mut response = Response::new(Body::from(message));

    *response.status_mut() = V1_STATUS_CODE;

    response
}
