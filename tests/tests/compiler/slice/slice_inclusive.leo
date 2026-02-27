
program test.aleo {
    // Slice with inclusive end bound
    fn foo(a: [u32; 8]) -> [u32; 4] {
        return a[2..=5];
    }
}
