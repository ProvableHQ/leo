program test.aleo {
    fn main(x: u32, y: u32) -> u32 {
        return foo::[1u32, 2u32]();
    }
}

fn foo::[N: u32, M: u32]() -> u32 {
    N = 5u32;
    M = 6u32;
    return M + N;
}
