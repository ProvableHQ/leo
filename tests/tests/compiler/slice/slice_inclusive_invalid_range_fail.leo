
program test.aleo {
    // Inclusive range with start > end - should fail
    transition foo(a: [u32; 8]) -> [u32; 2] {
        return a[5..=2];
    }
}
