
program test.aleo {
    // Slice of a nested array
    fn foo(a: [[u32; 4]; 4]) -> [[u32; 4]; 2] {
        return a[1..3];
    }
}
