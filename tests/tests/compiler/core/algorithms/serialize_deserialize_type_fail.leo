/*
namespace: Compile
expectation: Fail
*/

struct Message {
    data: u32,
}

program test.aleo {
    // Test serialize/deserialize with various type errors
    fn test_deserialize_wrong_sizes() -> (u32, field, scalar, address, [u8; 4]) {
        // Fail: u32 raw requires 32 bits, providing 64
        let u32_wrong: [bool; 64] = [true; 64];
        let u32_result: u32 = Deserialize::from_bits_raw::[u32](u32_wrong);

        // Fail: field raw requires 253 bits, providing 256
        let field_wrong: [bool; 256] = [true; 256];
        let f: field = Deserialize::from_bits_raw::[field](field_wrong);

        // Fail: scalar raw requires 251 bits, providing 253
        let scalar_wrong: [bool; 253] = [false; 253];
        let s: scalar = Deserialize::from_bits_raw::[scalar](scalar_wrong);

        // Fail: address raw requires 253 bits, providing 279 (non-raw size)
        let addr_wrong: [bool; 279] = [true; 279];
        let a: address = Deserialize::from_bits_raw::[address](addr_wrong);

        // Fail: [u8; 4] raw requires 32 bits (4 * 8), providing 64
        let array_wrong: [bool; 64] = [true; 64];
        let arr: [u8; 4] = Deserialize::from_bits_raw::[[u8; 4]](array_wrong);

        return (u32_result, f, s, a, arr);
    }

    // Test non-raw deserialize with raw-sized arrays
    fn test_nonraw_with_raw_size() -> u32 {
        // Fail: u32 non-raw requires 58 bits (with metadata), providing only 32 (raw size)
        let wrong_bits: [bool; 32] = [true; 32];
        let result: u32 = Deserialize::from_bits::[u32](wrong_bits);

        return result;
    }
}
