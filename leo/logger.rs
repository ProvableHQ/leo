use colored::Colorize;
use std::io::Write;

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
