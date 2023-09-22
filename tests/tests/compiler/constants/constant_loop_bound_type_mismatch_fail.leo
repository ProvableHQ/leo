/*
namespace: Compile
expectation: Fail
*/

program test.aleo {
    const START: u32 = 0u32;
    transition foo(a: u8, b: u8, flag: bool) -> u8 {
        const STOP: u8 = 10u8;
        let c:u8 = 1u8;
        for i: u8 in START..STOP {
            c = c + i;
        }
        return c;
    }
}