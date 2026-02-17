
program test.aleo {
    // Empty slice (start == end) - should fail
    transition foo(a: [u32; 8]) -> [u32; 0] {
        return a[5..5];
    }
}
