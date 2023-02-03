/*
namespace: Compile
expectation: Fail
*/

program test.aleo {    
    // The maximum number of arguments to a function is 8.
    function main(
        a: u8,
        b: u8,
        c: u8,
        d: u8,
        e: u8,
        f: u8,
        g: u8,
        h: u8,
        i: u8,
    ) -> u8 {
        return a + b + c + d + e + f + g + h + i
    }}
