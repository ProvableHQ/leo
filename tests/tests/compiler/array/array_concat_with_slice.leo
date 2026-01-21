
program test.aleo {
    // Concatenate sliced arrays
    transition foo(a: [u32; 4], b: [u32; 4]) -> [u32; 4] {
        return a[0..2] + b[2..4];
    }
}
