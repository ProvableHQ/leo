program test.aleo {}

@foo(?, bar, ?)
fn x() -> u8 {
    return 1u8;
}

@bar(123)
fn x() -> u8 {
    return 2u8;
}

@context
fn x() -> u8 {
    return 3u8;
}
