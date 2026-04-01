@no_inline
fn many_args(
    a1: u32,
    a2: u32,
    a3: u32,
    a4: u32,
    a5: u32,
    a6: u32,
    a7: u32,
    a8: u32,
    a9: u32,
    a10: u32,
    a11: u32,
    a12: u32,
    a13: u32,
    a14: u32,
    a15: u32,
    a16: u32,
    a17: u32,
) -> u32 {
    return a1 + a17;
}

program test.aleo {
    fn main(x: u32) -> u32 {
        return many_args(x, x, x, x, x, x, x, x, x, x, x, x, x, x, x, x, x);
    }
}
