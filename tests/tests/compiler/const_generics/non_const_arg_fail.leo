program test.aleo {
    fn main(x: u32) -> u32 {
        return bar::[x]();
    }
}

fn bar::[N: u32]() -> u32 {
    return N;
}
