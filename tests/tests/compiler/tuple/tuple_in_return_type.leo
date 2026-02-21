
program test.aleo {
    fn bar(a: u8) -> (u8, (u8, u8)) {
        return (a, (a + a, a * a));
    }
}

