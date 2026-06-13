const START: u32 = 0u32;

program test.aleo {
    fn foo(a: u32, b: u32, flag: bool) -> u32 {
        const STOP: u32 = 10u32;
        for _i: u32 in START..STOP {
            for _j: u32 in START..STOP {
                let (_c, _d): (u32, u32) = (1u32, 1u32);
            }
        }
        return 1u32;
    }

    @noupgrade
    constructor() {}
}
