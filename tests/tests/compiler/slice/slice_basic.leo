
program test.aleo {
    // Basic slice with both bounds
    fn foo(a: [u32; 8]) -> [u32; 3] {
        return a[2..5];
    }
}
