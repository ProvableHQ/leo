
// This should pass and insert
// a dummy instruction into the empty function.
fn x(a: bool) -> u8 {
    return 0u8;
}

program test.aleo {
    fn t(a: bool) -> u8 {
        return x(a);
    }
}
