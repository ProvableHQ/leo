/*
namespace: Compile
expectation: Fail
*/

program test.aleo {    
    transition main() -> bool {
        let c1 : u32 = 123i16 * 123i32;
        let c2 : u32 = "123i32" * 123i16 * "sss";
        let c3 : u32 = "123i32" * "sss";
        let c4 : u32 = 1i8 * 2i16 * 3i32 * 4i64 * 5u8 * 6u16 * 7u32 * 9u64;
        let c16: bool = ("123i32" & 123i16) == ("sss" / 1i8 - 1i8 + 22u32);
        return false;
    }
}
