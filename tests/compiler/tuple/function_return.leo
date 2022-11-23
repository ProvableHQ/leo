/*
namespace: Compile
expectation: Pass
input_file:
 - inputs/u8_u8.in
*/

program test.aleo {    
    transition main(a: u8, b: u8) -> (u8, u8) {
        return (a + b, b + a);
    }
}
