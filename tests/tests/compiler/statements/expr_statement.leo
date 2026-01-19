
fn foo(a: u8, b: u8) -> () {
    assert_eq(a, b);
}


program test.aleo {
    fn main(a: u8, b: u8) -> u8 {
        foo(a, b);
        return a + b;
    }
}
