use crate::{cli::CLI, cli_types::*};
use std::{
    fs::{create_dir, File},
    io::prelude::*,
    path::Path,
};

const LEO_CREDENTIALS_DIR: &str = ".leo";
const LEO_CREDENTIALS_FILE: &str = "credentials";

#[derive(Debug)]
pub struct LoginCommand;

impl CLI for LoginCommand {
    type Options = Option<String>;
    type Output = ();

    const ABOUT: AboutType = "Login to the package manager (*)";
    const ARGUMENTS: &'static [ArgumentType] = &[
        // (name, description, required, index)
        ("NAME", "Sets token for login to the package manager", false, 1u64),
    ];
    const FLAGS: &'static [FlagType] = &[];
    const NAME: NameType = "login";
    const OPTIONS: &'static [OptionType] = &[];
    const SUBCOMMANDS: &'static [SubCommandType] = &[];

    fn parse(arguments: &clap::ArgMatches) -> Result<Self::Options, crate::errors::CLIError> {
        match arguments.value_of("NAME") {
            Some(name) => Ok(Some(name.to_string())),
            None => Ok(None),
        }
    }

    fn output(options: Self::Options) -> Result<Self::Output, crate::errors::CLIError> {
        let token = match options {
            Some(token) => token,
            None => {
                // TODO JWT or make a request to login
                unimplemented!()
            }
        };

        match Path::new(LEO_CREDENTIALS_DIR).exists() {
            true => {}
            false => {
                create_dir(LEO_CREDENTIALS_DIR)?;
            }
        }

        let mut credentials = File::create(&format!("{}/{}", LEO_CREDENTIALS_DIR, LEO_CREDENTIALS_FILE))?;
        let mut buf = String::new();
        buf.push_str(token.as_str());
        credentials.write_all(&buf.as_bytes())?;
        Ok(())
    }
}

// Run tests in single thread
#[cfg(test)]
mod tests {

    use super::*;

    use std::{
        env,
        fs::{remove_dir, remove_file},
        io,
    };

    const TEST_DIR: &str = ".test";

    fn setup() -> Result<(), io::Error> {
        create_dir(TEST_DIR)?;
        env::set_current_dir(TEST_DIR)?;
        Ok(())
    }

    fn clean() -> Result<(), io::Error> {
        remove_file(&format!("{}/{}", LEO_CREDENTIALS_DIR, LEO_CREDENTIALS_FILE))?;
        remove_dir(LEO_CREDENTIALS_DIR)?;
        env::set_current_dir("../")?;
        remove_dir(TEST_DIR)?;
        Ok(())
    }

    fn get_token() -> Result<String, io::Error> {
        let mut credentials = File::open(&format!("{}/{}", LEO_CREDENTIALS_DIR, LEO_CREDENTIALS_FILE))?;
        let mut buf = String::new();
        credentials.read_to_string(&mut buf)?;
        Ok(buf)
    }

    fn create_token(token: &str) -> Result<(), io::Error> {
        let mut f = File::create(&format!("{}/{}", LEO_CREDENTIALS_DIR, LEO_CREDENTIALS_FILE))?;
        f.write_all(token.as_bytes())?;
        Ok(())
    }

    #[test]
    fn test_credential_dir_exists() -> Result<(), io::Error> {
        setup()?;
        create_dir(LEO_CREDENTIALS_DIR)?;

        let token = "SOME_TOKEN".to_string();
        let options = Some(token.clone());
        LoginCommand::output(options).unwrap();
        assert_eq!(token, get_token()?);
        clean()?;
        Ok(())
    }

    #[test]
    fn test_credential_file_exists() -> Result<(), io::Error> {
        setup()?;
        create_dir(LEO_CREDENTIALS_DIR)?;
        create_token("OLD_TOKEN")?;

        let token = "NEW_TOKEN".to_string();
        let options = Some(token.clone());
        LoginCommand::output(options).unwrap();

        assert_eq!(token, get_token()?);
        clean()?;
        Ok(())
    }

    #[test]
    fn test_credential_dir_does_not_exist() -> Result<(), io::Error> {
        setup()?;

        let token = "SOME_TOKEN".to_string();
        let options = Some(token.clone());
        LoginCommand::output(options).unwrap();

        assert_eq!(token, get_token()?);
        clean()?;
        Ok(())
    }

    #[test]
    fn test_credential_file_does_not_exist() -> Result<(), io::Error> {
        setup()?;
        create_dir(LEO_CREDENTIALS_DIR)?;

        let token = "SOME_TOKEN".to_string();
        let options = Some(token.clone());
        LoginCommand::output(options).unwrap();

        assert_eq!(token, get_token()?);
        clean()?;
        Ok(())
    }
}
