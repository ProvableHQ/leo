// Copyright (C) 2019-2023 Aleo Systems Inc.
// This file is part of the Leo library.

// The Leo library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The Leo library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the Leo library. If not, see <https://www.gnu.org/licenses/>.

use leo_errors::{LeoError, Result, UtilError};
use leo_package::package::Package;

// A valid hash is 61 characters long, with preface "ab1" and all characters lowercase or numbers.
pub fn is_valid_hash(hash: &str) -> Result<(), LeoError> {
    if hash.len() != 61 {
        Err(UtilError::invalid_input_id_len(hash, "hash").into())
    } else if !hash.starts_with("ab1") && hash.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit()) {
        Err(UtilError::invalid_input_id(hash, "hash", "ab1").into())
    } else {
        Ok(())
    }
}

// A valid transaction id is 61 characters long, with preface "at1" and all characters lowercase or numbers.
pub fn is_valid_transaction_id(transaction: &str) -> Result<(), LeoError> {
    if transaction.len() != 61 {
        Err(UtilError::invalid_input_id_len(transaction, "transaction").into())
    } else if !transaction.starts_with("at1")
        && transaction.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit())
    {
        Err(UtilError::invalid_input_id(transaction, "transaction", "at1").into())
    } else {
        Ok(())
    }
}

// A valid transition id is 61 characters long, with preface "au1" and all characters lowercase or numbers.
pub fn is_valid_transition_id(transition: &str) -> Result<(), LeoError> {
    if transition.len() != 61 {
        Err(UtilError::invalid_input_id_len(transition, "transition").into())
    } else if !transition.starts_with("au1") && transition.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit())
    {
        Err(UtilError::invalid_input_id(transition, "transition", "au1").into())
    } else {
        Ok(())
    }
}

// A valid numerical input is a u32.
pub fn is_valid_numerical_input(num: &str) -> Result<(), LeoError> {
    if num.parse::<u32>().is_err() { Err(UtilError::invalid_numerical_input(num).into()) } else { Ok(()) }
}

// A valid height or hash.
pub fn is_valid_height_or_hash(input: &str) -> Result<(), LeoError> {
    match (is_valid_hash(input), is_valid_numerical_input(input)) {
        (Ok(_), _) | (_, Ok(_)) => Ok(()),
        _ => Err(UtilError::invalid_height_or_hash(input).into()),
    }
}

// Checks if the string is a valid field, allowing for optional `field` suffix.
pub fn is_valid_field(field: &str) -> Result<String, LeoError> {
    let split = field.split("field").collect::<Vec<&str>>();

    if split.len() == 1 && split[0].chars().all(|c| c.is_numeric()) {
        Ok(format!("{}field", field))
    } else if split.len() == 2 && split[0].chars().all(|c| c.is_numeric()) && split[1].is_empty() {
        Ok(field.to_string())
    } else {
        Err(UtilError::invalid_field(field).into())
    }
}

// Checks if the string is a valid program name in Aleo.
pub fn check_valid_program_name(name: String) -> String {
    if name.ends_with(".aleo") {
        Package::is_aleo_name_valid(&name[0..name.len() - 5]);
        name
    } else {
        Package::is_aleo_name_valid(&name);
        format!("{}.aleo", name)
    }
}
