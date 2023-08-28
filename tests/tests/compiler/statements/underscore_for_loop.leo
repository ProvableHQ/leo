/*
namespace: Compile
expectation: Pass
*/

program test.aleo {
    transition main(x: u32) -> bool {

        // For i8
        let count_i8: i8 = 0__0i8 + 1_9i8;
        for i: i8 in 0______1i8..0__________5i8 {
            count_i8 += 1i8;
        }

        // For i16
        let count_i16: i16 = 0__0i16 + 0__1_9i16;
        for i: i16 in 1i16..5i16 {
            count_i16 += 1i16;
        }

        // For i32
        let count_i32: i32 = 0__0i32 + 0__0__1_9i32;
        for i: i32 in 000___0_1i32..0_05______1i32 {
            count_i32 += 1i32;
        }

        // For i64
        let count_i64: i64 = 0__0i64 * 000_0__1_9i64;
        for i: i64 in 000__1___1i64..1___000_5i64 {
            count_i64 += 1i64;
        }

        // For i128
        let count_i128: i128 = 000_0_000_0_0__0i128 + 000_0_1_9i128;
        for i: i128 in 000___0001__0________1i128..000_500______i128 {
            count_i128 += 1i128;
        }

        // For u8
        let count_u8: u8 = 000_0_0__0u8 + 000_0_1_9u8;
        for i: u8 in 000___1u8..00_5u8 {
            count_u8 += 1u8;
        }

        // For u16
        let count_u16: u16 = 0__000_0_0u16 + 000_0_1_9u16;
        for i: u16 in 01u16..000______1______5u16 {
            count_u16 += 1u16;
        }

        // For u32
        let count_u32: u32 = 0000_0_000_0___0u32 + 000_0_1_9u32;
        for i: u32 in 000___11u32..00000______1_5u32 {
            count_u32 += 1u32;
        }

        // For u64
        let count_u64: u64 = 0_000_0__0u64 + 000_0_000_0_000_0_000_0_1_9u64;
        for i: u64 in 0___01u64..0____1____5u64 {
            count_u64 += 1u64;
        }

        // For u128
        let count_u128: u128 = 0__000_0_000_0_0u128 + 000_0_000_0_1_9u128;
        for i: u128 in 0_0000_0000_0000_0001u128..0000_____00000___1_____________5u128 {
            count_u128 += 1u128;
        }
        return true;
    }
}