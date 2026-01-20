// Test: Functions that don't meet any condition should remain as fn (not converted to inline)
program test.aleo {
    fn main(x: u32) -> u32 {
        return helper(x);
    }
}

fn helper(x: u32) -> u32 {
    return x * 2u32;
}
