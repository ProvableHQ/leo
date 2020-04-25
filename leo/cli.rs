use crate::cli_types::*;
use crate::errors::CLIError;

use clap::{App, AppSettings, Arg, ArgMatches, SubCommand};

pub trait CLI {
    type Options;

    const NAME: NameType;
    const ABOUT: AboutType;
    const FLAGS: &'static [FlagType];
    const OPTIONS: &'static [OptionType];
    const SUBCOMMANDS: &'static [SubCommandType];

    #[cfg_attr(tarpaulin, skip)]
    fn new<'a, 'b>() -> App<'a, 'b> {
        let flags = &Self::FLAGS
            .iter()
            .map(|a| Arg::from_usage(a).global(true))
            .collect::<Vec<Arg<'static, 'static>>>();
        let options = &Self::OPTIONS
            .iter()
            .map(|a| match a.2.len() > 0 {
                true => Arg::from_usage(a.0)
                    .conflicts_with_all(a.1)
                    .possible_values(a.2)
                    .requires_all(a.3),
                false => Arg::from_usage(a.0).conflicts_with_all(a.1).requires_all(a.3),
            })
            .collect::<Vec<Arg<'static, 'static>>>();
        let subcommands = Self::SUBCOMMANDS
            .iter()
            .map(|s| {
                SubCommand::with_name(s.0)
                    .about(s.1)
                    .args(
                        &s.2.iter()
                            .map(|a| match a.2.len() > 0 {
                                true => Arg::from_usage(a.0)
                                    .conflicts_with_all(a.1)
                                    .possible_values(a.2)
                                    .requires_all(a.3),
                                false => Arg::from_usage(a.0).conflicts_with_all(a.1).requires_all(a.3),
                            })
                            .collect::<Vec<Arg<'static, 'static>>>(),
                    )
                    .settings(s.3)
            })
            .collect::<Vec<App<'static, 'static>>>();

        SubCommand::with_name(Self::NAME)
            .about(Self::ABOUT)
            .settings(&[
                AppSettings::ColoredHelp,
                AppSettings::DisableHelpSubcommand,
                AppSettings::DisableVersion,
            ])
            .args(flags)
            .args(options)
            .subcommands(subcommands)
    }

    #[cfg_attr(tarpaulin, skip)]
    fn parse(arguments: &ArgMatches) -> Result<Self::Options, CLIError>;

    #[cfg_attr(tarpaulin, skip)]
    fn output(options: Self::Options) -> Result<(), CLIError>;
}
