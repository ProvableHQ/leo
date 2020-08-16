use crate::{cli::CLI, cli_types::*, commands::BuildCommand, errors::CLIError};
use clap::ArgMatches;
use notify::{watcher, DebouncedEvent, RecursiveMode, Watcher};
use std::{sync::mpsc::channel, time::Duration};

const LEO_SOURCE_DIR: &str = "src/";

// Time interval for watching files, in seconds
const INTERVAL: u64 = 3;

pub struct WatchCommand;

impl CLI for WatchCommand {
    type Options = ();
    type Output = ();

    const ABOUT: AboutType = "Watch the changes of the leo's source files";
    const ARGUMENTS: &'static [ArgumentType] = &[];
    const FLAGS: &'static [FlagType] = &[];
    const NAME: NameType = "watch";
    const OPTIONS: &'static [OptionType] = &[];
    const SUBCOMMANDS: &'static [SubCommandType] = &[];

    #[cfg_attr(tarpaulin, skip)]
    fn parse(_arguments: &ArgMatches) -> Result<Self::Options, CLIError> {
        Ok(())
    }

    fn output(_options: Self::Options) -> Result<Self::Output, CLIError> {
        let (tx, rx) = channel();
        let mut watcher = watcher(tx, Duration::from_secs(INTERVAL)).unwrap();
        watcher.watch(LEO_SOURCE_DIR, RecursiveMode::Recursive).unwrap();

        log::info!("Watching Leo source code");
        loop {
            match rx.recv() {
                // See changes on the write event
                Ok(DebouncedEvent::Write(_write)) => {
                    let options = ();
                    match BuildCommand::output(options) {
                        Ok(_output) => {
                            log::info!("Built successfully");
                        }
                        Err(e) => {
                            // Syntax error
                            log::error!("Error {:?}", e);
                        }
                    };
                }
                // Other events
                Ok(_event) => {}

                // Watch error
                Err(e) => {
                    log::error!("watch error: {:?}", e)
                    // TODO (howardwu): Add graceful termination.
                }
            }
        }
    }
}
