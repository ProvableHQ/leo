/*
namespace: Compile
expectation: Pass
*/

struct Data {
    x: u32,
    y: u32,
}

program test.aleo {
    // Test all BHP raw hash variants
    fn main(input: u64) -> (
        address, field, group, i8, u32, scalar,  // BHP256 raw
        field, group, scalar,                     // BHP512 raw
        field, u64,                               // BHP768 raw
        field, i128,                              // BHP1024 raw
        bool, bool, bool                          // Comparison flags
    ) {
        let data: Data = Data { x: 10u32, y: 20u32 };

        // BHP256 raw hashes - test all output types
        let bhp256_addr_raw: address = BHP256::hash_to_address_raw(input);
        let bhp256_field_raw: field = BHP256::hash_to_field_raw(input);
        let bhp256_group_raw: group = BHP256::hash_to_group_raw(input);
        let bhp256_i8_raw: i8 = BHP256::hash_to_i8_raw(input);
        let bhp256_u32_raw: u32 = BHP256::hash_to_u32_raw(input);
        let bhp256_scalar_raw: scalar = BHP256::hash_to_scalar_raw(input);

        // BHP512 raw hashes
        let bhp512_field_raw: field = BHP512::hash_to_field_raw(input);
        let bhp512_group_raw: group = BHP512::hash_to_group_raw(input);
        let bhp512_scalar_raw: scalar = BHP512::hash_to_scalar_raw(input);

        // BHP768 raw hashes with struct
        let bhp768_field_raw: field = BHP768::hash_to_field_raw(data);
        let bhp768_u64_raw: u64 = BHP768::hash_to_u64_raw(data);

        // BHP1024 raw hashes with struct
        let bhp1024_field_raw: field = BHP1024::hash_to_field_raw(data);
        let bhp1024_i128_raw: i128 = BHP1024::hash_to_i128_raw(data);

        // Test that raw variants produce different results than non-raw
        let bhp256_field_normal: field = BHP256::hash_to_field(input);
        let bhp512_field_normal: field = BHP512::hash_to_field(input);
        let bhp768_field_normal: field = BHP768::hash_to_field(data);

        return (
            bhp256_addr_raw, bhp256_field_raw, bhp256_group_raw, bhp256_i8_raw, bhp256_u32_raw, bhp256_scalar_raw,
            bhp512_field_raw, bhp512_group_raw, bhp512_scalar_raw,
            bhp768_field_raw, bhp768_u64_raw,
            bhp1024_field_raw, bhp1024_i128_raw,
            bhp256_field_raw != bhp256_field_normal,
            bhp512_field_raw != bhp512_field_normal,
            bhp768_field_raw != bhp768_field_normal
        );
    }
}
