/*
namespace: Compile
expectation: Fail
*/

program test.aleo {
    struct Message {
        data: u32,
    }

    // Test hash functions with various type errors
    transition test_hash_wrong_input_sizes() -> field {
        // Fail: Pedersen64 requires input <= 64 bits, providing u128
        let ped64_fail: field = Pedersen64::hash_to_field_raw(100u128);

        return ped64_fail;
    }

    transition test_hash_byte_alignment() -> field {
        // Fail: Keccak256 raw requires byte-aligned input (multiple of 8 bits)
        // but boolean is 1 bit
        let result: field = Keccak256::hash_to_field_raw(true);

        return result;
    }

    transition test_hash_native_wrong_type() -> [bool; 256] {
        let msg: Message = Message { data: 42u32 };

        // Fail: hash_native doesn't accept struct types
        let result: [bool; 256] = Keccak256::hash_native(msg);

        return result;
    }

    transition test_pedersen128_oversized() -> field {
        // Fail: Pedersen128 requires input < 128 bits
        // but u128 with metadata is > 128 bits
        let large_val: u128 = 12345u128;
        let result: field = Pedersen128::hash_to_field(large_val);

        return result;
    }
}
