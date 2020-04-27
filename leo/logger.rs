use colored::Colorize;
use std::io::Write;

const LEVEL_NAME_LENGTH: usize = 10;

#[allow(dead_code)]
fn level_string(level: log::Level) -> colored::ColoredString {
    match level {
        log::Level::Error => "ERROR".bold().red(),
        log::Level::Warn => "WARN".bold().yellow(),
        log::Level::Info => "INFO".bold().blue(),
        log::Level::Debug => "DEBUG".bold().magenta(),
        log::Level::Trace => "TRACE".bold(),
    }
}

/// Initialize logger with custom format and verbosity.
///
/// # Arguments
///
/// * `verbosity` - Verbosity level. 0 for `Warn`, 1 for `Info`, 2 for `Debug`, more for `Trace`
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
                "[{:>5} {:>5}] {}",
                level_string(record.level()),
                app_name,
                record.args().to_string().replace("\n", &padding)
            )
        })
        .init();
}
