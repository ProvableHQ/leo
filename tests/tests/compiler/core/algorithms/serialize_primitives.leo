/*
namespace: Compile
expectation: Pass
*/

program test.aleo {
    // Test Serialize::to_bits on all primitive types
    fn main(
        addr: address,
        b: bool,
        f: field,
        g: group,
        i8_val: i8,
        i16_val: i16,
        i32_val: i32,
        i64_val: i64,
        i128_val: i128,
        u8_val: u8,
        u16_val: u16,
        u32_val: u32,
        u64_val: u64,
        u128_val: u128,
        s: scalar
    ) -> (
        bool, bool, bool, bool,  // Sample bits from address
        bool, bool, bool, bool,  // Sample bits from integers
        bool, bool, bool, bool,  // Sample bits from field/group
        bool, bool, bool, bool   // Sample bits from scalar and bool
    ) {
        // Serialize primitives to bits (non-raw includes metadata)
        let addr_bits: [bool; 279] = Serialize::to_bits(addr);
        let bool_bits: [bool; 27] = Serialize::to_bits(b);
        let field_bits: [bool; 279] = Serialize::to_bits(f);
        let group_bits: [bool; 279] = Serialize::to_bits(g);
        let i8_bits: [bool; 34] = Serialize::to_bits(i8_val);
        let i16_bits: [bool; 42] = Serialize::to_bits(i16_val);
        let i32_bits: [bool; 58] = Serialize::to_bits(i32_val);
        let i64_bits: [bool; 90] = Serialize::to_bits(i64_val);
        let i128_bits: [bool; 154] = Serialize::to_bits(i128_val);
        let u8_bits: [bool; 34] = Serialize::to_bits(u8_val);
        let u16_bits: [bool; 42] = Serialize::to_bits(u16_val);
        let u32_bits: [bool; 58] = Serialize::to_bits(u32_val);
        let u64_bits: [bool; 90] = Serialize::to_bits(u64_val);
        let u128_bits: [bool; 154] = Serialize::to_bits(u128_val);
        let scalar_bits: [bool; 277] = Serialize::to_bits(s);

        // Sample bits from each serialization
        return (
            addr_bits[0u8], addr_bits[100u8], addr_bits[200u8], addr_bits[278u16],
            i8_bits[0u8], i32_bits[16u8], i64_bits[32u8], i128_bits[64u8],
            field_bits[0u8], field_bits[128u8], group_bits[0u8], group_bits[278u16],
            scalar_bits[0u8], u8_bits[0u8], u64_bits[0u8], bool_bits[0u8]
        );
    }
}
