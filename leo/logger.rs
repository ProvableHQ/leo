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

use colored::Colorize;
use std::io::Write;
use tracing_subscriber::FmtSubscriber;

const LEVEL_NAME_LENGTH: usize = 10;

#[allow(dead_code)]
fn colored_string(level: log::Level, message: &str) -> colored::ColoredString {
    match level {
        log::Level::Error => message.bold().red(),
        log::Level::Warn => message.bold().yellow(),
        log::Level::Info => message.bold().cyan(),
        log::Level::Debug => message.bold().magenta(),
        log::Level::Trace => message.bold(),
    }
}

/// Initialize logger with custom format and verbosity.
pub fn init_logger(app_name: &'static str, verbosity: usize) {
    let subscriber = FmtSubscriber::builder()
        // all spans/events with a level higher than TRACE (e.g, debug, info, warn, etc.)
        // will be written to stdout.
        .with_max_level(match verbosity {
            0 => tracing::Level::WARN,
            1 => tracing::Level::INFO,
            2 => tracing::Level::DEBUG,
            _ => tracing::Level::TRACE
        })
        .without_time()
        .with_target(false)
        // completes the builder.
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    env_logger::builder()
        .filter_level(match verbosity {
            0 => log::LevelFilter::Warn,
            1 => log::LevelFilter::Info,
            2 => log::LevelFilter::Debug,
            _ => log::LevelFilter::Trace,
        })
        .format(move |buf, record| {
            let mut padding = String::from("\n");
            for _ in 0..(app_name.len() + LEVEL_NAME_LENGTH + 4) {
                padding.push(' ');
            }

            writeln!(
                buf,
                "{:>5}  {}",
                colored_string(record.level(), app_name),
                record.args().to_string().replace("\n", &padding)
            )
        })
        .init();
}
