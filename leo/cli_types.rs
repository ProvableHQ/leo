use clap::AppSettings;

pub type NameType = &'static str;

pub type AboutType = &'static str;

pub type DescriptionType = &'static str;

pub type RequiredType = bool;

pub type IndexType = u64;

pub type ArgumentType = (NameType, DescriptionType, RequiredType, IndexType);

pub type FlagType = &'static str;

pub type OptionType = (
    &'static str,
    &'static [&'static str],
    &'static [&'static str],
    &'static [&'static str],
);

pub type SubCommandType = (NameType, AboutType, &'static [OptionType], &'static [AppSettings]);
