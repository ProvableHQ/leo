/*
namespace: Compile
expectation: Pass
*/

program test.aleo {
    // Test Serialize::to_bits_raw on primitives (no struct test - raw only works on literals/arrays)
    fn main(
        f: field,
        g: group,
        u64_val: u64,
        i32_val: i32,
        addr: address
    ) -> (
        bool, bool, bool, bool,  // Sample bits from field (253 bits)
        bool, bool, bool, bool,  // Sample bits from group (253 bits)
        bool, bool, bool, bool,  // Sample bits from address and integers
        bool, bool, bool, bool   // Comparison tests
    ) {
        // Serialize with to_bits_raw (raw: field/group/address = 253 bits)
        let field_bits_raw: [bool; 253] = Serialize::to_bits_raw(f);
        let group_bits_raw: [bool; 253] = Serialize::to_bits_raw(g);
        let addr_bits_raw: [bool; 253] = Serialize::to_bits_raw(addr);
        let u64_bits_raw: [bool; 64] = Serialize::to_bits_raw(u64_val);
        let i32_bits_raw: [bool; 32] = Serialize::to_bits_raw(i32_val);

        // Compare raw vs non-raw (non-raw has metadata)
        let field_bits: [bool; 279] = Serialize::to_bits(f);
        let group_bits: [bool; 279] = Serialize::to_bits(g);

        // Sample bits
        return (
            field_bits_raw[0u8], field_bits_raw[100u8], field_bits_raw[200u8], field_bits_raw[252u8],
            group_bits_raw[0u8], group_bits_raw[100u8], group_bits_raw[200u8], group_bits_raw[252u8],
            addr_bits_raw[0u8], addr_bits_raw[252u8], u64_bits_raw[0u8], i32_bits_raw[0u8],
            u64_bits_raw[32u8], i32_bits_raw[16u8],
            field_bits[0u8] != field_bits_raw[0u8],
            group_bits[0u8] != group_bits_raw[0u8]
        );
    }
}
