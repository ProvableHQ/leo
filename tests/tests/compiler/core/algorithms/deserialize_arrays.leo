/*
namespace: Compile
expectation: Pass
*/

program test.aleo {
    // Test deserialization of arrays (non-raw and raw)
    transition main() -> (
        u8, u8, u8, u8,      // Deserialized [u8; 4] values
        u32, u32,            // Deserialized [u32; 2] values
        u8, u8, u8, u8,      // Raw deserialized [u8; 4] values
        bool, bool, bool, bool  // Roundtrip success flags
    ) {
        // Create arrays to serialize then deserialize (roundtrip test)
        let array_u8_orig: [u8; 4] = [10u8, 20u8, 30u8, 40u8];
        let array_u32_orig: [u32; 2] = [1000u32, 2000u32];

        // Non-raw roundtrip (with metadata)
        // Array serialization sizes: [u8; 4] = 234 bits, [u32; 2] = 182 bits
        let array_u8_bits: [bool; 234] = Serialize::to_bits(array_u8_orig);
        let array_u8_result: [u8; 4] = Deserialize::from_bits::[[u8; 4]](array_u8_bits);

        let array_u32_bits: [bool; 182] = Serialize::to_bits(array_u32_orig);
        let array_u32_result: [u32; 2] = Deserialize::from_bits::[[u32; 2]](array_u32_bits);

        // Raw roundtrip (no metadata)
        let array_u8_bits_raw: [bool; 32] = Serialize::to_bits_raw(array_u8_orig);  // 4 * 8 bits
        let array_u8_result_raw: [u8; 4] = Deserialize::from_bits_raw::[[u8; 4]](array_u8_bits_raw);

        return (
            array_u8_result[0u8], array_u8_result[1u8], array_u8_result[2u8], array_u8_result[3u8],
            array_u32_result[0u8], array_u32_result[1u8],
            array_u8_result_raw[0u8], array_u8_result_raw[1u8], array_u8_result_raw[2u8], array_u8_result_raw[3u8],
            array_u8_result[0u8] == array_u8_orig[0u8],
            array_u32_result[0u8] == array_u32_orig[0u8],
            array_u8_result_raw[0u8] == array_u8_orig[0u8],
            array_u8_result_raw[3u8] == array_u8_orig[3u8]
        );
    }
}
