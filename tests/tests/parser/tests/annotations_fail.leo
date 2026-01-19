
program test.aleo {}

@some_annotation
fn foo1() -> u8 {
    return 1u8;
}

@some_annotation(some_key = "SomeString")
fn foo2() -> u8 {
    return 2u8;
}

@some_annotation(some_key = "SomeString", another_key = "AnotherString",)
fn foo3() -> u8 {
    return 3u8;
}

@failing_annotation(some_key = "SomeString", another_key = "AnotherString" oops)
fn foo4() -> u8 {
    return 4u8;
}

