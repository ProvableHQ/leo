
program test.aleo {
    // Full slice (copy entire array)
    transition foo(a: [u32; 4]) -> [u32; 4] {
        return a[..];
    }
}
