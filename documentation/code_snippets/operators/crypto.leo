program crypto_ops_demo.aleo {

    fn bhp_hash_demos() {
        {
            // ANCHOR: bhp256_hash
            let result: address = BHP256::hash_to_address(1u8);
            let result_2: field = BHP256::hash_to_field(2i64);
            let result_3: group = BHP256::hash_to_group(1field);
            let result_4: scalar = BHP256::hash_to_scalar(1field);
            let result_5: i8 = BHP256::hash_to_i8(1field);
            let result_6: i16 = BHP256::hash_to_i16(1field);
            let result_7: i32 = BHP256::hash_to_i32(1field);
            let result_8: i64 = BHP256::hash_to_i64(1field);
            let result_9: i128 = BHP256::hash_to_i128(1field);
            let result_10: u8 = BHP256::hash_to_u8(1field);
            let result_11: u16 = BHP256::hash_to_u16(1field);
            let result_12: u32 = BHP256::hash_to_u32(1field);
            let result_13: u64 = BHP256::hash_to_u64(1field);
            let result_14: u128 = BHP256::hash_to_u128(1field);

            // Raw hash variants
            let result_15: address = BHP256::hash_to_address_raw(1u8);
            let result_16: field = BHP256::hash_to_field_raw(2i64);
            // ANCHOR_END: bhp256_hash
        }
        {
            // ANCHOR: bhp512_hash
            let result: address = BHP512::hash_to_address(1u8);
            let result_2: field = BHP512::hash_to_field(2i64);
            let result_3: group = BHP512::hash_to_group(1field);
            let result_4: scalar = BHP512::hash_to_scalar(1field);
            let result_5: i8 = BHP512::hash_to_i8(1field);
            let result_6: i16 = BHP512::hash_to_i16(1field);
            let result_7: i32 = BHP512::hash_to_i32(1field);
            let result_8: i64 = BHP512::hash_to_i64(1field);
            let result_9: i128 = BHP512::hash_to_i128(1field);
            let result_10: u8 = BHP512::hash_to_u8(1field);
            let result_11: u16 = BHP512::hash_to_u16(1field);
            let result_12: u32 = BHP512::hash_to_u32(1field);
            let result_13: u64 = BHP512::hash_to_u64(1field);
            let result_14: u128 = BHP512::hash_to_u128(1field);

            // Raw hash variants
            let result_15: address = BHP512::hash_to_address_raw(1u8);
            let result_16: field = BHP512::hash_to_field_raw(2i64);
            // ANCHOR_END: bhp512_hash
        }
        {
            // ANCHOR: bhp768_hash
            let result: address = BHP768::hash_to_address(1u8);
            let result_2: field = BHP768::hash_to_field(2i64);
            let result_3: group = BHP768::hash_to_group(1field);
            let result_4: scalar = BHP768::hash_to_scalar(1field);
            let result_5: i8 = BHP768::hash_to_i8(1field);
            let result_6: i16 = BHP768::hash_to_i16(1field);
            let result_7: i32 = BHP768::hash_to_i32(1field);
            let result_8: i64 = BHP768::hash_to_i64(1field);
            let result_9: i128 = BHP768::hash_to_i128(1field);
            let result_10: u8 = BHP768::hash_to_u8(1field);
            let result_11: u16 = BHP768::hash_to_u16(1field);
            let result_12: u32 = BHP768::hash_to_u32(1field);
            let result_13: u64 = BHP768::hash_to_u64(1field);
            let result_14: u128 = BHP768::hash_to_u128(1field);

            // Raw hash variants
            let result_15: address = BHP768::hash_to_address_raw(1u8);
            let result_16: field = BHP768::hash_to_field_raw(2i64);
            // ANCHOR_END: bhp768_hash
        }
        {
            // ANCHOR: bhp1024_hash
            let result: address = BHP1024::hash_to_address(1u8);
            let result_2: field = BHP1024::hash_to_field(2i64);
            let result_3: group = BHP1024::hash_to_group(1field);
            let result_4: scalar = BHP1024::hash_to_scalar(1field);
            let result_5: i8 = BHP1024::hash_to_i8(1field);
            let result_6: i16 = BHP1024::hash_to_i16(1field);
            let result_7: i32 = BHP1024::hash_to_i32(1field);
            let result_8: i64 = BHP1024::hash_to_i64(1field);
            let result_9: i128 = BHP1024::hash_to_i128(1field);
            let result_10: u8 = BHP1024::hash_to_u8(1field);
            let result_11: u16 = BHP1024::hash_to_u16(1field);
            let result_12: u32 = BHP1024::hash_to_u32(1field);
            let result_13: u64 = BHP1024::hash_to_u64(1field);
            let result_14: u128 = BHP1024::hash_to_u128(1field);

            // Raw hash variants
            let result_15: address = BHP1024::hash_to_address_raw(1u8);
            let result_16: field = BHP1024::hash_to_field_raw(2i64);
            // ANCHOR_END: bhp1024_hash
        }
    }

    fn pedersen_hash_demos() {
        {
            // ANCHOR: pedersen64_hash
            let result: address = Pedersen64::hash_to_address(1u8);
            let result_2: field = Pedersen64::hash_to_field(2u32);
            let result_3: group = Pedersen64::hash_to_group(1u32);
            let result_4: scalar = Pedersen64::hash_to_scalar(1u32);
            let result_5: i8 = Pedersen64::hash_to_i8(1u32);
            let result_6: i16 = Pedersen64::hash_to_i16(1u32);
            let result_7: i32 = Pedersen64::hash_to_i32(1u32);
            let result_8: i64 = Pedersen64::hash_to_i64(1u32);
            let result_9: i128 = Pedersen64::hash_to_i128(1u32);
            let result_10: u8 = Pedersen64::hash_to_u8(1u32);
            let result_11: u16 = Pedersen64::hash_to_u16(1u32);
            let result_12: u32 = Pedersen64::hash_to_u32(1u32);
            let result_13: u64 = Pedersen64::hash_to_u64(1u32);
            let result_14: u128 = Pedersen64::hash_to_u128(1u32);

            // Raw hash variants
            let result_15: address = Pedersen64::hash_to_address_raw(1u8);
            let result_16: field = Pedersen64::hash_to_field_raw(2u32);
            // ANCHOR_END: pedersen64_hash
        }
        {
            // ANCHOR: pedersen128_hash
            let result: address = Pedersen128::hash_to_address(1u8);
            let result_2: field = Pedersen128::hash_to_field(2i64);
            let result_3: group = Pedersen128::hash_to_group(1u64);
            let result_4: scalar = Pedersen128::hash_to_scalar(1u64);
            let result_5: i8 = Pedersen128::hash_to_i8(1u64);
            let result_6: i16 = Pedersen128::hash_to_i16(1u64);
            let result_7: i32 = Pedersen128::hash_to_i32(1u64);
            let result_8: i64 = Pedersen128::hash_to_i64(1u64);
            let result_9: i128 = Pedersen128::hash_to_i128(1u64);
            let result_10: u8 = Pedersen128::hash_to_u8(1u64);
            let result_11: u16 = Pedersen128::hash_to_u16(1u64);
            let result_12: u32 = Pedersen128::hash_to_u32(1u64);
            let result_13: u64 = Pedersen128::hash_to_u64(1u64);
            let result_14: u128 = Pedersen128::hash_to_u128(1u64);

            // Raw hash variants
            let result_15: address = Pedersen128::hash_to_address_raw(1u8);
            let result_16: field = Pedersen128::hash_to_field_raw(2i64);
            // ANCHOR_END: pedersen128_hash
        }
    }

    fn poseidon_hash_demos() {
        {
            // ANCHOR: poseidon2_hash
            let result: address = Poseidon2::hash_to_address(1u8);
            let result_2: field = Poseidon2::hash_to_field(2i64);
            let result_3: group = Poseidon2::hash_to_group(1field);
            let result_4: scalar = Poseidon2::hash_to_scalar(1field);
            let result_5: i8 = Poseidon2::hash_to_i8(1field);
            let result_6: i16 = Poseidon2::hash_to_i16(1field);
            let result_7: i32 = Poseidon2::hash_to_i32(1field);
            let result_8: i64 = Poseidon2::hash_to_i64(1field);
            let result_9: i128 = Poseidon2::hash_to_i128(1field);
            let result_10: u8 = Poseidon2::hash_to_u8(1field);
            let result_11: u16 = Poseidon2::hash_to_u16(1field);
            let result_12: u32 = Poseidon2::hash_to_u32(1field);
            let result_13: u64 = Poseidon2::hash_to_u64(1field);
            let result_14: u128 = Poseidon2::hash_to_u128(1field);

            // Raw hash variants
            let result_15: address = Poseidon2::hash_to_address_raw(1u8);
            let result_16: field = Poseidon2::hash_to_field_raw(2i64);
            // ANCHOR_END: poseidon2_hash
        }
        {
            // ANCHOR: poseidon4_hash
            let result: address = Poseidon4::hash_to_address(1u8);
            let result_2: field = Poseidon4::hash_to_field(2i64);
            let result_3: group = Poseidon4::hash_to_group(1field);
            let result_4: scalar = Poseidon4::hash_to_scalar(1field);
            let result_5: i8 = Poseidon4::hash_to_i8(1field);
            let result_6: i16 = Poseidon4::hash_to_i16(1field);
            let result_7: i32 = Poseidon4::hash_to_i32(1field);
            let result_8: i64 = Poseidon4::hash_to_i64(1field);
            let result_9: i128 = Poseidon4::hash_to_i128(1field);
            let result_10: u8 = Poseidon4::hash_to_u8(1field);
            let result_11: u16 = Poseidon4::hash_to_u16(1field);
            let result_12: u32 = Poseidon4::hash_to_u32(1field);
            let result_13: u64 = Poseidon4::hash_to_u64(1field);
            let result_14: u128 = Poseidon4::hash_to_u128(1field);

            // Raw hash variants
            let result_15: address = Poseidon4::hash_to_address_raw(1u8);
            let result_16: field = Poseidon4::hash_to_field_raw(2i64);
            // ANCHOR_END: poseidon4_hash
        }
        {
            // ANCHOR: poseidon8_hash
            let result: address = Poseidon8::hash_to_address(1u8);
            let result_2: field = Poseidon8::hash_to_field(2i64);
            let result_3: group = Poseidon8::hash_to_group(1field);
            let result_4: scalar = Poseidon8::hash_to_scalar(1field);
            let result_5: i8 = Poseidon8::hash_to_i8(1field);
            let result_6: i16 = Poseidon8::hash_to_i16(1field);
            let result_7: i32 = Poseidon8::hash_to_i32(1field);
            let result_8: i64 = Poseidon8::hash_to_i64(1field);
            let result_9: i128 = Poseidon8::hash_to_i128(1field);
            let result_10: u8 = Poseidon8::hash_to_u8(1field);
            let result_11: u16 = Poseidon8::hash_to_u16(1field);
            let result_12: u32 = Poseidon8::hash_to_u32(1field);
            let result_13: u64 = Poseidon8::hash_to_u64(1field);
            let result_14: u128 = Poseidon8::hash_to_u128(1field);

            // Raw hash variants
            let result_15: address = Poseidon8::hash_to_address_raw(1u8);
            let result_16: field = Poseidon8::hash_to_field_raw(2i64);
            // ANCHOR_END: poseidon8_hash
        }
    }

    fn keccak_hash_demos() {
        {
            // ANCHOR: keccak256_hash_to_bits
            let result: [bool; 256] = Keccak256::hash_to_bits(1u8);
            let result_2: [bool; 256] = Keccak256::hash_to_bits(2i64);
            let result_3: [bool; 256] = Keccak256::hash_to_bits(1u16);
            let result_4: [bool; 256] = Keccak256::hash_to_bits(1u32);
            let result_5: [bool; 256] = Keccak256::hash_to_bits(1u64);
            let result_6: [bool; 256] = Keccak256::hash_to_bits(1u128);
            let result_7: [bool; 256] = Keccak256::hash_to_bits(1i8);
            let result_8: [bool; 256] = Keccak256::hash_to_bits(1i16);
            let result_9: [bool; 256] = Keccak256::hash_to_bits(1i32);
            let result_10: [bool; 256] = Keccak256::hash_to_bits(1i64);
            let result_11: [bool; 256] = Keccak256::hash_to_bits(1i128);

            // Raw hash variants
            let result_12: [bool; 256] = Keccak256::hash_to_bits_raw(1u8);
            let result_13: [bool; 256] = Keccak256::hash_to_bits_raw(2i64);
            // ANCHOR_END: keccak256_hash_to_bits
        }
        {
            // ANCHOR: keccak256_hash
            let result: address = Keccak256::hash_to_address(1u8);
            let result_2: field = Keccak256::hash_to_field(2i64);
            let result_3: group = Keccak256::hash_to_group(1field);
            let result_4: scalar = Keccak256::hash_to_scalar(1field);
            let result_5: i8 = Keccak256::hash_to_i8(1field);
            let result_6: i16 = Keccak256::hash_to_i16(1field);
            let result_7: i32 = Keccak256::hash_to_i32(1field);
            let result_8: i64 = Keccak256::hash_to_i64(1field);
            let result_9: i128 = Keccak256::hash_to_i128(1field);
            let result_10: u8 = Keccak256::hash_to_u8(1field);
            let result_11: u16 = Keccak256::hash_to_u16(1field);
            let result_12: u32 = Keccak256::hash_to_u32(1field);
            let result_13: u64 = Keccak256::hash_to_u64(1field);
            let result_14: u128 = Keccak256::hash_to_u128(1field);

            // Raw hash variants
            let result_15: address = Keccak256::hash_to_address_raw(1u8);
            let result_16: field = Keccak256::hash_to_field_raw(2i64);
            // ANCHOR_END: keccak256_hash
        }
        {
            // ANCHOR: keccak384_hash_to_bits
            let result: [bool; 384] = Keccak384::hash_to_bits(1u8);
            let result_2: [bool; 384] = Keccak384::hash_to_bits(2i64);
            let result_3: [bool; 384] = Keccak384::hash_to_bits(1u16);
            let result_4: [bool; 384] = Keccak384::hash_to_bits(1u32);
            let result_5: [bool; 384] = Keccak384::hash_to_bits(1u64);
            let result_6: [bool; 384] = Keccak384::hash_to_bits(1u128);
            let result_7: [bool; 384] = Keccak384::hash_to_bits(1i8);
            let result_8: [bool; 384] = Keccak384::hash_to_bits(1i16);
            let result_9: [bool; 384] = Keccak384::hash_to_bits(1i32);
            let result_10: [bool; 384] = Keccak384::hash_to_bits(1i64);
            let result_11: [bool; 384] = Keccak384::hash_to_bits(1i128);

            // Raw hash variants
            let result_12: [bool; 384] = Keccak384::hash_to_bits_raw(1u8);
            let result_13: [bool; 384] = Keccak384::hash_to_bits_raw(2i64);
            // ANCHOR_END: keccak384_hash_to_bits
        }
        {
            // ANCHOR: keccak384_hash
            let result: address = Keccak384::hash_to_address(1u8);
            let result_2: field = Keccak384::hash_to_field(2i64);
            let result_3: group = Keccak384::hash_to_group(1field);
            let result_4: scalar = Keccak384::hash_to_scalar(1field);
            let result_5: i8 = Keccak384::hash_to_i8(1field);
            let result_6: i16 = Keccak384::hash_to_i16(1field);
            let result_7: i32 = Keccak384::hash_to_i32(1field);
            let result_8: i64 = Keccak384::hash_to_i64(1field);
            let result_9: i128 = Keccak384::hash_to_i128(1field);
            let result_10: u8 = Keccak384::hash_to_u8(1field);
            let result_11: u16 = Keccak384::hash_to_u16(1field);
            let result_12: u32 = Keccak384::hash_to_u32(1field);
            let result_13: u64 = Keccak384::hash_to_u64(1field);
            let result_14: u128 = Keccak384::hash_to_u128(1field);

            // Raw hash variants
            let result_15: address = Keccak384::hash_to_address_raw(1u8);
            let result_16: field = Keccak384::hash_to_field_raw(2i64);
            // ANCHOR_END: keccak384_hash
        }
        {
            // ANCHOR: keccak512_hash_to_bits
            let result: [bool; 512] = Keccak512::hash_to_bits(1u8);
            let result_2: [bool; 512] = Keccak512::hash_to_bits(2i64);
            let result_3: [bool; 512] = Keccak512::hash_to_bits(1u16);
            let result_4: [bool; 512] = Keccak512::hash_to_bits(1u32);
            let result_5: [bool; 512] = Keccak512::hash_to_bits(1u64);
            let result_6: [bool; 512] = Keccak512::hash_to_bits(1u128);
            let result_7: [bool; 512] = Keccak512::hash_to_bits(1i8);
            let result_8: [bool; 512] = Keccak512::hash_to_bits(1i16);
            let result_9: [bool; 512] = Keccak512::hash_to_bits(1i32);
            let result_10: [bool; 512] = Keccak512::hash_to_bits(1i64);
            let result_11: [bool; 512] = Keccak512::hash_to_bits(1i128);

            // Raw hash variants
            let result_12: [bool; 512] = Keccak512::hash_to_bits_raw(1u8);
            let result_13: [bool; 512] = Keccak512::hash_to_bits_raw(2i64);
            // ANCHOR_END: keccak512_hash_to_bits
        }
        {
            // ANCHOR: keccak512_hash
            let result: address = Keccak512::hash_to_address(1u8);
            let result_2: field = Keccak512::hash_to_field(2i64);
            let result_3: group = Keccak512::hash_to_group(1field);
            let result_4: scalar = Keccak512::hash_to_scalar(1field);
            let result_5: i8 = Keccak512::hash_to_i8(1field);
            let result_6: i16 = Keccak512::hash_to_i16(1field);
            let result_7: i32 = Keccak512::hash_to_i32(1field);
            let result_8: i64 = Keccak512::hash_to_i64(1field);
            let result_9: i128 = Keccak512::hash_to_i128(1field);
            let result_10: u8 = Keccak512::hash_to_u8(1field);
            let result_11: u16 = Keccak512::hash_to_u16(1field);
            let result_12: u32 = Keccak512::hash_to_u32(1field);
            let result_13: u64 = Keccak512::hash_to_u64(1field);
            let result_14: u128 = Keccak512::hash_to_u128(1field);

            // Raw hash variants
            let result_15: address = Keccak512::hash_to_address_raw(1u8);
            let result_16: field = Keccak512::hash_to_field_raw(2i64);
            // ANCHOR_END: keccak512_hash
        }
    }

    fn sha3_hash_demos() {
        {
            // ANCHOR: sha3_256_hash_to_bits
            let result: [bool; 256] = SHA3_256::hash_to_bits(1u8);
            let result_2: [bool; 256] = SHA3_256::hash_to_bits(2i64);
            let result_3: [bool; 256] = SHA3_256::hash_to_bits(1u16);
            let result_4: [bool; 256] = SHA3_256::hash_to_bits(1u32);
            let result_5: [bool; 256] = SHA3_256::hash_to_bits(1u64);
            let result_6: [bool; 256] = SHA3_256::hash_to_bits(1u128);
            let result_7: [bool; 256] = SHA3_256::hash_to_bits(1i8);
            let result_8: [bool; 256] = SHA3_256::hash_to_bits(1i16);
            let result_9: [bool; 256] = SHA3_256::hash_to_bits(1i32);
            let result_10: [bool; 256] = SHA3_256::hash_to_bits(1i64);
            let result_11: [bool; 256] = SHA3_256::hash_to_bits(1i128);

            // Raw hash variants
            let result_12: [bool; 256] = SHA3_256::hash_to_bits_raw(1u8);
            let result_13: [bool; 256] = SHA3_256::hash_to_bits_raw(2i64);
            // ANCHOR_END: sha3_256_hash_to_bits
        }
        {
            // ANCHOR: sha3_256_hash
            let result: address = SHA3_256::hash_to_address(1u8);
            let result_2: field = SHA3_256::hash_to_field(2i64);
            let result_3: group = SHA3_256::hash_to_group(1field);
            let result_4: scalar = SHA3_256::hash_to_scalar(1field);
            let result_5: i8 = SHA3_256::hash_to_i8(1field);
            let result_6: i16 = SHA3_256::hash_to_i16(1field);
            let result_7: i32 = SHA3_256::hash_to_i32(1field);
            let result_8: i64 = SHA3_256::hash_to_i64(1field);
            let result_9: i128 = SHA3_256::hash_to_i128(1field);
            let result_10: u8 = SHA3_256::hash_to_u8(1field);
            let result_11: u16 = SHA3_256::hash_to_u16(1field);
            let result_12: u32 = SHA3_256::hash_to_u32(1field);
            let result_13: u64 = SHA3_256::hash_to_u64(1field);
            let result_14: u128 = SHA3_256::hash_to_u128(1field);

            // Raw hash variants
            let result_15: address = SHA3_256::hash_to_address_raw(1u8);
            let result_16: field = SHA3_256::hash_to_field_raw(2i64);
            // ANCHOR_END: sha3_256_hash
        }
        {
            // ANCHOR: sha3_384_hash_to_bits
            let result: [bool; 384] = SHA3_384::hash_to_bits(1u8);
            let result_2: [bool; 384] = SHA3_384::hash_to_bits(2i64);
            let result_3: [bool; 384] = SHA3_384::hash_to_bits(1u16);
            let result_4: [bool; 384] = SHA3_384::hash_to_bits(1u32);
            let result_5: [bool; 384] = SHA3_384::hash_to_bits(1u64);
            let result_6: [bool; 384] = SHA3_384::hash_to_bits(1u128);
            let result_7: [bool; 384] = SHA3_384::hash_to_bits(1i8);
            let result_8: [bool; 384] = SHA3_384::hash_to_bits(1i16);
            let result_9: [bool; 384] = SHA3_384::hash_to_bits(1i32);
            let result_10: [bool; 384] = SHA3_384::hash_to_bits(1i64);
            let result_11: [bool; 384] = SHA3_384::hash_to_bits(1i128);

            // Raw hash variants
            let result_12: [bool; 384] = SHA3_384::hash_to_bits_raw(1u8);
            let result_13: [bool; 384] = SHA3_384::hash_to_bits_raw(2i64);
            // ANCHOR_END: sha3_384_hash_to_bits
        }
        {
            // ANCHOR: sha3_384_hash
            let result: address = SHA3_384::hash_to_address(1u8);
            let result_2: field = SHA3_384::hash_to_field(2i64);
            let result_3: group = SHA3_384::hash_to_group(1field);
            let result_4: scalar = SHA3_384::hash_to_scalar(1field);
            let result_5: i8 = SHA3_384::hash_to_i8(1field);
            let result_6: i16 = SHA3_384::hash_to_i16(1field);
            let result_7: i32 = SHA3_384::hash_to_i32(1field);
            let result_8: i64 = SHA3_384::hash_to_i64(1field);
            let result_9: i128 = SHA3_384::hash_to_i128(1field);
            let result_10: u8 = SHA3_384::hash_to_u8(1field);
            let result_11: u16 = SHA3_384::hash_to_u16(1field);
            let result_12: u32 = SHA3_384::hash_to_u32(1field);
            let result_13: u64 = SHA3_384::hash_to_u64(1field);
            let result_14: u128 = SHA3_384::hash_to_u128(1field);

            // Raw hash variants
            let result_15: address = SHA3_384::hash_to_address_raw(1u8);
            let result_16: field = SHA3_384::hash_to_field_raw(2i64);
            // ANCHOR_END: sha3_384_hash
        }
        {
            // ANCHOR: sha3_512_hash_to_bits
            let result: [bool; 512] = SHA3_512::hash_to_bits(1u8);
            let result_2: [bool; 512] = SHA3_512::hash_to_bits(2i64);
            let result_3: [bool; 512] = SHA3_512::hash_to_bits(1u16);
            let result_4: [bool; 512] = SHA3_512::hash_to_bits(1u32);
            let result_5: [bool; 512] = SHA3_512::hash_to_bits(1u64);
            let result_6: [bool; 512] = SHA3_512::hash_to_bits(1u128);
            let result_7: [bool; 512] = SHA3_512::hash_to_bits(1i8);
            let result_8: [bool; 512] = SHA3_512::hash_to_bits(1i16);
            let result_9: [bool; 512] = SHA3_512::hash_to_bits(1i32);
            let result_10: [bool; 512] = SHA3_512::hash_to_bits(1i64);
            let result_11: [bool; 512] = SHA3_512::hash_to_bits(1i128);

            // Raw hash variants
            let result_12: [bool; 512] = SHA3_512::hash_to_bits_raw(1u8);
            let result_13: [bool; 512] = SHA3_512::hash_to_bits_raw(2i64);
            // ANCHOR_END: sha3_512_hash_to_bits
        }
        {
            // ANCHOR: sha3_512_hash
            let result: address = SHA3_512::hash_to_address(1u8);
            let result_2: field = SHA3_512::hash_to_field(2i64);
            let result_3: group = SHA3_512::hash_to_group(1field);
            let result_4: scalar = SHA3_512::hash_to_scalar(1field);
            let result_5: i8 = SHA3_512::hash_to_i8(1field);
            let result_6: i16 = SHA3_512::hash_to_i16(1field);
            let result_7: i32 = SHA3_512::hash_to_i32(1field);
            let result_8: i64 = SHA3_512::hash_to_i64(1field);
            let result_9: i128 = SHA3_512::hash_to_i128(1field);
            let result_10: u8 = SHA3_512::hash_to_u8(1field);
            let result_11: u16 = SHA3_512::hash_to_u16(1field);
            let result_12: u32 = SHA3_512::hash_to_u32(1field);
            let result_13: u64 = SHA3_512::hash_to_u64(1field);
            let result_14: u128 = SHA3_512::hash_to_u128(1field);

            // Raw hash variants
            let result_15: address = SHA3_512::hash_to_address_raw(1u8);
            let result_16: field = SHA3_512::hash_to_field_raw(2i64);
            // ANCHOR_END: sha3_512_hash
        }
    }

    fn bhp_commit_demos() -> Final {
        return final {
            {
                // ANCHOR: bhp256_commit
                let salt: scalar = ChaCha::rand_scalar();
                let a: address = BHP256::commit_to_address(1u8, salt);
                let b: field = BHP256::commit_to_field(2i64, salt);
                let c: group = BHP256::commit_to_group(1field, salt);
                // ANCHOR_END: bhp256_commit
            }
            {
                // ANCHOR: bhp512_commit
                let salt: scalar = ChaCha::rand_scalar();
                let a: address = BHP512::commit_to_address(1u8, salt);
                let b: field = BHP512::commit_to_field(2i64, salt);
                let c: group = BHP512::commit_to_group(1field, salt);
                // ANCHOR_END: bhp512_commit
            }
            {
                // ANCHOR: bhp768_commit
                let salt: scalar = ChaCha::rand_scalar();
                let a: address = BHP768::commit_to_address(1u8, salt);
                let b: field = BHP768::commit_to_field(2i64, salt);
                let c: group = BHP768::commit_to_group(1field, salt);
                // ANCHOR_END: bhp768_commit
            }
            {
                // ANCHOR: bhp1024_commit
                let salt: scalar = ChaCha::rand_scalar();
                let a: address = BHP1024::commit_to_address(1u8, salt);
                let b: field = BHP1024::commit_to_field(2i64, salt);
                let c: group = BHP1024::commit_to_group(1field, salt);
                // ANCHOR_END: bhp1024_commit
            }
        };
    }

    fn pedersen_commit_demos() -> Final {
        return final {
            {
                // ANCHOR: pedersen64_commit
                let salt: scalar = ChaCha::rand_scalar();
                let a: address = Pedersen64::commit_to_address(1u8, salt);
                let b: field = Pedersen64::commit_to_field(2u32, salt);
                let c: group = Pedersen64::commit_to_group(1u32, salt);
                // ANCHOR_END: pedersen64_commit
            }
            {
                // ANCHOR: pedersen128_commit
                let salt: scalar = ChaCha::rand_scalar();
                let a: address = Pedersen128::commit_to_address(1u8, salt);
                let b: field = Pedersen128::commit_to_field(2i64, salt);
                let c: group = Pedersen128::commit_to_group(1u64, salt);
                // ANCHOR_END: pedersen128_commit
            }
        };
    }

    fn chacha_demo() -> Final {
        return final {
            // ANCHOR: chacha_rand
            let result: address = ChaCha::rand_address();
            let result_2: bool = ChaCha::rand_bool();
            let result_3: field = ChaCha::rand_field();
            let result_4: group = ChaCha::rand_group();
            let result_5: i8 = ChaCha::rand_i8();
            let result_6: i16 = ChaCha::rand_i16();
            let result_7: i32 = ChaCha::rand_i32();
            let result_8: i64 = ChaCha::rand_i64();
            let result_9: i128 = ChaCha::rand_i128();
            let result_10: u8 = ChaCha::rand_u8();
            let result_11: u16 = ChaCha::rand_u16();
            let result_12: u32 = ChaCha::rand_u32();
            let result_13: u64 = ChaCha::rand_u64();
            let result_14: u128 = ChaCha::rand_u128();
            let result_15: scalar = ChaCha::rand_scalar();
            // ANCHOR_END: chacha_rand
        };
    }

    // ANCHOR: signature_verify
    fn verify_field(s: signature, a: address, v: field) {
        let first: bool = signature::verify(s, a, v);
        let second: bool = s.verify(a, v);
        assert_eq(first, second);
    }
    // ANCHOR_END: signature_verify

    fn ecdsa_demos(
        sig: [u8; 65],
        addr: [u8; 33],
        eth_addr: [u8; 20],
        digest: [u8; 32],
        msg: [u8; 32],
    ) -> Final {
        return final {
            {
                // ANCHOR: ecdsa_verify_digest
                let valid: bool = ECDSA::verify_digest(sig, addr, digest);
                let valid_2: bool = ECDSA::verify_digest_eth(sig, eth_addr, digest);
                // ANCHOR_END: ecdsa_verify_digest
            }
            {
                // ANCHOR: ecdsa_verify_keccak256
                let valid: bool = ECDSA::verify_keccak256(sig, addr, msg);
                let valid_2: bool = ECDSA::verify_keccak256_raw(sig, addr, msg);
                let valid_3: bool = ECDSA::verify_keccak256_eth(sig, eth_addr, msg);
                // ANCHOR_END: ecdsa_verify_keccak256
            }
            {
                // ANCHOR: ecdsa_verify_keccak384
                let valid: bool = ECDSA::verify_keccak384(sig, addr, msg);
                let valid_2: bool = ECDSA::verify_keccak384_raw(sig, addr, msg);
                let valid_3: bool = ECDSA::verify_keccak384_eth(sig, eth_addr, msg);
                // ANCHOR_END: ecdsa_verify_keccak384
            }
            {
                // ANCHOR: ecdsa_verify_keccak512
                let valid: bool = ECDSA::verify_keccak512(sig, addr, msg);
                let valid_2: bool = ECDSA::verify_keccak512_raw(sig, addr, msg);
                let valid_3: bool = ECDSA::verify_keccak512_eth(sig, eth_addr, msg);
                // ANCHOR_END: ecdsa_verify_keccak512
            }
            {
                // ANCHOR: ecdsa_verify_sha3_256
                let valid: bool = ECDSA::verify_sha3_256(sig, addr, msg);
                let valid_2: bool = ECDSA::verify_sha3_256_raw(sig, addr, msg);
                let valid_3: bool = ECDSA::verify_sha3_256_eth(sig, eth_addr, msg);
                // ANCHOR_END: ecdsa_verify_sha3_256
            }
            {
                // ANCHOR: ecdsa_verify_sha3_384
                let valid: bool = ECDSA::verify_sha3_384(sig, addr, msg);
                let valid_2: bool = ECDSA::verify_sha3_384_raw(sig, addr, msg);
                let valid_3: bool = ECDSA::verify_sha3_384_eth(sig, eth_addr, msg);
                // ANCHOR_END: ecdsa_verify_sha3_384
            }
            {
                // ANCHOR: ecdsa_verify_sha3_512
                let valid: bool = ECDSA::verify_sha3_512(sig, addr, msg);
                let valid_2: bool = ECDSA::verify_sha3_512_raw(sig, addr, msg);
                let valid_3: bool = ECDSA::verify_sha3_512_eth(sig, eth_addr, msg);
                // ANCHOR_END: ecdsa_verify_sha3_512
            }
        };
    }

    fn snark_demo(
        vk: [u8; 32],
        version: u8,
        inputs: [field; 4],
        proof: [u8; 32],
        vks: [[u8; 32]; 2],
        batch_inputs: [[[field; 4]; 2]; 2],
    ) -> Final {
        return final {
            // ANCHOR: snark_verify
            let valid: bool = Snark::verify(vk, version, inputs, proof);
            // ANCHOR_END: snark_verify

            // ANCHOR: snark_verify_batch
            let valid_2: bool = Snark::verify_batch(vks, version, batch_inputs, proof);
            // ANCHOR_END: snark_verify_batch
        };
    }

    @noupgrade
    constructor() {}
}
