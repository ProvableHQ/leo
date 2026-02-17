
program test.aleo {
    // Inclusive range out of bounds - a[0..=8] on [u32; 8] means index 8 which is OOB
    transition foo(a: [u32; 8]) -> [u32; 9] {
        return a[0..=8];
    }
}
