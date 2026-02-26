
program test.aleo {
    fn foo(a: u8, b: u8) -> public u8 {
        return a + b;
    }

    // TODO: Need to test private modifier when it is supported.

    fn foo1(a: u8, b: u8) -> u8 {
        return a + b;
    }

    fn foo2(a: u8, public b: u8) -> public u8 {
        return a + b;
    }

}
