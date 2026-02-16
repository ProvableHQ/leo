
program test.aleo {
    // Concatenate a literal array with a variable array
    transition foo(a: [u32; 2]) -> [u32; 4] {
        return [1u32, 2u32] + a;
    }
}
