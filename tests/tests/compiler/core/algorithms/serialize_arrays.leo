/*
namespace: Compile
expectation: Pass
*/

program test.aleo {
    // Test serialization of arrays (non-raw and raw)
    transition main() -> (
        bool, bool, bool, bool,  // Sample bits from array serialization
        bool, bool, bool, bool,  // More array samples
        bool, bool, bool, bool,  // Raw serialization samples
        bool, bool, bool, bool   // Raw vs non-raw comparison
    ) {
        // 1D arrays (keep small to avoid 512-bit limit)
        let array_u8: [u8; 4] = [1u8, 2u8, 3u8, 4u8];
        let array_u32: [u32; 2] = [100u32, 200u32];

        // 2D arrays
        let array_2d: [[u8; 2]; 2] = [[1u8, 2u8], [3u8, 4u8]];

        // Non-raw serialization (includes metadata)
        // Array serialization has overhead: [u8; 4] = 234 bits, [u32; 2] = 182 bits
        let array_u8_bits: [bool; 234] = Serialize::to_bits(array_u8);
        let array_u32_bits: [bool; 182] = Serialize::to_bits(array_u32);

        // Raw serialization (no metadata, just concatenated elements)
        let array_u8_bits_raw: [bool; 32] = Serialize::to_bits_raw(array_u8);  // 4 * 8 bits
        let array_u32_bits_raw: [bool; 64] = Serialize::to_bits_raw(array_u32);  // 2 * 32 bits

        // 2D array raw serialization
        let array_2d_bits_raw: [bool; 32] = Serialize::to_bits_raw(array_2d);  // 4 * 8 bits

        // Sample bits from serializations
        return (
            array_u8_bits[0u8], array_u8_bits[50u8], array_u8_bits[100u8], array_u8_bits[200u8],
            array_u32_bits[0u8], array_u32_bits[50u8], array_u32_bits[100u8], array_u32_bits[150u8],
            array_u8_bits_raw[0u8], array_u8_bits_raw[8u8], array_u8_bits_raw[16u8], array_u8_bits_raw[24u8],
            array_2d_bits_raw[0u8], array_2d_bits_raw[16u8],
            array_u8_bits[0u8] != array_u8_bits_raw[0u8],
            array_u32_bits[0u8] != array_u32_bits_raw[0u8]
        );
    }
}
