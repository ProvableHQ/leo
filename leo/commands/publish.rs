use crate::{
    cli::*,
    cli_types::*,
    commands::BuildCommand,
    directories::{INPUTS_DIRECTORY_NAME, OUTPUTS_DIRECTORY_NAME},
    errors::CLIError,
    files::{
        Manifest,
        BYTES_FILE_EXTENSION,
        CHECKSUM_FILE_EXTENSION,
        INPUTS_FILE_EXTENSION,
        PROOF_FILE_EXTENSION,
        PROVING_KEY_FILE_EXTENSION,
        VERIFICATION_KEY_FILE_EXTENSION,
    },
};

use clap::ArgMatches;
use std::{
    convert::TryFrom,
    env::current_dir,
    fs::File,
    io::{Read, Write},
    path::{Path, PathBuf},
};
use walkdir::WalkDir;
use zip::write::{FileOptions, ZipWriter};

#[derive(Debug)]
pub struct PublishCommand;

impl CLI for PublishCommand {
    type Options = ();
    type Output = ();

    const ABOUT: AboutType = "Publish the current package to the package manager (*)";
    const ARGUMENTS: &'static [ArgumentType] = &[];
    const FLAGS: &'static [FlagType] = &[];
    const NAME: NameType = "publish";
    const OPTIONS: &'static [OptionType] = &[];
    const SUBCOMMANDS: &'static [SubCommandType] = &[];

    #[cfg_attr(tarpaulin, skip)]
    fn parse(_arguments: &ArgMatches) -> Result<Self::Options, CLIError> {
        Ok(())
    }

    #[cfg_attr(tarpaulin, skip)]
    fn output(options: Self::Options) -> Result<Self::Output, CLIError> {
        let (_program, _checksum_differs) = BuildCommand::output(options)?;

        // Get the package name
        let src_dir = current_dir()?;

        // Build walkdir iterator from current package
        let walkdir = WalkDir::new(src_dir.clone());

        // Create zip file
        let package_name = Manifest::try_from(&src_dir)?.get_package_name();
        let mut zip_file = src_dir.clone();
        zip_file.push(PathBuf::from(format!("{}{}", package_name, ".zip".to_string())));

        let file = &mut File::create(zip_file)?;
        let mut zip = ZipWriter::new(file);
        let zip_options = FileOptions::default()
            .compression_method(zip::CompressionMethod::Stored)
            .unix_permissions(0o755);

        // Walk through files in directory and write desired ones to the zip file
        let mut buffer = Vec::new();
        for entry in walkdir.into_iter().filter_map(|e| e.ok()) {
            let path = entry.path();
            let name = path.strip_prefix(src_dir.as_path()).unwrap();

            // filter excluded paths
            if is_excluded(name) {
                continue;
            }

            // write file or directory
            if path.is_file() {
                log::info!("adding file {:?} as {:?}", path, name);
                zip.start_file_from_path(name, zip_options)?;
                let mut f = File::open(path)?;

                f.read_to_end(&mut buffer)?;
                zip.write_all(&*buffer)?;
                buffer.clear();
            } else if name.as_os_str().len() != 0 {
                // Only if not root Avoids path spec / warning
                // and mapname conversion failed error on unzip
                log::info!("adding dir {:?} as {:?}", path, name);
                zip.add_directory_from_path(name, zip_options)?;
            }
        }

        zip.finish()?;

        log::info!("zip file created");

        Ok(())
    }
}

fn is_excluded(path: &Path) -> bool {
    // excluded directories: `/inputs`, `/outputs`
    if path.ends_with(INPUTS_DIRECTORY_NAME.trim_end_matches("/"))
        | path.ends_with(OUTPUTS_DIRECTORY_NAME.trim_end_matches("/"))
    {
        return true;
    }

    // excluded extensions: `.in`, `.bytes`, `lpk`, `lvk`, `.proof`, `.sum`
    path.extension()
        .map(|ext| {
            if ext.eq(INPUTS_FILE_EXTENSION.trim_start_matches("."))
                | ext.eq(BYTES_FILE_EXTENSION.trim_start_matches("."))
                | ext.eq(PROVING_KEY_FILE_EXTENSION.trim_start_matches("."))
                | ext.eq(VERIFICATION_KEY_FILE_EXTENSION.trim_start_matches("."))
                | ext.eq(PROOF_FILE_EXTENSION.trim_start_matches("."))
                | ext.eq(CHECKSUM_FILE_EXTENSION.trim_start_matches("."))
                | ext.eq("zip")
            {
                true
            } else {
                false
            }
        })
        .unwrap_or(false)
}
