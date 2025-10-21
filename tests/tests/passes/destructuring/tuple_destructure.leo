program test.aleo {
    transition main(x: u32, y: u32) -> u32 {
        let (a, b): (u32, u32) = (x, y);
        return a + b;
    }
}
