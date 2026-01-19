/*
namespace: Compile
expectation: Pass
*/

program test.aleo {
    // Test Deserialize::from_bits for primitive types (non-raw includes metadata)
    fn main() -> (
        bool, u8, u16, u32, u64, u128,
        i8, i16, i32, i64, i128,
        field, group, scalar, address
    ) {
        // Create bit arrays for deserialization (non-raw sizes with metadata)
        let bool_bits: [bool; 27] = [true; 27];
        let u8_bits: [bool; 34] = [true; 34];
        let u16_bits: [bool; 42] = [true; 42];
        let u32_bits: [bool; 58] = [true; 58];
        let u64_bits: [bool; 90] = [true; 90];
        let u128_bits: [bool; 154] = [true; 154];
        let i8_bits: [bool; 34] = [false; 34];
        let i16_bits: [bool; 42] = [false; 42];
        let i32_bits: [bool; 58] = [false; 58];
        let i64_bits: [bool; 90] = [false; 90];
        let i128_bits: [bool; 154] = [false; 154];
        let field_bits: [bool; 279] = [true; 279];
        let group_bits: [bool; 279] = [false; 279];
        let scalar_bits: [bool; 277] = [true; 277];
        let addr_bits: [bool; 279] = [false; 279];

        // Deserialize from bits
        let b: bool = Deserialize::from_bits::[bool](bool_bits);
        let u8_val: u8 = Deserialize::from_bits::[u8](u8_bits);
        let u16_val: u16 = Deserialize::from_bits::[u16](u16_bits);
        let u32_val: u32 = Deserialize::from_bits::[u32](u32_bits);
        let u64_val: u64 = Deserialize::from_bits::[u64](u64_bits);
        let u128_val: u128 = Deserialize::from_bits::[u128](u128_bits);
        let i8_val: i8 = Deserialize::from_bits::[i8](i8_bits);
        let i16_val: i16 = Deserialize::from_bits::[i16](i16_bits);
        let i32_val: i32 = Deserialize::from_bits::[i32](i32_bits);
        let i64_val: i64 = Deserialize::from_bits::[i64](i64_bits);
        let i128_val: i128 = Deserialize::from_bits::[i128](i128_bits);
        let f: field = Deserialize::from_bits::[field](field_bits);
        let g: group = Deserialize::from_bits::[group](group_bits);
        let s: scalar = Deserialize::from_bits::[scalar](scalar_bits);
        let addr: address = Deserialize::from_bits::[address](addr_bits);

        return (
            b, u8_val, u16_val, u32_val, u64_val, u128_val,
            i8_val, i16_val, i32_val, i64_val, i128_val,
            f, g, s, addr
        );
    }
}
