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

#[derive(Clone, PartialEq, Eq)]
pub enum AuthorityWrapper<N: Network> {
    Beacon(Signature<N>),
    Quorum,
}

impl<N: Network> FromBytes for AuthorityWrapper<N> {
    /// Reads the authority from the buffer.
    fn read_le<R: Read>(mut reader: R) -> IoResult<Self> {
        // Read the variant.
        let variant = u8::read_le(&mut reader)?;
        // Match the variant.
        match variant {
            0 => Ok(Self::Beacon(FromBytes::read_le(&mut reader)?)),
            1 => {
                SubdagWrapper::<N>::read_le(&mut reader)?;
                Ok(Self::Quorum)
            }
            2.. => Err(error("Invalid authority variant")),
        }
    }
}

impl<'de, N: Network> Deserialize<'de> for AuthorityWrapper<N> {
    #[inline]
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        match deserializer.is_human_readable() {
            true => unimplemented!("Human readable deserialization is not implemented for the authority wrapper"),
            false => FromBytesDeserializer::<Self>::deserialize_with_size_encoding(deserializer, "authority"),
        }
    }
}
