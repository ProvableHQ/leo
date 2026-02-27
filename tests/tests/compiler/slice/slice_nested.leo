
program test.aleo {
    // Nested slicing: slice the result of a slice
    fn foo(a: [u32; 8]) -> [u32; 2] {
        return a[1..5][0..2];
    }
}
