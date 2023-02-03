/*
namespace: Compile
expectation: Pass
*/

program test.aleo {    
    transition main(flag: u8, value: u8) -> u8 {
        if (flag == 0u8) {
            value += 1u8;
            return value;
        } else {
            value += 2u8;
        }
        return value;
    }}
