
program test.aleo {
    // Invalid range - start > end - should fail
    transition foo(a: [u32; 8]) -> [u32; 2] {
        return a[5..3];
    }
}
