// Copyright (C) 2019-2021 Aleo Systems Inc.
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

use crate::DPCRecordValues;
use leo_ast::Record as AstRecord;
use leo_errors::{Result, SnarkVMError, StateError};

use snarkvm_algorithms::traits::CommitmentScheme;
use snarkvm_dpc::{
    testnet1::{instantiated::Components, parameters::SystemParameters},
    DPCComponents,
};
use snarkvm_utilities::{bytes::ToBytes, to_bytes_le, FromBytes};

use std::convert::TryFrom;

/// Returns a serialized [`DPCRecordValues`] type if the record commitment is valid given the
/// system parameters.
pub fn verify_record_commitment(dpc: &SystemParameters<Components>, ast_record: &AstRecord) -> Result<DPCRecordValues> {
    // generate a dpc record from the typed record
    let record: DPCRecordValues = DPCRecordValues::try_from(ast_record)?;

    // verify record commitment
    let record_commitment_input = to_bytes_le![
        record.owner,
        record.is_dummy,
        record.value,
        record.payload,
        record.birth_program_id,
        record.death_program_id,
        record.serial_number_nonce
    ]
    .map_err(StateError::state_io_error)?;

    let commitment =
        <<Components as DPCComponents>::RecordCommitment as CommitmentScheme>::Output::read_le(&record.commitment[..])
            .map_err(StateError::state_io_error)?;
    let commitment_randomness =
        <<Components as DPCComponents>::RecordCommitment as CommitmentScheme>::Randomness::read_le(
            &record.commitment_randomness[..],
        )
        .map_err(StateError::state_io_error)?;

    let record_commitment = <Components as DPCComponents>::RecordCommitment::commit(
        &dpc.record_commitment,
        &record_commitment_input,
        &commitment_randomness,
    )
    .map_err(|_| SnarkVMError::default())?;

    if record_commitment == commitment {
        Ok(record)
    } else {
        Err(SnarkVMError::default().into())
    }
}
