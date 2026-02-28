
program test.aleo {
    // Slice from index to end
    fn foo(a: [u32; 8]) -> [u32; 5] {
        return a[3..];
    }
}
