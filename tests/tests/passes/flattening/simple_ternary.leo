program test.aleo {
    transition main(a: u32, b: u32, condition: bool) -> u32 {
        return condition ? a : b;
    }
}
