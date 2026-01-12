
function foo(a: u8, b: u8) -> () {
    assert_eq(a, b);
}


program test.aleo {
    transition main(a: u8, b: u8) -> u8 {
        foo(a, b);
        return a + b;
    }
}
