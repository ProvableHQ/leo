/*
 namespace: Compile
 expectation: Fail
 */

program test.aleo {
    const START: u32 = 0u32;
    transition foo(a: u32, b: u32, flag: bool) -> u32 {
        const STOP: u32 = 10u32;
        const A: () = ();
        const B: u8 = ((1u8,1u8),1u8);
        return 1u32;
    }
}