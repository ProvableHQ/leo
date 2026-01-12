
program test.aleo {
    // Slice from start to index
    transition foo(a: [u32; 8]) -> [u32; 4] {
        return a[..4];
    }
}
