// Copyright (C) 2019-2025 Provable Inc.
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

use super::*;

#[derive(Clone, Default)]
pub struct BatchHeaderWrapper<N: Network> {
    _phantom: PhantomData<N>,
}

impl<N: Network> FromBytes for BatchHeaderWrapper<N> {
    /// Reads the batch header from the buffer.
    fn read_le<R: Read>(mut reader: R) -> IoResult<Self> {
        // Read the version.
        let version = u8::read_le(&mut reader)?;
        // Ensure the version is valid.
        if version != 1 {
            return Err(error("Invalid batch header version"));
        }

        // Read the batch ID.
        let _ = Field::<N>::read_le(&mut reader)?;
        // Read the author.
        let _ = Address::<N>::read_le(&mut reader)?;
        // Read the round number.
        let _ = u64::read_le(&mut reader)?;
        // Read the timestamp.
        let _ = i64::read_le(&mut reader)?;
        // Read the committee ID.
        let _ = Field::<N>::read_le(&mut reader)?;

        // Read the number of transmission IDs.
        let num_transmission_ids = u32::read_le(&mut reader)?;
        // Read the transmission IDs.
        for _ in 0..num_transmission_ids {
            // Insert the transmission ID.
            TransmissionID::<N>::read_le(&mut reader)?;
        }

        // Read the number of previous certificate IDs.
        let num_previous_certificate_ids = u16::read_le(&mut reader)?;
        // Ensure the number of previous certificate IDs is within bounds.
        if num_previous_certificate_ids > N::LATEST_MAX_CERTIFICATES().map_err(error)? {
            return Err(error(format!(
                "Number of previous certificate IDs ({num_previous_certificate_ids}) exceeds the maximum.",
            )));
        }

        // Read the previous certificate ID bytes.
        let mut previous_certificate_id_bytes =
            vec![0u8; num_previous_certificate_ids as usize * Field::<N>::size_in_bytes()];
        reader.read_exact(&mut previous_certificate_id_bytes)?;

        // Read the signature.
        let _ = Signature::<N>::read_le(&mut reader)?;

        // Return the batch header.
        Ok(Self { _phantom: Default::default() })
    }
}
