/*
namespace: Compile
expectation: Pass
*/

program test.aleo {
    // Test all Keccak and SHA3 raw hash variants
    transition main(input: u64) -> (
        field, scalar, address, u128,  // Keccak256 raw
        field, group, i64,              // Keccak384 raw
        field, scalar, u32,             // Keccak512 raw
        field, group, address,          // SHA3-256 raw
        field, scalar, i16              // SHA3-384 & SHA3-512 raw
    ) {
        let array_u64: [u64; 2] = [100u64, 200u64];

        // Keccak256 raw hashes
        let keccak256_field_raw: field = Keccak256::hash_to_field_raw(input);
        let keccak256_scalar_raw: scalar = Keccak256::hash_to_scalar_raw(input);
        let keccak256_addr_raw: address = Keccak256::hash_to_address_raw(array_u64);
        let keccak256_u128_raw: u128 = Keccak256::hash_to_u128_raw(array_u64);

        // Keccak384 raw hashes
        let keccak384_field_raw: field = Keccak384::hash_to_field_raw(input);
        let keccak384_group_raw: group = Keccak384::hash_to_group_raw(input);
        let keccak384_i64_raw: i64 = Keccak384::hash_to_i64_raw(array_u64);

        // Keccak512 raw hashes
        let keccak512_field_raw: field = Keccak512::hash_to_field_raw(input);
        let keccak512_scalar_raw: scalar = Keccak512::hash_to_scalar_raw(input);
        let keccak512_u32_raw: u32 = Keccak512::hash_to_u32_raw(array_u64);

        // SHA3-256 raw hashes
        let sha3_256_field_raw: field = SHA3_256::hash_to_field_raw(input);
        let sha3_256_group_raw: group = SHA3_256::hash_to_group_raw(input);
        let sha3_256_addr_raw: address = SHA3_256::hash_to_address_raw(array_u64);

        // SHA3-384 & SHA3-512 raw hashes
        let sha3_384_field_raw: field = SHA3_384::hash_to_field_raw(input);
        let sha3_512_scalar_raw: scalar = SHA3_512::hash_to_scalar_raw(input);
        let sha3_512_i16_raw: i16 = SHA3_512::hash_to_i16_raw(array_u64);

        return (
            keccak256_field_raw, keccak256_scalar_raw, keccak256_addr_raw, keccak256_u128_raw,
            keccak384_field_raw, keccak384_group_raw, keccak384_i64_raw,
            keccak512_field_raw, keccak512_scalar_raw, keccak512_u32_raw,
            sha3_256_field_raw, sha3_256_group_raw, sha3_256_addr_raw,
            sha3_384_field_raw, sha3_512_scalar_raw, sha3_512_i16_raw
        );
    }
}
