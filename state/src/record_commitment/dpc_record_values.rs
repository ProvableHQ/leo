use crate::{utilities::*, DPCRecordValuesError};
use leo_typed::Record as TypedRecord;

use snarkos_dpc::base_dpc::instantiated::Components;
use snarkos_objects::AccountAddress;

use std::{convert::TryFrom, str::FromStr};

static SERIAL_NUMBER_PARAMETER_STRING: &str = "serial_number";
static OWNER_PARAMETER_STRING: &str = "owner";
static IS_DUMMY_PARAMETER_STRING: &str = "is_dummy";
static VALUE_PARAMETER_STRING: &str = "value";
static PAYLOAD_PARAMETER_STRING: &str = "payload";
static BIRTH_PROGRAM_ID_PARAMETER_STRING: &str = "birth_program_id";
static DEATH_PROGRAM_ID_PARAMETER_STRING: &str = "death_program_id";
static SERIAL_NUMBER_NONCE_PARAMETER_STRING: &str = "serial_number_nonce";
static COMMITMENT_PARAMETER_STRING: &str = "commitment";
static COMMITMENT_RANDOMNESS_PARAMETER_STRING: &str = "commitment_randomness";

pub struct DPCRecordValues {
    pub serial_number: Vec<u8>,
    pub owner: AccountAddress<Components>,
    pub is_dummy: bool,
    pub value: u64,
    pub payload: Vec<u8>,
    pub birth_program_id: Vec<u8>,
    pub death_program_id: Vec<u8>,
    pub serial_number_nonce: Vec<u8>,
    pub commitment: Vec<u8>,
    pub commitment_randomness: Vec<u8>,
}

impl TryFrom<&TypedRecord> for DPCRecordValues {
    type Error = DPCRecordValuesError;

    fn try_from(record: &TypedRecord) -> Result<Self, Self::Error> {
        let parameters = record.values();

        // Lookup serial number
        let serial_number_value = find_input(SERIAL_NUMBER_PARAMETER_STRING.to_owned(), &parameters)?;
        let serial_number = input_to_u8_vec(serial_number_value)?;

        // Lookup record owner
        let owner_value = find_input(OWNER_PARAMETER_STRING.to_owned(), &parameters)?;
        let owner = AccountAddress::<Components>::from_str(&format!("{}", owner_value))?;

        // Lookup record is_dummy
        let is_dummy_value = find_input(IS_DUMMY_PARAMETER_STRING.to_owned(), &parameters)?;
        let is_dummy = is_dummy_value.to_string().parse::<bool>()?;

        // Lookup record value
        let value_value = find_input(VALUE_PARAMETER_STRING.to_owned(), &parameters)?;
        let value = input_to_integer_string(value_value)?.parse::<u64>()?;

        // Lookup record payload
        let payload_value = find_input(PAYLOAD_PARAMETER_STRING.to_owned(), &parameters)?;
        let payload = input_to_u8_vec(payload_value)?;

        // Lookup record birth program id
        let birth_program_id_value = find_input(BIRTH_PROGRAM_ID_PARAMETER_STRING.to_owned(), &parameters)?;
        let birth_program_id = input_to_u8_vec(birth_program_id_value)?;

        // Lookup record death program id
        let death_program_id_value = find_input(DEATH_PROGRAM_ID_PARAMETER_STRING.to_owned(), &parameters)?;
        let death_program_id = input_to_u8_vec(death_program_id_value)?;

        // Lookup record serial number nonce
        let serial_number_nonce_value = find_input(SERIAL_NUMBER_NONCE_PARAMETER_STRING.to_owned(), &parameters)?;
        let serial_number_nonce = input_to_u8_vec(serial_number_nonce_value)?;

        // Lookup record commitment
        let commitment_value = find_input(COMMITMENT_PARAMETER_STRING.to_owned(), &parameters)?;
        let commitment = input_to_u8_vec(commitment_value)?;

        // Lookup record commitment randomness
        let commitment_randomness_value = find_input(COMMITMENT_RANDOMNESS_PARAMETER_STRING.to_owned(), &parameters)?;
        let commitment_randomness = input_to_u8_vec(commitment_randomness_value)?;

        Ok(Self {
            serial_number,
            owner,
            is_dummy,
            value,
            payload,
            birth_program_id,
            death_program_id,
            serial_number_nonce,
            commitment,
            commitment_randomness,
        })
    }
}
