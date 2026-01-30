// Test: Functions with const parameters should be converted to inline
program test.aleo {
    fn main(x: u32) -> u32 {
        return double::[2u32](x);
    }
}

// This function has const parameters, so it should become inline
fn double::[N: u32](x: u32) -> u32 {
    return x * N;
}
