use crate::errors::AddressError;
use leo_types::Span;

use snarkos_dpc::base_dpc::instantiated::Components;
use snarkos_objects::account::AccountPublicKey;
use std::str::FromStr;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Address(pub AccountPublicKey<Components>);

impl Address {
    pub(crate) fn new(address: String, span: Span) -> Result<Self, AddressError> {
        let address = AccountPublicKey::from_str(&address).map_err(|error| AddressError::account_error(error, span))?;

        Ok(Address(address))
    }
}

impl std::fmt::Display for Address {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
