/*
 namespace: Compile
 expectation: Pass
 */

program test.aleo {
    const START: u32 = 0u32;
    transition foo(a: u32, b: u32, flag: bool) -> u32 {
        const STOP: u32 = 10u32;
        for i: u32 in START..STOP {
            for j: u32 in START..STOP {
                let (c,d): (u32,u32) = (1u32, 1u32);
            }
        }
        return 1u32;
    }
}