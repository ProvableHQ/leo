fn f(x: u32, y: u32) -> u32 {
    return x + (17u32 * y);
}

program test.aleo {
    fn main() -> u32 {
        return f(12u32, 100u32);
    }
}
