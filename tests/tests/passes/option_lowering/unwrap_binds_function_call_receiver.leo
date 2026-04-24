// Module-level inline; allowed to return `u32?`.
fn maybe(x: u32) -> u32? {
    return x;
}

program test.aleo {
    // `.unwrap_or(...)` on a function-call receiver must bind the call to a
    // temporary before synthesizing `.is_some` and `.val` accesses — otherwise
    // the call subtree gets duplicated and will be emitted twice downstream.
    fn main(x: u32, d: u32) -> u32 {
        return maybe(x).unwrap_or(d);
    }
}
