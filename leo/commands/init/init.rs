use crate::commands::{cli::*, cli_types::*};
use crate::errors::{CLIError, ManifestError, NewError};
use crate::manifest::Manifest;

use clap::{ArgMatches, Values};
use colored::*;
use rand::{rngs::StdRng, Rng};
use rand_core::SeedableRng;
use serde::{Deserialize, Serialize};
use serde_json::from_str;
use std::{fmt, fmt::Display, fs, str::FromStr};
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct InitCommand {
    #[structopt(parse(from_os_str))]
    path: PathBuf,
}

impl CLI for InitCommand {
    type Options = ();

    const NAME: NameType = "init";
    const ABOUT: AboutType = "Creates a new Leo package (include -h for more options)";
    const FLAGS: &'static [FlagType] = &[];
    const OPTIONS: &'static [OptionType] = &[];
    const SUBCOMMANDS: &'static [SubCommandType] = &[];

    #[cfg_attr(tarpaulin, skip)]
    fn parse(arguments: &ArgMatches) -> Result<Self::Options, CLIError> {
        let mut options = ();
        // options.parse(arguments, &["count", "format", "json", "network"]);
        //
        // match arguments.subcommand() {
        //     ("hd", Some(arguments)) => {
        //         options.subcommand = Some("hd".into());
        //         options.parse(arguments, &["count", "json", "network"]);
        //         options.parse(arguments, &["derivation", "language", "password", "word count"]);
        //     }
        //     _ => {}
        // };

        Ok(options)
    }

    #[cfg_attr(tarpaulin, skip)]
    fn output(&mut self, options: Self::Options) -> Result<(), CLIError> {
        let package_name = self.path
            .file_stem()
            .ok_or_else(|| NewError::ProjectNameInvalid(self.path.as_os_str().to_owned()))?
            .to_string_lossy()
            .to_string();

        if self.path.exists() {
            return Err(NewError::DirectoryAlreadyExists(self.path.as_os_str().to_owned()).into());
        }
        fs::create_dir_all(&self.path).map_err(|error| {
            NewError::CreatingRootDirectory(self.path.as_os_str().to_owned(), error)
        })?;

        Manifest::new(&package_name).write_to(&self.path).map_err(NewError::ManifestError)?;

        Ok(())
    }
}
