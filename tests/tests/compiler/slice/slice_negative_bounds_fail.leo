program test.aleo {
    // Slice bounds must be unsigned integers; signed integers are rejected.
    fn foo(a: [u32; 8]) -> [u32; 3] {
        return a[-1i32..3i32];
    }
}
