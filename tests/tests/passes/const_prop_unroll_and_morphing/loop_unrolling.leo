program test.aleo {
    transition main() -> u32 {
        let a: u32 = 5u32 + 10u32;
        let b: u32 = a * 2u32;
        return b;
    }
}

