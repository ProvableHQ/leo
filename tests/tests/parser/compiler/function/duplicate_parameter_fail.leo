/*
namespace: Compile
expectation: Fail
*/

program test.aleo {    
    function main(a: u32, a: u32) -> u32 {
        console.log("{}", 1u8);
        return a;
    }}
