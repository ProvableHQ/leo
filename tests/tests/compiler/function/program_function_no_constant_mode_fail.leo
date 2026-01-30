
program test.aleo {

    fn foo(constant a: u8) {}

    fn foo(a: u8) -> constant u8 {
        return a + a;
    }
}
