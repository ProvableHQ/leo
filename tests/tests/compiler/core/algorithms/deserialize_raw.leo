/*
namespace: Compile
expectation: Pass
*/

program test.aleo {
    // Test Deserialize::from_bits_raw for primitive types (raw = no metadata)
    transition main() -> (
        u32, u64, i32, i64,
        field, group, scalar, address,
        bool, bool, bool, bool, bool, bool, bool, bool
    ) {
        // Create bit arrays (raw sizes: field/group/address=253, scalar=251, integers=their width)
        let u32_bits: [bool; 32] = [true; 32];
        let u64_bits: [bool; 64] = [false; 64];
        let i32_bits: [bool; 32] = [true; 32];
        let i64_bits: [bool; 64] = [false; 64];
        let field_bits: [bool; 253] = [true; 253];
        let group_bits: [bool; 253] = [false; 253];
        let scalar_bits: [bool; 251] = [true; 251];
        let addr_bits: [bool; 253] = [false; 253];

        // Deserialize using from_bits_raw
        let u32_raw: u32 = Deserialize::from_bits_raw::[u32](u32_bits);
        let u64_raw: u64 = Deserialize::from_bits_raw::[u64](u64_bits);
        let i32_raw: i32 = Deserialize::from_bits_raw::[i32](i32_bits);
        let i64_raw: i64 = Deserialize::from_bits_raw::[i64](i64_bits);
        let field_raw: field = Deserialize::from_bits_raw::[field](field_bits);
        let group_raw: group = Deserialize::from_bits_raw::[group](group_bits);
        let scalar_raw: scalar = Deserialize::from_bits_raw::[scalar](scalar_bits);
        let addr_raw: address = Deserialize::from_bits_raw::[address](addr_bits);

        // Compare with non-raw versions (need different sized arrays for non-raw)
        let u32_bits_normal: [bool; 58] = [true; 58];
        let u64_bits_normal: [bool; 90] = [false; 90];
        let i32_bits_normal: [bool; 58] = [true; 58];
        let i64_bits_normal: [bool; 90] = [false; 90];
        let field_bits_normal: [bool; 279] = [true; 279];
        let group_bits_normal: [bool; 279] = [false; 279];
        let scalar_bits_normal: [bool; 277] = [true; 277];
        let addr_bits_normal: [bool; 279] = [false; 279];

        let u32_normal: u32 = Deserialize::from_bits::[u32](u32_bits_normal);
        let u64_normal: u64 = Deserialize::from_bits::[u64](u64_bits_normal);
        let i32_normal: i32 = Deserialize::from_bits::[i32](i32_bits_normal);
        let i64_normal: i64 = Deserialize::from_bits::[i64](i64_bits_normal);
        let field_normal: field = Deserialize::from_bits::[field](field_bits_normal);
        let group_normal: group = Deserialize::from_bits::[group](group_bits_normal);
        let scalar_normal: scalar = Deserialize::from_bits::[scalar](scalar_bits_normal);
        let addr_normal: address = Deserialize::from_bits::[address](addr_bits_normal);

        return (
            u32_raw, u64_raw, i32_raw, i64_raw,
            field_raw, group_raw, scalar_raw, addr_raw,
            u32_raw != u32_normal,
            u64_raw != u64_normal,
            i32_raw != i32_normal,
            i64_raw != i64_normal,
            field_raw != field_normal,
            group_raw != group_normal,
            scalar_raw != scalar_normal,
            addr_raw != addr_normal
        );
    }
}
