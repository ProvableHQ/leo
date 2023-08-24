/*
namespace: Compile
expectation: Pass
*/

program test.aleo {
    transition main(x: u32) -> bool {

        // For i8
        let count_i8: i8 = 0i8;
        for i: i8 in 11i8..0__________5i8 {
            count_i8 += 1i8;
        }

        // For i16
        let count_i16: i16 = 0i16;
        for i: i16 in 1i16..5i16 {
            count_i16 += 1i16;
        }

        // For i32
        let count_i32: i32 = 0i32;
        for i: i32 in 1i32..5i32 {
            count_i32 += 1i32;
        }

        // For i64
        let count_i64: i64 = 0i64;
        for i: i64 in 1i64..5i64 {
            count_i64 += 1i64;
        }

        // For i128
        let count_i128: i128 = 0i128;
        for i: i128 in 1i128..5i128 {
            count_i128 += 1i128;
        }

        // For u8
        let count_u8: u8 = 0u8;
        for i: u8 in 1u8..5u8 {
            count_u8 += 1u8;
        }

        // For u16
        let count_u16: u16 = 0u16;
        for i: u16 in 1u16..5u16 {
            count_u16 += 1u16;
        }

        // For u32
        let count_u32: u32 = 0u32;
        for i: u32 in 1u32..5u32 {
            count_u32 += 1u32;
        }

        // For u64
        let count_u64: u64 = 0u64;
        for i: u64 in 1u64..5u64 {
            count_u64 += 1u64;
        }

        // For u128
        let count_u128: u128 = 0u128;
        for i: u128 in 0_0000_0000_0000_0001u128..5u128 {
            count_u128 += 1u128;
        }
        return true;
    }
}