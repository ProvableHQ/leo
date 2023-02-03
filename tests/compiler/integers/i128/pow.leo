/*
namespace: Compile
expectation: Pass
*/

program test.aleo {    
    transition main(a: i128, b: i128, c: i128) -> bool {
        return a ** 2u8 == a ** 2u16 && a ** 2u32 == c;
    }
}
