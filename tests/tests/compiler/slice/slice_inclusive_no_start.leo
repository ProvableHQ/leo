
program test.aleo {
    // Inclusive range with no start bound
    fn foo(a: [u32; 8]) -> [u32; 4] {
        return a[..=3];
    }
}
