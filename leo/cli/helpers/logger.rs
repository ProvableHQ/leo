// Copyright (C) 2019-2023 Aleo Systems Inc.
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

use leo_errors::Result;

use colored::Colorize;
use std::{fmt, sync::Once};
use tracing::{event::Event, subscriber::Subscriber};
use tracing_subscriber::{
    fmt::{format::*, time::*, FmtContext, FormattedFields},
    registry::LookupSpan,
    FmtSubscriber,
};

static START: Once = Once::new();

#[derive(Debug, Clone)]
pub struct Format<F = Full, T = SystemTime> {
    format: F,
    #[allow(dead_code)] // todo: revisit this after merging span module
    pub timer: T,
    pub ansi: bool,
    pub display_target: bool,
    pub display_level: bool,
    pub display_thread_id: bool,
    pub display_thread_name: bool,
}

impl<F, T> Format<F, T> {
    /// Use the full JSON format.
    ///
    /// The full format includes fields from all entered spans.
    ///
    /// # Example Output
    ///
    /// ```ignore,json
    /// {"timestamp":"Feb 20 11:28:15.096","level":"INFO","target":"mycrate","fields":{"message":"some message", "key": "value"}}
    /// ```
    ///
    /// # Options
    ///
    /// - [`Format::flatten_event`] can be used to enable flattening event fields into the root
    /// object.
    ///
    /// [`Format::flatten_event`]: #method.flatten_event
    #[cfg(feature = "json")]
    pub fn json(self) -> Format<Json, T> {
        Format {
            format: Json::default(),
            timer: self.timer,
            ansi: self.ansi,
            display_target: self.display_target,
            display_level: self.display_level,
            display_thread_id: self.display_thread_id,
            display_thread_name: self.display_thread_name,
        }
    }

    /// Use the given [`timer`] for log message timestamps.
    ///
    /// See [`time`] for the provided timer implementations.
    ///
    /// Note that using the `chrono` feature flag enables the
    /// additional time formatters [`ChronoUtc`] and [`ChronoLocal`].
    ///
    /// [`time`]: ./time/index.html
    /// [`timer`]: ./time/trait.FormatTime.html
    /// [`ChronoUtc`]: ./time/struct.ChronoUtc.html
    /// [`ChronoLocal`]: ./time/struct.ChronoLocal.html
    pub fn with_timer<T2>(self, timer: T2) -> Format<F, T2> {
        Format {
            format: self.format,
            timer,
            ansi: self.ansi,
            display_target: self.display_target,
            display_level: self.display_level,
            display_thread_id: self.display_thread_id,
            display_thread_name: self.display_thread_name,
        }
    }

    /// Do not emit timestamps with log messages.
    pub fn without_time(self) -> Format<F, ()> {
        Format {
            format: self.format,
            timer: (),
            ansi: self.ansi,
            display_target: self.display_target,
            display_level: self.display_level,
            display_thread_id: self.display_thread_id,
            display_thread_name: self.display_thread_name,
        }
    }

    /// Enable ANSI terminal colors for formatted output.
    pub fn with_ansi(self, ansi: bool) -> Format<F, T> {
        Format { ansi, ..self }
    }

    /// Sets whether or not an event's target is displayed.
    pub fn with_target(self, display_target: bool) -> Format<F, T> {
        Format { display_target, ..self }
    }

    /// Sets whether or not an event's level is displayed.
    pub fn with_level(self, display_level: bool) -> Format<F, T> {
        Format { display_level, ..self }
    }

    /// Sets whether or not the [thread ID] of the current thread is displayed
    /// when formatting events
    ///
    /// [thread ID]: https://doc.rust-lang.org/stable/std/thread/struct.ThreadId.html
    pub fn with_thread_ids(self, display_thread_id: bool) -> Format<F, T> {
        Format { display_thread_id, ..self }
    }

    /// Sets whether or not the [name] of the current thread is displayed
    /// when formatting events
    ///
    /// [name]: https://doc.rust-lang.org/stable/std/thread/index.html#naming-threads
    pub fn with_thread_names(self, display_thread_name: bool) -> Format<F, T> {
        Format { display_thread_name, ..self }
    }
}

impl Default for Format<Full, SystemTime> {
    fn default() -> Self {
        Format {
            format: Full,
            timer: SystemTime,
            ansi: true,
            display_target: true,
            display_level: true,
            display_thread_id: false,
            display_thread_name: false,
        }
    }
}
impl<S, N, T> FormatEvent<S, N> for Format<Full, T>
where
    S: Subscriber + for<'a> LookupSpan<'a>,
    N: for<'a> FormatFields<'a> + 'static,
    T: FormatTime,
{
    fn format_event(&self, context: &FmtContext<'_, S, N>, mut writer: Writer, event: &Event<'_>) -> fmt::Result {
        let meta = event.metadata();

        if self.display_level {
            fn colored_string(level: &tracing::Level, message: &str) -> colored::ColoredString {
                match *level {
                    tracing::Level::ERROR => message.bold().red(),
                    tracing::Level::WARN => message.bold().yellow(),
                    tracing::Level::INFO => message.bold().cyan(),
                    tracing::Level::DEBUG => message.bold().magenta(),
                    tracing::Level::TRACE => message.bold(),
                }
            }

            let mut message = "".to_string();

            match context.lookup_current() {
                Some(span_ref) => {
                    let scope = span_ref.scope();

                    for span in scope {
                        message += span.metadata().name();

                        let ext = span.extensions();
                        let fields = &ext
                            .get::<FormattedFields<N>>()
                            .expect("Unable to find FormattedFields in extensions; this is a bug");
                        if !fields.is_empty() {
                            message = format!("{message} {{{fields}}}");
                        }
                    }
                }
                None => return Err(std::fmt::Error),
            }

            write!(&mut writer, "{:>10} ", colored_string(meta.level(), &message)).expect("Error writing event");
        }

        context.format_fields(writer.by_ref(), event)?;
        writeln!(&mut writer)
    }
}

/// Initialize logger with custom format and verbosity.
pub fn init_logger(_app_name: &'static str, verbosity: usize) -> Result<()> {
    // This line enables Windows 10 ANSI coloring API.
    #[cfg(target_family = "windows")]
    ansi_term::enable_ansi_support().map_err(|_| leo_errors::CliError::failed_to_enable_ansi_support())?;

    use tracing_subscriber::fmt::writer::MakeWriterExt;

    let stderr = std::io::stderr.with_max_level(tracing::Level::WARN);
    let mk_writer = stderr.or_else(std::io::stdout);

    let subscriber = FmtSubscriber::builder()
        // all spans/events with a level higher than TRACE (e.g, debug, info, warn, etc.)
        // will be written to stdout.
        .with_max_level(match verbosity {
            0 => tracing::Level::WARN,
            1 => tracing::Level::INFO,
            2 => tracing::Level::DEBUG,
            _ => tracing::Level::TRACE
        })
        .with_writer(mk_writer)
        .without_time()
        .with_target(false)
        .event_format(Format::default())
        .finish();

    // call this line only once per process. needed for tests using same thread
    START.call_once(|| {
        tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
    });
    Ok(())
}
