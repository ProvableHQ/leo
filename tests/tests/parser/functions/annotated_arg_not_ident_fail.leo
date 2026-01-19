
program test.aleo {}

@foo(?, bar, ?)
fn x() -> u8 {
    return 1u8;
}

@bar(123) // ints not vali
fn x() -> u8 {
    return 2u8;
}


@context // recovery witness
fn x() -> u8 {
    return 3u8;
}
