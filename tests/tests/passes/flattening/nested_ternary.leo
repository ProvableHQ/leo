program test.aleo {
    transition main(x: u32, c1: bool, c2: bool) -> u32 {
        return c1 ? (c2 ? x : x + 1u32) : x + 2u32;
    }
}
