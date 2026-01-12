
program test.aleo {
    // This should fail: element types don't match
    transition foo(a: [u32; 2], b: [u8; 2]) -> [u32; 4] {
        return a + b;
    }
}
