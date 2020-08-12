use crate::RecordVerificationError;
use leo_typed::{InputValue, Record as TypedRecord};

use snarkos_dpc::base_dpc::instantiated::Components;
use snarkos_objects::AccountAddress;
use std::{convert::TryFrom, str::FromStr};

static SERIAL_NUMBER_PARAMETER_STRING: &str = "serial_number";
static OWNER_PARAMETER_STRING: &str = "owner";
static IS_DUMMY_PARAMETER_STRING: &str = "is_dummy";
static VALUE_PARAMETER_STRING: &str = "value";
static PAYLOAD_PARAMETER_STRING: &str = "parameter";
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
    type Error = RecordVerificationError;

    fn try_from(record: &TypedRecord) -> Result<Self, Self::Error> {
        // Lookup serial number
        let serial_number_value = get_parameter_value(SERIAL_NUMBER_PARAMETER_STRING.to_owned(), record)?;
        let serial_number = input_to_u8_vec(serial_number_value)?;

        // Lookup record owner
        let owner_value = get_parameter_value(OWNER_PARAMETER_STRING.to_owned(), record)?;
        let owner = AccountAddress::<Components>::from_str(&format!("{}", owner_value))?;

        // Lookup record is_dummy
        let is_dummy_value = get_parameter_value(IS_DUMMY_PARAMETER_STRING.to_owned(), record)?;
        let is_dummy = is_dummy_value.to_string().parse::<bool>()?;

        // Lookup record value
        let value_value = get_parameter_value(VALUE_PARAMETER_STRING.to_owned(), record)?;
        let value = input_to_integer_string(value_value)?.parse::<u64>()?;

        // Lookup record payload
        let payload_value = get_parameter_value(PAYLOAD_PARAMETER_STRING.to_owned(), record)?;
        let payload = input_to_u8_vec(payload_value)?;

        // Lookup record birth program id
        let birth_program_id_value = get_parameter_value(BIRTH_PROGRAM_ID_PARAMETER_STRING.to_owned(), record)?;
        let birth_program_id = input_to_u8_vec(birth_program_id_value)?;

        // Lookup record death program id
        let death_program_id_value = get_parameter_value(DEATH_PROGRAM_ID_PARAMETER_STRING.to_owned(), record)?;
        let death_program_id = input_to_u8_vec(death_program_id_value)?;

        // Lookup record serial number nonce
        let serial_number_nonce_value = get_parameter_value(SERIAL_NUMBER_NONCE_PARAMETER_STRING.to_owned(), record)?;
        let serial_number_nonce = input_to_u8_vec(serial_number_nonce_value)?;

        // Lookup record commitment
        let commitment_value = get_parameter_value(COMMITMENT_PARAMETER_STRING.to_owned(), record)?;
        let commitment = input_to_u8_vec(commitment_value)?;

        // Lookup record commitment randomness
        let commitment_randomness_value =
            get_parameter_value(COMMITMENT_RANDOMNESS_PARAMETER_STRING.to_owned(), record)?;
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

fn get_parameter_value(name: String, record: &TypedRecord) -> Result<InputValue, RecordVerificationError> {
    let parameters = record.values();
    let matched_parameter = parameters
        .iter()
        .find(|(parameter, _value)| parameter.variable.name == name);

    match matched_parameter {
        Some((_parameter, value_option)) => match value_option {
            Some(value) => Ok(value.clone()),
            None => Err(RecordVerificationError::MissingParameter(name)),
        },
        None => Err(RecordVerificationError::MissingParameter(name)),
    }
}

fn input_to_integer_string(input: InputValue) -> Result<String, RecordVerificationError> {
    match input {
        InputValue::Integer(_type, string) => Ok(string),
        value => Err(RecordVerificationError::ExpectedInteger(value.to_string())),
    }
}

fn input_to_u8_vec(input: InputValue) -> Result<Vec<u8>, RecordVerificationError> {
    let input_array = match input {
        InputValue::Array(values) => values,
        value => return Err(RecordVerificationError::ExpectedBytes(value.to_string())),
    };

    let mut result_vec = vec![];

    for input in input_array {
        let integer_string = input_to_integer_string(input)?;
        let byte = integer_string.parse::<u8>()?;

        result_vec.push(byte);
    }

    Ok(result_vec)
}
