/*
namespace: Compile
expectation: Pass
*/

program test.aleo {    
    transition main(a: i8, b: i8, c: i8) -> bool {
        return a << 2u8 == a << 2u16 && a << 2u32 == c;
    }
}
