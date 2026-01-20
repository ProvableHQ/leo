// Test: Functions with no arguments should be converted to inline
program test.aleo {
    fn main() -> u32 {
        return get_value() + get_empty([]);
    }
}

// This function has no arguments, so it should become inline
fn get_value() -> u32 {
    return 42u32;
}

// This function only has empty arguments, so it should become inline
fn get_value(a: [u32;0]) -> u32 {
    return 42u32;
}
