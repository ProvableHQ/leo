
program test.aleo {
    // Non-const variable slice bounds - should fail (bounds must be compile-time constants)
    transition foo(a: [u32; 8], start: u32, end: u32) -> [u32; 3] {
        return a[start..end];
    }
}
