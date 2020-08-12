use crate::{DPCRecordValues, RecordVerificationError};
use leo_typed::Record as TypedRecord;

use snarkos_dpc::base_dpc::instantiated::RecordCommitment;
use snarkos_models::{algorithms::CommitmentScheme, curves::Fp256};
use snarkos_utilities::{bytes::ToBytes, to_bytes, FromBytes};
use std::convert::TryFrom;

pub fn verify_record_commitment(
    typed_record: TypedRecord,
    record_commitment: RecordCommitment,
) -> Result<bool, RecordVerificationError> {
    // generate a dpc record from the typed record
    let record = DPCRecordValues::try_from(typed_record)?;

    let record_commitment_input = to_bytes![
        record.owner,
        record.is_dummy,
        record.value,
        record.payload,
        record.birth_program_id,
        record.death_program_id,
        record.serial_number_nonce
    ]?;

    let commitment = Fp256::read(&record.commitment[..])?;
    let commitment_randomness = Fp256::read(&record.commitment_randomness[..])?;

    let record_commitment =
        RecordCommitment::commit(&record_commitment, &record_commitment_input, &commitment_randomness)?;

    let result = record_commitment == commitment;

    Ok(result)
}
