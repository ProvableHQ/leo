
program test.aleo {
    // Full slice (copy entire array)
    fn foo(a: [u32; 4]) -> [u32; 4] {
        return a[..];
    }
}
