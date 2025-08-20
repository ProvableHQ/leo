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
use snarkvm::prelude::SizeInBytes;

#[derive(Clone, Default)]
pub struct BatchCertificateWrapper<N: Network> {
    _phantom: PhantomData<N>,
}

impl<N: Network> FromBytes for BatchCertificateWrapper<N> {
    /// Reads the batch certificate from the buffer.
    fn read_le<R: Read>(mut reader: R) -> IoResult<Self> {
        // Read the version.
        let version = u8::read_le(&mut reader)?;
        // Ensure the version is valid.
        if version != 1 {
            return Err(error("Invalid batch certificate version"));
        }

        // Read the batch header.
        let _ = BatchHeaderWrapper::<N>::read_le(&mut reader)?;
        // Read the number of signatures.
        let num_signatures = u16::read_le(&mut reader)?;
        // Read the signature bytes.
        let mut signature_bytes = vec![0u8; num_signatures as usize * Signature::<N>::size_in_bytes()];
        reader.read_exact(&mut signature_bytes)?;
        // Return the batch certificate.
        Ok(Self { _phantom: Default::default() })
    }
}

impl<'de, N: Network> Deserialize<'de> for BatchCertificateWrapper<N> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        match deserializer.is_human_readable() {
            true => {
                unimplemented!("Human readable deserialization is not implemented for the batch certificate wrapper")
            }
            false => FromBytesDeserializer::<Self>::deserialize_with_size_encoding(deserializer, "batch certificate"),
        }
    }
}
