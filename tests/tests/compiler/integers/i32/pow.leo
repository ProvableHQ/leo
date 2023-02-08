/*
namespace: Compile
expectation: Pass
*/

program test.aleo {    
    transition main(a: i32, b: i32, c: i32) -> bool {
        return a ** 2u8 == a ** 2u16 && a ** 2u32 == c;
    }
}
