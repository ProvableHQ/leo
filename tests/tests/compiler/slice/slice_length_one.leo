
program test.aleo {
    // Minimum non-empty slice (length 1)
    fn foo(a: [u32; 8]) -> [u32; 1] {
        return a[3..4];
    }
}
