/*
namespace: Compile
expectation: Pass
*/

program test.aleo {
    const START: u8 = 0u8;
    const START_2: u8 = 1u8;
    const STOP_2: u8 = 20u8;
    transition foo(a: u8, b: u8, flag: bool) -> u8 {
        const STOP: u8 = 10u8;
        let c:u8 = 1u8;
        for i: u8 in START..STOP {
            c = c + i;
        }
        for i: u8 in 0u8..STOP {
            c = c + i;
        }
        for i: u8 in START..10u8 {
            c = c + i;
        }
        for i: u8 in START_2..STOP_2 {
            c = c + i;
        }
        return c;
    }
}