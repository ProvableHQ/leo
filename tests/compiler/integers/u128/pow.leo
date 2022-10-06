/*
namespace: Compile
expectation: Pass

# The exponent must be u8, u16, or u32
*/

program test.aleo {    
    transition main(a: u128, b: u128, c: u128) -> bool {
        return a ** 2u8 == a ** 2u16 && a ** 2u32 == c;
    }
}
