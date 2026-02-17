
program test.aleo {
    // Slice with non-u32 element type
    transition foo(a: [bool; 4]) -> [bool; 2] {
        return a[1..3];
    }
}
