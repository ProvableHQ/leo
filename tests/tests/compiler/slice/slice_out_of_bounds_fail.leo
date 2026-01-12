
program test.aleo {
    // Slice out of bounds - should fail
    transition foo(a: [u32; 8]) -> [u32; 5] {
        return a[5..10];
    }
}
