use leo::errors::CLIError;
use leo::{cli::*, commands::*, logger};

use clap::{App, AppSettings};

#[cfg_attr(tarpaulin, skip)]
fn main() -> Result<(), CLIError> {
    logger::init_logger("leo", 1);

    let arguments = App::new("leo")
        .version("v0.1.0")
        .about("Leo compiler and package manager")
        .author("The Aleo Team <hello@aleo.org>")
        .settings(&[
            AppSettings::ColoredHelp,
            AppSettings::DisableHelpSubcommand,
            AppSettings::DisableVersion,
            AppSettings::SubcommandRequiredElseHelp,
        ])
        // TODO (howardwu): Print subcommands in non-alphabetical order, instead print as ordered here.
        .subcommands(vec![
            NewCommand::new(),
            InitCommand::new(),
            BuildCommand::new(),
            SetupCommand::new(),
            RunCommand::new(),
        ])
        .set_term_width(0)
        .get_matches();

    match arguments.subcommand() {
        ("new", Some(arguments)) => NewCommand::output(NewCommand::parse(arguments)?),
        ("init", Some(arguments)) => InitCommand::output(InitCommand::parse(arguments)?),
        ("build", Some(arguments)) => {
            BuildCommand::output(BuildCommand::parse(arguments)?)?;
            Ok(())
        }
        ("setup", Some(arguments)) => {
            SetupCommand::output(SetupCommand::parse(arguments)?)?;
            Ok(())
        }
        ("run", Some(arguments)) => RunCommand::output(RunCommand::parse(arguments)?),
        _ => unreachable!(),
    }
}
