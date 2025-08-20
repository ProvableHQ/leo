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

#[derive(Clone)]
pub struct SubdagWrapper<N: Network> {
    _phantom: PhantomData<N>,
}

impl<N: Network> FromBytes for SubdagWrapper<N> {
    /// Reads the subdag from the buffer.
    fn read_le<R: Read>(mut reader: R) -> IoResult<Self> {
        // Read the version.
        let version = u8::read_le(&mut reader)?;
        // Ensure the version is valid.
        if version != 1 {
            return Err(error(format!("Invalid subdag version ({version})")));
        }
        // Read the number of rounds.
        let num_rounds = u32::read_le(&mut reader)?;
        // Read the round certificates.
        for _ in 0..num_rounds {
            // Read the round.
            let _ = u64::read_le(&mut reader)?;
            // Read the number of certificates.
            let num_certificates = u16::read_le(&mut reader)?;
            // Ensure the number of certificates is within bounds.
            if num_certificates > N::LATEST_MAX_CERTIFICATES().map_err(error)? {
                return Err(error(format!("Number of certificates ({num_certificates}) exceeds the maximum.",)));
            }
            // Read the certificates.
            for _ in 0..num_certificates {
                // Read the certificate.
                BatchCertificateWrapper::<N>::read_le(&mut reader)?;
            }
        }

        Ok(Self { _phantom: Default::default() })
    }
}

impl<'de, N: Network> Deserialize<'de> for SubdagWrapper<N> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        match deserializer.is_human_readable() {
            true => unimplemented!("Human readable deserialization is not implemented for the subdag wrapper"),
            false => FromBytesDeserializer::<Self>::deserialize_with_size_encoding(deserializer, "subdag"),
        }
    }
}
