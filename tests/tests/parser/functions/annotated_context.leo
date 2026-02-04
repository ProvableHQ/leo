
program test.aleo {}

@context
fn f() -> u8 {
    return 1u8;
}

@context // recovery witness
fn g() -> u8 {
    return 2u8;
}
