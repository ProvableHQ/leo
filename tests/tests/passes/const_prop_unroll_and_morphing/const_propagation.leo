program test.aleo {
    const a: u32 = 10u32;
    transition main(x: u32) -> u32 {
        let b: u32 = a + 5u32;
        return x + b;
    }
}

