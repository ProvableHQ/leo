
program test.aleo {
    // Slice from start to index
    fn foo(a: [u32; 8]) -> [u32; 4] {
        return a[..4];
    }
}
