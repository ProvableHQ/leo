use crate::InputValueError;
use leo_typed::InputValue;

pub fn input_to_integer_string(input: InputValue) -> Result<String, InputValueError> {
    match input {
        InputValue::Integer(_type, string) => Ok(string),
        value => Err(InputValueError::ExpectedInteger(value.to_string())),
    }
}

pub fn input_to_u8_vec(input: InputValue) -> Result<Vec<u8>, InputValueError> {
    let input_array = match input {
        InputValue::Array(values) => values,
        value => return Err(InputValueError::ExpectedBytes(value.to_string())),
    };

    let mut result_vec = vec![];

    for input in input_array {
        let integer_string = input_to_integer_string(input)?;
        let byte = integer_string.parse::<u8>()?;

        result_vec.push(byte);
    }

    Ok(result_vec)
}
