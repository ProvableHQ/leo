
program test.aleo {}

@test
fn foo(a: u8, b: u8) -> u8 {
    return a + b;
}

@program
fn bar(a: u8, b: u8) -> u8 {
    return a * b;
}
