/*
 namespace: Compile
 expectation: Pass
 */

program test.aleo {
    const START: u32 = 0u32;
    transition foo(a: u32, b: u32, flag: bool) -> u32 {
        const STOP: u32 = 10u32;
        let c:u32 = 1u32;
        for i: u32 in START..STOP {
            for j: u32 in START..STOP {
                const BIG_NUMBER:u32 = 1u32;
                if flag {
                    const SUPER_SMALL_NUMBER:u32 = 99u32;
                    c = c + a + b + BIG_NUMBER + SUPER_SMALL_NUMBER;
                } else {
                    const SUPER_SMALL_NUMBER:u32 = 88u32;
                    c = c + a - b + BIG_NUMBER + SUPER_SMALL_NUMBER;
                    for k: u32 in START..STOP {
                        c = c + a - b;
                    }
                }
            }
        }
        return c;
    }
}