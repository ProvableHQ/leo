
program test.aleo {
    // Slice of a nested array
    transition foo(a: [[u32; 4]; 4]) -> [[u32; 4]; 2] {
        return a[1..3];
    }
}
