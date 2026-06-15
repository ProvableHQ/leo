@no_inline
fn many_args(
    a1: u32,
    _a2: u32,
    _a3: u32,
    _a4: u32,
    _a5: u32,
    _a6: u32,
    _a7: u32,
    _a8: u32,
    _a9: u32,
    _a10: u32,
    _a11: u32,
    _a12: u32,
    _a13: u32,
    _a14: u32,
    _a15: u32,
    _a16: u32,
    a17: u32,
) -> u32 {
    return a1 + a17;
}

program test.aleo {
    fn main(x: u32) -> u32 {
        return many_args(x, x, x, x, x, x, x, x, x, x, x, x, x, x, x, x, x);
    }

    @noupgrade
    constructor() {}
}
