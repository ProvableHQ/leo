
program test.aleo {
    // Chained array concatenation: a + b + c
    fn foo(a: [u32; 2], b: [u32; 2], c: [u32; 2]) -> [u32; 6] {
        return a + b + c;
    }
}
