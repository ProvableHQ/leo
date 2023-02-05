/*
namespace: Compile
expectation: Fail
*/

program test.aleo {    
    function main(x: u32) -> u32 {
        return x;
        let double: u32 = x + x;
        return double;
    }
}
