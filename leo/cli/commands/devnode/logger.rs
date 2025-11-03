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

use tracing_subscriber::EnvFilter;
use tracing_subscriber::prelude::*;
use is_terminal::IsTerminal;
use std::{io, str::FromStr};

use super::*;

pub fn initialize_terminal_logger(verbosity: u8) -> Result<()> {
    let stdout_filter = parse_log_verbosity(verbosity)?;

    // At high verbosity or when there is a custom log filter we show the target
    // of the log event, i.e., the file/module where the log message was created.
    let show_target = verbosity > 2;

    // Initialize tracing.
    let _ = tracing_subscriber::registry()
        .with(
            // Add layer using LogWriter for stdout / terminal
            tracing_subscriber::fmt::Layer::default()
                .with_ansi(io::stdout().is_terminal())
                .with_target(show_target)
                // .event_format(DynamicFormatter::new(Arc::new(AtomicBool::new(false))))
                .with_filter(stdout_filter),
        )
        .try_init();

    Ok(())
}

fn parse_log_verbosity(verbosity: u8) -> Result<EnvFilter> {
    // First, set default log verbosity.
    // Note, that this must not be prefixed with `RUST_LOG=`.
    let default_log_str = match verbosity {
        0 => "info",
        1 => "debug",
        2.. => "trace",
    };
    let filter = EnvFilter::from_str(default_log_str).unwrap();

    // Now, set rules for specific crates.
    let filter = if verbosity >= 2 {
        filter.add_directive("snarkos_node_sync=trace".parse().unwrap())
    } else {
        filter.add_directive("snarkos_node_sync=debug".parse().unwrap())
    };

    let filter = if verbosity >= 3 {
        filter
            .add_directive("snarkos_node_bft=trace".parse().unwrap())
            .add_directive("snarkos_node_bft::gateway=debug".parse().unwrap())
    } else {
        filter.add_directive("snarkos_node_bft=debug".parse().unwrap())
    };

    let filter = if verbosity >= 4 {
        let filter = filter.add_directive("snarkos_node_bft::gateway=trace".parse().unwrap());

        // At high log levels, also show warnings of third-party crates.
        filter
            .add_directive("mio=warn".parse().unwrap())
            .add_directive("tokio_util=warn".parse().unwrap())
            .add_directive("hyper=warn".parse().unwrap())
            .add_directive("reqwest=warn".parse().unwrap())
            .add_directive("want=warn".parse().unwrap())
            .add_directive("h2=warn".parse().unwrap())
            .add_directive("tower=warn".parse().unwrap())
            .add_directive("axum=warn".parse().unwrap())
            .add_directive("ureq=warn".parse().unwrap())
    } else {
        let filter = filter.add_directive("snarkos_node_bft::gateway=debug".parse().unwrap());

        // Disable logs from third-party crates by default.
        filter
            .add_directive("mio=off".parse().unwrap())
            .add_directive("tokio_util=off".parse().unwrap())
            .add_directive("hyper=off".parse().unwrap())
            .add_directive("reqwest=off".parse().unwrap())
            .add_directive("want=off".parse().unwrap())
            .add_directive("h2=off".parse().unwrap())
            .add_directive("tower=off".parse().unwrap())
            .add_directive("axum=off".parse().unwrap())
            .add_directive("ureq=off".parse().unwrap())
    };

    let filter = if verbosity >= 5 {
        filter.add_directive("snarkos_node_router=trace".parse().unwrap())
    } else {
        filter.add_directive("snarkos_node_router=debug".parse().unwrap())
    };

    let filter = if verbosity >= 6 {
        filter.add_directive("snarkos_node_tcp=trace".parse().unwrap())
    } else {
        filter.add_directive("snarkos_node_tcp=off".parse().unwrap())
    };

    Ok(filter)
}