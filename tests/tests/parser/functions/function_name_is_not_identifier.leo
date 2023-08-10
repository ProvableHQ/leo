/*
namespace: Parse
expectation: Fail
*/

program test.aleo {
    function x(x: u32, constant y: i32) -> u8 {
        return 100u8(0u8);
    }
}
