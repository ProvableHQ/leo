
program test.aleo {
    // Slice from index to end
    transition foo(a: [u32; 8]) -> [u32; 5] {
        return a[3..];
    }
}
