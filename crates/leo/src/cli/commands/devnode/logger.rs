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

use is_terminal::IsTerminal;
use std::{io, str::FromStr};
use tracing_subscriber::{EnvFilter, prelude::*};

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
                .with_filter(stdout_filter),
        )
        .try_init();

    Ok(())
}

fn parse_log_verbosity(verbosity: u8) -> Result<EnvFilter> {
    // Note, that this must not be prefixed with `RUST_LOG=`.
    let default_log_str = match verbosity {
        0 => "info",
        1 => "debug",
        2.. => "trace",
    };
    let filter = EnvFilter::from_str(default_log_str).unwrap();

    let filter = if verbosity >= 3 {
        filter.add_directive("leo_devnode_tcp=trace".parse().unwrap())
    } else {
        filter.add_directive("leo_devnode_tcp=off".parse().unwrap())
    };

    Ok(filter)
}
