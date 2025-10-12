program test.aleo {
    transition main(x: u32) -> u32 {
        let a: u32 = 10u32;
        let b: u32 = a + 5u32;
        return x + b;
    }
}

